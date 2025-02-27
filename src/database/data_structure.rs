use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use ordered_float::OrderedFloat;

pub struct RList {
    pub list: VecDeque<String>,
}

impl RList {
    pub fn new() -> Self {
        RList {
            list: VecDeque::new(),
        }
    }

    //LPUSH, LPOP, RPUSH, RPOP
    pub fn lpush(&mut self, value: String) {
        self.list.push_front(value);
    }

    pub fn lpop(&mut self) -> Option<String> {
        self.list.pop_front()
    }
    pub fn rpush(&mut self, value: String) {
        self.list.push_back(value);
    }
    pub fn rpop(&mut self) -> Option<String> {
        self.list.pop_back()
    }

    pub fn lrange(&mut self, start: usize, end: usize) -> Vec<String> {
        self.list
            .iter()
            .skip(start)
            .take(end - start + 1)
            .cloned()
            .collect()
    }
}

pub struct RSets {
    pub set: HashSet<String>,
}

impl RSets {
    pub fn new() -> Self {
        RSets {
            set: HashSet::new(),
        }
    }
    //SADD, SREM, SMEMBERS, SISMEMBER
    pub fn sadd(&mut self, value: String) -> bool {
        self.set.insert(value)
    }

    pub fn srem(&mut self, value: &str) -> bool {
        self.set.remove(value)
    }

    pub fn smembers(&mut self) -> Vec<String> {
        self.set.iter().cloned().collect()
    }

    pub fn ismember(&mut self, key: &str) -> bool {
        self.set.contains(key)
    }
}

//Sorted sets or ordered sets
#[derive(Clone, Eq)]
pub struct SortedMembers {
    pub member: String,
    pub score: OrderedFloat<f64>,
}

impl PartialEq for SortedMembers {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.member == other.member
    }
}

impl Ord for SortedMembers {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .partial_cmp(&other.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.member.cmp(&other.member))
    }
}

impl PartialOrd for SortedMembers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct RSortedSet {
    pub members: HashMap<String, OrderedFloat<f64>>,
    pub sorted: BTreeSet<SortedMembers>,
}

impl RSortedSet {
    //ZADD, ZREM, ZRANGE
    pub fn new() -> Self {
        RSortedSet {
            members: HashMap::new(),
            sorted: BTreeSet::new(),
        }
    }

    pub fn zadd(&mut self, score: f64, member: String) -> bool {
        let ordered_score = OrderedFloat(score);
        if let Some(&old_score) = self.members.get(&member) {
            if old_score == ordered_score {
                return false;
            }
            self.sorted.remove(&SortedMembers {
                member: member.clone(),
                score: old_score,
            });
        }
        let new_entry = SortedMembers {
            member: member.clone(),
            score: ordered_score,
        };
        let inserted = self.sorted.insert(new_entry);
        self.members.insert(member, ordered_score);
        inserted
    }

    pub fn zrem(&mut self, member: String) -> bool {
        if let Some(score) = self.members.remove(&member) {
            self.sorted.remove(&SortedMembers { member, score });
            true
        } else {
            false
        }
    }

    pub fn zrange(&self, start: usize, end: usize) -> Vec<String> {
        self.sorted
            .iter()
            .skip(start)
            .take(end - start + 1)
            .map(|sorted_mem| sorted_mem.member.clone())
            .collect()
    }

    pub fn zscore(&self, member: String) -> Option<f64> {
        self.members.get(&member).map(|score| score.0)
    }
}
