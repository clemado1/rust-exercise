use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    with_arc();
}

fn with_arc() {
    // let mut m = 5;
    // ensure not holding same object same time across thread.
    let m = Arc::new(Mutex::new(String::from("moving")));
    let m2 = m.clone();
    std::thread::spawn(move || {
        println!("This is the new thread");
        let mut s = m2.lock().unwrap();
        s.push_str("on the new thread");
        println!("m = {}", s);
    });

    std::thread::sleep(Duration::from_millis(1000));
    println!("This is the initial thread");
    let s = m.lock().unwrap();
    println!("now m = {}", s);
}