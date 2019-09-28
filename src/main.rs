fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{}", s);

    let x = 5;
    let y = 2;

    println!("x: {}, y: {}", x, y);

    let s1 = String::from("str1");
    let s2 = s1.clone();

    println!("{}", s1);
    println!("{}", s2);

    let str1 = givees_ownership();

    let str2 = String::from("hello");

    let str3 = takes_and_gives(str2);

    let x = 5;

    makes_copy(x);

    println!("{}", str1);
    println!("{}", str3);

    let len = calculate_length(&str1);

    println!("The length of {} is {}.", str1, len);

    let mut str4 = String::from("hello");
    change(&mut str4);

    println!("str4: {}", str4);

    {
        let r1 = &mut str4;
        println!("r1: {}", r1);
    }
    let r2 = &mut str4;

    println!("r2: {}", r2);

    let rs1 = &str4;
    let rs2 = &str4;
    //let rs3 = &mut str4; 불가능

    println!("{}, {}", rs1, rs2);

    let rs3 = &mut str4; //가능
    println!("{}", rs3);

    let word = first_word(&str4);

    str4.clear();

    println!("word: {}", word);

    let mut str4 = String::from("hello, world");
    let len: usize = str4.len();

    let slice1 = &str4[..];
    let slice2 = &str4[7..len];

    let slice3 = first_word2(&str4);

    println!("{}", slice3);

    str4.clear();

    let my_string = String::from("hello world");
    let word = first_word2(&my_string[..]);
    let my_string_literal = "hello world";
    let word = first_word2(&my_string_literal[..]);
    let word = first_word2(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{}", slice[1]);
}

fn givees_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives(some_string: String) -> String {
    some_string //not &some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
