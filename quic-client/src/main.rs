use quinn::{Endpoint, Incoming};
use std::net::{SocketAddr};

fn client_addr() -> SocketAddr {
    "127.0.0.1:8091".parse::<SocketAddr>().unwrap()
}

fn server_addr() -> SocketAddr {
    "127.0.0.1:8092".parse::<SocketAddr>().unwrap()
}

async fn start_server() {
    let (server, mut incoming) = create_endpoint(&client_addr()).await;
    //println!("{:?}", server);
    while let Some(connecting) = incoming.next().await {
        println!("{:?}", connecting);
    } 
}

async fn start_client() {
    let (client, incoming) = create_endpoint(&client_addr()).await;
    //println!("{:?}", client);
    client.connect(&server_addr(), "localhost");
}

#[tokio::main]
async fn main() {
    start_server().await;
    start_client().await;
}

async fn create_endpoint(addr: &SocketAddr) -> (Endpoint, Incoming) {
    let result_endpoint = Endpoint::builder().bind(addr);
    let endpoint = result_endpoint.unwrap();
    endpoint
}
