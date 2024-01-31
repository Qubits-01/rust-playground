fn main() {
    let number: i32 = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    loop {
        println!("again!");
        break;
    }

    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count: i32 = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number: i32 = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut index: usize = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let b: [i32; 5] = [-1, -2, -3, -4, -5];
    for elem in b {
        println!("the value is: {elem}");
    }

    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!");
}
