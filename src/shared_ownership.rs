use std::rc::Rc;

//* lets deep dive rc and arc */
#[allow(unused_variables, dead_code)]
fn main() {
    // let str1 = "harshit".to_string();
    // let str2 = str1.clone(); //* here the string value is copied, so now you have 2 harshits, what if we dont want this, but we want to have multiple onwners */
    let str1 = Rc::new(String::from("harshit"));
    let str2 = str1.clone();
    // let str3 = str2.clone();or
    let str3 = str1.clone();
    println!("{}", Rc::strong_count(&str1));

    /*
     *For mind you its any type,any type T, an Rc<T> value is a pointer to a heap-allocated T that has had a reference count affixed to it.
     *Cloning an Rc<T> value does not copy the T; instead, it simply creates another pointer to it and increments the reference count. */

    /*
    ! also these are immutable,because you have multiple readers
     */
    // str1.push('p');
}
