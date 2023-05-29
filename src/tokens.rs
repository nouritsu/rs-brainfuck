#[derive(Debug, PartialEq, Clone, Copy)]
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
    End,
}
