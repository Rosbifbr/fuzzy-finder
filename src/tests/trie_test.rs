use tree::{print_tree, TREE_NODE_CHILD, TREE_NODE_CHILD_LENGTH};
use trie::insert_trie;
mod filetable;
mod tree;
mod trie;

fn main() {
    let mut tree_root = tree::TreeNode {
        ft_index: None,
        value: 'a',
        children: [TREE_NODE_CHILD; TREE_NODE_CHILD_LENGTH],
    };
    insert_trie(&mut tree_root, "this is a test".to_string(), 70);
    insert_trie(&mut tree_root, "this is another test".to_string(), 22);
    insert_trie(&mut tree_root, "that is differente".to_string(), 32);
    //print_tree(&tree_root);
    let node = trie::search_trie(&tree_root, "that is differente");
    match node {
        Some(node) => {
            println!("Found node with ft_index: {}", node.ft_index.unwrap());
        }
        None => {
            println!("Node not found");
        }
    }
}
