
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<(usize, String)>, id: usize) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..n]).to_string();
                tx.send((id, message)).unwrap();
            }
            Err(_) => break,
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("192.168.100.8:8080")?;
    println!("El servidor se inicia en el puerto 8080...");

    let (tx, rx) = mpsc::channel::<(usize, String)>();
    let clients: Arc<Mutex<Vec<(usize, TcpStream)>>> = Arc::new(Mutex::new(Vec::new()));
    let client_id = Arc::new(Mutex::new(0));

    let clients_clone = Arc::clone(&clients);
    thread::spawn(move || {
        for (id, message) in rx {
            let mut clients = clients_clone.lock().unwrap();
            for (client_id, client) in clients.iter_mut() {
                if *client_id != id {
                    client.write_all(message.as_bytes()).unwrap();
                }
            }
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut id = client_id.lock().unwrap();
                *id += 1;
                let current_id = *id;
                println!("Nueva conexión: {:?}, ID: {}", stream.peer_addr(), current_id);

                let clients = Arc::clone(&clients);
                clients.lock().unwrap().push((current_id, stream.try_clone().unwrap()));
                let tx = tx.clone();
                thread::spawn(move || {
                    handle_client(stream, tx, current_id);
                });
            }
            Err(e) => {
                eprintln!("Error al aceptar la conexión: {}", e);
            }
        }
    }
    Ok(())
}

