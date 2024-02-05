fn main() {
    let s: String = String::from("hello world");
    let word: usize = first_word(&s); // word will get the value 5
    println!("{word}");

    let mut s: String = String::from("hello world");
    let word: usize = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word in now totally invalid!
    println!("{word}");

    // [ STRING SLICES ]
    let s: String = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{hello}, {world}");

    let s: String = String::from("hello");
    let slice1: &str = &s[0..2];
    let slice2: &str = &s[..2];
    println!("{slice1}, {slice2}");

    let s: String = String::from("hello");
    let len: usize = s.len();
    let slice1: &str = &s[3..len];
    let slice2: &str = &s[3..];
    println!("{slice1}, {slice2}");

    let s: String = String::from("hello");
    let len: usize = s.len();
    let slice1: &str = &s[0..len];
    let slice2: &str = &s[..];
    println!("{slice1}, {slice2}");

    let s: String = String::from("hello world");
    let word: &str = first_word_v2(&s);
    println!("{word}");

    // let mut s: String = String::from("hello world");
    // let word: &str = first_word_v2(&s);
    // s.clear(); // error!
    // println!("the first word is: {}", word);

    // [ STRING LITERALS AS SLICES ]
    let _s: &str = "Hello, world!";

    // [ STRING SLICES AS PARAMETERS ]
    let my_string: String = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word: &str = first_word_v3(&my_string[0..6]);
    let _word: &str = first_word_v3(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word: &str = first_word_v3(&my_string);

    let my_string_literal: &str = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word: &str = first_word_v3(&my_string_literal[0..6]);
    let _word: &str = first_word_v3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word: &str = first_word_v3(my_string_literal);

    // [ OTHER SLICES ]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    println!("{:?}", slice);
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// [ STRING SLICES ]
fn first_word_v2(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// [ STRING SLICES AS PARAMETERS ]
fn first_word_v3(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
