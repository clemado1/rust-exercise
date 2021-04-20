use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    // with_arc();
    with_channels();
}

pub fn with_arc() {
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

fn with_channels() {
    // return two part of channel, sender and receiver.
    // MPSC stand for multi producer single consumer.
    let (ch_s, ch_r) = std::sync::mpsc::channel::<Box<dyn Fn(&mut String) + Send>>();

    // empty tuple. make sure doesn't waste extra usage
    let (done_s, done_r) = std::sync::mpsc::channel::<()>();

    std::thread::spawn(move || {
        let mut hidden = String::new();
        loop {
            match ch_r.recv() {
                Ok(f) => {
                    f(&mut hidden);
                    println!("hidden = {}", hidden);
                }
                Err(e) => {
                    println!("Done");
                    done_s.send(());
                    return;
                }
            }
        }
    });

    ch_s.send(Box::new(|s: &mut String| {
        s.push_str("Hello");
    })).unwrap();

    let ch_2 = ch_s.clone();

    ch_2.send(Box::new(|s: &mut String| {
        s.push_str(" world");
    })).unwrap();

    drop(ch_s);
    drop(ch_2);

    // std::thread::sleep(Duration::from_millis(1000));
    done_r.recv().ok();
}