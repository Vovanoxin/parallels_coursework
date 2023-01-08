use std::{collections::{HashMap, HashSet},
          sync::{Mutex, Arc}, fs::{File},
          io::{BufReader, Read},
          path::{PathBuf}
};
use bimap::BiMap;
use threadpool::ThreadPool;

pub struct IndexBuilder {
    cur_id: usize,
    file_ids: BiMap::<PathBuf, usize>,
    inverted_index: Arc<Mutex<HashMap<String, HashSet<usize>>>>,
    thread_pool: ThreadPool,
}

impl IndexBuilder {
    pub fn new(thread_num: usize) -> IndexBuilder {
        IndexBuilder {
            cur_id: 0,
            file_ids: BiMap::new(),
            inverted_index: Arc::new(Mutex::new(HashMap::new())),
            thread_pool: ThreadPool::new(thread_num),
        }
    }

    pub fn proceed(&mut self, filename: PathBuf) {
        let file_id = self.cur_id;
        self.file_ids.insert(filename.clone(), file_id);
        self.cur_id += 1;
        let inverted_index = self.inverted_index.clone();
        self.thread_pool.execute(move ||{
            let mut file_content = String::new();
            let file = File::open(filename.as_path()).expect("no such file");
            let mut buf = BufReader::new(file);
            buf.read_to_string(&mut file_content);

            let words: HashSet<&str>  = file_content.split(&[' ', '\n'][..]).collect();
            for word in &words {
                let mut guard = inverted_index.lock().unwrap();
                guard
                    .entry(word.to_string())
                    .and_modify(|set| { set.insert(file_id); })
                    .or_insert(HashSet::from_iter(vec![file_id]));
            }
        });
    }

    pub fn build(self) -> HashMap<String, HashSet<usize>>{
        self.thread_pool.join();

        let index = match Arc::try_unwrap(self.inverted_index) {
            Ok(index) => index,
            Err(_) => panic!(),
        };

        index.into_inner().unwrap()
    }
}