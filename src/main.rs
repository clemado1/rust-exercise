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
