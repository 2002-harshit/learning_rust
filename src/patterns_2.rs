// use std::fmt::Display;

// //* tuple and struct patterns */
// fn describe_point(x: i32, y: i32) -> &'static str {
//     use std::cmp::Ordering::*;

//     match (x.cmp(&0), y.cmp(&0)) {
//         (Equal, Equal) => "at the origin",
//         (_, Equal) => "on the x axis",
//         (Equal, _) => "on the y axis",
//         (Greater, Greater) => "in the first quadrant",
//         (Less, Greater) => "in the second quadrant",
//         _ => "Somewhere else",
//     }
// }

// #[derive(Clone, Copy)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     // println!("{}", describe_point(0, 0));
//     let p1 = Point { x: 2, y: 3 };

//     match p1 {
//         Point { x: 0, y: height } => println!("straight up {} metres", height),
//         Point { x, y } => println!("at ( {}m {}m )", x, y),
//     }
// }

// struct Large {
//     name: String,
//     language: String,
//     age: u8,
//     status: String,
//     ip_addr: (u8, u8, u8, u8),
//     location: String,
// }

// fn main() {
//     let l1 = Large {
//         name: "Harshit".to_string(),
//         language: "Hindi".to_string(),
//         age: 20,
//         status: "Commited".to_string(),
//         ip_addr: (192, 168, 1, 1),
//         location: "Hawaii".to_string(),
//     };

//     match l1 {
//         Large { name, language, .. } => println!("Found"),
//     }
// }
