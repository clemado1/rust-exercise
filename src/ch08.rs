fn main() {
    let v: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3, 4, 5];

    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    let third: &i32 = &v1[2];
    println!("The third element is {}", third);

    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //let does_not_exist = &v1[100]; <-- panic
    let does_not_exist = v1.get(100);

    let mut v3 = vec![1, 2, 3, 4, 5];

    //let first = &v3[0]; <--panic
    let mut first = &v2[0];
    v3.push(9);

    println!("The first element is {}", first);

    for i in &v3 {
        println!("{}", i);
    }

    for i in &mut v3 {
        *i += 50;
    }

    for i in &v3 {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "initial contents";

    //literal -> String
    let s = data.to_string();
    let s = "initial contents".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); //string slice

    println!("s1 is {} s2 is {}", s1, s2);
    s1.push('a');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 has been moved and no longer be used
                       //let s3 = &s1 + &s2; <- error

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); //format dont take ownership

    let len = String::from("Здравствуйте").len();
    let hello = "Здравствуйте";
    let hello2 = String::from("Здравствуйте");

    println!("hello! {}", len);
    println!("initial {} ", &hello2[0..4]);

    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{}", initial_scores[0]); //teams & initial_scores stil exist

    let field_name = String::from("Favorite color");
    let field_value = "Blue";

    let mut map = HashMap::new();

    map.insert(field_name, field_value); //field_name no longer be used

    let team_name = String::from("Blue");

    let score = scores.get(&team_name); //cant use field_value

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(60);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
