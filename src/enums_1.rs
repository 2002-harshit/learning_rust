// // enum TimeUnit {
// //     Seconds,
// //     Minutes,
// //     Hours,
// //     Days,
// //     Months,
// //     Years,
// // }

// // impl TimeUnit {
// //     fn giveSeconds(self) -> i32 {
// //         match self {
// //             TimeUnit::Seconds => 1,
// //             TimeUnit::Minutes => 60,
// //             TimeUnit::Hours => 60 * 60,
// //             TimeUnit::Days => 24 * 60 * 60,
// //             TimeUnit::Months => 30 * 24 * 60 * 60,
// //             TimeUnit::Years => 365 * 24 * 60 * 60,
// //         }
// //     }
// // }

// // fn main() {
// //     println!("{}", TimeUnit::Days.giveSeconds())
// // }

// //* ENUMS WITH DATA */
// enum TimeUnit {
//     Seconds,
//     Minutes,
//     Hours,
//     Days,
//     Months,
//     Years,
// }

// enum RoughTime {
//     InThePast(TimeUnit, u32), //* these are tuple variants */
//     JustNow,
//     InTheFuture(TimeUnit, u32),
// }

// //* enums can also have struct variants */
// struct Point3d {
//     x: f32,
//     y: f32,
//     z: f32,
// }
// enum Shape {
//     Sphere { center: Point3d, radius: f32 },
//     Cuboid { corner1: Point3d, corner2: Point3d },
// }

use std::fmt::Display;

// fn main() {
//     let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Years, 3);
// }
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

//* this is a trait bound, which says that T is something that implements the Display trait */
fn preorder<T: Display>(root: &BinaryTree<T>) -> () {
    match root {
        BinaryTree::Empty => return,
        BinaryTree::NonEmpty(node) => {
            println!("{}", node.element);
            preorder(&node.left);
            preorder(&node.right);
        }
    }
}

fn main() {
    let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let mercury_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "mercury",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let root = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "root",
        left: mercury_tree,
        right: jupiter_tree,
    }));

    preorder::<&str>(&root);
}
