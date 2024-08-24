use std::io::{Read, Write};

// Simple non-binary tree structure
// pub const TREE_NODE_CHILD: Option<Box<TreeNode>> = None; //Typing of this is needed in const so that rust compiler does not go crazy when initing our pointer arr structs.
// pub const TREE_NODE_CHILD_LENGTH: usize = 255; //supporting only ascii for now
pub const TREE_NODE_VALUE_SIZE: usize = 64; //supporting only ascii for now
pub const TREE_NODE_PATH_SIZE: usize = 255; //supporting only ascii for now

// binary trie
#[derive(Clone)]
pub struct TreeNode {
    pub value: [u8; TREE_NODE_VALUE_SIZE], //will also work with patricia trees. this string will later be compreesed if there is time
    pub path: [u8; TREE_NODE_PATH_SIZE],   //path to file the node references
}

// Tree-related debug functions
pub fn print_tree(tree: &Vec<TreeNode>, index: usize) {
    let tree_length = tree.len();
    let left_child = 2 * index + 1;
    let right_child = 2 * index + 2;

    println!(
        "Index: {}, Value: {:?}, Path: {:?}",
        index,
        tree[index].value,
        tree[index].path
    );

    if left_child < tree_length {
        print_tree(tree, left_child);
    }
    if right_child < tree_length {
        print_tree(tree, right_child);
    }
}

pub fn write_tree_to_file(tree: Vec<TreeNode>, path: &str) {
    let mut file = match std::fs::File::create(&path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to create file: {}", e),
    };
    for node in tree {
        file.write_all(&node.value).unwrap();
    }
}

pub fn read_tree_from_file(path: &str) -> Vec<TreeNode> {
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {}", e),
    };

    let mut byte_count = 0;
    let mut tree: Vec<TreeNode> = Vec::new();
    file.bytes().for_each(|byte| {
        if byte_count == TREE_NODE_VALUE_SIZE + TREE_NODE_PATH_SIZE {
            byte_count = 0;
        }
        if byte_count == 0 {
            tree.push(TreeNode {
                value: [0; TREE_NODE_VALUE_SIZE],
                path: [0; TREE_NODE_PATH_SIZE],
            });
        }

        //push byte to node
        let current_index = tree.len() - 1;
        if byte_count < TREE_NODE_VALUE_SIZE {
            tree[current_index].value[byte_count] = byte.unwrap();
        } else {
            tree[current_index].path[byte_count] = byte.unwrap();
        }

        byte_count += 1;
    });
    return tree;
}
