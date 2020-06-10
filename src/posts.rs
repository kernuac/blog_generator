use crate::metadata;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct Post {
    pub metadata: metadata::MetaData,
    pub filename: String,
    pub url: String,
    pub contents: String,
}

impl Post {
    pub fn new(filename: PathBuf) -> Post {
        let mut pst: Post = Post {
            filename: String::new(),
            metadata: metadata::MetaData::new(),
            url: "".to_string(),
            contents: "".to_string(),
        };

        pst.filename = filename.file_name().unwrap().to_str().unwrap().into();
        pst.contents = Post::read_file(filename);
        pst.metadata = metadata::MetaData::retrieve(pst.contents.clone());
        pst.generate_url();

        pst
    }

    pub fn read_file(filename: PathBuf) -> String {
        let mut file_descriptor = File::open(&filename).unwrap();
        let mut content = String::new();
        file_descriptor.read_to_string(&mut content).unwrap();
        //let content2: Vec<&str> = content.splitn(2,"C").collect();
        //content2[1].to_string()
        content
    }

    pub fn generate_url(&mut self) {
        self.url = format!(
            "/{}/{}/{}/{}.html",
            self.metadata.year,
            self.metadata.month,
            self.metadata.day,
            self.filename.replace(".md", "")
        );
    }

    fn get_title_from_filename(self) -> String {
        let url: String = String::new();

        url
    }
}
