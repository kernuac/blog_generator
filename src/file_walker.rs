use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

pub struct FilePost {
    pub name: String,
    pub path: PathBuf,
    pub contents: String,
}

impl FilePost {
    pub fn new(path: PathBuf) -> Self {
        FilePost {
            name: String::new(),
            path: PathBuf::new(),
            contents: String::new()
        }
        .open( path )
    }

    pub fn open(&self, path: PathBuf) -> Self {
        let mut file_descriptor = File::open( &path ).unwrap();
        let mut contents = String::new();
        file_descriptor.read_to_string(&mut contents).unwrap();
        let fname: String = self.get_name( path.clone() );
        
        FilePost {
            name: fname,
            path: path,
            contents: contents
        }
    }

    fn get_name(&self, path: PathBuf ) -> String {
         let file_name = path.file_name().unwrap();
         file_name.to_str().unwrap().to_string()
    }
}