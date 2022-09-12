use std::path::Path;

use fsn::parse_file;

fn main() {
    match parse_file(Path::new("./examples/c.fsn")){
        Ok(s) => {
            for dir in s.directories {
                println!("Dir: {dir}")
            }
            for file in s.files {
                println!("File: {}", file.name);
                if file.name == "main.c" {
                    println!("{}", file.contents)
                }
            }
        },
        Err(e) => println!("err {e}")
    }
}
