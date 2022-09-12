use std::{
    fs::read_to_string,
    io::{self},
    path::Path,
};

use regex::Regex;

pub struct FileData {
    pub name: String,
    pub contents: String,
}
pub struct Structure {
    pub directories: Vec<String>,
    pub files: Vec<FileData>,
}

pub fn parse_file(path: &Path) -> io::Result<Structure> {
    match read_to_string(path) {
        Ok(s) => {
            let mut strc: Structure = Structure {
                directories: vec![],
                files: vec![],
            };
            let dir_pattern = Regex::new(r"\[:(.*)\]").unwrap();
            let file_pattern = Regex::new(r"\[(.*)\]").unwrap(); // Look aheads aren't supported
            for caps in dir_pattern.captures_iter(&s) {
                strc.directories.push(caps[1].to_owned())
            }
            for caps in file_pattern.captures_iter(&s) {
                if caps[1] == "EOF".to_owned() || caps[1].starts_with(':') {continue}
                let mut file: FileData = FileData {
                    name: String::default(),
                    contents: String::default(),
                };
                file.name = caps[1].to_owned();
                let start =
                    s.find(format!("[{}]", &file.name).as_str()).unwrap() + 3 + file.name.len();
                let end = s.split_at(start).1.find("[EOF]").unwrap();
                file.contents = s[start..(start+end)].to_owned();
                strc.files.push(file)
            }

            Ok(strc)
        }
        Err(e) => Err(e),
    }
}
