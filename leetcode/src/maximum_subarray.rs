fn main() {
        let mm = vec![-4, -3, 10, -2, -5, 100];
        print!("{}", max_sub_array(mm));
}

fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = Vec::with_capacity(nums.len() - 1);
        dp.push(nums[0]);
        let mut result = dp[0];

        for i in 1..nums.len() {
                if dp[i - 1] > 0 {
                        dp.push(nums[i] + dp[i - 1]);
                } else {
                        dp.push(nums[i]);
                };

                result = result.max(dp[i]);
        }

        result
}
