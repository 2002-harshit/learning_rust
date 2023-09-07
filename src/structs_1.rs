// // // // //* named field structs */
// // // // struct GrayscaleMap {
// // // //     pixels: Vec<u8>,
// // // //     size: (usize, usize),
// // // // }

// // // // //* tuple like structs */
// // // // struct Bounds(usize, usize);

// // // // //* unit like structs */
// // // // struct Onesuch;
// // // // fn main() {
// // // //     {
// // // //         let width = 1024;
// // // //         let height = 576;
// // // //         let image = GrayscaleMap {
// // // //             pixels: vec![0; width * height],
// // // //             size: (width, height),
// // // //         };
// // // //     }

// // // //     {
// // // //         let image_bounds = Bounds(2, 3);
// // // //     }
// // // // }

// // // struct Queue {
// // //     older: Vec<char>,
// // //     younger: Vec<char>,
// // // }

// // // impl Queue {
// // //     fn push(&mut self, c: char) {
// // //         self.younger.push(c);
// // //     }

// // //     fn pop(&mut self) -> Option<char> {
// // //         if self.older.is_empty() {
// // //             if self.younger.is_empty() {
// // //                 return None;
// // //             }

// // //             use std::mem::swap;
// // //             swap(&mut self.older, &mut self.younger);
// // //             self.older.reverse();
// // //         }

// // //         self.older.pop()
// // //     }
// // // }

// // // fn main() {
// // //     let mut q = Queue {
// // //         older: Vec::new(),
// // //         younger: Vec::new(),
// // //     };

// // //     // q.push('a');//* this is same as the below line*/
// // //     // (&mut q).push('a');

// // //     let mut boxed_q = Box::new(Queue {
// // //         older: Vec::new(),
// // //         younger: Vec::new(),
// // //     });

// // use std::rc::Rc;

// // //     boxed_q.push('a'); //*  rust automatically borrows a &mut self from the Box */
// // // }
// // struct Node {
// //     tag: String,
// //     children: Vec<Rc<Node>>,
// // }

// // impl Node {
// //     fn new(tag: &str) -> Node {
// //         Node {
// //             tag: tag.to_string(),
// //             children: vec![],
// //         }
// //     }

// //     fn append_to_parent(self: Rc<Self>, parent: &mut Node) {
// //         parent.children.push(self);
// //     }
// // }

// // fn main() {
// //     let mut root = Node::new("root");
// //     let left = Rc::new(Node::new("left"));
// //     // left.append_to_parent(&mut root);
// //     // println!("{}", left.tag);//* here leftg will be unitilaised since we have transferred the ownerhip to the vector, and hence the reference count also remnains same */
// //     //* if we want to retain left, we can pass a clone, but then ref count will increment by 1 */
// //     left.clone().append_to_parent(&mut root);

// //     let right = Node::new("right");
// //     Rc::new(right).append_to_parent(&mut root);
// //     println!(
// //         "Ref count of left {} and ref count of right {}",
// //         Rc::strong_count(&root.children[0]), //* this is 2 because is left and the root.children[0] both are pointing it */
// //         Rc::strong_count(&root.children[1])
// //     );
// // }

// // //* associated consts */
// // struct Vector2 {
// //     x: f32,
// //     y: f32,
// // }

// // impl Vector2 {
// //     const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
// //     const UNIT_X: Vector2 = Vector2 { x: 1.0, y: 0.0 };
// //     const name: &'static str = "harshit";
// // }

// //* GENERIC STRUCTS */
// struct Queue<T> {
//     elements: Vec<T>,
// }

// //* “for any type T, here are some associated functions available on Queue<T>. */
// impl<T> Queue<T> {
//     fn new() -> Self {
//         Queue {
//             elements: Vec::<T>::new(),
//         }
//     }
//     fn is_empty(&self) -> bool {
//         if self.elements.len() == 0 {
//             return true;
//         }

//         false
//     }
// }

// //* these are assocuiated functions for the Queue<char> */
// impl Queue<char> {
//     // fn add(&self) -> Option<String> {
//     //     if (self.elements.len() == 0) {
//     //         return None;
//     //     }
//     //     let mut ans = String::new();

//     //     for ch in &self.elements {
//     //         ans.push(*ch);
//     //     }

//     //     Some(ans)
//     // }
// }

// fn main() {
//     // let q = Queue::<i32>::new();
// }
