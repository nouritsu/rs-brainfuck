#[derive(Debug)]
pub enum Token {
    IncrementPtr,
    DecrementPtr,
    IncrementValue,
    DecrementValue,
    LoopBegin,
    LoopEnd,
    PutChar,
    GetChar,
    End,
}
