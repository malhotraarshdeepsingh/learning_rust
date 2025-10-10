use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:8080";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking");

    let (tx, rx) = mpsc::channel::<String>();
    let tx_clone = tx.clone();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = String::from_utf8_lossy(&buff)
                    .trim_matches(char::from(0))
                    .to_string();
                println!("Message received: {}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(err) => {
                eprintln!("Connection error: {}", err);
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("Message sent: {}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Type a message and press Enter to send it:");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_string();
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting chat...");
            break;
        }
        if !input.is_empty() {
            tx_clone.send(input).expect("Failed to send message");
        }
    }
}
