// // use std::collections::HashMap;

// // /* All the pointer types we’ve seen so far—the simple Box<T>
// // heap pointer, and the pointers internal to String and Vec
// // values—are owning pointers: when the owner is dropped,
// // the referent goes with it */
// // #[allow(dead_code, unused_variables)]
// // fn show(table: &Table) {
// //     for (artist, works) in table {
// //         println!("Works by{}", artist);
// //         for work in works {
// //             println!("  {}", work);
// //         }
// //     }
// // }
// // type Table = HashMap<String, Vec<String>>;
// // fn main() {
// //     let mut table = Table::new();
// //     table.insert(
// //         "Gesualdo".to_string(),
// //         vec![
// //             "many madrigals".to_string(),
// //             "Tenebrae Responsoria".to_string(),
// //         ],
// //     );
// //     table.insert(
// //         "Caravaggio".to_string(),
// //         vec![
// //             "The Musicians".to_string(),
// //             "The Calling of St. Matthew".to_string(),
// //         ],
// //     );
// //     table.insert(
// //         "Cellini".to_string(),
// //         vec![
// //             "Perseus with the head of
// // Medusa"
// //                 .to_string(),
// //             "a salt cellar".to_string(),
// //         ],
// //     );

// //     show(&table);
// // }

// fn main() {
//     let x = 10;
//     let r = &x;
//     println!("{}", *r); //* but it is worthy noting that even printf r works , it is just a convenience provided by printf */
// }

fn main() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;

    assert_eq!((*anime_ref).name, "Aria: The Animation");
    assert_eq!(anime_ref.name, "Aria: The Animation");

    /*
    !Since references are so widely used in Rust, the . operator implicitly dereferences its left operand,

    ! The . operator can also implicitly borrow a reference to its left operand, if needed for a method call
    */

    let mut v = vec![5, 3, 9];
    // v.sort();
    //* the sort function requires a mut ref which the '.' operator borrows automatically */
    // (&mut v).sort();

    let x = 10;
    let rx = &x; //* this is a fucking simple cpp pointer */
    assert_eq!(*rx, x);
    let r2x = &x;
    println!("{}", std::ptr::eq(rx, r2x)); //* this is ued to compare the addresses, if you write */
    println!("{}", rx == r2x); //* these ==,>= etc operators see throught the reference and perform the operations on the values, which is UNIQUE!!! */
}
