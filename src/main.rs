use std::fmt;
use std::fs;
use std::io;

struct File {
    name: String,
}

impl File {
    fn new(name: String) -> Self {
        File { name: name }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn get_files(path: String) -> Result<Vec<File>, io::Error> {
    let mut v = Vec::new();
    let file_iter = fs::read_dir(path)?;

    for file in file_iter {
        match file {
            Ok(f) => v.push(File::new(f.file_name().into_string().unwrap())),
            Err(_) => continue,
        }
    }

    Ok(v)
}

fn main() {
    for file in get_files(String::from(".")).unwrap() {
        println!("{file}");
    }
}
