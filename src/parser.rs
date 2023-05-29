use snafu::{prelude::*, Whatever};

use crate::statements::Statement;
use crate::tokens::Token;
use std::vec::IntoIter;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Statement>, Whatever> {
        if !self.check_braces(&tokens) {
            whatever!("Unclosed loop encountered, close with ']'.")
        }
        let mut statements = Vec::<Statement>::new();
        let mut iter = tokens.into_iter();

        while let Some(token) = iter.next() {
            match token {
                Token::End => break,
                Token::LoopBegin => match self.handle_loop(&mut iter) {
                    Ok(v) => statements.push(Statement::Loop(v)),
                    Err(e) => return Err(e),
                },
                _ => statements.push(Statement::Expression(token)),
            }
        }

        Ok(statements)
    }

    pub fn handle_loop(&mut self, iter: &mut IntoIter<Token>) -> Result<Vec<Token>, Whatever> {
        let mut loop_body = Vec::new();
        while let Some(token) = iter.next() {
            match token {
                Token::LoopEnd => break,
                Token::End => whatever!("Unclosed Loop encountered, close with ']'."),
                _ => loop_body.push(token),
            }
        }
        Ok(loop_body)
    }

    fn check_braces(&mut self, tokens: &Vec<Token>) -> bool {
        let mut c: i32 = 0;
        for token in tokens.iter() {
            match token {
                Token::LoopBegin => c += 1,
                Token::LoopEnd => c -= 1,
                _ => continue,
            }

            if c < 0 {
                return false;
            }
        }
        return c == 0;
    }
}
