use std::io::{self, stdin, stdout, Write, Read};
use std::net::{TcpStream, SocketAddr};
use std::thread;
use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct ClientMessage {
    id: u32,
    name: String,
    message: String,
}


fn main() -> io::Result<()> {
    let server_address = loop {
        let mut input = String::new();
        print!("Introduce la dirección IP y puerto del servidor (formato: IP:PUERTO): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.parse::<SocketAddr>().is_ok() {
            break input.to_string();
        } else {
            println!("Formato incorrecto. Por favor, intenta de nuevo.");
        }
    };

    let mut stream = TcpStream::connect(server_address.clone())?;
    println!("Conectado al servidor en {}", server_address);

    let mut stream_clone = stream.try_clone()?;
    thread::spawn(move || {
        loop {
            let mut buf = vec![0; 512];
            match stream_clone.read(&mut buf) {
                Ok(n) if n > 0 => {
                    println!("Mensaje del servidor: {}", String::from_utf8_lossy(&buf[..n]));
                }
                Ok(_) => break,
                Err(e) => {
                    eprintln!("Error al leer del servidor: {}", e);
                    break;
                }
            }
        }
    });

    let client_id = 1; // Puedes generar un ID único para cada cliente
    let client_name = "Cliente1".to_string(); // Nombre del cliente

    loop {
        let mut input = String::new();
        print!("Escribe un mensaje: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let client_message = ClientMessage {
            id: client_id,
            name: client_name.clone(),
            message: input.trim().to_string(),
        };

        let json_message = serde_json::to_string(&client_message).unwrap();
        stream.write_all(json_message.as_bytes())?;
        println!("Mensaje enviado: {}", json_message);
    }
}
