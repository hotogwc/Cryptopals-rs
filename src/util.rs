use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::cmp::Ordering;

pub fn read_file_to_vec_string(path: &str) -> Vec<String> {
    let file = File::open(Path::new(path)).unwrap();
    let buffer = BufReader::new(file);
    buffer.lines().filter_map(|b| b.ok()).collect()
}


pub fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater
    }
    return Ordering::Equal;
}