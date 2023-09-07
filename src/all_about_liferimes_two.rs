// fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
//     let mut min = &v[0];
//     for element in &v[1..] {
//         if (*element < *min) {
//             min = element;
//         }
//     }
//     min
// }

// fn main() {
//     let vec = vec![1, 2, 3, 4, -1];
//     let ref_to_smallest = smallest(&vec);
//     println!("{}", *ref_to_smallest);
// }

// fn testFunc<'a>(names: &'a Vec<String>) {
//     // let harshit = (*names)[0];
// }

// fn main() {
//     let names = vec!["harshit".to_string(), "peeya".to_string()];
// }

// fn main() {
//     struct S<'a> {
//         r: &'a i32,
//     }
//     let s;
//     {
//         let x = 10;
//         s = S { r: &x };
//     }
//     assert_eq!(*(s.r), 10);
// }

// struct S<'a, 'b> {
//     x: &'a i32,
//     y: &'b i32,
// }

// fn main() {
//     let x = 10;
//     let r;
//     {
//         let y = 20;
//         {
//             let s = S { x: &x, y: &y };
//             r = s.x;
//         }
//     }
//     println!("{}", r);
// }
