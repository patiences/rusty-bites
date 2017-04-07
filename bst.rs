enum BST {
    Node(i32, Box<BST>, Box<BST>),
    Empty,
}

/// Inserts a node with this key into the tree rooted at root.
///     Don't do this, taking ownership causes unnecessary allocations/deallocations.
fn insert_taking_ownership(root: Box<BST>, key: i32) -> Box<BST> {
    let copy = *root;
    return match copy {
        BST::Empty => Box::new(BST::Node(key, Box::new(BST::Empty), Box::new(BST::Empty))),
        BST::Node(data, left, right) => if key < data {
            Box::new(BST::Node(data, insert_taking_ownership(left, key), right))
        } else {
            Box::new(BST::Node(data, left, insert_taking_ownership(right, key)))
        }
    };
}

/// Inserts a node with this key into the tree rooted at root (borrowing root as a mutable reference).
fn insert(root: &mut BST, key: i32) -> () {
    match *root {
        BST::Empty => *root = BST::Node(key, Box::new(BST::Empty), Box::new(BST::Empty)),
        BST::Node(data, ref mut left, ref mut right) => if key < data {
            insert(left, key);
        } else {
            insert(right, key);
        }
    }
}

/// Finds the node with the given key in the tree, if it exists,
///     else returns a reference to an empty tree.
fn find(root: &BST, key: i32) -> &BST {
    match *root {
        BST::Empty => root,
        BST::Node(data, ref left, ref right) => {
            if data == key{
                root
            }
            else if key < data {
                find(left, key)
            } else {
                find(right, key)
            }
        }
    }
}

/// Finds the parent of the node with the key in the tree rooted at root.
fn find_parent(root: &BST, key: i32) -> &BST {
    match *root {
        BST::Empty => root,
        BST::Node(data, ref left, ref right) => {
            let found_parent = match **left {
                BST::Empty => false,
                BST::Node(data, _, _) => data == key,
            } || match **right {
                BST::Empty => false,
                BST::Node(data, _, _) => data == key,
            };

            if found_parent {
                root
            } else if key < data {
                find_parent(left, key)
            } else {
                find_parent(right, key)
            }
        }
    }
}

/// Returns the number of nodes in the tree rooted at this node.
fn num_nodes(root: &BST) -> i32 {
    return match *root {
        BST::Empty => 0,
        BST::Node(_, ref left, ref right) => 1 + num_nodes(left) + num_nodes(right),
    };
}

/// Returns the number of leaves in the tree rooted at this node.
fn num_leaves(root: &BST) -> i32 {
    return match *root {
        BST::Empty => 0,
        BST::Node(_, ref left, ref right) => {
            match **left {
                BST::Empty => {
                    match **right {
                        BST::Empty => return 1, // it's a leaf!
                        BST::Node(_, _, _) => num_leaves(left) + num_leaves(right),
                    }
                },
                BST::Node(_, _, _) => num_leaves(left) + num_leaves(right),
            }
        }
    };
}

/// Returns the height of node x.
fn height(x: &BST) -> i32 {
    return match *x {
        BST::Empty => -1,
        BST::Node(_, ref left, ref right) => {
            let height_left = height(left);
            let height_right = height(right);
            if height_left > height_right {
                return 1 + height_left;
            }
            return 1 + height_right;
        }
    };
}

/// Returns the depth of node x in the tree rooted at root.
fn depth(root: &BST, x: &BST) -> i32 {
    return match *x {
        BST::Empty => -1,
        BST::Node(x_data, _, _) => {
            match *root {
                BST::Empty => -1,
                BST::Node(root_data, ref root_left, ref root_right) => {
                    if x_data == root_data {
                        return 0;
                    }
                    if x_data < root_data {
                        return 1 + depth(root_left, x);
                    }
                    return 1 + depth(root_right, x);
                }
            }
        }
    };
}

/// Prints the tree sideways.
fn print(root: &BST) -> () {
    println!("Printing tree: ");
    print_helper(root, 0)
}

fn print_helper(root: &BST, depth: i32) -> () {
    match *root {
        BST::Empty => (),
        BST::Node(data, ref left, ref right) => {
            //println!("{:?}", data);
            print_helper(right, depth+1);
            print!("{}", "  ".repeat(depth as usize));
            println!("{:?}", data);
            print_helper(left, depth+1);
    }
  }
}

fn main() {
    let mut tree = Box::new(BST::Empty);
    insert(&mut tree, 8);
    insert(&mut tree, 22);
    insert(&mut tree, 1);
    print(&tree);
    println!("Num nodes: {:?}", num_nodes(&tree));
    println!("Num leaves: {:?}", num_leaves(&tree));
    println!("Height: {:?}", height(&tree));

    insert(&mut tree, 17);
    insert(&mut tree, 20);
    insert(&mut tree, 2);
    insert(&mut tree, 9);
    print(&tree);

    println!("Num nodes: {:?}", num_nodes(&tree));
    let x = Box::new(BST::Node(17, Box::new(BST::Empty), Box::new(BST::Empty)));
    println!("Depth of 17 in tree: {:?}", depth(&tree, &x));
}
