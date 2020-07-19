fn main() {

    length_of_longest_substring("abcabcbb".to_string());

}

fn length_of_longest_substring(s: String) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut start = 0;
    let mut result = 0;

    for (i, c) in s.chars().enumerate() {
        map.entry(c)
            .and_modify(|x| {
                start = start.max(*x + 1);
                *x = i;
            })
            .or_insert(i);
        result = result.max(i - start + 1);
        println!("{:?}", map);
    }

    result as i32
}