// struct Extrema<'a> {
//     greatest: &'a i32,
//     least: &'a i32,
// }

// fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
//     let mut greatest = &slice[0];
//     let mut smallest = &slice[0];

//     for i in 1..slice.len() {
//         if slice[i] > *greatest {
//             greatest = &slice[i];
//         }
//         if slice[i] < *smallest {
//             smallest = &slice[i];
//         }
//     }

//     Extrema {
//         greatest,
//         least: smallest,
//     }
// }

// fn main() {}

// #[derive(Clone, Copy, PartialEq)]
// struct Point {
//     x: f32,
//     y: f32,
// }

// //* Each of these traits can be implemented automatically for a struct, provided that each of its fields implements the trait. We can ask Rust to derive PartialEq for Point because its two fields are both of type f64, which already implements PartialEq. */
//* INTERIOR MUTABILITY */
struct SpiderRobot {
    species: String,
    web_enabled: bool,
}

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct SpiderSenses {
    robot: Rc<SpiderRobot>,
    eyes: [i32; 32],
}

//* lets see scenraio, as we saw above all the parts of the spiderRobot contsain an rc to the spiderRobot, that means it is always shared and hence it is imuutbale or read only, but what if we want to chnage some things of that struct, here comes Cell<T> or RefCell<T> */
//* Cell<T>, you can chnage the valuen in cell without the mut access or a mutable reference */
//* ALSO IT IS WOTH NOTING THAT WITH ceLL::GET , YOU GET A COPY OF THE VALUE, SO IT SHOULD IMPLEMENT COPY TRAIT, here comes in picture RefCell, where you can have borrowing of its value */
// struct demo {
//     count: Cell<u32>,
// }

// impl demo {
//     fn add_count(&self) {
//         self.count.set(self.count.get() + 1);
//         //* even with a shared ref, it can be updated, but how does this help us */
//     }
// }

fn main() {
    let ref_cell = RefCell::new("harshit".to_string());
    let r = ref_cell.borrow(); //* a shared ref */
    let w = ref_cell.borrow_mut(); //*a  mut ref*/
                                   //* see right now yow wont get any errors, because these check the rust rules at thr run time */
}
