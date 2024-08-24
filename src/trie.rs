use std::{string, thread::current};

use crate::tree::*;

//Retuns vector of best-matching paths
pub fn search_trie(tree: Vec<TreeNode>, key: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut key_bit_index = 0;
    let mut key_byte_index = 0;
    let mut vec_index = 0;
    loop {
        // if vec index is out of bounds, we have reached the end of the tree
        if vec_index >= tree.len() {
            break;
        }
        let curr_str_bit = key.as_bytes()[key_byte_index] & (1 << key_bit_index);
        let current_node = &tree[vec_index];
        let left_child = 2 * vec_index + 1;
        let right_child = 2 * vec_index + 2;

        if String::from_utf8(current_node.value.to_vec())
            .unwrap()
            .contains(key)
        {
            results.push(String::from_utf8(current_node.path.to_vec()).unwrap());
        }
        
        if key_byte_index < key.len() {
            if curr_str_bit != 0 {
                vec_index = right_child
            } else {
                vec_index = left_child
            }
        } else {
            break;
        }

        key_bit_index += 1;
        if key_bit_index == 7 {
            key_bit_index = 0;
            key_byte_index += 1;
        }
    }

    return results;
}

pub fn insert_trie(tree: &mut Vec<TreeNode>, key: String, path: String) {
    let mut key_bit_index = 0;
    let mut key_byte_index = 0;
    let mut vec_index = 0;
    //seek most similar node
    loop {
        println!("Vec index: {}, key: {}, key_bit_index: {}, key_byte_index: {}", vec_index, key, key_bit_index, key_byte_index);
        if key_byte_index < key.len() {
            let curr_str_bit = key.as_bytes()[key_byte_index] & (1 << key_bit_index);
            let left_child = 2 * vec_index + 1;
            let right_child = 2 * vec_index + 2;
            if curr_str_bit != 0 {
                vec_index = right_child;
            } else {
                vec_index = left_child;
            }

            // If vec_index is out of bounds, we need to extend the vector until we reach the correct index
            if vec_index >= tree.len() {
                // Fill array with zeros until we reach the desired index
                tree.resize(vec_index + 1, TreeNode {
                    value: [0; TREE_NODE_VALUE_SIZE],
                    path: [0; TREE_NODE_PATH_SIZE]
                });
            }
        } else {
            // Write data to correct node and quit
            let new_node = tree.get_mut(vec_index).unwrap();

            // Convert params to byte arrays of appropriate size and assign
            let key_bytes = key.as_bytes();
            let path_bytes = path.as_bytes();

            // Copy to new node
            for i in 0..key.len() {
                new_node.value[i] = key_bytes[i];
            }

            for i in 0..path.len() {
                new_node.path[i] = path_bytes[i];
            }
            return;
        }

        key_bit_index += 1;
        if key_bit_index > 7 {
            key_bit_index = 0;
            key_byte_index += 1;
        }
    }
}
