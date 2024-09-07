use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Servidor escuchando en 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Error al leer del socket: {:?}", e);
                        return;
                    }
                };

                // Mostrar el mensaje recibido
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("Mensaje recibido: {}", msg);

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Error al escribir en el socket: {:?}", e);
                    return;
                }
            }
        });
    }
}
