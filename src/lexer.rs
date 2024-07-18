use std::{cmp::max, fmt};

use crate::ast::leak_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(&'static str),
    Ident(&'static str),

    IntLit(i64),
    BoolLit(bool),
    UnitLit,

    Pipe,
    Colon,
    Comma,
    POpen,
    PClose,
    BOpen,
    BClose,

    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Keyword(k) => write!(f, "{}", k),
            TokenKind::Ident(i) => write!(f, "{}", i),
            TokenKind::IntLit(i) => write!(f, "{}", i),
            TokenKind::BoolLit(b) => write!(f, "{}", b),
            TokenKind::UnitLit => write!(f, "()"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::POpen => write!(f, "("),
            TokenKind::PClose => write!(f, ")"),
            TokenKind::BOpen => write!(f, "{{"),
            TokenKind::BClose => write!(f, "}}"),
            TokenKind::EOF => write!(f, "<EOF>"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
    pub length: usize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Token: {} at {}:{}", self.kind, self.line, self.col)
        write!(f, "{:?}", self)
    }
}

pub struct Tokens {
    pub list: Vec<Token>,
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for token in &self.list {
            writeln!(f, "{}", token)?;
        }

        Ok(())
    }
}

pub struct Scanner {
    pub input: String,
    pub tokens: Tokens,

    line: usize,
    col: usize,

    buf: String,
}

const KEYWORDS: &'static [&str] = &[
    "match", "data", "fn", "let", "if", "else", "=>", "=", "->"
];

impl Scanner {
    pub fn new(input: String) -> Self {
        Self {
            input: input,
            tokens: Tokens {
                list: Vec::new(),
            },

            line: 1,
            col: 0,

            buf: String::new(),
        }
    }

    fn try_parse_int_lit(&mut self) -> Option<i64> {
        self.buf.parse::<i64>().ok()
    }

    fn parse_buf(&mut self) {
        if self.buf.len() == 0 {
            return;
        }

        let kind = match self.buf.as_str() {
            "true" => TokenKind::BoolLit(true),
            "false" => TokenKind::BoolLit(false),
            "()" => TokenKind::UnitLit,
            _ => {
                if let Some(i) = KEYWORDS.iter().position(|&k| k == self.buf) {
                    TokenKind::Keyword(KEYWORDS[i])
                } else

                if let Some(i) = self.try_parse_int_lit() {
                    TokenKind::IntLit(i)
                } else {
                    TokenKind::Ident(leak_str(self.buf.clone()))
                }
            }
        };

        self.tokens.list.push(Token {
            kind,
            line: self.line,
            col: max(self.col - self.buf.len(), 1),
            length: self.buf.len(),
        });

        self.buf.clear();
    }

    fn parse_symbol(c: char) -> Option<TokenKind> {
        match c {
            '(' => Some(TokenKind::POpen),
            ')' => Some(TokenKind::PClose),
            '{' => Some(TokenKind::BOpen),
            '}' => Some(TokenKind::BClose),
            ',' => Some(TokenKind::Comma),
            '|' => Some(TokenKind::Pipe),
            ':' => Some(TokenKind::Colon),
            _ => None,
        }
    }

    fn push_single(&mut self, kind: TokenKind) {
        self.tokens.list.push(Token {
            kind,
            line: self.line,
            col: self.col,
            length: 1,
        });
    }

    pub fn tokenize(&mut self) {
        let input = self.input.clone();
        let mut input = input.chars();

        while let Some(c) = input.next() {
            self.col += 1;

            match c {
                '\n' => {
                    self.parse_buf();
                    self.line += 1;
                    self.col = 0;
                },

                ' ' | '\t' => self.parse_buf(),

                _ => {
                    if let Some(t) = Self::parse_symbol(c) {
                        self.parse_buf();
                        self.push_single(t);
                    } else {
                        self.buf.push(c);
                    }
                }
            }
        }

        self.parse_buf();
        self.push_single(TokenKind::EOF);
    }
}
