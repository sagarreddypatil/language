use phf::phf_map;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // delimiters
    Colon,
    Comma,
    Eq,
    Pipe,
    POpen,
    PClose,
    BOpen,
    BClose,
    Endl,

    // keywords
    FnDef,
    Let,
    Match,
    DataDef,
    TypeDef,
    Arrow,

    // literals, names
    Name(String),
    Number(i64),

    //
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Eq => write!(f, "="),
            Token::Pipe => write!(f, "|"),
            Token::POpen => write!(f, "("),
            Token::PClose => write!(f, ")"),
            Token::BOpen => write!(f, "{{"),
            Token::BClose => write!(f, "}}"),
            Token::Endl => write!(f, "\n"),

            Token::FnDef => write!(f, "fn"),
            Token::Let => write!(f, "let "),
            Token::Match => write!(f, "match "),
            Token::DataDef => write!(f, "data "),
            Token::TypeDef => write!(f, "type "),
            Token::Arrow => write!(f, "->"),

            Token::Name(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),

            Token::EOF => write!(f, ""),
        }
    }
}

pub struct Tokens<'a>(pub &'a Vec<Token>);

impl<'a> fmt::Display for Tokens<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for token in self.0 {
            s.push_str(&format!("{}", token));
        }
        write!(f, "{}", s)
    }
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "data" => Token::DataDef,
    "type" => Token::TypeDef,
    "let" => Token::Let,
    "fn" => Token::FnDef,
    "match" => Token::Match,
    "->" => Token::Arrow,
};

static DELIMS: phf::Map<char, Token> = phf_map! {
    ':' => Token::Colon,
    ',' => Token::Comma,
    '=' => Token::Eq,
    '|' => Token::Pipe,
    '(' => Token::POpen,
    ')' => Token::PClose,
    '{' => Token::BOpen,
    '}' => Token::BClose,
    ';' => Token::Endl,
    '\n' => Token::Endl,
};

fn is_delim(c: char) -> bool {
    DELIMS.contains_key(&c)
}

fn is_keyword(s: &str) -> bool {
    KEYWORDS.contains_key(s)
}

fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}

pub struct Scanner {
    pub source: Vec<char>,
    position: usize,

    pub tokens: Vec<Token>,
    buffer: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,

            tokens: Vec::new(),
            buffer: String::new(),
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.position).cloned()
    }

    pub fn next(&mut self) -> char {
        let c = self.source[self.position];
        self.position += 1;
        c
    }

    pub fn handle_word(&mut self) {
        if is_keyword(&self.buffer) {
            self.tokens.push(KEYWORDS[&self.buffer].clone());
        } else if is_numeric(self.buffer.as_str()) {
            self.tokens
                .push(Token::Number(self.buffer.parse().unwrap()));
        } else {
            self.tokens.push(Token::Name(self.buffer.clone()));
        }
    }

    pub fn tokenize(&mut self) {
        while let Some(c) = self.peek() {
            let mut flush = false;
            let mut next: Option<Token> = None;

            if is_delim(c) {
                flush = true;
                let c = self.next();
                next = Some(DELIMS[&c].clone());
            } else if c.is_whitespace() {
                flush = true;
                self.next();
            }

            if flush && !self.buffer.is_empty() {
                self.handle_word();
                self.buffer.clear();
            }

            if flush {
                next.map(|t| self.tokens.push(t));
                continue;
            }

            let next = self.next();
            self.buffer.push(next);
        }
        if !self.buffer.is_empty() {
            self.handle_word();
        }

        self.tokens.push(Token::EOF);
    }
}
