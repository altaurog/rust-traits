use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum StackVal {
    IVal(i32),
    BVal(bool),
    Void,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StackExp {
    PushI(i32),
    PushB(bool),
    Add,
    Mul,
    And,
    Or,
}

pub type Stack = VecDeque<StackVal>;
pub type Program = VecDeque<StackExp>;

pub fn stackvm<'a>(program: Program) -> Result<StackVal, String> {
    execute(VecDeque::new(), program)
}

fn err_type<'a, T>(op: &str) -> Result<T, String> {
    Err(format!("Encountered '{}' opcode with ill-typed stack.", op))
}

fn err_underflow<'a, T>(op: &str) -> Result<T, String> {
    Err(format!("Stack underflow with '{}' opcode.", op))
}

fn execute<'a>(stack: Stack, program: Program) -> Result<StackVal, String> {
    if stack.is_empty() & program.is_empty() {
        return Ok(StackVal::Void);
    } else if program.is_empty() {
        let val = stack[0].clone();
        return Ok(val);
    }
    let mut newprogram = program.clone();
    let op = newprogram.pop_front().unwrap();
    let mut newstack = stack.clone();
    let len = newstack.len();
    match op {
        StackExp::PushI(x) => {
            newstack.push_front(StackVal::IVal(x));
            execute(newstack, newprogram)
        },
        StackExp::PushB(x) => {
            newstack.push_front(StackVal::BVal(x));
            execute(newstack, newprogram)
        },
        StackExp::Add if len < 2 => { err_underflow("Add") },
        StackExp::Add => {
            let a = newstack.pop_front().unwrap();
            let b = newstack.pop_front().unwrap();
            match (a, b) {
                (StackVal::IVal(ia), StackVal::IVal(ib)) => {
                    newstack.push_front(StackVal::IVal(ia + ib));
                    execute(newstack, newprogram)
                },
                _ => err_type("Add"),
            }
        },
        StackExp::Mul if len < 2 => { err_underflow("Mul") },
        StackExp::Mul => {
            let a = newstack.pop_front().unwrap();
            let b = newstack.pop_front().unwrap();
            match (a, b) {
                (StackVal::IVal(ia), StackVal::IVal(ib)) => {
                    newstack.push_front(StackVal::IVal(ia * ib));
                    execute(newstack, newprogram)
                },
                _ => err_type("Mul"),
            }
        },
        StackExp::And if len < 2 => { err_underflow("And") },
        StackExp::And => {
            let a = newstack.pop_front().unwrap();
            let b = newstack.pop_front().unwrap();
            match (a, b) {
                (StackVal::BVal(ba), StackVal::BVal(bb)) => {
                    newstack.push_front(StackVal::BVal(ba & bb));
                    execute(newstack, newprogram)
                },
                _ => err_type("And"),
            }
        },
        StackExp::Or if len < 2 => { err_underflow("Or") },
        StackExp::Or => {
            let a = newstack.pop_front().unwrap();
            let b = newstack.pop_front().unwrap();
            match (a, b) {
                (StackVal::BVal(ba), StackVal::BVal(bb)) => {
                    newstack.push_front(StackVal::BVal(ba | bb));
                    execute(newstack, newprogram)
                },
                _ => err_type("Or"),
            }
        },
    }
}

#[cfg(test)]
mod stackvm_tests {
    use super::*;
    use StackExp::*;

    #[test]
    fn test_stackvm() {
        let program = vec![PushI(3), PushI(5), Add];
        let result = stackvm(VecDeque::from(program));
        assert_eq!(result, Ok(StackVal::IVal(8)));
    }
}
