use crate::instructions::Instructions;
use snafu::{prelude::*, Whatever};
use std::io::{self, Read, Write};
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

    pub fn interpret(&mut self, instructions: &Vec<Instructions>) -> Result<(), Whatever> {
        for instruction in instructions {
            match instruction {
                Instructions::IncrementPtr => self.handle_increment_ptr(),
                Instructions::DecrementPtr => self.handle_decrement_ptr(),
                Instructions::IncrementValue => self.handle_increment_value(),
                Instructions::DecrementValue => self.handle_decrement_value(),
                Instructions::PutChar => self.handle_put_char(),
                Instructions::GetChar => self.handle_get_char(),
                Instructions::PrintStatus => self.handle_print_status(),
                Instructions::Loop(body) => self.handle_loop(body),
            }?
        }
        Ok(())
    }

    fn handle_loop(&mut self, body: &Vec<Instructions>) -> Result<(), Whatever> {
        let loop_idx = self.ptr;
        while self.arr[loop_idx] != 0 {
            self.interpret(body)?
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
        let mut inp: [u8; 1] = [0; 1];
        match io::stdin().read_exact(&mut inp) {
            Ok(_) => {}
            Err(_) => whatever!("Unable to read from stdin."),
        }
        self.arr[self.ptr] = inp[0] as usize;
        Ok(())
    }

    fn handle_print_status(&mut self) -> Result<(), Whatever> {
        todo!()
    }
}
