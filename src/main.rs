mod tree;
use tree::*; //import all my module code

fn main() {
    println!("Hello, world!");
    let mut root = TreeNode {
        name: "root".to_string(),
        full_path: "root".to_string(),
        extension: "root".to_string(),
        children: Vec::new()
    };

    let c1 = TreeNode {
        name: "child1".to_string(),
        full_path: "root".to_string(),
        extension: "root".to_string(),
        children: Vec::new()
    };

    let c2 = TreeNode {
        name: "child2".to_string(),
        full_path: "root".to_string(),
        extension: "root".to_string(),
        children: Vec::new()
    };

    root.children.push(c1);
    root.children.push(c2);
    print_tree(root);
}
