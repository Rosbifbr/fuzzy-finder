// Simple non-binary tree structure
pub struct TreeNode<'a> {
    pub ft_index: Option<i32>, // index of file table
    pub value: char,           // later we need to support patricia trees
    pub children: [Option<&'a TreeNode<'a>>; 26],
}

// Tree-related debug functions
pub fn print_tree(tree: &TreeNode) {
    println!("{}", tree.value);
    for &child in &tree.children {
        if child.is_some() {
            print_tree(child.unwrap());
        }
    }
}
