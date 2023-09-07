// fn main() {
//     // let v = vec!["harshit".to_string(), "garv".to_string()];
//     // let r = &v;
//     // let aside = v;
//     // println!("{}", r[0]); //* this would be a dangling ref */
//     //* Throughout its lifetime, a shared reference makes its referent read-only: you may not assign to the referent or move its value elsewhere. */
// }

// fn main() {
//     let mut v = vec!["harshit".to_string(), "garv".to_string()];
//     let ref_to_v = &v;
//     // v.push("ritu".to_string()); //* this is an  error since we have a ref to v, now v cannot be changed because for that we had to take a mutable reference */
//     println!("{}", ref_to_v[0]);
// }

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

fn main() {
    let mut wave: Vec<f64> = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    println!("{:?}", wave);

    // extend(&mut wave, &wave);

    //* taking a shared ref to something makes that "something" read-only, so you cannot have mutable refs to it simultaneously  */
}
