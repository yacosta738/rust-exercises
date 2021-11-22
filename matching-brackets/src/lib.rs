pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                let option = stack.pop();
                if option.is_none() || option.unwrap() != '(' {
                    return false;
                }
            },
            '{' => stack.push(c),
            '}' => {
                let option = stack.pop();
                if option.is_none() || option.unwrap() != '{' {
                    return false;
                }
            },
            '[' => stack.push(c),
            ']' => {
                let option = stack.pop();
                if option.is_none() || option.unwrap() != '[' {
                    return false;
                }
            },
            _ => (),
        }
    }
    stack.is_empty()
}
