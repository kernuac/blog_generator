use std::io::Read;
use std::fs::File;
use std::path::Path;

pub struct FilePost {
    name: &'static str,
    path: Path,
    contents: String,
}

impl FilePost {
    pub fn new(path: Path) -> Self {
        FilePost {
            name: "",
            path: "",
            contents: String::new()
        }
        .open( path )
    }

    pub fn open(&self, path: Path) -> Self {
        let mut file_descriptor = File::open( path ).unwrap();
        let mut contents = String::new();
        file_descriptor.read_to_string(&mut contents).unwrap();

        FilePost {
            name: "",
            path: path,
            contents: contents
        }
    }

    fn get_name(&self) -> String {
        let name: String = String::new();
        let splitted = self.path.file_name().unwrap().to_str().unwrap();
        
    }
}