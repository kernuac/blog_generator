use serde_derive::{Deserialize};

#[derive(Debug,PartialEq,Deserialize)]
pub struct MetaData {
    pub title: String,
    pub author: String,
    pub description: String,
    pub year: String,
    pub month: String,
    pub day: String,
}

impl MetaData {
    pub fn new() -> Self {
        MetaData {
            title: "".to_string(),
            author: "".to_string(),
            description: "".to_string(),
            year: "".to_string(),
            month: "".to_string(),
            day: "".to_string()
        }
    }

    pub fn retrieve(contents: String) -> Self {
        let end_metadata = contents[4..].find("---").unwrap() + 4;
        let metadata_content = &contents[..end_metadata];

        serde_yaml::from_str( metadata_content ).unwrap()
    }
}