use std::path::PathBuf;


fn main() {
    let cwd = PathBuf::from(".");
    if let Ok(path) = cwd.canonicalize() {
            println!("{:?}", path);
    }
    
}