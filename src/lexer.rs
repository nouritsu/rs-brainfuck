use crate::tokens::Token;

pub struct Lexer {
    text: String,
    idx: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer { text, idx: 0 }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.at_end() {
            self.idx += 1;
            let c = self.text.chars().nth(self.idx - 1).unwrap();
            tokens.push(match c {
                '>' => Token::IncrementPtr,
                '<' => Token::DecrementPtr,
                '+' => Token::IncrementValue,
                '-' => Token::DecrementValue,
                '[' => Token::LoopBegin,
                ']' => Token::LoopEnd,
                '.' => Token::Write,
                ',' => Token::Read,
                _ => Token::Comment,
            })
        }

        tokens
    }

    fn at_end(&self) -> bool {
        self.text.len() == 0 || self.text.len() - 1 == self.idx
    }
}
