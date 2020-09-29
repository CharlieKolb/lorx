pub fn keyword_to_token_type(s: &String) -> Option<TokenType> {
    match s.as_str() {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

// lazy_static! {
//     pub static ref KEYWORDS: HashMap<String, TokenType> = [
//         ("and",    TokenType::And),
//         ("class",  TokenType::Class),
//         ("else",   TokenType::Else),
//         ("false",  TokenType::False),
//         ("for",    TokenType::For),
//         ("fun",    TokenType::Fun),
//         ("if",     TokenType::If),
//         ("nil",    TokenType::Nil),
//         ("or",     TokenType::Or),
//         ("print",  TokenType::Print),
//         ("return", TokenType::Return),
//         ("super",  TokenType::Super),
//         ("this",   TokenType::This),
//         ("true",   TokenType::True),
//         ("var",    TokenType::Var),
//         ("while",  TokenType::While),
//     ].iter().map(|(k, v)| (k.to_string(), v.clone())).collect();
// }

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    Text(String), // Instead of String to avoid name conflict
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}
