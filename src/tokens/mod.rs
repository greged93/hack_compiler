#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A keyword in the Jack language (e.g. class, method, function, etc.)
    Keyword(Keyword),
    /// A symbol in the Jack language (e.g. {, }, (, ), etc.)
    Symbol(Symbol),
    /// An identifier in the Jack language (e.g. x, foo, bar, etc.)
    Identifier(String),
    /// An integer constant in the Jack language (e.g. 123, 456, etc.)
    IntConst(u16),
    /// A string constant in the Jack language (e.g. "Hello, World!", "foo", etc.)
    StringConst(String),
}

impl Token {
    pub fn start_xml(&self) -> String {
        match self {
            Self::Keyword(_) => "<keyword>",
            Self::Symbol(_) => "<symbol>",
            Self::Identifier(_) => "<identifier>",
            Self::IntConst(_) => "<integerConstant>",
            Self::StringConst(_) => "<stringConstant>",
        }
        .to_string()
    }

    pub fn end_xml(&self) -> String {
        match self {
            Self::Keyword(_) => r"</keyword>",
            Self::Symbol(_) => r"</symbol>",
            Self::Identifier(_) => r"</identifier>",
            Self::IntConst(_) => r"</integerConstant>",
            Self::StringConst(_) => r"</stringConstant>",
        }
        .to_string()
    }

    pub fn to_xml(&self) -> String {
        match self {
            Self::Keyword(k) => k.to_str().to_string(),
            Self::Symbol(s) => s.to_xml().to_string(),
            Self::Identifier(i) => i.clone(),
            Self::IntConst(i) => i.to_string(),
            Self::StringConst(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

impl From<String> for Keyword {
    fn from(value: String) -> Self {
        match value.as_str() {
            "class" => Self::Class,
            "constructor" => Self::Constructor,
            "function" => Self::Function,
            "method" => Self::Method,
            "field" => Self::Field,
            "static" => Self::Static,
            "var" => Self::Var,
            "int" => Self::Int,
            "char" => Self::Char,
            "boolean" => Self::Boolean,
            "void" => Self::Void,
            "true" => Self::True,
            "false" => Self::False,
            "null" => Self::Null,
            "this" => Self::This,
            "let" => Self::Let,
            "do" => Self::Do,
            "if" => Self::If,
            "else" => Self::Else,
            "while" => Self::While,
            "return" => Self::Return,
            _ => panic!("not a keyword"),
        }
    }
}

impl Keyword {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Class => "class",
            Self::Constructor => "constructor",
            Self::Function => "function",
            Self::Method => "method",
            Self::Field => "field",
            Self::Static => "static",
            Self::Var => "var",
            Self::Int => "int",
            Self::Char => "char",
            Self::Boolean => "boolean",
            Self::Void => "void",
            Self::True => "true",
            Self::False => "false",
            Self::Null => "null",
            Self::This => "this",
            Self::Let => "let",
            Self::Do => "do",
            Self::If => "if",
            Self::Else => "else",
            Self::While => "while",
            Self::Return => "return",
        }
    }

    pub fn is_keyword(s: &str) -> bool {
        Self::Class.to_str() == s
            || Self::Constructor.to_str() == s
            || Self::Function.to_str() == s
            || Self::Method.to_str() == s
            || Self::Field.to_str() == s
            || Self::Static.to_str() == s
            || Self::Var.to_str() == s
            || Self::Int.to_str() == s
            || Self::Char.to_str() == s
            || Self::Boolean.to_str() == s
            || Self::Void.to_str() == s
            || Self::True.to_str() == s
            || Self::False.to_str() == s
            || Self::Null.to_str() == s
            || Self::This.to_str() == s
            || Self::Let.to_str() == s
            || Self::Do.to_str() == s
            || Self::If.to_str() == s
            || Self::Else.to_str() == s
            || Self::While.to_str() == s
            || Self::Return.to_str() == s
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    CurlLeft,
    CurlRight,
    ParenthesisLeft,
    ParenthesisRight,
    SquareBracketLeft,
    SquareBracketRight,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Mul,
    Divide,
    And,
    Or,
    LessThan,
    MoreThan,
    Equal,
    Tilte,
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '{' => Self::CurlLeft,
            '}' => Self::CurlRight,
            '(' => Self::ParenthesisLeft,
            ')' => Self::ParenthesisRight,
            '[' => Self::SquareBracketLeft,
            ']' => Self::SquareBracketRight,
            '.' => Self::Dot,
            ',' => Self::Comma,
            ';' => Self::Semicolon,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Mul,
            '/' => Self::Divide,
            '&' => Self::And,
            '|' => Self::Or,
            '<' => Self::LessThan,
            '>' => Self::MoreThan,
            '=' => Self::Equal,
            '~' => Self::Tilte,
            _ => panic!("not a symbol"),
        }
    }
}

impl Symbol {
    pub fn to_str(&self) -> &str {
        match self {
            Self::CurlLeft => "{",
            Self::CurlRight => "}",
            Self::ParenthesisLeft => "(",
            Self::ParenthesisRight => ")",
            Self::SquareBracketLeft => "[",
            Self::SquareBracketRight => "]",
            Self::Dot => ".",
            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Mul => "*",
            Self::Divide => "/",
            Self::And => "&",
            Self::Or => "|",
            Self::LessThan => "<",
            Self::MoreThan => ">",
            Self::Equal => "=",
            Self::Tilte => "~",
        }
    }

    pub fn to_char(&self) -> char {
        self.to_str()
            .chars()
            .collect::<Vec<char>>()
            .first()
            .cloned()
            .unwrap_or_default()
    }

    pub fn is_symbol(s: &char) -> bool {
        &Self::CurlLeft.to_char() == s
            || &Self::CurlRight.to_char() == s
            || &Self::ParenthesisLeft.to_char() == s
            || &Self::ParenthesisRight.to_char() == s
            || &Self::SquareBracketLeft.to_char() == s
            || &Self::SquareBracketRight.to_char() == s
            || &Self::Dot.to_char() == s
            || &Self::Comma.to_char() == s
            || &Self::Semicolon.to_char() == s
            || &Self::Plus.to_char() == s
            || &Self::Minus.to_char() == s
            || &Self::Mul.to_char() == s
            || &Self::Divide.to_char() == s
            || &Self::And.to_char() == s
            || &Self::Or.to_char() == s
            || &Self::LessThan.to_char() == s
            || &Self::MoreThan.to_char() == s
            || &Self::Equal.to_char() == s
            || &Self::Tilte.to_char() == s
    }

    pub fn contains_symbol(s: &str) -> bool {
        s.contains(Self::CurlLeft.to_str())
            || s.contains(Self::CurlRight.to_str())
            || s.contains(Self::ParenthesisLeft.to_str())
            || s.contains(Self::ParenthesisRight.to_str())
            || s.contains(Self::SquareBracketLeft.to_str())
            || s.contains(Self::SquareBracketRight.to_str())
            || s.contains(Self::Dot.to_str())
            || s.contains(Self::Comma.to_str())
            || s.contains(Self::Semicolon.to_str())
            || s.contains(Self::Plus.to_str())
            || s.contains(Self::Minus.to_str())
            || s.contains(Self::Minus.to_str())
            || s.contains(Self::Divide.to_str())
            || s.contains(Self::And.to_str())
            || s.contains(Self::Or.to_str())
            || s.contains(Self::LessThan.to_str())
            || s.contains(Self::MoreThan.to_str())
            || s.contains(Self::Equal.to_str())
            || s.contains(Self::Tilte.to_str())
    }

    pub fn to_xml(&self) -> &str {
        match self {
            Self::LessThan => "&lt;",
            Self::MoreThan => "&gt;",
            Self::And => "&amp;",
            _ => self.to_str(),
        }
    }
}
