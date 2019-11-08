use std::slice;

static HELLO_WORD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/*
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
*/

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

//impl OutlinePrint for Point{}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

impl OutlinePrint for Point {}

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
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *mut i32;

    //println!("r1 is: {}", *r1);
    //dangerous();

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    /*
    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
    */

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

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

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); //return raw pointer of a slice

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid), //to get a raw pointer that starts at mid
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe trait Foo {
    fn add(x: i32) -> i32;
}

unsafe impl Foo for i32 {
    fn add(x: i32) -> i32 {
        x + 1
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