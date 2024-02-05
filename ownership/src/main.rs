fn main() {
    // [ VARIABLE SCOPE ]
    let s: &str = "Hello";

    {
        // s is not valid here, it’s not yet declared
        let s: &str = "hi"; // s is valid from this point forward
        println!("{s}");

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    println!("{s}");

    // [ THE STRING TYPE ]
    let mut s: String = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // [ MEMORY AND ALLOCATION ]
    {
        let _s: String = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid

    // [ VARIABLES AND DATA INTERACTING WITH MOVE ]
    let x: i32 = 5;
    let _y: i32 = x;

    let s1: String = String::from("hello");
    let _s2: String = s1;

    // println!("{s1}");

    // [ VARIABLE S AND DATA INTERACTING WITH CLONE ]
    let s1: String = String::from("hello");
    let s2: String = s1.clone();
    println!("{s1} {s2}");

    // [ STACK-ONLY DATA: COPY ]
    let x: i32 = 5;
    let y: i32 = x;
    println!("{x} {y}");

    // [ OWNERSHIP AND FUNCTIONS ]
    let s: String = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{s}");

    let x: i32 = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    // [ RETURN VALUES AND SCOPE ]
    let _s1: String = gives_ownership(); // gives_ownership moves its return
                                         // value into s1
    let s2: String = String::from("hello"); // s2 comes into scope
    let _s3: String = takes_and_gives_back(s2); // s2 is moved into
                                                // takes_and_gives_back, which also
                                                // moves its return value into s3

    let s1: String = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
} // [ OWNERSHIP AND FUNCTIONS ]
  // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

// [ RETURN VALUES AND SCOPE ]
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

// [ OWNERSHIP AND FUNCTIONS ]
fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// [ RETURN VALUES AND SCOPE ]
fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len(); // len() returns the length of a String

    (s, length)
}
