use crate::tokens::Token;

#[derive(Debug)]
pub enum Statement {
    Expression(Token),
    Loop(Vec<Token>),
}
