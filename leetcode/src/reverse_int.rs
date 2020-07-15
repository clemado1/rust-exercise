fn main() {
    println!("Hello, world!");
    println!("{}", chachchc(1513513350));
}

fn chachchc(x: i32) -> i32 {
    let mut num = x;
    let mut result: i32 = 0;
    let round = 10;

    while num != 0 {
        result = match result.checked_mul(round) {
            Some(x) => match x.checked_add(num % round) {
                Some(y) => y,
                None => {
                    result = 0;
                    break;
                }
            },
            None => {
                result = 0;
                break;
            }
        };
        num /= round;
    }

    result
    
}