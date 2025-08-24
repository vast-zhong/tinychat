use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // connect to server
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server at 127.0.0.1:8080");
    
    // split the stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    enable_raw_mode()?;

    let input_task = tokio::spawn(async move {
        loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    match key_event {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } => {
                            println!("\r\nquit...");
                            break;
                        }
                        _ => {
                            // println!("检测到按键: {:?}", key_event);
                        }
                    }
                }
            }
        }
    });
    
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

    tokio::select! {
        _ = receive_task => {},
        _ = input_task => {},
    }
    
    disable_raw_mode()?;

    println!("\nClient shutting down...");
    Ok(())
}