pub mod all_about_liferimes_two;
mod all_about_lifetimes;
mod all_about_refs;
pub mod enums_1;
pub mod full_fledged_binary_tree;
mod ownerhsip_and_moves;
pub mod patterns_1;
pub mod patterns_2;
pub mod ref_patterns;
pub mod results_1;
mod shared_ownership;
pub mod sharing_vs_mutation;
mod strings_tut;
pub mod structs_1;
pub mod structs_2;
pub mod test;

#[allow(unused_variables)]
#[allow(unused_mut)]

fn main() {
    // Rust basically has 3 pointer types
    // References, boxes and the unsafe pointers

    //* References
    {
        let mut num1 = 23;
        let ref_num1 = &num1;
        // let mut_ref_num1 = &mut num1;
        // println!("{}", ref_num1);
    }

    //* boxes */
    {
        //* used to alloate value in the heap */
        let t = (12, "eggs");
        let b = Box::new(t);
        // println!("{} {}", t.0, t.1);

        //* when b goes out of scope, the memory allocated in the heap is automatically destroyed */
    }

    //* ARRAYS VECTORS AND SLICES */
    //* lets startg with arrays */
    {
        // let num = [1, 2, 3, 4, 5];
        // println!("{}", num.len());
        // println!("{}", num[0]);
        // println!("{}", num.len());
        // let num = [true; 5];
        // const N: usize = 10;
        // let num = [true; N];
        // println!("{}", num.len());
        let mut num = [5, 7, 9, 10, 0];
        num.sort();
    }
    //* vectors */
    {
        let mut vec1: Vec<i32> = vec![1, 2, 3, 4];
        let last_val = vec1.pop();
        // last_val.unwrap_or_default();
    }
    //* now lets go with slices */
    {
        //*     A slice, written [T] without specifying the length, is a region of an array or vector. Since a slice can be any length, slices can’t be stored directly in variables or passed as function arguments. Slices are always passed by reference.*/
        //* A reference to a slice is a fat pointer: a two-word value comprising a pointer to the slice’s first element, and the number of elements in the slice. */
        let v = vec![1, 2, 3, 4, 5];
        let a = [1, 2, 3, 4];

        // let sv = &v; //* this is ref to a vector */
        // let sa = &a; //* this is a ref to an array */
        //* now lets change them to references to slices which is nothing but a portion of an array of vector */
        let sv: &[i32] = &v; //* automatic conversion */
        let sv: &[i32] = &a;

        //* lets get some slices */
        let slice1 = &v[0..3];
        // println!("{}", slice1[2]);
        // slice1[2] = 2;//* error */

        //* Since slices almost always appear behind references, we often just refer to types like &[T] or &str as “slices,” using the shorter name for the more common concept. */
    }
}
