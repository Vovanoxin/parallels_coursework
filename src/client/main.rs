use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;


use std::error::Error;
use std::io::{self, Write, BufRead};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut buffer = vec![0; 1024];
    loop {
        print!("Please enter your request\n");
        let user_input = prompt(">>");

        stream.write(user_input.as_bytes()).await.unwrap();
        let n = stream.read(&mut buffer).await.unwrap();
        let response = std::str::from_utf8(&buffer[0..n]).unwrap().to_string();
        println!("{response}");
    }

    Ok(())
}

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{name}");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line)
                    .expect("Could not read from stdin");
    line.trim().to_string()
}