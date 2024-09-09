use std :: {TcpListener, TcpStream};
use std ::io::{Read, Write};
use std::thread;

fn handle_client (mut stream: TcpStream){
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer){
            ok(n) => {
                if n == 0 {
                    break;
                }
                if let Err(_) = stream.write_all(&buffer[..n]){
                    break;
                }
                Err(_) => break
            }
        }
    }
}

fn main() -> std::io::Result <()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("El servidor esta escuchando en el puerto 8080...");
    for stream in listener.incoming(){
        match stream {
            Ok(stream) =>{
                println!("Nueva conexion: {:?}", stream.peer_addr());
                thread::spawn(move ||{
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error al aceptar la conexion: {}", e);
            }
        }
    }
    Ok(())
    
}
