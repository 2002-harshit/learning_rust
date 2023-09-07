#[allow(dead_code, unused_variables, unused_mut)]
// fn main() {
//     struct Person {
//         name: String,
//         birth: i32,
//     }
//     let mut composers = Vec::new();
//     //* since composers is a vec, its elements are stoed in the heap */
//     composers.push(Person {
//         name: "Palestrina".to_string(),
//         birth: 1525,
//     });
//     composers.push(Person {
//         name: "Dowland".to_string(),
//         birth: 1563,
//     });
//     composers.push(Person {
//         name: "Lully".to_string(),
//         birth: 1632,
//     });
//     for composer in &composers {
//         println!("{}, born {}", composer.name, composer.birth);
//     }
// }

// fn main() {
// let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
// let t = s;
// println!("{:?}", s); //* this is an error because the vbalue is moved or the ownership is transferred from s to t */
//* but what if you want a copy instead of move */
// let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
// let t = s.clone();

//
//* the previous value harshilet mut s = "harshit".to_string();
// s = "peeya".to_string(); is dropped */
// struct S {
// stri: String,
// }
// let s = "harshit".to_string();
// let s_obj = S { stri: s };
// println!("{}", s); //* again the value is moved to the structure or the ownership is transferred to the structure */
// {
//     //* letgs see some moves in the loop */
//     let mut s = "harshit".to_string();
//     let mut i = 5;
//     while i > 0 {
//         func(s);
//         /*
//         !this is an error because s will be moved in the very firstg iteration, and will be uninitialised in the next iteration which is wrong
//         * but we can fix it by assigning a new value in each iter
//          */
//         s = i.to_string();
//         i = i - 1;
//     }
// }
// }

// fn func(s: String) {
//     todo!("ainve hi function, just tok showcase move")
// }

//* -------------------------------------------- MOVES AND THE INDEXED CONTENT */
//* We’ve mentioned that a move leaves its source uninitialized, as the destination takes ownership of the value. But not every kind of value owner is prepared to become uninitialized */
// fn main() {
// let mut v: Vec<String> = Vec::new();
// for i in 101..106 {
//     v.push(i.to_string());
// }

// println!("{:?}", v);

// let second = v[1];
// let fifth = v[4]; //* this is prohibhited, Rust would somehow need to remember that the second and fifth elements of the vector have become uninitialized, and track that information until the vector is dropped. In the most general case, vectors would need to carry around extra information with them to indicate which elements are live and which have become uninitialized. */
// let mut v: Vec<String> = vec![
//     "harshit".to_string(),
//     "peeya".to_string(),
//     "garv".to_string(),
// ];

// for string in v {
//     println!("{}", string);
//     // func(v);
// }

/*
! Ek pate ki baat, when you write "for string in v,, THE VECTOR V IS FUCKING MOVED INTO THE FOR LOOP, so it unusabnle now"
*/
// println!("{:?}", v);//* this is wrong */
// struct Person {
// name: Option<String>,
// birth: i32,
// }
// let mut composers = vec![Person {
// name: Some("Harshit".to_string()),
// birth: 23,
// }];

// let first_name = composers[0].name; //* this is an error */
// let first_name = std::mem::replace(&mut composers[0].name, None);
//* now the ownership of harshit is transferred to first_name */
// }

// fn func(vec: Vec<String>) {}

/*
* --------------------Now lets see some values that are copied,
*The standard Copy types include all the machine integer and floating-point numeric types, the char and bool types, and a few others. A tuple or fixed-size array of Copy types is itself a Copy type. */

/*
! What about types you define yourself? By default, struct and enum types are not Copy:
 */
#[derive(Clone, Copy)]
struct Label {
    number: i32,
}

#[allow(dead_code, unused_variables, unused_mut)]
fn print(l: Label) {
    println!("STAMP: {}", l.number)
}
#[allow(dead_code, unused_variables, unused_mut)]
fn main() {
    let l = Label { number: 3 };
    print(l); //* l is moved here, even though it contains a copy type, by default struct and enums are non copy tyes */

    /*  If
     *If all the fields of your struct are themselves Copy, then you can make the type Copy as well by placing the attribute
     * #[derive(Copy, Clone)]
     */
}
