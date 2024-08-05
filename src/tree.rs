//Simple non-binary tree structure
pub struct TreeNode{
    pub _value: String,
    pub name: String,
    pub full_path: String,
    pub extension: String,
    pub children: Vec <TreeNode>
}

//Tree-related helper functions
pub fn print_tree(tree: TreeNode){
    println!("{}", tree._value);
    for child in tree.children{
        print_tree(child);
    }
}
