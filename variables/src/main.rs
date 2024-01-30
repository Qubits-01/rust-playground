use std::{f32::consts::E, io};

fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let x: i32 = 5;
    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces: &str = "    ";
    let spaces: usize = spaces.len();
    println!("The value of spaces is: {spaces}");

    // let mut spaces = "    ";
    // spaces = spaces.len();

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let truncated_quotient = -5 / 3;
    println!("The value of truncated_quotient is: {truncated_quotient}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    let t = true;
    println!("The value of t is: {t}");
    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {f}");

    let c: char = 'z';
    println!("The value of c is: {c}");
    let z: char = 'ℤ';
    println!("The value of z is: {z}");
    let heart_eyed_cat: char = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:#?}", tup);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {x}, {y}, {z}");
    let five_hundred: i32 = tup.0;
    println!("The value of five_hundred is: {five_hundred}");
    let unit = ();
    println!("The value of unit is: {:#?}", unit);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:#?}", a);

    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let b: [i32; 5] = [3; 5];
    println!("The value of b is: {:#?}", b);
    let first: i32 = b[0];
    println!("The value of first is: {first}");
    let second: i64 = b[1].into();
    println!("The value of second is: {second}");

    let c: [i32; 5] = [6, 7, 8, 9, 10];

    println!("Please enter an array index.");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: i32 = c[index];

    println!("The value of the element at index {index} is: {element}");
}
