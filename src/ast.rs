pub struct Name(String);

pub enum Type {
    Int,
    Fn(Vec<Type>, Box<Type>),
    UserDef(Name, Vec<Cons>),
    Var(usize),
}

pub struct Cons(Name, Vec<Type>);

pub enum Literal {
    Int(i64),
    Cons(Name, Vec<Literal>),
}



pub enum ExprKind {
    FnDef(Name, Vec<Name>, Vec<Type>, Box<Expr>),
    Match(Box<Expr>, Vec<(Cons, Expr)>),
    FnCall(Name, Vec<Expr>),
    // Let(
    Lit(Literal),
}

pub struct Expr {
    pub kind: ExprKind,
    pub typ: Type,
}

