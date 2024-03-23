use std::{env, path::PathBuf, fs};



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        eprintln!("Usage: cargo run -- <path> ");
        return;
    }

    let mut  path = PathBuf::from("false");

    if let Some(index) = args.iter().position(|arg| arg == "-p"){
        if let Some(p) = args.get(index+1){
            path = PathBuf::from(p);
        }
    }
    if  !path.exists() && !path.to_string_lossy().eq("false"){
        if let Err(err) = fs::create_dir_all(path){
            eprintln!("Error creating directory: {}", err)
        }
    }
    else{
        println!("mkdir: cannot create directory {:?}: File exists", path);
    }
    
    
    
}
