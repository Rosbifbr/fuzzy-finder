use std::{string, thread::current};

use crate::tree::*;

//Retuns vector of best-matching paths
pub fn search_trie(tree: Vec<TreeNode>, key: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut key_bit_index = 0;
    let mut vec_index = 0;
    loop {
        if vec_index >= tree.len() {
            break;
        }
        let curr_str_bit = key.as_bytes()[key_bit_index];
        let current_node = &tree[vec_index];
        let left_child = 2 * vec_index + 1;
        let right_child = 2 * vec_index + 2;

        if String::from_utf8(current_node.value.to_vec())
            .unwrap()
            .contains(key)
        {
            results.push(String::from_utf8(current_node.path.to_vec()).unwrap());
        }
        
        if key_bit_index < key.len() {
            if curr_str_bit != 0 {
                vec_index = right_child
            } else {
                vec_index = left_child
            }
        } else {
            break;
        }
        key_bit_index += 1;
    }

    return results;
}

pub fn insert_trie(tree: &mut Vec<TreeNode>, key: String, path: String) {
    let mut key_bit_index = 0;
    let mut vec_index = 0;
    //seek most similar node
    loop {
        let curr_str_bit = key.as_bytes()[key_bit_index];
        let current_node = &tree[vec_index];
        let left_child = 2 * vec_index + 1;
        let right_child = 2 * vec_index + 2;
        if key_bit_index < key.len() {
            if curr_str_bit != 0 {
                vec_index = right_child
            } else {
                vec_index = left_child
            }

            //If vec_index is out of bounds, we need to add a new node
            if vec_index >= tree.len() {
                let new_node = TreeNode {
                    value: [0; TREE_NODE_VALUE_SIZE],
                    path: [0; TREE_NODE_PATH_SIZE],
                };
                tree[vec_index] = new_node;
            }
        } else {
            //write data to correct node and quit
            let mut new_node = tree.get_mut(vec_index).unwrap();
            new_node.value = key.as_bytes().to_owned().try_into().unwrap();
            new_node.path = path.as_bytes().to_owned().try_into().unwrap();
            break;
        }
        key_bit_index += 1;
    }
}
