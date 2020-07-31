fn main() {
    let test = vec![1,8,6,2,5,4,8,3,7];
    println!("{}", max_area(test));
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut max_vol: usize = 0;

    while l < r {
        max_vol = max_vol.max((r - l) * height[l].min(height[r]) as usize);

        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    max_vol as i32
}