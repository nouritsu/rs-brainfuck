use crate::tokens::Token;

pub struct Lexer {
    text: String,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer { text }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        self.text
            .chars()
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
                    _ => return None, //filter other characters out
                })
            })
            .chain([Token::End])
            .collect::<Vec<Token>>()
    }
}
