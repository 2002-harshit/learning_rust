#[allow(dead_code, unused_variables, unused_mut)]

fn main() {
    {
        let str1 = "noodles".to_string();
        let str_slice_str1 = &str1[0..2];
        //* the above is a string slice, is a reference to a run of UTF-8 text owned by someone else: it “borrows” the text. */
        //* like a slice/ref to a slice, &str is also a fat pointer */
        let poodles = "आ_आ"; //* this is a string literal and is also a fat pointer, */
                             //* the "आ_आ" is stored in the program read only memory */
        println!("{}", poodles.len());
    }
}
