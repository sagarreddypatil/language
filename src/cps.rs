use crate::ast::{Name, Op};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct CntDef<Lit> {
    pub name: Name,
    pub args: Vec<Name>,
    pub body: CpsExpr<Lit>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunDef<Lit> {
    pub name: Name,
    pub ret: Name,
    pub args: Vec<Name>,
    pub body: CpsExpr<Lit>,
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
                            "if ({} {}) {}() else {}()",
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
                            "if ({} {} {}) {}() else {}()",
                            args[0], op, args[1], tr, fl
                        )
                    }
                } else {
                    write!(
                        f,
                        "if ({}({})) {}() else {}()",
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
