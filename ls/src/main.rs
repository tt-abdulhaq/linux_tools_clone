use std::fs;
use std::env;

use  std::path::{Path, PathBuf};




fn main(){

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path> [-r]");
        return;
    }

    let mut path = PathBuf::from(".");

    if let Some(index) = args.iter().position(|arg| arg=="-p"){
        if let Some(p) = args.get(index+1){
            path = PathBuf::from(p);
        }
    }

    if args.iter().any(|arg| arg == "-r"){
        if let Err(e) = handle_recursive(&path){
            eprintln!("{}", e)
        }
    }
    else {
        if let Err(e) = list_files(&path){
            eprintln!("{}", e);
        }
    }
}
fn list_files(path: &Path) -> Result<(), std::io::Error> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let full_path = entry.path().canonicalize()?;
                println!("{}", full_path.display());
            }
        }
    }
    Ok(())
}


fn handle_recursive(path: &Path) -> Result<(), std::io::Error> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let full_path = entry.path().canonicalize()?;
                if entry.file_type()?.is_dir() {
                    handle_recursive(&full_path)?;
                } else {
                    println!("{}", full_path.display());
                }
            }
        }
    }
    Ok(())
}
