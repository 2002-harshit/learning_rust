use std::fmt::Display;

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord + Display> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }

    fn preorder(&self) {
        match *self {
            BinaryTree::NonEmpty(ref node) => {
                println!("{}", node.element);
                node.left.preorder();
                node.right.preorder();
            }
            BinaryTree::Empty => return,
        }
    }
}

fn main() {
    let mut bt1: BinaryTree<i32> = BinaryTree::Empty;
    bt1.add(1);
    bt1.add(2);
    bt1.add(3);
    bt1.add(4);
    bt1.preorder();
}
