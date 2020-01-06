const POSTS_FOLDER: &str =  "__posts";

pub mod metadata;
pub mod posts;
pub mod file_walker;

use std::path::PathBuf;

fn main() {
    let _pt = posts::Post::new( format!("{}{}", POSTS_FOLDER, "/2019-12-11-Hello-World.md"));
    // println!("author: {}\ntitle: {}\nurl: {}\n", pt.metadata.author, pt.metadata.title, pt.url );
    let fln = file_walker::FilePost::new(
        PathBuf::from(
            format!("{}{}", POSTS_FOLDER, "/2019-12-11-Hello-World.md")
        )
    );

    println!("filename: {}\npath: {}\ncontents: {}\n", fln.name, fln.path.to_string_lossy(), fln.contents);
}
