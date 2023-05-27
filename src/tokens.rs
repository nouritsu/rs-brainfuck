enum Token {
    IncrementPtr,
    DecrementPtr,
    IncrementValue,
    DecrementValue,
    Loop(Vec<Token>),
    Read,
    Write,
}
