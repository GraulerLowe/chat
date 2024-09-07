use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Conectado al servidor en 127.0.0.1:8080");

    loop {
        // Leer entrada del usuario
        let mut input = String::new();
        print!("Escribe un mensaje: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        // Enviar mensaje al servidor
        stream.write_all(input.as_bytes()).await?;
        println!("Mensaje enviado: {:?}", input.trim());

        // Leer respuesta del servidor
        let mut buf = [0; 1024];
        let n = stream.read(&mut buf).await?;
        println!("Respuesta del servidor: {:?}", String::from_utf8_lossy(&buf[..n]));
    }
}
