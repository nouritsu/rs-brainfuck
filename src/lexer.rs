use crate::tokens::Token;

pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Lexer
    }

    pub fn scan(&mut self, text: String) -> Vec<Token> {
        text.chars()
            .filter_map(|c| {
                Some(match c {
                    '>' => Token::IncrementPtr,
                    '<' => Token::DecrementPtr,
                    '+' => Token::IncrementValue,
                    '-' => Token::DecrementValue,
                    '[' => Token::LoopBegin,
                    ']' => Token::LoopEnd,
                    '.' => Token::PutChar,
                    ',' => Token::GetChar,
                    '^' => Token::PrintStatus,
                    _ => return None, //filter other characters out
                })
            })
            .chain([Token::End])
            .collect::<Vec<Token>>()
    }
}
