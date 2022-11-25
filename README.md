# File Structure Notation
File Structure Notation (.fsn) format parser

**Install**
```
$ cargo add fsn
```
or add `fsn = "0.1.3"` to `[dependencies]` in `Cargo.toml`

**Examples**
```
match parse_file(Path::new("./examples/c.fsn")){
    Ok(s) => {
        for dir in s.directories {
            println!("Dir: {dir}")
        }
        for file in s.files {
            println!("File: {}", file.name);
           println!("{}", file.contents)
        }
    },
    Err(e) => println!("err {e}")
}
```