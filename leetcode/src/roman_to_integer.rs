fn main() {
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}

fn roman_to_int(s: String) -> i32 {
    let mut pre = 1000;
    let mut sum = 0;

    for c in s.chars() {
        let rom: i32 = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        sum += rom;

        if pre < rom {
            sum -= 2 * pre;
        }

        pre = rom;
    }

    sum
}
