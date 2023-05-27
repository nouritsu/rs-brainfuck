use crate::tokens::Token;

pub struct Interpreter {
    arr: Vec<usize>,
    ptr: usize,
}

impl Interpreter {
    pub fn new(base_size: usize) -> Self {
        Interpreter {
            arr: vec![0; base_size],
            ptr: 0,
        }
    }

    pub fn interpret(self, l: Vec<Token>) {
        todo!()
    }
}
