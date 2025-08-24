use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::MoveToColumn,
    execute,
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
    println!("Press Ctrl+C to quit. Type messages and press Enter to send.");

    let mut input_buffer = String::new();

    // input handling task
    let input_task = tokio::spawn(async move {
        loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    // Only handle key press to avoid duplicates from Release/Repeat
                    if let KeyEvent { kind: KeyEventKind::Press, .. } = key_event {
                        match key_event {
                            KeyEvent {
                                code: KeyCode::Char('c'),
                                modifiers: KeyModifiers::CONTROL,
                                ..
                            } => {
                                break;
                            }
                            KeyEvent {
                                code: KeyCode::Char(c),
                                ..
                            } => {
                                input_buffer.push(c);
                                print!("{}", c);
                                io::stdout().flush().unwrap();
                            }
                            KeyEvent {
                                code: KeyCode::Backspace,
                                ..
                            } => {
                                if !input_buffer.is_empty() {
                                    input_buffer.pop();
                                    print!("\x08 \x08"); // backspace, space, backspace
                                    io::stdout().flush().unwrap();
                                }
                            }
                            KeyEvent {
                                code: KeyCode::Enter,
                                ..
                            } => {
                                if !input_buffer.trim().is_empty() {
                                    // Clear current input line so only server broadcast shows the message
                                    let mut stdout = io::stdout();
                                    let _ = execute!(stdout, Clear(ClearType::CurrentLine), MoveToColumn(0));
                                    stdout.flush().unwrap();

                                    let message = format!("{}\n", input_buffer);
                                    if let Err(e) = writer.write_all(message.as_bytes()).await {
                                        eprintln!("Error sending message: {}", e);
                                        break;
                                    }
                                    input_buffer.clear();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    });
    
    // message receiving task
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
                    println!("{}", line.trim());
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