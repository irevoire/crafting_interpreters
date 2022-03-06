use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character tokens
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
    // One or two characters tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    // Keywords
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
    // EoF
    EoF,
}

impl Eq for TokenType {}

impl std::hash::Hash for TokenType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            TokenType::LeftParen => state.write_u8(0),
            TokenType::RightParen => state.write_u8(1),
            TokenType::LeftBrace => state.write_u8(2),
            TokenType::RightBrace => state.write_u8(3),
            TokenType::Comma => state.write_u8(4),
            TokenType::Dot => state.write_u8(5),
            TokenType::Minus => state.write_u8(6),
            TokenType::Plus => state.write_u8(7),
            TokenType::Semicolon => state.write_u8(8),
            TokenType::Slash => state.write_u8(9),
            TokenType::Star => state.write_u8(10),
            TokenType::Bang => state.write_u8(11),
            TokenType::BangEqual => state.write_u8(12),
            TokenType::Equal => state.write_u8(13),
            TokenType::EqualEqual => state.write_u8(14),
            TokenType::Greater => state.write_u8(15),
            TokenType::GreaterEqual => state.write_u8(16),
            TokenType::Less => state.write_u8(17),
            TokenType::LessEqual => state.write_u8(18),
            TokenType::Identifier(i) => {
                state.write_u8(19);
                i.hash(state);
            }
            TokenType::String(s) => {
                state.write_u8(20);
                s.hash(state);
            }
            TokenType::Number(n) => {
                state.write_u8(21);
                n.to_bits().hash(state);
            }
            TokenType::And => state.write_u8(22),
            TokenType::Class => state.write_u8(23),
            TokenType::Else => state.write_u8(24),
            TokenType::False => state.write_u8(25),
            TokenType::Fun => state.write_u8(26),
            TokenType::For => state.write_u8(27),
            TokenType::If => state.write_u8(28),
            TokenType::Nil => state.write_u8(29),
            TokenType::Or => state.write_u8(30),
            TokenType::Print => state.write_u8(31),
            TokenType::Return => state.write_u8(32),
            TokenType::Super => state.write_u8(33),
            TokenType::This => state.write_u8(34),
            TokenType::True => state.write_u8(35),
            TokenType::Var => state.write_u8(36),
            TokenType::While => state.write_u8(37),
            TokenType::EoF => state.write_u8(38),
        }
    }
}

impl TokenType {
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        let keyword = match keyword {
            "and" => Self::And,
            "class" => Self::Class,
            "else" => Self::Else,
            "false" => Self::False,
            "fun" => Self::Fun,
            "for" => Self::For,
            "if" => Self::If,
            "nil" => Self::Nil,
            "or" => Self::Or,
            "print" => Self::Print,
            "return" => Self::Return,
            "super" => Self::Super,
            "this" => Self::This,
            "true" => Self::True,
            "var" => Self::Var,
            "while" => Self::While,
            _ => return None,
        };
        Some(keyword)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} `{}`", self.ty, self.lexeme)
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.lexeme
    }
}
