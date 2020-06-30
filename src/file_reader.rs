use std::fs::{DirEntry, File};
use std::io::prelude::Read;

pub fn read(filename: DirEntry) -> String {
    let mut content = String::new();
    let mut f = File::open(filename.path()).unwrap();

    f.read_to_string(&mut content).unwrap();

    content
}
