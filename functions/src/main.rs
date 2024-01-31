fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'm');

    let _y: i32 = 6;
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let five: i32 = five();
    println!("The value of five is: {five}");

    let x: i32 = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The value of x is: {value} {unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
