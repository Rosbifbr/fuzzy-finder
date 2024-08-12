// Simple non-binary tree structure
pub const TREE_NODE_CHILD: Option<Box<TreeNode>> = None; //Typing of this is needed in const so that rust compiler does not go crazy when initing our pointer arr structs.
pub const TREE_NODE_CHILD_LENGTH: usize = 255; //supporting only ascii for now
pub struct TreeNode {
    pub ft_index: Option<i32>, // index of file table
    pub value: char,           // later we need to support patricia trees
    pub children: [Option<Box<TreeNode>>; TREE_NODE_CHILD_LENGTH], //too much space for ascii? optimize if needed
}

// Tree-related debug functions
pub fn print_tree(tree: &TreeNode) {
    match tree.ft_index {
        Some(val) => print!("{}", val),
        None => {}
    }
    println!("{}", tree.value);
    for child in tree.children.iter() {
        if child.is_some() {
            print_tree(child.as_ref().unwrap());
        }
    }
}
