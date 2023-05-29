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
        for statement in statements.into_iter() {
            let r = match statement {
                Statement::Expression(e) => self.handle_expression(e),
                Statement::Loop(body) => self.handle_loop(body),
            };

            match r {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn handle_expression(&mut self, expr: Token) -> Result<(), Whatever> {
        match expr {
            Token::IncrementPtr => self.handle_increment_ptr(),
            Token::DecrementPtr => self.handle_decrement_ptr(),
            Token::IncrementValue => self.handle_increment_value(),
            Token::DecrementValue => self.handle_decrement_value(),
            Token::PutChar => self.handle_put_char(),
            Token::GetChar => self.handle_get_char(),
            _ => whatever!("Unkown token encountered."), //Unreachable
        }
    }

    fn handle_loop(&mut self, body: Vec<Token>) -> Result<(), Whatever> {
        while let Ok(_) = self.handle_decrement_ptr() {
            for stmt in body.iter() {
                match self.handle_expression(*stmt) {
                    Ok(_) => continue,
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
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
