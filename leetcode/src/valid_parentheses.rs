fn main() {
    println!("{}", is_valid("{{()()()[{}()]}}".to_string()));
}

fn is_valid(s: String) -> bool {
    let mut paren_stack: Vec<char> = Vec::new();

    for item in s.chars() {
        match item {
            '{' => paren_stack.push('}'),
            '(' => paren_stack.push(')'),
            '[' => paren_stack.push(']'),
            ']'|'}'|')' => {
                match paren_stack.pop() {
                    Some(x) => {
                        if x != item {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
            _ => (),
        }
    }

    paren_stack.is_empty()
}