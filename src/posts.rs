use comrak::{markdown_to_html, ComrakOptions};
use std::io::Read;
use std::fs::File;
use crate::metadata;

pub struct Post {
    pub metadata: metadata::MetaData,
    pub filename: String,
    pub url: String,
    pub contents: String
}

impl Post {
    pub fn new(filename: String) -> Post {

        let mut pst: Post = Post{
            filename: "".to_string(),
            metadata: metadata::MetaData::new(),
            url: "".to_string(),
            contents: "".to_string()
        };

        pst.filename = filename;
        pst.contents = Post::read_file(pst.filename.clone());
        pst.metadata = metadata::MetaData::retrieve(pst.contents.clone());
        pst.generate_url();

        pst
    }

    pub fn read_file(filename: String) -> String {
        let mut file_descriptor = File::open( filename ).unwrap();
        let mut content = String::new();
        file_descriptor.read_to_string(&mut content).unwrap();
        content
    }

    pub fn generate_url(&mut self) {
        self.url = format!("/{}/{}/{}/{}.html", 
            self.metadata.year,
            self.metadata.month,
            self.metadata.day,
            self.filename.replace(".md", ""));
    }

    fn get_title_from_filename(self) -> String {
        let url: String = String::new();
        
        url
    }
}
