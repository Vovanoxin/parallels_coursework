use std::{collections::{HashMap, HashSet}, sync::{Mutex, Arc}, fs::{File, ReadDir}, io::{BufReader, Read}, thread, time::Duration, path::{Path, PathBuf}};
use bimap::BiMap;
use clap::Parser;
use threadpool::ThreadPool;
use std::fs;

mod cli;
use cli::Cli;

fn build_index<'a>(filenames: impl Iterator<Item = PathBuf>) {
    let mut inversed_index =
        Arc::new(Mutex::new(HashMap::<String, HashSet<usize>>::new()));
    let mut file_ids = BiMap::<PathBuf, usize>::new();
    let thread_pool = ThreadPool::new(8);
    let mut cur_id: usize = 0;

    for file in filenames {
        let file = file.to_path_buf();
        let inversed_index = inversed_index.clone();
        file_ids.insert(file.clone(), cur_id);

        thread_pool.execute(move ||{
            let file_id = cur_id;
            let mut file_content = String::new();
            let file = File::open(file).expect("no such file");
            let mut buf = BufReader::new(file);
            buf.read_to_string(&mut file_content);

            let words: HashSet<&str>  = file_content.split(&[' ', '\n'][..]).collect();
            for word in &words {
                let mut guard = inversed_index.lock().unwrap();
                guard
                    .entry(word.to_string())
                    .and_modify(|set| { set.insert(file_id); })
                    .or_insert(HashSet::from_iter(vec![file_id]));
            }

        });
        cur_id += 1;
    }
    thread_pool.join();
    println!("{:?}", inversed_index);
}

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
    //let filenames = vec!["filename1.txt"];
    build_index(filenames);
}