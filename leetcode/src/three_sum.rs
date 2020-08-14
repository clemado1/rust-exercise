fn main() {
    println!("{:?}", three_sum(vec![0, 0, 0, 0, 0, 0, 0, 0]));
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }

    let mut array: Vec<Vec<i32>> = Vec::new();
    let mut mum = nums;
    mum.sort();

    for i1 in 0..mum.len() - 2 {
        if i1 > 0 && mum[i1] == mum[i1 - 1] {
            continue;
        }

        if mum[i1] > 0 {
            break;
        }

        let mut i2 = i1 + 1;
        let mut i3 = mum.len() - 1;

        while i2 < i3 {
            let sum = mum[i1] + mum[i2] + mum[i3];
            
            if sum < 0 {
                i2 += 1;
            } else if sum > 0 {
                i3 -= 1;
            } else {
                array.push(vec![mum[i1], mum[i2], mum[i3]]);

                while i2 < i3 && mum[i2] == mum[i2 + 1] {
                    i2 += 1;
                }
                while i2 < i3 && mum[i3] == mum[i3 - 1] {
                    i3 -= 1;
                }

                i2 += 1;
                i3 -= 1;

            }
        }
    }

    array
    
}
