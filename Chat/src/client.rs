use std::io::{self, Write, Read};
use std::net::{TcpStream, SocketAddr};
use std::thread;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ClientMessage {
    name: String,
    message: String,
}

fn main() -> io::Result<()> {
    let server_address = loop {
        let mut input = String::new();
        print!("Introduce la dirección IP y puerto del servidor (formato: IP:PUERTO): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
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

    let client_name = loop {
        let mut input = String::new();
        print!("Introduce tu nombre de cliente: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if !input.is_empty() {
            break input;
        } else {
            println!("Nombre inválido. Por favor, intenta de nuevo.");
        }
    };
    
    stream.write_all(client_name.as_bytes())?;

    loop {
        let mut input = String::new();
        println!("Escribe un mensaje: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let client_message = ClientMessage {
            name: client_name.clone(),
            message: input.trim().to_string(),
        };

        let json_message = serde_json::to_string(&client_message).unwrap();
        stream.write_all(json_message.as_bytes())?;
        println!("Mensaje enviado: {}", json_message);
    }
}
