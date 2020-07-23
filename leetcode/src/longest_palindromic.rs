fn main() {
    println!("{}",longest_palindrome("d12321".to_string()));
}

fn longest_palindrome(s: String) -> String {
    if s.len() < 2 {
        return s;
    }

    let mut max_num = -1;
    let mut max_str = String::from("");
    let mut pal_len = 0;
    let mut count = 0;

    let v: Vec<char> = s.chars().collect();

    for i in 1..(s.len()) * 2 - 1 {

        count = v[0..i/2]
            .iter().rev()
            .zip(v[(i+1)/2..s.len()].iter())
            .take_while(|(l,r)| l == r)
            .count();

        pal_len = count * 2 + i % 2;

        if max_num < pal_len as i32 {
            max_str = v[(i/2)-count..(i+1)/2+count].iter().collect::<String>();
            max_num = pal_len as i32;
        }
    }

    max_str
}

