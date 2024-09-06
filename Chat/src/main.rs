use std::net::{SocketAddr, TcpListener};

let addrs = [
    SocketAddr::from(([127, 0, 0, 1], 80)),
    SocketAddr::from(([127, 0, 0, 1], 443)),
];
let listener = TcpListener::bind(&addrs[..]).unwrap();
