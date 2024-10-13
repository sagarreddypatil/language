use std::fmt::Display;
use serde_lexpr::to_string;
use serde::{Serialize, Deserialize};
use crate::ast::Name;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LitHigh {
    Int(i64),
    Data(i64, Vec<Name>), // i64 is the discriminator, i.e. index of the constructor
}

impl Display for LitHigh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // s-expressions
        use LitHigh::*;
        match self {
            Int(n) => write!(f, "{}", n),
            Data(d, args) => {
                write!(f, "({}", d)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CpsExpr<Lit> {
    Const { name: Name, value: Lit, body: Box<CpsExpr<Lit>> },
    Prim { name: Name, op: Name, args: Vec<Name>, body: Box<CpsExpr<Lit>> },

    Cnts { cnts: Vec<CntDef<Lit>>, body: Box<CpsExpr<Lit>> },
    Funs { funs: Vec<FunDef<Lit>>, body: Box<CpsExpr<Lit>> },

    AppC { cnt: Name, args: Vec<Name> },
    AppF { fun: Name, ret: Name, args: Vec<Name> },

    If { op: Name, args: Vec<Name>, t: Name, f: Name },

    // Continuation defining the program end
    Halt(Name),
}

fn lispify_vec<T: Display>(v: &Vec<T>) -> String {
    let mut s = String::new();
    for (i, item) in v.iter().enumerate() {
        if i > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{}", item));
    }
    s
}

impl Display for CpsExpr<LitHigh> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // s-expressions
        use CpsExpr::*;
        match self {
            Const { name, value, body } => write!(f, "(letc ({} {}) {})", name, value, body),
            Prim { name, op, args, body } => write!(f, "(letp {} ({} {}) {})", name, op, lispify_vec(args), body),

            Cnts { cnts, body } => {
                write!(f, "(cnts ")?;
                for cnt in cnts {
                    write!(f, "{} ", cnt)?;
                }
                write!(f, "{}", body)?;
                write!(f, ")")
            },
            Funs { funs, body } => {
                write!(f, "(funs ")?;
                for fun in funs {
                    write!(f, "{} ", fun)?;
                }
                write!(f, "{}", body)?;
                write!(f, ")")
            },

            AppC { cnt, args } => {
                write!(f, "(appc {} ", cnt)?;
                for arg in args {
                    write!(f, "{} ", arg)?;
                }
                write!(f, ")")
            },
            AppF { fun, ret, args } => {
                write!(f, "(appf {} {} ", fun, ret)?;
                for arg in args {
                    write!(f, "{} ", arg)?;
                }
                write!(f, ")")
            },

            If { op, args, t: tr, f: fl } => write!(f, "(if {} {} {} {})", op, args.len(), tr, fl),

            Halt(val) => write!(f, "(halt {})", val),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CntDef<Lit> {
    pub name: Name,
    pub args: Vec<Name>,
    pub body: CpsExpr<Lit>,
}

impl Display for CntDef<LitHigh> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(defcnt {} (", self.name)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ") ")?;
        write!(f, "{}", self.body)?;
        write!(f, ")")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunDef<Lit> {
    pub name: Name,
    pub ret: Name,
    pub args: Vec<Name>,
    pub body: CpsExpr<Lit>,
}

impl Display for FunDef<LitHigh> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(deffun {} {} (", self.name, self.ret)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ") ")?;
        write!(f, "{}", self.body)?;
        write!(f, ")")
    }
}
