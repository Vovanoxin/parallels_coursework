use std::path::PathBuf;

use clap::{Parser, ArgGroup};

// --build_index --files_dir --index_path --start_server
// якщо вказано і --build_index можна щоб --index_path була відсутня
// - тоді структура створюється в памʼяті.

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("mode").required(true).multiple(true).args(["build_index", "start_server"])))]
pub struct Cli {
    #[arg(long, required = true)]
    pub files_dir: PathBuf,

    #[arg(long, required_unless_present="build_index")]
    pub index_path: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    pub build_index: bool,

    #[arg(long, default_value_t = false)]
    pub start_server: bool,

    #[arg(long, default_value_t = 1)]
    pub thread_number: usize,

    #[arg(long, default_value_t = false, requires="files_dir")]
    pub benchmark: bool,
}