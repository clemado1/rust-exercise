fn main() {
  println!("{:?}", letter_combinations("239".to_string()));
}

fn letter_combinations(digits: String) -> Vec<String> {
    let comb: std::collections::HashMap<char, &str> = [
      ('2', "abc"),
      ('3', "def"),
      ('4', "ghi"),
      ('5', "jkl"),
      ('6', "mno"),
      ('7', "pqrs"),
      ('8', "tuv"),
      ('9', "wxyz")
    ].iter().cloned().collect();

    let mut result: Vec<String> = Vec::new();
    let mut target = String::from("");

    for ch in digits.chars() {
      if result.len() == 0 {
        result = comb.get(&ch).unwrap().chars().map(|c| c.to_string()).collect();
      } else {
        let mut newsul: Vec<String> = vec![];
        for l in result {
          for r in comb.get(&ch).unwrap().chars() {
            target = l.clone();
            target.push_str(&r.to_string());
            newsul.push(target.to_string());
          }
        }

        result = newsul;
      }
    }

    result

}