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
    let mut program = program.clone();
    let mut stack = stack.clone();
    loop {
        if stack.is_empty() & program.is_empty() {
            return Ok(StackVal::Void);
        } else if program.is_empty() {
            let val = stack[0].clone();
            return Ok(val);
        }
        let op = program.pop_front().unwrap();
        let len = stack.len();
        match op {
            StackExp::PushI(x) => { stack.push_front(StackVal::IVal(x)); },
            StackExp::PushB(x) => { stack.push_front(StackVal::BVal(x)); },
            StackExp::Add if len < 2 => { return err_underflow("Add"); },
            StackExp::Add => {
                let a = stack.pop_front().unwrap();
                let b = stack.pop_front().unwrap();
                match (a, b) {
                    (StackVal::IVal(ia), StackVal::IVal(ib)) => {
                        stack.push_front(StackVal::IVal(ia + ib));
                    },
                    _ => { return err_type("Add"); },
                }
            },
            StackExp::Mul if len < 2 => { return err_underflow("Mul"); },
            StackExp::Mul => {
                let a = stack.pop_front().unwrap();
                let b = stack.pop_front().unwrap();
                match (a, b) {
                    (StackVal::IVal(ia), StackVal::IVal(ib)) => {
                        stack.push_front(StackVal::IVal(ia * ib));
                    },
                    _ => { return err_type("Mul"); },
                }
            },
            StackExp::And if len < 2 => { return err_underflow("And"); },
            StackExp::And => {
                let a = stack.pop_front().unwrap();
                let b = stack.pop_front().unwrap();
                match (a, b) {
                    (StackVal::BVal(ba), StackVal::BVal(bb)) => {
                        stack.push_front(StackVal::BVal(ba & bb));
                    },
                    _ => { return err_type("And"); },
                }
            },
            StackExp::Or if len < 2 => { return err_underflow("Or"); },
            StackExp::Or => {
                let a = stack.pop_front().unwrap();
                let b = stack.pop_front().unwrap();
                match (a, b) {
                    (StackVal::BVal(ba), StackVal::BVal(bb)) => {
                        stack.push_front(StackVal::BVal(ba | bb));
                    },
                    _ => { return err_type("Or"); },
                }
            },
        }
    }
}

#[cfg(test)]
mod stackvm_tests {
    use super::*;
    use StackExp::*;

    #[test]
    fn test_stackvm_simple_add() {
        let program = vec![PushI(3), PushI(5), Add];
        let result = stackvm(VecDeque::from(program));
        assert_eq!(result, Ok(StackVal::IVal(8)));
    }

    #[test]
    fn test_stackvm_simple_mul() {
        let program = vec![PushI(3), PushI(5), Mul];
        let result = stackvm(VecDeque::from(program));
        assert_eq!(result, Ok(StackVal::IVal(15)));
    }

    #[test]
    fn test_stackvm_add_err_type() {
        let program = vec![PushB(true), PushB(true), Add];
        let result = stackvm(VecDeque::from(program));
        assert_eq!(result, Err(String::from("Encountered 'Add' opcode with ill-typed stack.")));
    }

    #[test]
    fn test_stackvm_add_underflow() {
        let program = vec![PushI(3), Add];
        let result = stackvm(VecDeque::from(program));
        assert_eq!(result, Err(String::from("Stack underflow with 'Add' opcode.")));
    }

    #[test]
    fn test_stackvm_simple_and() {
        let program = vec![PushB(true), PushB(true), And];
        let result = stackvm(VecDeque::from(program));
        assert_eq!(result, Ok(StackVal::BVal(true)));
    }
}
