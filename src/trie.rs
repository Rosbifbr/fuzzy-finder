use crate::tree::*;

pub fn search_trie<'a>(root: &'a TreeNode, key: &str) -> Option<&'a TreeNode> {
    if key.is_empty() {
        return Some(root);
    }
    else {
        let desired_index = root.value as usize - 'a' as usize;
        let next_node = &root.children[desired_index];

        match next_node {
            Some(ref node) => search_trie(node, &key[1..]),
            None => None,
        }
    }
}

pub fn insert_trie(root: &mut TreeNode, key: String, ft_index: i32) {
    let desired_index = key.chars().next().unwrap() as usize - 'a' as usize;
    let next_key = key[1..].to_string();
    let next_node = &root.children[desired_index];
    const ARRAY_REPEAT_VALUE: Option<Box<TreeNode>> = None; //Typing of this is needed in const so that rust compiler does not go crazy when initing our pointer arr structs.

    //will nuke existing node if existing (revise later)
    if next_key.len() == 0 {
        root.children[desired_index] = Some(Box::new(TreeNode {
            ft_index: Some(ft_index),
            value: key.chars().next().unwrap(),
            children: [ARRAY_REPEAT_VALUE; 26],
        }));
    }
    else {
        if next_node.is_none() {
            root.children[desired_index] = Some(Box::new(TreeNode {
                ft_index: None,
                value: key.chars().next().unwrap(),
                children: [ARRAY_REPEAT_VALUE; 26],
            }));
        }
        insert_trie(root.children[desired_index].as_mut().unwrap(), next_key, ft_index);
    }
}
