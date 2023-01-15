use std::{collections::{HashMap, HashSet},
          sync::{Mutex, Arc}, fs::{File},
          io::{BufReader, Read},
          path::{PathBuf}
};
use threadpool::ThreadPool;
use walkdir::WalkDir;

pub struct IndexBuilder {
    inverted_index: Arc<Mutex<HashMap<String, HashSet<PathBuf>>>>,
    thread_pool: ThreadPool,
}

impl IndexBuilder {
    pub fn new(thread_num: usize) -> IndexBuilder {
        IndexBuilder {
            inverted_index: Arc::new(Mutex::new(HashMap::new())),
            thread_pool: ThreadPool::new(thread_num),
        }
    }

    pub fn proceed(&mut self, filename: PathBuf) {
        let inverted_index = self.inverted_index.clone();
        self.thread_pool.execute(move ||{
            let mut file_content = String::new();
            let file = File::open(filename.as_path()).expect("no such file");
            let mut buf = BufReader::new(file);
            buf.read_to_string(&mut file_content).unwrap();

            let words: HashSet<&str>  = file_content.split(&[' ', '\n'][..]).collect();
            for word in &words {
                let mut guard = inverted_index.lock().unwrap();
                guard
                    .entry(word.to_string())
                    .and_modify(|set| { set.insert(filename.clone()); })
                    .or_insert(HashSet::from_iter(vec![filename.clone()]));
            }
        });
    }

    pub fn build(self) -> HashMap<String, HashSet<PathBuf>> {
        self.thread_pool.join();

        let index = match Arc::try_unwrap(self.inverted_index) {
            Ok(index) => index,
            Err(_) => panic!(),
        };

        index.into_inner().unwrap()
    }
}

pub fn build_for_directory(thread_number: usize, files_dir: PathBuf) ->
            HashMap<String, HashSet<PathBuf>>{
    let mut builder = IndexBuilder::new(thread_number);

    for entry in WalkDir::new(files_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            builder.proceed(path.to_path_buf());
        }
    }
    builder.build()
}