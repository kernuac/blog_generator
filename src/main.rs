#![allow(dead_code)]
const POSTS_FOLDER: &str = "__posts";

pub mod file_reader;
pub mod meta_data;

use std::fs;

enum FileContent {
    MetaData = 1,
    Content = 2,
}

fn main() {
    let filenames = fs::read_dir(POSTS_FOLDER).unwrap();

    for filename in filenames {
        let content = file_reader::read(filename.unwrap());
        let file_content: Vec<&str> = content.split("---").collect();
        let meta =
            meta_data::Metadata::new(file_content[FileContent::MetaData as usize].to_string());

        println!(
            "filename: {}\npath: {}\ncontents: {}\n",
            "fln.name",
            meta.title,
            file_content[FileContent::Content as usize].trim()
        );
    }
}
