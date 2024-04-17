#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Name(pub &'static str);

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Unit,
    Fn(Vec<Type>, Box<Type>),
    UserDef(Name),
    Var(usize), // unresolved type variable
}

#[derive(Debug)]
pub struct DataDef {
    pub name: Name,
    pub cons: Vec<Cons>,
}

#[derive(Debug)]
pub struct TypeDef {
    pub name: Name,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Cons {
    // ADT Constructor Definition
    pub tag: Name,
    pub args: Vec<Type>,
}

#[derive(Debug)]
pub enum Pattern {
    Var(Name), // keep it simple for now
}

#[derive(Debug)]
pub enum Expr {
    Bind(Name, Pattern, Box<Expr>),
    Simp(Simp)
}

#[derive(Debug)]
pub enum Simp {
    FnDef(FnDef),
    Match(Box<Simp>, Vec<(Pattern, Simp)>),
    FnCall(Name, Vec<Simp>),
    Ref(Name),

    // literals
    Int(i64),
    Unit,
    Data(Name, Vec<Simp>),
}

#[derive(Debug)]
pub struct FnDef {
    pub args: Vec<(Name, Type)>,
    pub body: Box<Simp>,
    pub ret: Type,
}

#[derive(Debug)]
pub struct Program {
    pub data_defs: Vec<DataDef>,
    pub type_defs: Vec<TypeDef>,
    pub expr: Option<Expr>,
}