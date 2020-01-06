const POSTS_FOLDER: &str = "__posts";

pub mod file_walker;
pub mod metadata;
pub mod posts;

use std::fs;

fn main() {
    let files = fs::read_dir(POSTS_FOLDER).unwrap();

    for file in files {
        let file = file.unwrap();
        let fln = file_walker::FilePost::new(file.path());

        println!(
            "filename: {}\npath: {}\ncontents: {}\n",
            fln.name,
            fln.path.to_string_lossy(),
            fln.contents
        );
    }
}
