use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Constants for server configuration and message size
const LOCAL: &str = "127.0.0.1:8080";
const MSG_SIZE: usize = 32;

fn main() {
    // Bind the TCP listener to the specified address
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    println!("Server listening on {}", LOCAL);

    // Set the server to non-blocking mode
    // server
    //     .set_nonblocking(true)
    //     .expect("failed to initialize non-blocking");
    // println!("Server running in non-blocking mode");

    // Shared list of connected clients
    let clients = Arc::new(Mutex::new(Vec::<std::net::TcpStream>::new()));
    let clients_ref = Arc::clone(&clients);

    // Channel for inter-thread communication
    let (tx, rx) = mpsc::channel::<(std::net::SocketAddr, String)>();

    // Thread to handle incoming messages and broadcast them
    thread::spawn(move || {
        for (sender_addr, msg) in rx {
            let mut clients = clients_ref.lock().unwrap();
            clients.retain(|mut client| {
                if client.peer_addr().unwrap() == sender_addr {
                    // Skip sending to sender
                    return true;
                }
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                match client.write_all(&buff) {
                    Ok(_) => true,
                    Err(e) if e.kind() == ErrorKind::BrokenPipe => false,
                    Err(e) => {
                        eprintln!("Error sending message: {}", e);
                        false
                    }
                }
            });
        }
    });

    // Accept incoming connections
    for stream in server.incoming() {
        match stream {
            Ok(mut stream) => {
                let addr = stream.peer_addr().unwrap();
                println!("New client connected: {}", addr);

                // Add client to shared list
                clients.lock().unwrap().push(stream.try_clone().unwrap());

                let tx = tx.clone();
                let mut buff = vec![0; MSG_SIZE];

                // Handle this client
                thread::spawn(move || loop {
                    match stream.read_exact(&mut buff) {
                        Ok(_) => {
                            let msg = String::from_utf8_lossy(&buff)
                                .trim_matches(char::from(0))
                                .to_string();
                            if !msg.is_empty() {
                                println!("Received from {}: {}", addr, msg);
                                tx.send((addr, msg)).unwrap();
                            }
                        }
                        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                            println!("Client disconnected");
                            break;
                        }
                        Err(e) => {
                            eprintln!("Read error: {}", e);
                            break;
                        }
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
