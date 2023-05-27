pub struct Interpreter {
    arr: Vec<u64>,
    ptr: u64,
}

impl Interpreter {
    pub fn new(base_size: u64) -> Self {
        Interpreter {
            arr: vec![0; base_size],
            ptr: 0,
        }
    }

    pub fn interpret(self, l: List<Token>) {
        todo!()
    }
}
