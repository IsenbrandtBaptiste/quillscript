use core::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {

    LeftParen,
    RightParen, 
    LeftBrace, 
    RightBrace,
    Comma, 
    Dot,  
    Semicolon, 
 
    Plus,
    Minus, 
    Slash, 
    Star,
    Bang, 
    BangEqual,
    Equal, 
    EqualEqual,
    Greater,
    GreaterEqual,
    Less, 
    LessEqual,

    Identifier(String), 
    String(String), 
    Number(f64),

    Class,
    Else, 
    False, 
    Func, 
    For, 
    If,
    And, 
    Nil, 
    Or,
    Print, 
    Return, 
    Super, 
    This, 
    True, 
    Var, 
    While,
    Eof,

    Space,
    Line,
    Invalid(i32, i32, String)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Semicolon => write!(f, ";"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Bang => write!(f, "!"),
            Token::BangEqual => write!(f, "!="),
            Token::Equal => write!(f, "="),
            Token::EqualEqual => write!(f, "=="),
            Token::Greater =>write!(f, ">"),
            Token::GreaterEqual =>write!(f, ">="),
            Token::Less =>write!(f, "<"),
            Token::LessEqual =>write!(f, "<="),
            Token::Identifier(_) =>write!(f, "identifier"),
            Token::String(_) =>write!(f, "string"),
            Token::Number(_) =>write!(f, "number"),
            Token::Class => write!(f, "class"),
            Token::Else =>write!(f, "else"),
            Token::False =>write!(f, "false"),
            Token::Func =>write!(f, "func"),
            Token::For =>write!(f, "for"),
            Token::If =>write!(f, "if"),
            Token::And => write!(f, "and"),
            Token::Nil =>write!(f, "nil"),
            Token::Or =>write!(f, "or"),
            Token::Print =>write!(f, "print"),
            Token::Return =>write!(f, "return"),
            Token::Super =>write!(f, "super"),
            Token::This =>write!(f, "this"),
            Token::True =>write!(f, "true"),
            Token::Var =>write!(f, "var"),
            Token::While =>write!(f, "while"),
            Token::Eof =>write!(f, "eof"),
            Token::Invalid(_,_,_) =>write!(f, "invalid"),
            Token::Space => write!(f, "space"),
            Token::Line => write!(f, "line"),
        }
    }
}
