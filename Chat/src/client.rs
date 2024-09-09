use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::io::{stdin, stdout, Write};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_address = loop {
        let mut input = String::new();
        print!("Introduce la dirección IP y puerto del servidor (formato: IP:PUERTO): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Verificar si la entrada es una dirección válida
        if input.parse::<SocketAddr>().is_ok() {
            break input.to_string();
        } else {
            println!("Formato incorrecto. Por favor, intenta de nuevo.");
        }
    };

    // Clonar la dirección del servidor antes de moverla
    let mut stream = TcpStream::connect(server_address.clone()).await?;
    println!("Conectado al servidor en {}", server_address);

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
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        println!("Respuesta del servidor: {:?}", String::from_utf8_lossy(&buf[..n]));
    }
}
