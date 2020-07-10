fn main() {
    print!("{:?}", two_sum(vec![3, 2, 4], 6));
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..numbers.len() {
        if let Some(j) = &numbers[(i + 1)..]
            .binary_search(&(target - numbers[i]))
            .ok()
        {
            return vec![(i + 1) as i32, (i + 1 + j + 1) as i32];
        }
    }

    vec![]
}
