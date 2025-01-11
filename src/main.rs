use self::server::create_server;

mod command;
mod database;
mod server;

#[tokio::main]
async fn main() {
    let add = "127.0.0.1:7635";
    eprintln!("Server will start at {:?}", add);
    //start server
    create_server(add).await;
}
