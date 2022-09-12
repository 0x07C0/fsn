use std::{
    fs::read_to_string,
    io::{self},
    path::Path,
};

use regex::Regex;

pub struct FileData {
    name: String,
    contents: String,
}
pub struct Structure {
    directories: Vec<String>,
    files: Vec<FileData>,
}

pub fn parse_file(path: &Path) -> io::Result<Structure> {
    match read_to_string(path) {
        Ok(s) => {
            let mut strc: Structure = Structure {
                directories: vec![],
                files: vec![],
            };
            let dir_pattern = Regex::new(r"^\[:(.*)\]$").unwrap();
            let file_pattern = Regex::new(r"^\[(?!EOF)(?!:)(.*)\]$").unwrap();
            for caps in dir_pattern.captures_iter(&s) {
                strc.directories.push(caps[1].to_owned())
            }
            for caps in file_pattern.captures_iter(&s) {
                let mut file: FileData = FileData {
                    name: String::default(),
                    contents: String::default(),
                };
                file.name = caps[1].to_owned();
                let start =
                    s.find(format!("[{}]", &file.name).as_str()).unwrap() + 3 + file.name.len();
                let end = s.split_at(start).1.find("[EOF]\n").unwrap();
                file.contents = s[start..end].to_owned();
                strc.files.push(file)
            }

            Ok(strc)
        }
        Err(e) => Err(e),
    }
}