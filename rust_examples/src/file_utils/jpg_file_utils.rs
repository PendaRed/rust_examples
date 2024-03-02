use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::io;

use log::{ info, warn, error, debug, };


struct JpgFile {
    dir_path: String,
}

impl JpgFile {
    pub fn new(dir_path:&str) -> Self {
        let p = format!("{}{}",dir_path, {if !dir_path.ends_with("/") {""} else {"/"}});
        JpgFile{dir_path: p}
    }

    fn trunc(&self, p:&str) -> String {
        if p.len() <10 {
            p.to_string()
        } else {
            let mut v:String = p[0..7].to_owned();
            format!("{}...", v)
        }
    }

    // xxxjpg discuss with adam the trunc function
    pub fn save_file(&self, file_name:&str, payload:&str) -> Result<String, io::Error> { 
        let path_n_file = format!("{}{}",self.dir_path,file_name);
        info!("save_to_file: [{}] payload[{}]", &path_n_file, self.trunc(payload));

        let mut file = File::create(&path_n_file)?;
        file.write_all(payload.as_bytes())?;
        Ok(path_n_file)
    }

    // pub fn read_file() -> std::io::Result<()> {
    //     let mut file = File::open("foo.txt")?;
    //     let mut contents = String::new();
    //     file.read_to_string(&mut contents)?;
    //     assert_eq!(contents, "Hello, world!");
    //     Ok(())
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_write() {
        let f = JpgFile::new(&"");
        let full_path = f.save_file("tmp", "payload");
//        File::
    }
}