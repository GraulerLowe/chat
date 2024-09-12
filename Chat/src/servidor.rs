use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<String>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..n]).to_string();
                tx.send(message).unwrap();
            }
            Err(_) => break,
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("192.168.100.8:8080")?;
    println!("El servidor se inicia en el puerto 8080...");

    let (tx, rx) = mpsc::channel::<String>();
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let clients_clone = Arc::clone(&clients);
    thread::spawn(move || {
        for message in rx {
            let clients = clients_clone.lock().unwrap();
            for mut client in clients.iter() {
                client.write_all(message.as_bytes()).unwrap();
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
