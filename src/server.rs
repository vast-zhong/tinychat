// create a new tcp listener and bind it to 127.0.0.1:8080
use tokio::net::{TcpListener, TcpStream};   
// store the client id and the client writer
use std::collections::HashMap;              
// Arc is a thread-safe reference-counted pointer, 
// it can be cloned and shared between multiple threads
use std::sync::Arc;
// broadcast is a channel that can be used to send messages to multiple receivers
use tokio::sync::{broadcast, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

// define a type ClientId == unsized 32
type ClientId = u32;
// create a collection to store the client id and the client writer
type Clients = Arc<Mutex<HashMap<ClientId, tokio::net::tcp::OwnedWriteHalf>>>;

// change main to async
#[tokio::main]
// if success return (), else return Box<dyn std::error::Error>
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new tcp listener and bind it to 127.0.0.1:8080
    // await is async wait
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("server run on 127.0.0.1:8080");

    // broadcast channel
    let (tx, _rx) = broadcast::channel::<String>(100);
    
    // store all connected clients
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let client_counter = Arc::new(Mutex::new(0u32));

    // main loop
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

