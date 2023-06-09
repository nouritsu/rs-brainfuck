#[derive(Debug, Clone)]
pub enum Token {
    IncrementPtr,
    DecrementPtr,
    IncrementValue,
    DecrementValue,
    LoopBegin,
    LoopEnd,
    PutChar,
    GetChar,
    PrintStatus,
}
