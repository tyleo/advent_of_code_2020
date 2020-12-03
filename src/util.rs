use std::fs;
use std::io::Read as _;
use std::path::Path;

pub fn read<P: AsRef<Path>>(path: P) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
