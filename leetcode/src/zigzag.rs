fn main() {
    println!("{}",convert("PAYPALISHIRING".to_string(), 3));
}

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let mut chars_arr = vec![vec![]; num_rows as usize];
    let mut depth = 0;
    let mut turn = 1;

    for c in s.chars() {
        chars_arr[depth as usize].push(c);

        depth += turn;

        if depth == 0 || depth == num_rows - 1 {
            turn *= -1;
        }

    }

    chars_arr.iter().map(|x| x.iter().collect::<String>()).collect()
    
}
