use crate::tree::*;

//Retuns vector of best-matching paths
//after traversing to prefix, also returns non-null child nodes below best-matching node
pub fn search_trie(tree: &Vec<TreeNode>, key: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut key_bit_index = 0;
    let mut key_byte_index = 0;
    let mut vec_index = 0;
    loop {
        // if vec index is out of bounds, we have reached the end of the tree
        if vec_index >= tree.len() {
            break;
        }
        let curr_str_bit = key.as_bytes()[key_byte_index] & (1 << key_bit_index);
        let current_node = &tree[vec_index];
        let left_child = 2 * vec_index + 1;
        let right_child = 2 * vec_index + 2;

        //if we traverse non empty value, add to list
        if current_node.value[0] !=  TRIE_KEY_EMPTY as u8 {
            let path = String::from_utf8(current_node.path.to_vec())
                .unwrap()
                .trim_matches(char::from(TRIE_KEY_EMPTY))
                .to_string();
            results.push(path);
        }
        
        //if not end of tree, keep traversing
        if key_byte_index < key.len() {
            if curr_str_bit != 0 {
                vec_index = right_child
            } else {
                vec_index = left_child
            }
        } else {
            let children = collect_all_children(tree, vec_index);
            results.extend(children);
            break;
        }

        key_bit_index += 1;
        if key_bit_index == 8 {
            key_bit_index = 0;
            key_byte_index += 1;
        }
    }

    return results;
}

pub fn collect_all_children(tree: &Vec<TreeNode>, vec_index: usize) -> Vec<String> {
    let mut results = Vec::new();
    let left_child = 2 * vec_index + 1;
    let right_child = 2 * vec_index + 2;
    if left_child < tree.len() {
        let left_results = collect_all_children(tree, left_child);
        results.extend(left_results);
    }
    if right_child < tree.len() {
        let right_results = collect_all_children(tree, right_child);
        results.extend(right_results);
    }
    let current_node = &tree[vec_index];
    if current_node.value[0] != TRIE_KEY_EMPTY as u8 {
        let path = String::from_utf8(current_node.path.to_vec())
            .unwrap()
            .trim_matches(char::from(TRIE_KEY_EMPTY))
            .to_string();
        results.push(path);
    }
    return results;
}

pub fn insert_trie(tree: &mut Vec<TreeNode>, key: String, path: String) {
    let mut key_bit_index = 0;
    let mut key_byte_index = 0;
    let mut vec_index = 0;
    //seek most similar node
    loop {
        if key_byte_index < key.len() {
            let curr_str_bit = key.as_bytes()[key_byte_index] & (1 << key_bit_index);
            let left_child = 2 * vec_index + 1;
            let right_child = 2 * vec_index + 2;
            if curr_str_bit != 0 {
                vec_index = right_child;
            } else {
                vec_index = left_child;
            }

            //Found leaf before finding key. Extend tree and write value to leaf
            if vec_index >= tree.len() {
                tree.resize(vec_index, TreeNode{value: [TRIE_KEY_EMPTY as u8; TREE_NODE_VALUE_SIZE], path: [TRIE_KEY_EMPTY as u8; TREE_NODE_PATH_SIZE]}); //create nodes filled with invalid_val
                tree.push(TreeNode {
                    value: string_to_padded_array::<TREE_NODE_VALUE_SIZE>(key, TRIE_KEY_EMPTY as u8),
                    path: string_to_padded_array::<TREE_NODE_PATH_SIZE>(path, TRIE_KEY_EMPTY as u8),
                });
                break;
            //If current node is a non-empty leaf, we need to decide how to conile it with the
            //inserted node.
            } else if tree.get(vec_index).unwrap().value[0] != TRIE_KEY_EMPTY as u8 {
                //Therefore, we'll split this node into 2 and add an empty parent node. Both nodes
                let old_node_copy = tree.get(vec_index).unwrap().clone();
                let new_node = TreeNode {
                    value: string_to_padded_array::<TREE_NODE_VALUE_SIZE>(key, TRIE_KEY_EMPTY as u8),
                    path: string_to_padded_array::<TREE_NODE_PATH_SIZE>(path, TRIE_KEY_EMPTY as u8),
                };

                //Check who goes left and who goes right
                let mut old_node_bit: u8;
                let mut new_node_bit: u8;
                loop {
                    old_node_bit = old_node_copy.value[key_byte_index] & (1 << key_bit_index);
                    new_node_bit = new_node.value[key_byte_index] & (1 << key_bit_index);
                    if old_node_bit != new_node_bit {
                        break;
                    }
                    key_bit_index += 1;
                    if key_bit_index == 8 {
                        key_bit_index = 0;
                        key_byte_index += 1;
                    }
                }

                let old_node = tree.get_mut(vec_index).unwrap();
                //case 1 -> both nodes are similar in the radical but differ later -> create new parent and
                //put them as children
                if old_node_copy.value[key_byte_index] != TRIE_KEY_EMPTY as u8 && new_node.value[key_byte_index] != TRIE_KEY_EMPTY as u8 {
                    //reset parent
                    old_node.value = [TRIE_KEY_EMPTY as u8; TREE_NODE_VALUE_SIZE];
                    old_node.path = [TRIE_KEY_EMPTY as u8; TREE_NODE_PATH_SIZE];

                    let left_child = 2 * vec_index + 1;
                    let right_child = 2 * vec_index + 2;
                    //Resize into right child
                    tree.resize(right_child + 1, TreeNode{value: [TRIE_KEY_EMPTY as u8; TREE_NODE_VALUE_SIZE], path: [TRIE_KEY_EMPTY as u8; TREE_NODE_PATH_SIZE]}); //create nodes filled with invalid_val
                    if new_node_bit != 0 {
                        tree[right_child] = new_node;
                        tree[left_child] = old_node_copy;
                    } else {
                        tree[right_child] = old_node_copy;
                        tree[left_child] = new_node;
                    }
                }
                //case 2 -> one of the nodes is a substring of the other -> bigger one goes as child,
                //substring goes as parent
                else {
                    let parent: TreeNode;
                    let child: TreeNode;
                    if old_node_copy.value[key_byte_index] == TRIE_KEY_EMPTY as u8 {
                        parent = old_node_copy;
                        child = new_node;
                    } else {
                        parent = new_node;
                        child = old_node_copy;
                    }

                    //Write assigned parent into parent position
                    *old_node = parent;

                    //determine child position
                    let desired_position: usize;
                    if child.value[key_byte_index] & (1 << key_bit_index) != 0 {
                        desired_position = 2 * vec_index + 2; //right
                    } else {
                        desired_position = 2 * vec_index + 1; //left
                    }

                    //fill tree with empty nodes until desired position
                    tree.resize(desired_position + 1, TreeNode{value: [TRIE_KEY_EMPTY as u8; TREE_NODE_VALUE_SIZE], path: [TRIE_KEY_EMPTY as u8; TREE_NODE_PATH_SIZE]});
                    tree[desired_position] = child;
                }
                break;
            }
        } else {
            // Found existing node matching the key.
            // Write data to correct node and quit
            let new_node = tree.get_mut(vec_index).unwrap();
            new_node.value = string_to_padded_array::<TREE_NODE_VALUE_SIZE>(key, TRIE_KEY_EMPTY as u8);
            new_node.path = string_to_padded_array::<TREE_NODE_PATH_SIZE>(path, TRIE_KEY_EMPTY as u8);
            return;
        }

        key_bit_index += 1;
        if key_bit_index > 7 {
            key_bit_index = 0;
            key_byte_index += 1;
        }
    }
}


//string must be smaller than arr
fn string_to_padded_array<const N: usize>(key: String, pad_val: u8) -> [u8; N] {
    let mut array = [pad_val; N];
    let len = key.len();
    array[..len].copy_from_slice(key.as_bytes());
    array
}

//each initial u8 value becomes ID ranked from 0 to 37
//alphabet reduction helps keep trie size small, as we only need to store 37 values with 6 bits (26 alphabetic chars, 10 numbers, 1 invalid char)
//values,however, are stored as-is for comparsion purposes 
//some values might replace others but this heuristic is acceptable
//pub fn optimize_trie_key(key: String) -> String {
//    let optimized_key = key.chars().map(|c| {
//        let mut new_c = c as u8;
//        if c.is_alphabetic() {
//            if c.is_uppercase() {
//                new_c -= 65;
//            } else {
//                new_c -= 97;
//            }
//        } else if c.is_numeric() {
//            new_c += 26;
//        } else {
//            new_c = 63 //top of 6bit representation mapped as * invalid char
//        }
//        return new_c as char;
//    }).collect();
//
//    return optimized_key;
//}

//pub const TRIE_KEY_WILDCARD: char = 1 as char;
pub const TRIE_KEY_EMPTY: char = 0 as char;
