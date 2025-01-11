use crate::database::Database;

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
        _ => Ok("-ERR\r\nUnknow commnd, please try again!!".to_string()),
    }
}
