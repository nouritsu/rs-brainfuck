#[derive(Debug)]
pub enum Token {
    IncrementPtr,
    DecrementPtr,
    IncrementValue,
    DecrementValue,
    LoopBegin,
    LoopEnd,
    Write,
    Read,
    Comment,
}
