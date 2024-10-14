use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Name(pub String);

impl Name {
    pub fn new(s: &str) -> Self {
        Name(s.to_string())
    }
}

pub trait Op {
    fn valid(&self) -> bool;
    fn prec(&self) -> i32;
    fn assoc(&self) -> i32;
    fn unary(&self) -> bool;
}

impl Op for Name {
    fn valid(&self) -> bool {
        match self.0.as_str() {
            "+" | "-" | "*" | "/" | "%" | "~" => true,
            "==" | "!=" | "<" | ">" | "<=" | ">=" => true,
            "&&" | "||" => true,
            "!" => true,
            _ => false,
        }
    }

    fn prec(&self) -> i32 {
        match self.0.as_str() {
            "+" | "-" => 1,
            "*" | "/" | "%" => 2,
            "==" | "!=" | "<" | ">" | "<=" | ">=" => 0,
            "&&" | "||" => 0,
            "!" => 0,
            _ => 0,
        }
    }

    fn assoc(&self) -> i32 {
        match self.0.as_str() {
            "+" | "-" | "*" | "/" | "%" => 1,
            _ => 1,
        }
    }

    fn unary(&self) -> bool {
        match self.0.as_str() {
            // "-" | "~" => true,
            "~" => true,
            "!" => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Unit,
    Fn(Vec<Type>, Box<Type>),
    UserDef(Name),
    TyVar(usize), // unresolved type variable
}

pub fn fresh_tv() -> Type {
    static mut TVAR_COUNTER: usize = 0;
    unsafe {
        let ret = Type::TyVar(TVAR_COUNTER);
        TVAR_COUNTER += 1;
        ret
    }
}

#[derive(Debug, Clone)]
pub struct DataDef {
    pub name: Name,
    pub cons: HashMap<Name, Cons>,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: Name,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct Cons {
    pub args: Vec<Type>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Var(Name, Type), // naive binding
    Int(i64),
    Bool(bool),
    Data(DataDef, Name, Vec<Pattern>),

    // TODO: structural matching
}

impl Pattern {
    pub fn bindings(&self) -> Vec<(Name, Type)> {
        use Pattern::*;
        match self {
            Var(name, ty) => vec![(name.clone(), ty.clone())],
            Int(_) => vec![],
            Bool(_) => vec![],
            Data(_, _, pats) => pats.iter().flat_map(|pat| pat.bindings()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Bind(Pattern, Simp, Box<Expr>),
    Simp(Simp)
}

#[derive(Debug, Clone)]
pub enum Simp {
    FnDef(FnDef),
    Match(Box<Simp>, Vec<(Pattern, Simp)>),
    FnCall(Box<Simp>, Vec<Simp>),
    Block(Box<Expr>),
    Ref(Name),

    // literals
    Int(i64),
    Bool(bool),
    Unit,
    Data(Name, Vec<Simp>),
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub args: Vec<(Name, Type)>,
    pub body: Box<Simp>,
    pub ret: Type,
}

#[derive(Debug)]
pub struct Program {
    pub data_defs: Vec<DataDef>,
    // pub type_defs: Vec<TypeDef>, // TODO: implement later, parser commented
    pub expr: Option<Expr>,
}
