#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut call_stack: Vec<i32> = vec![];
    for token in inputs {
        if let CalculatorInput::Value(num) = *token {
            call_stack.push(num);
        } else {
            if call_stack.len() < 2 {
                return None;
            }
            let eval: Option<i32> = match *token {
                CalculatorInput::Add => Some(call_stack.pop().unwrap() + call_stack.pop().unwrap()),
                CalculatorInput::Divide => {
                    let right = call_stack.pop().unwrap();
                    let left = call_stack.pop().unwrap();
                    Some(left / right)
                }
                CalculatorInput::Multiply => {
                    Some(call_stack.pop().unwrap() * call_stack.pop().unwrap())
                }
                CalculatorInput::Subtract => {
                    let right = call_stack.pop().unwrap();
                    let left = call_stack.pop().unwrap();
                    Some(left - right)
                }
                _ => None,
            };
            if let Some(res) = eval {
                call_stack.push(res);
            }
        }
    }
    if call_stack.len() != 1 {
        return None;
    }

    return Some(call_stack[0]);
}
