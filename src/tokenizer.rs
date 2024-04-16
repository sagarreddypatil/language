use phf::phf_map;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Colon,
    Comma,
    Eq,
    POpen,
    PClose,
    BOpen,
    BClose,
    FnDef,
    Let,
    Match,
    Endl,
    Unk,
    Name(String),
    Number(i64),
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "let" => Token::Let,
    "fn" => Token::FnDef,
    "match" => Token::Match,
    "_" => Token::Unk,
};

static DELIMS: phf::Map<char, Token> = phf_map! {
    ':' => Token::Colon,
    ',' => Token::Comma,
    '=' => Token::Eq,
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
                if is_keyword(&self.buffer) {
                    self.tokens.push(KEYWORDS[&self.buffer].clone());
                } else if is_numeric(self.buffer.as_str()) {
                    self.tokens
                        .push(Token::Number(self.buffer.parse().unwrap()));
                } else {
                    self.tokens.push(Token::Name(self.buffer.clone()));
                }

                self.buffer.clear();
            }

            if flush {
                next.map(|t| self.tokens.push(t));
                continue;
            }

            let next = self.next();
            self.buffer.push(next);
        }
    }
}
