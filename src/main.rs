use clap::Parser;
use std::fs;

mod cli;
use cli::Cli;
mod index_builder;
use crate::index_builder::IndexBuilder;

fn main() {
    let cli = Cli::parse();
    let filenames = match fs::read_dir(&cli.files_dir) {
        Ok(read_dir) => read_dir
            .filter_map(|entry|
                entry.ok().and_then(|e|
                    Some(e.path()))),
        Err(e) => panic!("Error reading filenames from directory"),
    };

    println!("{:?} {} {}", cli.files_dir, cli.build_index, cli.start_server);

    let mut builder = IndexBuilder::new(8);
    for file in filenames {
        builder.proceed(file);
    }
    let index = builder.build();
    println!("{:?}", index);
}