use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // connect to server
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server at 127.0.0.1:8080");
    println!("Listening for messages from server. Press Ctrl+C to exit.\n");
    
    // split the stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    
    // 发送测试消息
    // let test_message = "test\n";
    // writer.write_all(test_message.as_bytes()).await?;
    // println!("Test message sent to server.");
    
    // read the data from server
    let receive_task = tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    println!("\nServer disconnected");
                    break;
                }
                Ok(_) => {
                    print!("{}", line);
                }
                Err(e) => {
                    eprintln!("\nError reading from server: {}", e);
                    break;
                }
            }
        }
    });
    
    // 等待接收任务完成
    receive_task.await.unwrap();
    println!("\nClient shutting down...");
    Ok(())
}