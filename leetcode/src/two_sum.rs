fn main() {
    print!("{:?}", two_sum(vec![3, 2, 4], 6));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::new();

    for (idx, num) in nums.iter().enumerate() {
        let alpha = idx + 1;
        match nums[alpha..].iter().position(|&x| x == target - num) {
            Some(n) => {
                array.push(idx as i32);
                array.push((n + alpha) as i32);

                break;
            }
            None => continue,
        }
    }

    array
}

fn convert2(s: String, num_rows: i32) -> String {
    let mut zigzags: Vec<_> = (0..num_rows)
        .chain((1..num_rows-1).rev())
        .cycle()
        .zip(s.chars())
        .collect();

    println!("{:?}", zigzags);

    zigzags.sort_by_key(|&(row, _)| row);
    zigzags.into_iter()
        .map(|(_, c)| c)
        .collect()
}