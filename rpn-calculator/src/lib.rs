#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    /*
    If there is not exactly one element in the stack at the end, return None.

    If there is an operator with too few operands (such as the input 2 +), return None.

    You are given the following enum and stubbed function as a starting point.
    */
    let mut stack = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                if stack.len() < 2 {
                    return None;
                }
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a.unwrap() + b.unwrap());
            }
            CalculatorInput::Subtract => {
                if stack.len() < 2 {
                    return None;
                }
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b.unwrap() - a.unwrap());
            }
            CalculatorInput::Multiply => {
                if stack.len() < 2 {
                    return None;
                }
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a.unwrap() * b.unwrap());
            }
            CalculatorInput::Divide => {
                if stack.len() < 2 {
                    return None;
                }
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b.unwrap() / a.unwrap());
            }
            CalculatorInput::Value(value) => stack.push(*value),
        }
    }
    if stack.len() == 1 {
        Some(stack.pop().unwrap())
    } else {
        None
    }
}

