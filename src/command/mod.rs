use std::usize;

use crate::database::db::Database;

pub async fn command_parser(db: &Database, command: &str) -> Result<String, String> {
    let splitted_commad: Vec<&str> = command.split_whitespace().collect();

    match splitted_commad.as_slice() {
        ["SET", key, value, "EXP", ttl] => {
            let ttl = ttl.parse::<u64>().expect("Invalid time to live value");
            db.set(key.to_string(), value.to_string(), Some(ttl)).await;
            Ok("+OK\r\n".to_string())
        }
        ["SET", key, value] => {
            db.set(key.to_string(), value.to_string(), None).await;
            Ok("+OK\r\n".to_string())
        }
        ["GET", key] => {
            if let Some(value) = db.get(key).await {
                Ok(format!("${}\r\n{}\r\n", value.len(), value))
            } else {
                Ok("$-1\r\n".to_string())
            }
        }
        ["DEL", key] => {
            if db.delete(key).await {
                Ok("$-1\r\n".to_string())
            } else {
                Ok("$-0\r\n".to_string())
            }
        }
        //LIST
        ["LPUSH", key, value] => {
            db.lpush(key.to_string(), value.to_string()).await;
            Ok("+OK\r\n".to_string())
        }
        ["RPUSH", key, value] => {
            db.rpush(key.to_string(), value.to_string()).await;
            Ok("+OK\r\n".to_string())
        }
        ["LPOP", key] => {
            if let Some(value) = db.lpop(key).await {
                Ok(format!("${}\r\n{}\r\n", value.len(), value))
            } else {
                Ok("+OK\r\n".to_string())
            }
        }
        ["RPOP", key] => {
            if let Some(value) = db.rpop(key).await {
                Ok(format!("${}\r\n{}\r\n", value.len(), value))
            } else {
                Ok("+OK\r\n".to_string())
            }
        }
        ["LRANGE", start, end, key] => {
            let start = start.parse::<usize>().map_err(|_| "Invalid start index")?;
            let end = end.parse::<usize>().map_err(|_| "Invalid start index")?;

            if let Some(list) = db.lrange(start, end, key).await {
                let mut response = format!("*{}\r\n", list.len());
                for item in list {
                    response.push_str(&format!("${}\r\n{}\r\n", item.len(), item))
                }
                Ok(response)
            } else {
                Ok("NOT FOUND\r\n".to_string())
            }
        }
        //SET
        ["SADD", key, value] => {
            let add = db.sadd(key.to_string(), value.to_string()).await;
            Ok(format!("${}\r\n", if add { 1 } else { 0 }))
        }

        ["SREM", key, value] => {
            let add = db.srem(key, value).await;
            Ok(format!("${}\r\n", if add { 1 } else { 0 }))
        }

        ["SMEMBERS", key] => {
            if let Some(list) = db.smembers(key).await {
                let mut response = format!("*{}\r\n", list.len());
                for item in list {
                    response.push_str(&format!("${}\r\n{}\r\n", item.len(), item))
                }
                Ok(response)
            } else {
                Ok("NOT FOUND\r\n".to_string())
            }
        }

        ["SISMEMBER", key, member] => {
            let ismember = db.sismember(key, member).await;
            Ok(format!(
                "${}\r\n",
                if ismember { "FOUND!" } else { "NOT FOUND!" }
            ))
        }

        //Ordered SET
        ["ZADD", score, member, key] => {
            let score = score.parse::<f64>().map_err(|_| "Invalid score")?;
            let added = db.zadd(key.to_string(), score, member.to_string()).await;
            Ok(format!(":{}\r\n", if added { 1 } else { 0 }))
        }

        ["ZREM", key, member] => {
            let removed = db.zrem(&key.to_string(), member.to_string()).await;
            Ok(format!(":{}\r\n", if removed { 1 } else { 0 }))
        }
        ["ZRANGE", key, start, end] => {
            let start = start.parse::<usize>().map_err(|_| "Invalid start index")?;
            let end = end.parse::<usize>().map_err(|_| "Invalid end index")?;
            if let Some(members) = db.zrange(&key.to_string(), start, end).await {
                let mut response = format!("*{}\r\n", members.len());
                for member in members {
                    response.push_str(&format!("${}\r\n{}\r\n", member.len(), member));
                }
                Ok(response)
            } else {
                Ok("$-1\r\n".to_string())
            }
        }
        ["ZSCORE", key, member] => {
            if let Some(score) = db.zscore(key, member).await {
                let score_str = score.to_string();
                Ok(format!("${}\r\n{}\r\n", score_str.len(), score_str))
            } else {
                Ok("$-1\r\n".to_string())
            }
        }

        _ => Ok("-ERR\r\nUnknow commnd, please try again!!".to_string()),
    }
}
