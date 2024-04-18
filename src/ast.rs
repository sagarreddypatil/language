use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Name(pub &'static str);

impl Name {
    pub fn from_string_ref(s: &String) -> Name {
        Name(Box::leak(Box::new(s.clone())).as_str())
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
        match self.0 {
            "+" | "-" | "*" | "/" | "%" | "~" => true,
            _ => false,
        }
    }

    fn prec(&self) -> i32 {
        match self.0 {
            "+" | "-" => 1,
            "*" | "/" | "%" => 2,
            _ => 0,
        }
    }

    fn assoc(&self) -> i32 {
        match self.0 {
            "+" | "-" | "*" | "/" | "%" => 1,
            _ => 0,
        }
    }

    fn unary(&self) -> bool {
        match self.0 {
            // "-" | "~" => true,
            "~" => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
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
    // pub cons: Vec<Cons>,
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
    Int(i64), // literal
    Data(DataDef, Name, Vec<Pattern>),

    // TODO: structural matching
}

impl Pattern {
    pub fn bindings(&self) -> Vec<(Name, Type)> {
        use Pattern::*;
        match self {
            Var(name, ty) => vec![(*name, ty.clone())],
            Int(_) => vec![],
            Data(_, _, pats) => pats.iter().flat_map(|pat| pat.bindings()).collect(),
        }
    }
}

impl Pattern {
    pub fn get_type(&self) -> Type {
        use Pattern::*;
        match self {
            Var(_, ty) => ty.clone(),
            Int(_) => Type::Int,
            Data(data, _, _) => Type::UserDef(data.name),
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
    Unit,
    Data(Name, Vec<Simp>),
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub args: Vec<(Name, Type)>,
    pub body: Box<Simp>,
    pub ret: Type,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub data_defs: Vec<DataDef>,
    // pub type_defs: Vec<TypeDef>, // TODO: implement later, parser commented
    pub expr: Option<Expr>,
}
