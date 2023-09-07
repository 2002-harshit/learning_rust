enum Color {
    Black,
    Brown,
    Golden,
}

// struct Account {
//     name: &'static str,
//     language: &'static str,
//     id: u32,
//     status: &'static str,
//     address: &'static str,
//     birthday: &'static str,
//     eye_color: Color,
// }
struct Account {
    name: String,
    language: &'static str,
    id: u32,
    status: String,
    address: &'static str,
    birthday: String,
    eye_color: Color,
}

fn func(acc: &Account) {
    println!("hello");
}

fn main() {
    //* Lets study about teh reference patterns , 2 types (ref,&) */
    {
        //* note that you can actually move each value out of a struct, its not like a Vector, where this is not allowed , but then you wont be able to have any refs to it, because it will be partially moved*/
        let account = Account {
            name: "Harshit".to_string(),
            language: "Hindi",
            id: 1,
            status: "Married".to_string(),
            address: "Moti Nagar",
            birthday: "Oct".to_string(),
            eye_color: Color::Golden,
        };

        // * lets match this */
        // match account {
        //     Account {
        //         name,
        //         language,
        //         id: _,
        //         status: st, //* here account.status is moved */
        //         address: _,
        //         birthday: _,
        //         eye_color: _,
        //     } => {
        //         //* name is moved out */
        //         println!("{}", account.birthday);
        //     }
        // }
        //* shorter way of above */
        // match account {
        //     Account { name, language, .. } => {
        //         // func(&account);//* partially moved, so cannot take ref */
        //     }
        // }

        // println!("{}", account.birthday);
        // func(&account);//* partially moved cannot take ref */
        //*BUT WE WANT REFS TO account, so we shouldnt partiall move it, hence we should not move its components, and therefore give them by reference so !!!!!!!!!!!!!!!!!! REF and REF MUT */
        match account {
            Account {
                ref name, language, ..
            } => {
                //* here out of name and language, only movable value was name, rest were dont care, so we used "ref name" , to borrow account.name,now accounnt is still intact and not partially moved*/
                func(&account); //* WORKS */
            }
        }

        //* the other reference pattern is &, which is used to match references */
    }
}
