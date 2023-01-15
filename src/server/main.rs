extern crate serde;
extern crate serde_json;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::error::Error;

use clap::Parser;
use walkdir::WalkDir;

mod cli;
use cli::Cli;
mod index_builder;
use crate::index_builder::IndexBuilder;

mod server;
use crate::server::IndexServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("{:?} {} {}", cli.files_dir, cli.build_index, cli.start_server);
    let index =
        if cli.build_index {
            let mut builder = IndexBuilder::new(cli.thread_number);

            for entry in WalkDir::new(cli.files_dir) {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_file() {
                    builder.proceed(path.to_path_buf());
                }
            }
            let index = builder.build();
            if cli.index_path.is_some() {
                let serialized = serde_json::to_string(&index).unwrap();
                std::fs::write(cli.index_path.unwrap(), serialized).unwrap();
            }
            index
        } else {
            let serialized =
                std::fs::read_to_string(cli.index_path.unwrap()).unwrap();
            serde_json::from_str(&serialized).unwrap()
        };
    if cli.start_server {
        let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        IndexServer::new(index, socket_addr).start().await?;
    }
    Ok(())
}