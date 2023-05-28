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

    pub fn interpret(&mut self, tokens: Vec<Token>) -> Result<(), String> {
        let mut current = 0;
        match tokens.get(current) {
            '>' => handle_increment_ptr(),
            _ => {}
        }

        Ok(())
    }

    fn handle_increment_ptr() {}
}
