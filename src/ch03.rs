fn main() {
    let condition = true;
    let x = plus_one(1);
    let y = if condition { 5 } else { 6 };
    another_function(x, y);
    let a = [10, 20, 30, 40, 50];
    let mut index = 0; //변화하는 값은 muttable 로 선언한다.

    while index < 5 {
        //println!("the value is: {}", a[index]);
        if index == 3 {
            println!("333");
        }

        index += 1;
    }

    for element in a.iter().rev() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}

fn another_function(x: i32, y: i32) {
    println!("this is another function x: {}, y: {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
