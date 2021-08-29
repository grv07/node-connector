// Provide functionality to create client node.
trait ClientBuilder {
    type IpAddr: std::net::SocketAddr;

    fn connect() {}

    fn close() {}

    fn listen() {}
}

// Provide functionality to create server node.
trait ServerBuilder {
    type IpAddr: std::net::SocketAddr;

    fn connect();
}
