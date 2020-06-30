use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub author: String,
    pub description: String,
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl Metadata {
    pub fn new(metadata: String) -> Self {
        let m: Metadata = serde_yaml::from_str(&metadata).unwrap();
        m
    }
}
