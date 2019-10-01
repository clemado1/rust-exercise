fn main() {
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage;
    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);

    impl Message {
        fn call(&self) {}
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i8> = None;

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("lucky Penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn value_state(coin: Coin) -> i8 {
        let mut count: i8 = 0;

        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }

        count
    }

    value_in_cents(Coin::Penny);
    let coin_q = Coin::Quarter(UsState::Alabama);
    value_in_cents(coin_q);
    value_state(Coin::Quarter(UsState::Alaska));
    let cnt: i8 = value_state(Coin::Dime);
    println!("cnt: {}", cnt);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        _ => println!("other"),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}
