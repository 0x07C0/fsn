use std::{error::Error, fs::read_to_string, path::Path};

use regex::Regex;

/// Describes file with it's name and contents.
pub struct FileData {
    pub name: String,
    pub contents: String,
}
/// Describes directory and file structute.
pub struct Structure {
    pub directories: Vec<String>,
    pub files: Vec<FileData>,
}

/// Parses FSN format file into Structure struct.
/// # Examples 
/// 
/// ```
/// use std::path::Path;
/// let structure = fsn::parse_file(Path::new("./examples/test.fsn")).unwrap();
/// assert_eq!(structure.directories, vec!["test", "rust/doc"]);
/// assert_eq!(structure.files[0].name, "hello");
/// assert_eq!(structure.files[0].contents, "world\n");
pub fn parse_file(path: &Path) -> Result<Structure, Box<dyn Error>> {
    let string = read_to_string(path)?;

    parse_string(&string)
}

/// Parses FSN format string into Structure struct.
/// # Examples 
/// 
/// ```
/// let string = "\
/// [:test]
/// [hello]
/// world
/// [EOF]".to_owned();
/// let structure = fsn::parse_string(&string).unwrap();
/// assert_eq!(structure.directories, vec!["test"]);
/// assert_eq!(structure.files[0].name, "hello");
/// assert_eq!(structure.files[0].contents, "world\n");
pub fn parse_string(s: &String) -> Result<Structure, Box<dyn Error>> {
    let mut structure: Structure = Structure {
        directories: vec![],
        files: vec![],
    };

    let pattern = Regex::new(r"\[(.*)\]")?; // Look-aheads aren't supported

    for caps in pattern.captures_iter(&s) {
        if caps[1] == "EOF".to_owned() {
            continue;
        }
        if caps[1].starts_with(':') {
            structure.directories.push(caps[1][1..].to_owned());
            continue;
        }
        let start = s.find(&format!("[{}]", &caps[1])).unwrap() + 3 + caps[1].len();
        let end = s.split_at(start).1.find("[EOF]").unwrap_or(start);
        let file: FileData = FileData {
            name: caps[1].to_owned(),
            contents: s[start..(start + end)].to_owned(),
        };
        structure.files.push(file)
    }

    Ok(structure)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn file_parsing() {
        let structure = parse_file(Path::new("./examples/test.fsn")).unwrap();
        assert_eq!(structure.directories, vec!["test", "rust/doc"]);
        assert_eq!(structure.files[0].name, "hello");
        assert_eq!(structure.files[0].contents, "world\n");
        assert_eq!(structure.files[1].name, "empty");
        assert_eq!(structure.files[1].contents, "");
    }
    #[test]
    fn string_parsing() {
        let string = "[:test]".to_owned();
        let structure = parse_string(&string).unwrap();
        assert_eq!(structure.directories, vec!["test"]);
    }
}
