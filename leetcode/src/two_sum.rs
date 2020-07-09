fn main() {
    print!("{:?}", two_sum(vec![3, 2, 4], 6));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::new();

    for (idx, num) in nums.iter().enumerate() {
        let alpha = idx + 1;
        match nums[alpha..].iter().position(|&x| x == target - num) {
            Some(n) => {
                println!("{}", idx);
                println!("{}", n);
                array.push(idx as i32);
                array.push((n + alpha) as i32);

                break;
            }
            None => continue,
        }
    }

    array
}
