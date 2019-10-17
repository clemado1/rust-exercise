use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));

}

#[test]
fn it_works() {
    assert_eq!(3, add_one::add_one(2));
}