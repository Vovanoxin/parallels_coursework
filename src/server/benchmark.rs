use crate::index_builder::build_for_directory;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub fn benchmark(files_dir: PathBuf) {
    for i in 1..=16 {
        let mut total_time = Duration::new(0, 0);
        for j in 0..10 {
            let start = Instant::now();
            let _index = build_for_directory(i, files_dir.clone());
            let duration = start.elapsed();
            total_time += duration;
        }
        println!("Time elapsed in build_for_directory({}, ..) is: {:?}", i, total_time/10);
    }

}