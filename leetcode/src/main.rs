fn main() {
    reverse(100);
}

fn reverse(x: i32) -> i32 {
    let mut number = x;
    let mut result = 0;
    let mut multi = 1;

    let mut numbers: Vec<i32> = Vec::new();

    while number != 0 {
        numbers.push(number % 10);
        number /= 10;

    }

    for (i, n) in numbers.iter().enumerate(){
        result += i32::pow(*n, (numbers.len()-1-i).try_into().unwrap());
    }

    result
}