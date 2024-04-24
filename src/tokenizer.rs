use phf::phf_map;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
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
    FatArrow,

    // literals, names
    Name(String),
    Number(i64),
    Bool(bool),

    // end of file
    EOF,
}

#[derive(Debug)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Pos,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            Colon => write!(f, ":"),
            Comma => write!(f, ","),
            Eq => write!(f, "="),
            Pipe => write!(f, "|"),
            POpen => write!(f, "("),
            PClose => write!(f, ")"),
            BOpen => write!(f, "{{"),
            BClose => write!(f, "}}"),
            Endl => write!(f, ";"),

            FnDef => write!(f, "fn"),
            Let => write!(f, "let"),
            Match => write!(f, "match"),
            DataDef => write!(f, "data"),
            TypeDef => write!(f, "type"),
            Arrow => write!(f, "->"),
            FatArrow => write!(f, "=>"),

            Name(s) => write!(f, "{}", s),
            Number(n) => write!(f, "{}", n),
            Bool(b) => write!(f, "{}", b),

            EOF => write!(f, "<EOF>"),
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub list: Vec<Token>,
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for token in self.list.iter() {
            s.push_str(&format!("{}:{}: {}\n", token.pos.line, token.pos.col, token.kind));
        }

        write!(f, "{}", s)
    }
}

impl Tokens {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn push(&mut self, token: Token) {
        use TokenKind::*;
        match (self.list.last().map(|x| &x.kind), &token.kind) {
            (Some(Endl), Endl) => return,
            _ => self.list.push(token),
        }
    }
}

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "data" => TokenKind::DataDef,
    "type" => TokenKind::TypeDef,
    "true" => TokenKind::Bool(true),
    "false" => TokenKind::Bool(false),
    "let" => TokenKind::Let,
    "fn" => TokenKind::FnDef,
    "match" => TokenKind::Match,
    "=" => TokenKind::Eq,
    "->" => TokenKind::Arrow,
    "=>" => TokenKind::FatArrow,
};

static DELIMS: phf::Map<char, TokenKind> = phf_map! {
    ':' => TokenKind::Colon,
    ',' => TokenKind::Comma,
    '|' => TokenKind::Pipe,
    '(' => TokenKind::POpen,
    ')' => TokenKind::PClose,
    '{' => TokenKind::BOpen,
    '}' => TokenKind::BClose,
    ';' => TokenKind::Endl,
    '\n' => TokenKind::Endl,
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

    pub tokens: Tokens,
    buffer: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,

            tokens: Tokens::new(),
            buffer: String::new(),
        }
    }

    fn pos(&self) -> Pos {
        let mut line = 1;
        let mut col = 0;

        for c in &self.source[..self.position] {
            if *c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }

        Pos { line, col }
    }

    fn push(&mut self, token: TokenKind) {
        self.tokens.push(Token {
            kind: token,
            pos: self.pos(),
        });
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position).cloned()
    }

    fn next(&mut self) -> char {
        let c = self.source[self.position];
        self.position += 1;
        c
    }

    fn handle_word(&mut self) {
        if is_keyword(&self.buffer) {
            self.push(KEYWORDS[&self.buffer].clone());
        } else if is_numeric(self.buffer.as_str()) {
            self.push(TokenKind::Number(self.buffer.parse().unwrap()));
        } else {
            self.push(TokenKind::Name(self.buffer.clone()));
        }
    }

    pub fn tokenize(&mut self) {
        while let Some(c) = self.peek() {
            let mut flush = false;
            let mut next: Option<TokenKind> = None;

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
                next.map(|t| self.push(t));
                continue;
            }

            let next = self.next();
            self.buffer.push(next);
        }

        if !self.buffer.is_empty() {
            self.handle_word();
        }

        self.push(TokenKind::EOF);
    }
}
