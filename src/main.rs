use std::env::args;
use std::fmt;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;

const BLUE: &'static str = "\x1b[34m";
const CYAN: &'static str = "\x1b[36m";
const NC: &'static str = "\x1b[0m";

enum FileType {
    File,
    Dir,
    Link,
    Undefined,
}

struct File {
    name: String,
    file_type: FileType,
    permissions: String,
    link_target: Option<String>,
}

impl File {
    fn get_file_type(entry: &fs::DirEntry) -> FileType {
        entry
            .file_type()
            .map(|ft| {
                if ft.is_file() {
                    FileType::File
                } else if ft.is_dir() {
                    FileType::Dir
                } else if ft.is_symlink() {
                    FileType::Link
                } else {
                    FileType::Undefined
                }
            })
            .unwrap_or(FileType::Undefined)
    }

    fn get_permissions_string(entry: &fs::DirEntry) -> String {
        let mut s = String::new();
        let bin_mode = format!("{:016b}", entry.metadata().unwrap().permissions().mode());

        let (_file_mode_type, permission_bits) = bin_mode.split_at(7);
        let perms = vec![
            &permission_bits[0..3],
            &permission_bits[3..6],
            &permission_bits[6..9],
        ];

        let symbols = vec!["r", "w", "x"];
        for current in perms {
            s.push_str(
                &current
                    .chars()
                    .zip(symbols.iter())
                    .map(|(bit, &symbol)| if bit == '1' { symbol } else { "-" })
                    .collect::<String>(),
            );
        }

        s
    }

    fn get_link_target(entry: &fs::DirEntry) -> Option<String> {
        let path = entry.path();
        if !path.is_symlink() {
            return None;
        }

        let target = fs::read_link(&path).ok()?;
        Some(target.to_string_lossy().into_owned())
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.file_type {
            FileType::File => write!(f, "{} {}", self.permissions, self.name),
            FileType::Dir => write!(f, "{} {}{}/{}", self.permissions, BLUE, self.name, NC),
            FileType::Link => write!(
                f,
                "{} {}{} -> {}{}",
                self.permissions,
                CYAN,
                self.name,
                self.link_target.as_deref().unwrap_or("<invalid>"),
                NC
            ),
            FileType::Undefined => write!(f, "-"),
        }
    }
}

impl From<fs::DirEntry> for File {
    fn from(value: fs::DirEntry) -> Self {
        File {
            name: value.file_name().into_string().unwrap(),
            file_type: File::get_file_type(&value),
            permissions: File::get_permissions_string(&value),
            link_target: File::get_link_target(&value),
        }
    }
}

fn get_files(path: String) -> Result<Vec<File>, io::Error> {
    let mut v = Vec::new();
    let file_iter = fs::read_dir(path)?;

    for file in file_iter {
        match file {
            Ok(f) => v.push(File::from(f)),
            Err(_) => continue,
        }
    }

    Ok(v)
}

fn main() {
    let path = args().nth(1).unwrap_or(String::from("."));
    for file in get_files(path).unwrap() {
        println!("{file}");
    }
}
