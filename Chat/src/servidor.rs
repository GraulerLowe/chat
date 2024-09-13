use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use json::{ClientMessage, to_json, from_json};
mod json;

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<ClientMessage>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let message_str = String::from_utf8_lossy(&buffer[..n]).to_string();
                match from_json(&message_str) {
                    Ok(message) => tx.send(message).unwrap(),
                    Err(_) => eprintln!("Mensaje JSON inválido recibido: {}", message_str),
                }
            }
            Err(_) => break,
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("El servidor se inicia en el puerto 8080...");

    let (tx, rx) = mpsc::channel::<ClientMessage>();
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let clients_clone = Arc::clone(&clients);
    thread::spawn(move || {
        for message in rx {
            let mut clients = clients_clone.lock().unwrap();
            for client in clients.iter_mut() {
                match to_json(&message) {
                    Ok(json_message) => {
                        if let Err(e) = client.write_all(json_message.as_bytes()) {
                            eprintln!("Error al enviar el mensaje a un cliente: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error al convertir el mensaje a JSON: {}", e),
                }
            }
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nueva conexión: {:?}", stream.peer_addr());

                let clients = Arc::clone(&clients);
                clients.lock().unwrap().push(stream.try_clone().unwrap());
                let tx = tx.clone();
                thread::spawn(move || {
                    handle_client(stream, tx);
                });
            }
            Err(e) => {
                eprintln!("Error al aceptar la conexión: {}", e);
            }
        }
    }
    Ok(())
}
