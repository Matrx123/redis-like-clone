use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::spawn;

use crate::command::command_parser;
use crate::database::db::Database;

pub async fn create_server(add: &str) {
    let listner = TcpListener::bind(add).await.unwrap();
    let db = Database::new();

    loop {
        //accept incoming connection
        let (socket, _) = listner.accept().await.unwrap();
        let db = db.clone();

        spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut buffer = String::new();

            while reader.read_line(&mut buffer).await.unwrap() > 0 {
                let response = command_parser(&db, buffer.trim())
                    .await
                    .unwrap_or_else(|e| format!("-ERR {}\r\n", e));

                writer.write_all(response.as_bytes()).await.unwrap();
                buffer.clear();
            }
        });
    }
}
