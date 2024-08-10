use crate::tree::*;

pub fn search_trie<'a>(root: &'a TreeNode<'a>, key: String) -> Option<&'a TreeNode<'a>> {
    if key == "" { //string expired
        return Some(root);
    }
    else {
        let desired_index = root.value as usize - 'a' as usize;
        let next_node = root.children[desired_index];
        let next_key = key[1..].to_string();

        if next_node.is_none(){
            return None;
        }
        else {
            return search_trie(next_node.unwrap(), next_key);
        }
    }
}

pub fn insert_trie(root: &mut TreeNode, node: &mut TreeNode) {
}
