// Provide functionality to create client node.
trait Client{
  type IpAddr: std::net::SocketAddr;  

  pub fn connect(){}

  pub fn close(){}

  pub fn listen(){}


}


// Provide functionality to create server node.
trait Server{
   type IpAddr: std::net::SocketAddr;  
    
   pub fn connect();
}
