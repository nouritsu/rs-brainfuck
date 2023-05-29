use crate::statements::Statement;
use crate::tokens::Token;
use snafu::{prelude::*, Whatever};
use std::io::{self, Write};
use std::usize;

pub struct Interpreter {
    pub arr: Vec<usize>,
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
            Token::PrintStatus => self.handle_print_status(),
            _ => whatever!("Unkown token encountered."), //Unreachable
        }
    }

    fn handle_loop(&mut self, body: Vec<Token>) -> Result<(), Whatever> {
        let loop_idx = self.ptr;
        while self.arr[loop_idx] != 0 {
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
        if self.arr.len() == 0 || self.ptr == self.arr.len() - 1 {
            whatever!(
                "Unable to increment pointer past array size (size: {}).",
                self.arr.len()
            )
        }
        self.ptr += 1;
        Ok(())
    }
    fn handle_decrement_ptr(&mut self) -> Result<(), Whatever> {
        if self.ptr == 0 {
            whatever!("Unable to decrement pointer below zero.")
        }
        self.ptr -= 1;
        Ok(())
    }

    fn handle_increment_value(&mut self) -> Result<(), Whatever> {
        if self.arr[self.ptr] == usize::MAX {
            whatever!(
                "Unable to increment value, already at max (max: {}, ptr: {})",
                usize::MAX,
                self.ptr
            )
        }
        self.arr[self.ptr] += 1;
        Ok(())
    }

    fn handle_decrement_value(&mut self) -> Result<(), Whatever> {
        if self.arr[self.ptr] == usize::MIN {
            whatever!(
                "Unable to decrement value, already at min (min: {}, ptr: {}).",
                usize::MIN,
                self.ptr
            )
        }
        self.arr[self.ptr] -= 1;
        Ok(())
    }

    fn handle_put_char(&mut self) -> Result<(), Whatever> {
        let c = match char::from_u32(self.arr[self.ptr] as u32) {
            Some(r) => r,
            None => whatever!(
                "Cannot convert value to character (value: {}).",
                self.arr[self.ptr]
            ),
        };
        print!("{}", c);
        match io::stdout().flush() {
            Ok(_) => Ok(()),
            Err(_) => whatever!("Unable to flush output buffer."),
        }
    }

    fn handle_get_char(&mut self) -> Result<(), Whatever> {
        todo!()
    }

    fn handle_print_status(&mut self) -> Result<(), Whatever> {
        todo!()
    }
}
