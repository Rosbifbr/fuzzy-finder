use crate::tree::*;

pub fn search_trie(tree: Vec<TreeNode>, key: &str) -> Option<String> {
    let mut found_node: Option<&TreeNode> = None;
    let mut key_bit_index = 0;
    let mut vec_index =  0;
    while found_node.is_none() {
        let str_number = *key & 2 << key_bit_index;
        let current_node = &tree[vec_index];
        if current_node.value == key.as_bytes() {
            found_node = Some(current_node);
        } else {
            let left_child = 2 * vec_index + 1;
            let right_child = 2 * vec_index + 2;
            //TODO
        }
    }
    if found_node.is_none() {
        return None;
    } else {
        return found_node.map(|node| String::from_utf8(node.value.to_vec()).unwrap()); //u8 array to vec to string. optimize if there is time
    }
}

pub fn insert_trie(tree: Vec<TreeNode>, key: String) {
}
