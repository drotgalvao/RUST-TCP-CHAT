use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::io;
use std::thread::JoinHandle;

fn main() -> io::Result<()> {
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    println!("Connecting to server: {}", server_address);
    let stream: TcpStream = TcpStream::connect(&server_address)?;

    println!("Connected to server: {}", server_address);

    let mut writer: BufWriter<&TcpStream> = BufWriter::new(&stream);

    println!("Connected to server. Enter your username:");
    let mut username: String = String::new();
    io::stdin().read_line(&mut username)?;

    let username: &str = username.trim();

    writer.write_all(format!("{}\n", username).as_bytes())?;
    writer.flush()?;

    let incoming_stream: TcpStream = stream.try_clone().expect("Failed to clone stream");
    let mut incoming_reader: BufReader<TcpStream> = BufReader::new(incoming_stream);

    let outgoing_stream: TcpStream = stream.try_clone().expect("Failed to clone stream");
    let mut outgoing_writer: BufWriter<TcpStream> = BufWriter::new(outgoing_stream);

    let receive_handle: JoinHandle<()>  = std::thread::spawn(move || {
        loop {
            let mut msg: String = String::new();
            match incoming_reader.read_line(&mut msg) {
                Ok(0) => {
                    println!("Server closed the connection");
                    break;
                }
                Ok(_) => {
                    print!("{}", msg);
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    let send_handle: JoinHandle<()> = std::thread::spawn(move || {
        loop {
            let mut msg: String = String::new();
            io::stdin().read_line(&mut msg).expect("Failed to read line");
            if msg.trim() == "quit" {
                break;
            }
            outgoing_writer.write_all(format!("{}\n", msg).as_bytes()).expect("Failed to write message");
            outgoing_writer.flush().expect("Failed to flush");
        }
    });
    

    receive_handle.join().expect("Receive thread panicked");
    send_handle.join().expect("Send thread panicked");

    Ok(())
}
