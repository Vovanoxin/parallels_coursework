use std::{collections::{HashMap, HashSet}, sync::{Mutex, Arc}, fs::File, io::{BufReader, Read}, thread, time::Duration};
use bimap::BiMap;
use threadpool::ThreadPool;

fn main() {
    let filenames = ["file1.txt", "file2.txt", "file3.txt", "file4.txt"];
    let mut inversed_index = 
        Arc::new(Mutex::new(HashMap::<String, HashSet<usize>>::new()));
    let mut file_ids = BiMap::<String, usize>::new();
    let thread_pool = ThreadPool::new(8);
    let mut cur_id: usize = 0;
    
    for file in filenames {
        let inversed_index = inversed_index.clone();
        file_ids.insert(file.to_string(), cur_id);
       
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