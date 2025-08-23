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
        let (socket, addr) = listener.accept().await?;
        println!("new client connected: {:?}", addr);

        let tx_clone = tx.clone();
        // only increments the reference count, no data is copied 
        let clients_clone = clients.clone();
        // share one counter
        let counter_clone = client_counter.clone();

        // create independent async process task for each client
        // the code in {} become a async code block, compile into a Future
        tokio::spawn(async move {
            // Err(e) is the return value of handle_client
            if let Err(e) = handle_client(socket, tx_clone, clients_clone, counter_clone).await {
                eprintln!("error: {}", e);
            }
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    tx_clone: broadcast::Sender<String>,
    clients_clone: Clients,
    counter_clone: Arc<Mutex<u32>>,
) -> Result<(), Box<dyn std::error::Error>> {

    // split the socket into reader and writer
    // TcpStream.into_split() a bidirectional TCP connection into two ends:
    // reader: OwnedReadHalf
    // writer: OwnedWriteHalf
    let (reader, writer) = socket.into_split();
    // use BufReader to read the data from the reader
    let mut reader = BufReader::new(reader);
    
    // get the client id 
    let client_id = {
        let mut counter_guard = counter_clone.lock().await;
        *counter_guard += 1;
        *counter_guard
    };
    
    {
        let mut clients_guard = clients_clone.lock().await;
        clients_guard.insert(client_id, writer);
    }
    
    println!("client {} connected", client_id);
    
    let join_msg = format!("system meaasge: client {} join the chat\n", client_id);
    let _ = tx_clone.send(join_msg);

    let mut rx = tx_clone.subscribe();
    let clients_for_broadcast = clients_clone.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let mut clients_guard = clients_for_broadcast.lock().await;
            let mut to_remove = Vec::new();
            
            for (id, writer) in clients_guard.iter_mut() {
                if let Err(_) = writer.write_all(msg.as_bytes()).await {
                    to_remove.push(*id);
                }
            }
            
            for id in to_remove {
                clients_guard.remove(&id);
                println!("client {} disconnect", id);
            }
        }
    });

    // {
    //     let mut clients_guard = clients_clone.lock().await;
    //     clients_guard.remove(&client_id);
    // }
    
    // let leave_msg = format!("系统消息: 客户端 {} 离开了聊天室\n", client_id);
    // let _ = tx.send(leave_msg);
    
    // println!("客户端 {} 断开连接", client_id);


    Ok(())
}
