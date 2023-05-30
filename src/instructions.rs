#[derive(Debug)]
pub enum Instructions {
    IncrementPtr,
    DecrementPtr,
    IncrementValue,
    DecrementValue,
    GetChar,
    PutChar,
    PrintStatus,
    Loop(Vec<Instructions>),
}
