fn main() {
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {len}.", s1);

    // let s: String = String::from("hello");
    // change(&s);

    // [ MUTABLE REFERENCES ]
    let mut s: String = String::from("hello");
    change(&mut s);
    println!("{s}");

    // let mut s: String = String::from("hello");
    // let r1: &mut String = &mut s;
    // let r2: &mut String = &mut s;
    // println!("{r1}, {r2}");

    let mut s: String = String::from("hello");
    {
        let _r1: &mut String = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2: &mut String = &mut s;
    println!("{r2}");

    let mut s: String = String::from("hello");
    let _r1: &String = &s; // no problem
    let _r2: &String = &s; // no problem
    let r3: &mut String = &mut s; // BIG PROBLEM
                                  // println!("{r1}, {r2}, {r3}");
    println!("{r3}");

    let mut s: String = String::from("hello");
    let r1: &String = &s; // no problem
    let r2: &String = &s; // no problem
    println!("{r1}, {r2}");
    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");

    // [ DANGLING REFERENCES ]
    // let reference_to_nothing: &String = dangle();
    println!("{}", no_dangle());
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(", world!");
// }

// [ MUTABLE REFERENCES ]
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// [ DANGLING REFERENCES ]
// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s: String = String::from("hellow");

    s
}
