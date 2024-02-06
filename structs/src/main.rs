// [ DEFINING AND INSTANTIATING STRUCTS]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// [ USING TUPLE STRUCTS WITHOUT NAMED FIELDS TO CREATE DIFFERENT TYPES ]
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// [ UNIT-LIKE STRUCTS WITHOUT ANY FIELDS ]
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    // [ DEFINING AND INSTANTIATING STRUCTS]
    let _user1: User = User {
        active: true,
        username: String::from("test1"),
        email: String::from("test1@gmail.com"),
        sign_in_count: 1,
    };

    let mut user1: User = User {
        active: true,
        username: String::from("test1"),
        email: String::from("test1@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("test2@gmail.com");

    // [ CREATING INSTANCES FROM OTHER INSTANCES WITH STRUCT UPDATE SYNTAX ]
    let user1: User = build_user(String::from("test1@gmail.com"), String::from("test1"));
    let user2: User = User {
        active: user1.active,
        username: user1.username,
        email: String::from("test2@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("user2: {:?}", user2);

    let user1: User = User {
        email: String::from("test1@gmail.com"),
        username: String::from("test1"),
        active: true,
        sign_in_count: 1,
    };
    let user2: User = User {
        email: String::from("test3@gmail.com"),
        ..user1
    };
    println!("user2: {:?}", user2);

    // [ USING TUPLE STRUCTS WITHOUT NAMED FIELDS TO CREATE DIFFERENT TYPES ]
    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    println!("black: {:?}", black);
    println!("origin: {:?}", origin);

    // [ UNIT-LIKE STRUCTS WITHOUT ANY FIELDS ]
    let subject: AlwaysEqual = AlwaysEqual;
    println!("subject: {:?}", subject);
}

// [ USING THE FIELD INIT SHORTHAND ]
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
