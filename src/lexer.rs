use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // keywords
    #[token("match")]
    Match,
    #[token("data")]
    Data,
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,

    // Literals
    #[regex("-?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Int(i64),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    // Symbols
    #[token("=>")]
    FatArrow,
    #[token("=")]
    Eq,
    #[token("->")]
    Arrow,

    #[token("|")]
    Pipe,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("(")]
    POpen,
    #[token(")")]
    PClose,
    #[token("{")]
    BOpen,
    #[token("}")]
    BClose,

    // Identifiers
    #[regex("([a-zA-Z_<>][a-zA-Z0-9_]*)|([=<>]{2,})|[+*/]|-", |lex| lex.slice().to_string())]
    Ident(String),

    // comments
    #[regex("//[^\n]*", logos::skip)]
    #[regex("/\\*([^*]|\\*+[^*/])*\\*+/", logos::skip)]
    Comment,

    EOF,
}
