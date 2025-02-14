#[derive(Debug)]
pub enum SimpleTokens {
    Operator(String),
    Identifier(String),
    Number(String),
}

#[derive(Debug)]
pub enum AdvancedTokens {
    // Comparisons
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    IsEqual,
    IsNotEqual,

    // Operations
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,

    // Variable
    SetEqualTo,
    MemberOf,

    // Formatting
    SemiColon,
    OpeningBrace,
    ClosingBrace,
    OpeningBracket,
    ClosingBracket,
    OpeningParentheses,
    ClosingParentheses,
    StringOpenClose,

    // Keywords
    If,
    While,
    Else,
    For,
    In,
    Pub,
    Struct,
    Enum,
    Use,
    Const,
    Let,

    Identifier(String),
    Integer(String),
    Float(String),
    String(String),
    Char(String),
}
