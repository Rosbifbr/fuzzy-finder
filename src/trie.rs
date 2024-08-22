use std::{string, thread::current};

use crate::tree::*;

// not perfect
pub fn search_trie(tree: Vec<TreeNode>, key: &str) -> (Option<String>, usize) {
    let mut found_node: Option<&TreeNode> = None;
    let mut key_bit_index = 0;
    let mut vec_index = 0;
    while found_node.is_none() {
        if vec_index >= tree.len() {
            break;
        }
        let str_number = key.as_bytes()[key_bit_index];
        let current_node = &tree[vec_index];
        let left_child = 2 * vec_index + 1;
        let right_child = 2 * vec_index + 2;

        if String::from_utf8(current_node.value.to_vec())
            .unwrap()
            .contains(key)
            &&
        {
            //change this can be prefix search therefore not useful
            found_node = Some(current_node);
        } else {
            if str_number != 0 {
                vec_index = right_child
            } else {
                vec_index = left_child
            }
        }
        key_bit_index += 1;
    }

    //decide what to return
    if found_node.is_none() {
        return (None, 999);
    } else {
        return (
            Some(String::from_utf8(found_node.unwrap().path.to_vec()).unwrap()),
            vec_index,
        );
    }
}

pub fn insert_trie(tree: Vec<TreeNode>, key: String) {}
