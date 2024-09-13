use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use json::{ClientMessage, to_json, from_json};
mod json;

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<ClientMessage>, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
    let mut buffer = [0; 512];

    // Solicitar nombre del cliente y verificar si ya está en uso
    let client_name = loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    return; // El cliente se desconectó antes de enviar un nombre
                }

                let name = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                let mut clients_lock = clients.lock().unwrap();

                if clients_lock.contains_key(&name) {
                    // Nombre ya está en uso, notificar al cliente
                    let error_message = "Nombre en uso. Por favor, elige otro nombre.\n";
                    stream.write_all(error_message.as_bytes()).unwrap();
                } else {
                    // Nombre válido, agregar al HashMap
                    clients_lock.insert(name.clone(), stream.try_clone().unwrap());
                    println!("Nuevo cliente: {}", name);

                    // Enviar mensaje de confirmación al cliente
                    let confirmation_message = "Nombre aceptado.\n";
                    stream.write_all(confirmation_message.as_bytes()).unwrap();
                    break name; // Salir del loop con el nombre válido
                }
            }
            Err(_) => return, // Error al leer, terminar la conexión
        }
    };

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break; // El cliente se desconectó
                }

                let message_str = String::from_utf8_lossy(&buffer[..n]).to_string();
                match from_json(&message_str) {
                    Ok(mut message) => {
                        message.name = client_name.clone(); // Asegurar que el mensaje tenga el nombre correcto
                        if let Err(e) = tx.send(message) {
                            eprintln!("Error enviando mensaje al canal: {}", e);
                        }
                    }
                    Err(_) => eprintln!("Mensaje JSON inválido recibido: {}", message_str),
                }
            }
            Err(_) => break, // Error al leer, terminar la conexión
        }
    }

    // Eliminar al cliente del HashMap al desconectarse
    let mut clients_lock = clients.lock().unwrap();
    clients_lock.remove(&client_name);
    println!("Cliente desconectado: {}", client_name);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("El servidor se inicia en el puerto 8080...");

    let (tx, rx) = mpsc::channel::<ClientMessage>();
    let clients: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    let clients_clone = Arc::clone(&clients);
    thread::spawn(move || {
        for message in rx {
            let clients = clients_clone.lock().unwrap();

            // Enviar mensaje a todos los clientes conectados excepto al remitente
            for (name, mut client) in clients.iter() {
                if *name != message.name {
                    match to_json(&message) {
                        Ok(json_message) => {
                            if let Err(e) = client.write_all(json_message.as_bytes()) {
                                eprintln!("Error al enviar el mensaje a {}: {}", name, e);
                            }
                        }
                        Err(e) => eprintln!("Error al convertir el mensaje a JSON: {}", e),
                    }
                }
            }
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nueva conexión desde: {:?}", stream.peer_addr());

                let clients = Arc::clone(&clients);
                let tx = tx.clone();
                thread::spawn(move || {
                    handle_client(stream, tx, clients);
                });
            }
            Err(e) => {
                eprintln!("Error al aceptar la conexión: {}", e);
            }
        }
    }
    Ok(())
}
