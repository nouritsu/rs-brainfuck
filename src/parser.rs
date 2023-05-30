use snafu::{prelude::*, Whatever};

use crate::instructions::Instructions;
use crate::tokens::Token;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Instructions>, Whatever> {
        let mut instructions = Vec::new();
        let mut loop_counter = 0;
        let mut loop_start = 0;

        for (i, token) in tokens.iter().enumerate() {
            if loop_counter == 0 {
                let instruction = match token {
                    Token::IncrementPtr => Some(Instructions::IncrementPtr),
                    Token::DecrementPtr => Some(Instructions::DecrementPtr),
                    Token::IncrementValue => Some(Instructions::IncrementValue),
                    Token::DecrementValue => Some(Instructions::DecrementValue),
                    Token::PutChar => Some(Instructions::PutChar),
                    Token::GetChar => Some(Instructions::GetChar),
                    Token::PrintStatus => Some(Instructions::PrintStatus),
                    Token::LoopBegin => {
                        loop_start = i;
                        loop_counter += 1;
                        None
                    }
                    Token::LoopEnd => whatever!("Unopened Loop encountered, open with '['."),
                };

                if let Some(instruction) = instruction {
                    instructions.push(instruction);
                }
            } else {
                match token {
                    Token::LoopBegin => {
                        loop_counter += 1;
                    }
                    Token::LoopEnd => {
                        loop_counter -= 1;
                        if loop_counter == 0 {
                            instructions.push(Instructions::Loop(
                                self.parse(tokens[loop_start + 1..i].to_vec())?,
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }

        if loop_counter != 0 {
            whatever!("Unclosed Loop encountered, close with ']'.")
        }
        Ok(instructions)
    }
}
