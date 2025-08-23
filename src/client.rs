use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到服务器
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server at 127.0.0.1:8080");
    
    // 分离读写端
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    
    // 启动接收消息的任务
    let receive_task = tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    println!("Server disconnected");
                    break;
                }
                Ok(_) => {
                    print!("{}", line);
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });
    
    // 发送一条测试消息
    let test_msg = "Hello from client!\n";
    writer.write_all(test_msg.as_bytes()).await?;
    
    // 等待接收任务完成（或者保持连接）
    println!("Client is running. Press Ctrl+C to exit.");
    
    // 简单的保持连接，等待用户中断
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    
    println!("Client shutting down...");
    Ok(())
}