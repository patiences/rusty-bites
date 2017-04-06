enum BST {
    Node(i32, Box<BST>, Box<BST>),
    Empty
}

/// Inserts a node with this key into the tree rooted at root.
fn insert(root: Box<BST>, key: i32) -> Box<BST> {
    let copy = *root;
    return match copy {
        BST::Empty => Box::new(BST::Node(key, Box::new(BST::Empty), Box::new(BST::Empty))),
        BST::Node(data, left, right) => if key < data {
            Box::new(BST::Node(data, insert(left, key), right))
        } else {
            Box::new(BST::Node(data, left, insert(right, key)))
        },
    };
}

/// Returns the number of nodes in the tree rooted at this node.
fn num_nodes(root: &BST) -> i32 {
    return match *root {
        BST::Empty => 0,
        BST::Node(data, ref left, ref right) => 1 + num_nodes(left) +num_nodes(right),
    };
}

/// Returns the number of leaves in the tree rooted at this node.
fn num_leaves(root: &BST) -> i32 {
    return match *root {
        BST::Empty => 0,
        BST::Node(data, ref left, ref right) => {
            match **left {
                BST::Empty => {
                    match **right {
                        BST::Empty => return 1, // it's a leaf!
                        BST::Node(right_data, ref right_left, ref right_right) => num_leaves(left) + num_leaves(right),
                    }
                },
                BST::Node(left_data, ref left_left, ref left_right) => num_leaves(left) + num_leaves(right),
            }
        }
    };
}

/// Returns the height of node x.
fn height(x: &BST) -> i32 {
    return match *x {
        BST::Empty => -1,
        BST::Node(data, ref left, ref right) => {
            let height_left = height(left);
            let height_right = height(right);
            if height_left > height_right {
                return 1 + height_left;
            }
            return 1 + height_right;
        }
    }
}

/// Returns the depth of node x in the tree rooted at root.
fn depth(root: &BST, x: &BST) -> i32 {
    return match *x {
        BST::Empty => -1,
        BST::Node(x_data, ref x_left, ref x_right) => {
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
    }
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
    tree = insert(tree, 8);
    tree = insert(tree, 22);
    tree = insert(tree, 1);
    print(&tree);
    println!("Num nodes: {:?}", num_nodes(&tree));
    println!("Num leaves: {:?}", num_leaves(&tree));
    println!("Height: {:?}", height(&tree));

    tree = insert(tree, 17);
    tree = insert(tree, 20);
    tree = insert(tree, 2);
    tree = insert(tree, 9);
    print(&tree);

    println!("Num nodes: {:?}", num_nodes(&tree));
    let x = Box::new(BST::Node(17, Box::new(BST::Empty), Box::new(BST::Empty)));
    println!("Depth of 17 in tree: {:?}", depth(&tree, &x));
}
