use std::{fmt::format, fs::metadata, hash::Hasher};

mod tree;
mod trie;
use tree::*;
use trie::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    //calculate trie name as hash for storage in /tmp
    let current_path = std::env::current_dir().unwrap();
    let mut hasher = std::collections::hash_map::DefaultHasher::new(); //simple hasher. we dont need cryptographic security
    hasher.write(current_path.to_str().unwrap().as_bytes());
    let trie_name = "__fstb".to_owned() + &hasher.finish().to_string();
    let trie_path = format!("/tmp/{}", trie_name);


    let result = read_tree_from_file(trie_path.as_str());
    let tree = if result.is_some() {
        result.unwrap()
    } else {
        let current_path = std::env::current_dir().unwrap();
        let paths = expand_file_system(current_path.to_str().unwrap().to_string());
        let mut tree: Vec<TreeNode> = Vec::new();
        for path in paths {
            let filename = path.split("/").last().unwrap().to_string();
            insert_trie(&mut tree, filename, path);
        }
        tree
    };

    //test search main (todo)
    println!("{:?}", search_trie(&tree, args[1].clone().as_str()));

}

fn expand_file_system(path: String) -> Vec<String> {
    let mut results = Vec::new();
    let dir_contents = std::fs::read_dir(path).unwrap();
    for element in dir_contents {
        let result = match element {
            Ok(e) => e,
            Err(e) => {
                println!("Error reading file: {}", e);
                continue;
            }
        };

        //if is dir, recurse
        if result.metadata().unwrap().is_dir() {
            let sub_results = expand_file_system(result.path().to_str().unwrap().to_string());
            results.extend(sub_results);
        } else {
            //if is file, add to list 
            let file_path = result.path().to_str().unwrap().to_string();
            results.push(file_path);
        }
    }
    return results;
}
