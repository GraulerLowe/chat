use std::io::{self, stdin, stdout, Write, Read};
use std::net::{TcpStream, SocketAddr};
use std::thread;

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

    loop {
        let mut input = String::new();
        print!("Escribe un mensaje: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        stream.write_all(input.as_bytes())?;
        println!("Mensaje enviado: {}", input.trim());
    }
}
