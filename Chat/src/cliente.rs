use std::net::{IpAddr, Ipv4Addr, SocketAddr};
let socket =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
fn to_socket_addrs(&self) -> Result<Self::Iter>
    let mut addrs_iter = "localhost:443".to_socket_addrs().unwrap();
pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpStream>
    let addrs = [
SocketAddr::from(([127, 0, 0, 1], 8080)),
SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];
if let Ok(stream) = TcpStream::connect(&addrs[..]) {
println!("Connectado al servidor!");
} else {
println!("No se pudo conectar...");
}
pub fn shutdown(&self, how: Shutdown) -> Result<()>
