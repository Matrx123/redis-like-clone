use std::collections::HashMap;
use std::sync::Arc;
use std::usize;

use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

use super::data_structure::{RList, RSets, RSortedSet};

#[derive(Clone)]
pub struct Database {
    db: Arc<RwLock<HashMap<String, String>>>,
    expiry: Arc<RwLock<HashMap<String, Instant>>>,
    list: Arc<RwLock<HashMap<String, RList>>>,
    set: Arc<RwLock<HashMap<String, RSets>>>,
    sorted_set: Arc<RwLock<HashMap<String, RSortedSet>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            db: Arc::new(RwLock::new(HashMap::new())),
            expiry: Arc::new(RwLock::new(HashMap::new())),
            list: Arc::new(RwLock::new(HashMap::new())),
            set: Arc::new(RwLock::new(HashMap::new())),
            sorted_set: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, key: String, value: String, ttl: Option<u64>) {
        let mut db_map = self.db.write().await;
        db_map.insert(key.clone(), value);

        if let Some(sec) = ttl {
            let exp_time = Instant::now() + Duration::from_secs(sec);
            let mut exp_map = self.expiry.write().await;
            exp_map.insert(key, exp_time);
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        if self.is_expired(key).await {
            self.delete(key).await;
            return None;
        }
        let db_map = self.db.read().await;
        db_map.get(key).cloned()
    }

    pub async fn is_expired(&self, key: &str) -> bool {
        let exp_map = self.expiry.read().await;
        if let Some(&exp_time) = exp_map.get(key) {
            return Instant::now() > exp_time;
        }

        false
    }
    pub async fn delete(&self, key: &str) -> bool {
        let mut db_map = self.db.write().await;
        let mut exp_map = self.expiry.write().await;
        exp_map.remove(key);
        db_map.remove(key).is_some()
    }
    //LIST
    pub async fn lpush(&self, key: String, value: String) {
        let mut list_map = self.list.write().await;
        let list = list_map.entry(key).or_insert_with(RList::new);
        list.lpush(value);
    }

    pub async fn rpush(&self, key: String, value: String) {
        let mut list_map = self.list.write().await;
        let list = list_map.entry(key).or_insert_with(RList::new);
        list.rpush(value);
    }

    pub async fn lpop(&self, key: &str) -> Option<String> {
        let mut list_map = self.list.write().await;
        if let Some(list) = list_map.get_mut(key) {
            list.lpop()
        } else {
            None
        }
    }

    pub async fn rpop(&self, key: &str) -> Option<String> {
        let mut list_map = self.list.write().await;
        if let Some(list) = list_map.get_mut(key) {
            list.rpop()
        } else {
            None
        }
    }

    pub async fn lrange(&self, start: usize, end: usize, key: &str) -> Option<Vec<String>> {
        let mut list_map = self.list.write().await;
        if let Some(list) = list_map.get_mut(key) {
            Some(list.lrange(start, end))
        } else {
            None
        }
    }
    //SET
    pub async fn sadd(&self, key: String, value: String) -> bool {
        let mut set_map = self.set.write().await;
        let set = set_map.entry(key).or_insert_with(RSets::new);
        set.sadd(value)
    }

    pub async fn srem(&self, key: &str, value: &str) -> bool {
        let mut set_map = self.set.write().await;
        if let Some(set) = set_map.get_mut(key) {
            set.srem(value)
        } else {
            false
        }
    }

    pub async fn smembers(&self, key: &str) -> Option<Vec<String>> {
        let mut set_map = self.set.write().await;
        if let Some(set) = set_map.get_mut(key) {
            Some(set.smembers())
        } else {
            None
        }
    }

    pub async fn sismember(&self, key: &str, value: &str) -> bool {
        let mut set_map = self.set.write().await;
        if let Some(set) = set_map.get_mut(key) {
            set.ismember(value)
        } else {
            false
        }
    }

    //SORTED SETS
    pub async fn zadd(&self, key: String, score: f64, member: String) -> bool {
        let mut ss_map = self.sorted_set.write().await;
        let sorted_set = ss_map.entry(key).or_insert_with(RSortedSet::new);
        sorted_set.zadd(score, member)
    }

    pub async fn zrem(&self, key: &String, member: String) -> bool {
        let mut ss_map = self.sorted_set.write().await;
        if let Some(sorted_set) = ss_map.get_mut(key) {
            sorted_set.zrem(member)
        } else {
            false
        }
    }

    pub async fn zrange(&self, key: &String, start: usize, end: usize) -> Option<Vec<String>> {
        let ss_map = self.sorted_set.read().await;
        if let Some(sorted_set) = ss_map.get(key) {
            Some(sorted_set.zrange(start, end))
        } else {
            None
        }
    }

    pub async fn zscore(&self, key: &str, member: &str) -> Option<f64> {
        let ss_map = self.sorted_set.read().await;
        if let Some(sorted_set) = ss_map.get(key) {
            sorted_set.zscore(member.to_string())
        } else {
            None
        }
    }
}
