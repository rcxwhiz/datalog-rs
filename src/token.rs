pub enum TokenValue {
    Comma,
    Period,
    QMark,
    LeftParen,
    RightParen,
    Colon,
    ColonDash,
    Multiply,
    Add,
    Schemes,
    Facts,
    Rules,
    Queries,
    Id(String),
    String(String),
    Comment(String),
    Undefined(String),
    Eof,
}

pub struct Token {
    pub value: TokenValue,
    pub line_num: usize,
    pub char_num: usize,
}
