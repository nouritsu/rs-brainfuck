use crate::statements::Statement;
use crate::tokens::Token;
use snafu::{prelude::*, Whatever};
use std::io::{self, Read, Write};
use std::usize;

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

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), Whatever> {
        todo!()
    }

    fn handle_expression(&mut self, expr: Token) -> Result<(), Whatever> {
        todo!()
    }

    fn handle_loop(&mut self, body: Vec<Token>) -> Result<(), Whatever> {
        todo!()
    }

    fn handle_increment_ptr(&mut self) -> Result<(), Whatever> {
        todo!()
    }
    fn handle_decrement_ptr(&mut self) -> Result<(), Whatever> {
        todo!()
    }

    fn handle_increment_value(&mut self) -> Result<(), Whatever> {
        todo!()
    }
    fn handle_decrement_value(&mut self) -> Result<(), Whatever> {
        todo!()
    }

    fn handle_put_char(&mut self) -> Result<(), Whatever> {
        todo!()
    }

    fn handle_get_char(&mut self) -> Result<(), Whatever> {
        todo!()
    }
}
