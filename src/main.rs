use clap::Parser;
use walkdir::WalkDir;

mod cli;
use cli::Cli;
mod index_builder;
use crate::index_builder::IndexBuilder;

fn main() {
    let cli = Cli::parse();

    println!("{:?} {} {}", cli.files_dir, cli.build_index, cli.start_server);

    let mut builder = IndexBuilder::new(8);

    for entry in WalkDir::new(cli.files_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            builder.proceed(path.to_path_buf());
        }
    }

    let index = builder.build();
    //println!("{:?}", index);
}