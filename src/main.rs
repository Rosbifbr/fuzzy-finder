use std::{fmt::format, fs::metadata, hash::Hasher};

mod filetable;
mod tree;
mod trie;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    //calculate file table name as hash for storage in /tmp
    let current_path = std::env::current_dir().unwrap();
    let mut hasher = std::collections::hash_map::DefaultHasher::new(); //simple hasher. we dont need cryptographic security
    hasher.write(current_path.to_str().unwrap().as_bytes());
    let file_table_name = "__fstb".to_owned() + &hasher.finish().to_string();
    let file_table_path = format!("/tmp/{}", file_table_name);

    //check if ft exists
    let file_table: Vec<filetable::FileTableEntry>;
    if metadata(file_table_path.clone()).is_err() {
        //cloning path for else clause
        file_table = Vec::new();
    } else {
        file_table = filetable::load_table_from_file(&file_table_path);
    }
}

//fn generate_file_table() -> Vec<filetable::FileTableEntry> {}
