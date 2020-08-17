fn main() {
    println!("{:?}",generate_parenthesis2(3));
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    _gen(&mut result, n, n, "".to_string());

    result

}

fn _gen(result: &mut Vec<String>, left: i32, right: i32, sublist: String) {
    if left == 0 && right == 0 {
        result.push(sublist);
        return;
    }
    if left > 0 {
        _gen(result, left - 1, right, sublist.clone() + "(");
    }
    if right > left {
        _gen(result, left, right - 1, sublist.clone() + ")");
    }
}

fn generate_parenthesis2(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut dp = vec![vec![vec![]; n + 1]; n + 1];
    dp[0][0].push(String::new());

    for i in 0..=n {
        for j in 0..=i {
            if i == n && j == i {
                return dp[i].pop().unwrap()
            }

            while let Some(s) = dp[i][j].pop() {
                if i < n {
                    dp[i + 1][j].push(s.clone() + "(");
                }
                if i > j {
                    dp[i][j + 1].push(s + ")");
                }
            }

            println!("i: {}, j: {}, {:?}", i, j, dp);
        }
    }

    vec![]
}