use std::net::SocketAddr;
use std::{collections::{HashMap, HashSet}, path::{PathBuf}};

use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use std::sync::Arc;

pub struct IndexServer {
    index: Arc<HashMap<String, HashSet<PathBuf>>>,
    addr: SocketAddr,
}

impl IndexServer {
    pub fn new(index: HashMap<String, HashSet<PathBuf>>,
            addr: SocketAddr) -> IndexServer {
        IndexServer {
            index: Arc::new(index),
            addr,
        }
    }
    pub async fn start(self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.addr).await.unwrap();

        loop {
            let (stream, _addr) = listener.accept().await?;
            let index = self.index.clone();
            tokio::spawn(async move {
                if let Err(_e) = process_connection(stream, index).await {
                    println!("Error processing connection\n");
                }
            });
        }
    }
}

async fn process_connection(mut stream: TcpStream, index: Arc<HashMap<String, HashSet<PathBuf>>>) -> Result<(), Box<dyn Error>> {
    let (mut read, mut write) = stream.split();
    let mut buffer = vec![0; 1024];
    loop {
        let n = read.read(&mut buffer).await?;
        if n == 0 {
            return Ok(());
        }
        let request = std::str::from_utf8(&buffer[0..n]).unwrap().to_string();
        let words: HashSet<&str>  = request.split(&[' ', '\n'][..]).collect();

        let mut response = String::new();
        for word in words {
            if !index.contains_key(word) {
                response.push_str(&format!("No results for {word}\n"));
                continue;
            }
            let search_result: Vec<String> = index[word].iter().map(|x| x.as_path().display().to_string()).collect();
            response.push_str(&format!("Results for {word} are: "));
            response.push_str(&search_result.join(", "));
            response.push_str("\n");
        }
        println!("response is: {response}");
        write.write(response.as_bytes()).await.unwrap();
        println!("response was written");
        ()
    }
}