#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for element in inputs {
        match element {
            CalculatorInput::Add => {
                let values = match pop_two(&mut stack) {
                    Some(val) => val,
                    None => return None,
                };
                stack.push(values.1 + values.0);
            }
            CalculatorInput::Subtract => {
                let values = match pop_two(&mut stack) {
                    Some(val) => val,
                    None => return None,
                };
                stack.push(values.1 - values.0);
            }
            CalculatorInput::Multiply => {
                let values = match pop_two(&mut stack) {
                    Some(val) => val,
                    None => return None,
                };
                stack.push(values.0 * values.1);
            }
            CalculatorInput::Divide => {
                let values = match pop_two(&mut stack) {
                    Some(val) => val,
                    None => return None,
                };
                stack.push(values.1 / values.0);
            }
            CalculatorInput::Value(val) => stack.push(*val),
        }
    }
    if stack.len() == 1 {
        return stack.pop();
    } else {
        return None;
    }
}
fn pop_two(stack: &mut Vec<i32>) -> Option<(i32, i32)> {
    let last: i32 = match stack.pop() {
        Some(val) => val,
        None => return None,
    };
    let last_but_one: i32 = match stack.pop() {
        Some(val) => val,
        None => return None,
    };
    return Some((last, last_but_one));
}
