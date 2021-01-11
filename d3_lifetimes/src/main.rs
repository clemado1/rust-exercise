#[derive(Debug)]
pub struct StringHolder<'a> {
    s: &'a str,
    t: &'a str,
}

fn main() {
    println!("Hello, world!");

    let mut s = make_str(7);
    s = to_people(s);
    to_froges(&mut s);
    let p = part(&s);

    // p referencing s p has lifetime connected s
    // s.push_str("p");

    println!("p = {}", p);
    println!("s = {}", s);

    let tog = two_strings(p, &s);
    println!("Tog = {:?}", tog);

    s.push_str("anyhting");

    println!("final s = {}", s);
}

fn to_people(mut s: String) -> String {
    s.push_str(" people");
    s
}

fn to_froges(s: &mut String) {
    s.push_str(" frogs");
}

fn make_str(n: i32) -> String {
    format!("hello {}", n)
}

// these two str have the same lifetime so as long as P exists, we have a pointer to S
fn part<'a>(s: &'a str) -> &'a str {
    if s.len() > 4 {
        &s[0..4]
    } else {
        s
    }
}

pub fn two_strings<'a>(s: &'a str, t: &'a str) -> StringHolder<'a> {
    StringHolder { s, t }
}