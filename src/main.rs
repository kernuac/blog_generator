const POSTS_FOLDER: &str =  "__posts";
pub mod metadata;
pub mod posts;

fn main() {
    let pt = posts::Post::new( format!("{}{}", POSTS_FOLDER, "/2019-12-11-Hello-World.md"));
    println!("author: {}\ntitle: {}\nurl: {}\n", pt.metadata.author, pt.metadata.title, pt.url );
}
