use std::fmt;
use std::io::Error;

type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, Error>;

/*
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
*/

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    type kilometers = i32;

    let x: i32 = 5;
    let y: kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    let f2: Thunk = Box::new(|| println!("hi"));
    let mut guess = Some("asdf");

    let mut guess = String::new();

    for number in (1..4) {
        let guess: i32 = match "4".trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //continue return !
        };
    }

    //let s1: str = "Hello there!";

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    for status in list_of_statuses {
        println!("{:?}", status);
    }
}

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}

fn takes_long_type2(f: Thunk) {}

//fn return_long_type() -> Box<dyn Fn() +Send + 'static> {}

//fn return_long_type2() -> Thunk {}

fn bar() -> ! {
    loop {
        println!("forever");
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

/*
fn return_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
*/

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
