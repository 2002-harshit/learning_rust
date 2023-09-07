//* lets deep dive into some rust lifetimes */
// #[allow(dead_code, unused_assignments, unused_variables)]
// fn main() {
//     {
//         let r;
//         {
//             let x = 1;
//             r = &x;
//         }
//         // assert_eq!(*r, x);
//     }

//     {
//         let v = vec![1, 2, 3];
//         let first = &v[1];
//         //* Since v owns the vector, which owns its elements, the lifetime of v must enclose that of the reference type of &v[1]. Similarly, if you store a reference in some data structure, its lifetime must enclose that of the data structure. , you have to look from both the sides, rhs-> v's lifetime must enclose &v[1] lifteime, also lhs->till first is getting used, the reference inside it should be valid */
//     }
// }
// #[allow(dead_code, unused_assignments, unused_variables)]
// static mut STASH: &i32 = &57; /* these have to be initiliased, a static is a global variable */
// fn f(p: &'static i32) {
//     unsafe {
//         STASH = p;
//     }
//     //* aur ek static lifetime wali cheez khud ek static hi hogi */
// }

//*  You can read <'a> as “for any lifetime 'a” so when we write fn f<'a>(p: &'a i32), we’re defining a function that takes a reference to an i32 with any given lifetime 'a. */
// fn main() {
// {
//     static MY_STATIC: i32 = 1000;
//     f(&MY_STATIC);
// }
// }

// fn func<'a>(p: &'a i32) {
//     todo!("karenge abhi tumhein");
// }

// fn func(p: &'static i32) {
//     todo!();
// }
// fn main() {
//     let x = 10;
//     // func(&x); //*  yeh galat hai kyunki p is stat*/
// }

//* RETURNING REFS */
fn smallest(v: &[i32]) -> &i32 {
    let mut min = &v[0];
    for element in &v[1..] {
        if (*element < *min) {
            min = element;
        }
    }
    min
}

fn main() {
    let vec = vec![1, 2, 3, 4, -1];
    let ref_to_smallest = smallest(&vec);
    println!("{}", *ref_to_smallest);
}
