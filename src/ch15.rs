use crate::List::{Cons, Nil};
use std::mem::drop;
use std::ops::Deref;
use crate::List2::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let a = 5;
    let b = MyBox::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);

    let m = MyBox::new(String::from("Rust"));
    let n = String::from("Rust");

    hello(&m);
    hello(&n);

    hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c);

    println!("CustomSmartPointers created.");

    let a = Rc::new(Cons(5, Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
