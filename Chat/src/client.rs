use std::io::{self, Write, Read};
use std::net::{TcpStream, SocketAddr};
use std::thread;
use json::{ClientMessage, to_json, from_json};
use uuid::Uuid;  // Para generar un identificador único
mod json;

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
    let client_id = Uuid::new_v4();  // Generar un identificador único para el cliente
    println!("ID de cliente: {}", client_id);

    // Hilo para escuchar mensajes del servidor
    thread::spawn(move || {
        loop {
            let mut buf = vec![0; 512];
            match stream_clone.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let received = String::from_utf8_lossy(&buf[..n]);
                    match from_json(&received) {
                        Ok(message) => {
                            // Solo imprimir si el mensaje proviene de otro cliente
                            if message.id != client_id.to_string() {
                                println!("{}: {}", message.name, message.message);
                            }
                        }
                        Err(_) => println!("Mensaje recibido no es JSON válido"),
                    }
                }
                Ok(_) => break,
                Err(e) => {
                    eprintln!("Error al leer del servidor: {}", e);
                    break;
                }
            }
        }
    });

    // Solicitar y validar nombre de cliente con el servidor
    let client_name = loop {
        let mut input = String::new();
        print!("Introduce tu nombre de cliente: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        // Enviar el nombre del cliente al servidor
        stream.write_all(input.as_bytes())?;

        // Esperar la respuesta del servidor
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(n) => {
                let response = String::from_utf8_lossy(&buffer[..n]);
                if response.contains("Nombre en uso") {
                    println!("{}", response); // Mostrar mensaje de error y solicitar un nuevo nombre
                } else {
                    println!("Nombre aceptado por el servidor.");
                    break input; // Nombre aceptado, salir del loop
                }
            }
            Err(e) => {
                eprintln!("Error al leer la respuesta del servidor: {}", e);
                return Err(e); // Manejo de error si la conexión falla
            }
        }
    };

    // Bucle principal de envío de mensajes
    loop {
        let mut input = String::new();
        println!("Escribe un mensaje: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let client_message = ClientMessage {
            id: client_id.to_string(),  // Incluir el ID del cliente en el mensaje
            name: client_name.clone(),
            message: input.trim().to_string(),
        };

        // Enviar el mensaje al servidor
        match to_json(&client_message) {
            Ok(json_message) => {
                stream.write_all(json_message.as_bytes())?;
                println!("Mensaje enviado");
            }
            Err(e) => eprintln!("Error al convertir el mensaje a JSON: {}", e),
        }
    }
}
