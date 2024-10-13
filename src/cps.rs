use crate::ast::{Name, Op};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LitHigh {
    Int(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CpsExpr<Lit> {
    Const {
        name: Name,
        value: Lit,
        body: Box<CpsExpr<Lit>>,
    },
    Prim {
        name: Name,
        op: Name,
        args: Vec<Name>,
        body: Box<CpsExpr<Lit>>,
    },

    Cnts {
        cnts: Vec<CntDef<Lit>>,
        body: Box<CpsExpr<Lit>>,
    },
    Funs {
        funs: Vec<FunDef<Lit>>,
        body: Box<CpsExpr<Lit>>,
    },

    AppC {
        cnt: Name,
        args: Vec<Name>,
    },
    AppF {
        fun: Name,
        ret: Name,
        args: Vec<Name>,
    },

    If {
        op: Name,
        args: Vec<Name>,
        t: Name,
        f: Name,
    },

    // Continuation defining the program end
    Halt(Name),
}

pub trait Substitutable {
    fn subst(&self, subst: Subst) -> Self;
}

impl<T: Clone> Substitutable for CpsExpr<T> {
    fn subst(&self, subst: Subst) -> CpsExpr<T> {
        use CpsExpr::*;

        match self {
            Const { name, value, body } => Const {
                name: subst.apply(name),
                value: value.clone(),
                body: Box::new(body.subst(subst)),
            },
            Prim { name, op, args, body } => Prim {
                name: subst.apply(name),
                op: op.clone(),
                args: args.iter().map(|a| subst.apply(a)).collect(),
                body: Box::new(body.subst(subst)),
            },
            Cnts { cnts, body } => Cnts {
                cnts: cnts
                    .iter()
                    .map(|cnt| cnt.subst(subst.clone()))
                    .collect::<Vec<_>>(),
                body: Box::new(body.subst(subst)),
            },
            Funs { funs, body } => Funs {
                funs: funs
                    .iter()
                    .map(|fun| fun.subst(subst.clone()))
                    .collect::<Vec<_>>(),
                body: Box::new(body.subst(subst)),
            },
            AppC { cnt, args } => AppC {
                cnt: subst.apply(cnt),
                args: args.iter().map(|a| subst.apply(a)).collect(),
            },
            AppF { fun, ret, args } => AppF {
                fun: subst.apply(fun),
                ret: subst.apply(ret),
                args: args.iter().map(|a| subst.apply(a)).collect(),
            },
            If { op, args, t, f } => If {
                op: subst.apply(op),
                args: args.iter().map(|a| subst.apply(a)).collect(),
                t: subst.apply(t),
                f: subst.apply(f),
            },
            Halt(name) => Halt(subst.apply(name)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CntDef<Lit> {
    pub name: Name,
    pub args: Vec<Name>,
    pub body: CpsExpr<Lit>,
}

impl<T: Clone> Substitutable for CntDef<T> {
    fn subst(&self, subst: Subst) -> CntDef<T> {
        CntDef {
            name: subst.apply(&self.name),
            args: self.args.iter().map(|a| subst.apply(a)).collect(),
            body: self.body.subst(subst),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunDef<Lit> {
    pub name: Name,
    pub ret: Name,
    pub args: Vec<Name>,
    pub body: CpsExpr<Lit>,
}

impl<T: Clone> Substitutable for FunDef<T> {
    fn subst(&self, subst: Subst) -> FunDef<T> {
        FunDef {
            name: subst.apply(&self.name),
            ret: subst.apply(&self.ret),
            args: self.args.iter().map(|a| subst.apply(a)).collect(),
            body: self.body.subst(subst),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Subst {
    pub map: std::collections::HashMap<Name, Name>,
}

impl Subst {
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn one(from: Name, to: Name) -> Self {
        assert!(from != to);

        let mut map = std::collections::HashMap::new();
        map.insert(from, to);
        Self { map }
    }

    pub fn insert(&mut self, key: Name, value: Name) {
        self.map.insert(key, value);
    }

    pub fn apply(&self, name: &Name) -> Name {
        let mut name = name;

        while self.map.contains_key(name) {
            name = &self.map[name];
        }

        name.clone()
    }
}

impl Display for LitHigh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LitHigh::Int(n) => write!(f, "{}", n),
        }
    }
}

impl<Lit: Display> Display for CpsExpr<Lit> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use CpsExpr::*;

        match self {
            Const { name, value, body } => write!(f, "const {} = {};\n{}", name, value, body),
            Prim {
                name,
                op,
                args,
                body,
            } => if op.valid() {
                if op.unary() {
                    assert!(args.len() == 1);
                    write!(
                        f,
                        "let {} = {} {};\n{}",
                        name,
                        op,
                        args[0],
                        body
                    )
                } else {
                    assert!(args.len() == 2);
                    write!(
                        f,
                        "let {} = {} {} {};\n{}",
                        name,
                        args[0],
                        op,
                        args[1],
                        body
                    )
                }
            } else {
                write!(
                    f,
                    "let {} = {}({});\n{}",
                    name,
                    op,
                    args.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    body
                )
            },
            Cnts { cnts, body } => {
                let cnts_str = cnts
                    .iter()
                    .map(|cnt| format!("{}", cnt))
                    .collect::<Vec<_>>()
                    .join("\n");
                write!(f, "{}\n{}", cnts_str, body)
            }
            Funs { funs, body } => {
                let funs_str = funs
                    .iter()
                    .map(|fun| format!("{}", fun))
                    .collect::<Vec<_>>()
                    .join("\n");
                write!(f, "{}\n{}", funs_str, body)
            }
            AppC { cnt, args } => write!(
                f,
                "{}({})",
                cnt,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            AppF { fun, ret, args } => write!(
                f,
                "{}({}, {})",
                fun,
                ret,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            If {
                op,
                args,
                t: tr,
                f: fl,
            } => {
                if op.valid() {
                    if op.unary() {
                        assert!(args.len() == 1);
                        write!(
                            f,
                            "if ({} {}) {{ {}() }} else {{ {}() }}",
                            op,
                            args.iter()
                                .map(|a| a.to_string())
                                .collect::<Vec<_>>()
                                .join(" "),
                            tr,
                            fl
                        )
                    } else {
                        assert!(args.len() == 2);
                        write!(
                            f,
                            "if ({} {} {}) {{ {}() }} else {{ {}() }}",
                            args[0], op, args[1], tr, fl
                        )
                    }
                } else {
                    write!(
                        f,
                        "if ({}({})) {{ {}() }} else {{ {}() }}",
                        op,
                        args.iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<_>>()
                            .join(" "),
                        tr,
                        fl
                    )
                }
            }
            Halt(name) => write!(f, "halt({})", name),
        }
    }
}

impl<Lit: Display> Display for CntDef<Lit> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "cnt {}({}) {{\n{}\n}}",
            self.name,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            // indent body
            self.body
                .to_string()
                .lines()
                .map(|l| format!("    {}", l))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl<Lit: Display> Display for FunDef<Lit> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "function {}({}, {}) {{\n{}\n}}",
            self.name,
            self.ret,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            // indent body
            self.body
                .to_string()
                .lines()
                .map(|l| format!("    {}", l))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
