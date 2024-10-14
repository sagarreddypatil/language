use std::{collections::HashMap, ops::Rem};

use crate::{
    ast::Name,
    cps::{CntDef, CpsExpr as BaseCpsExpr, FunDef, LitHigh, Subst, Substitutable},
};
type CpsExpr = BaseCpsExpr<LitHigh>;

pub trait TreePass {
    fn apply(self, tree: CpsExpr) -> CpsExpr;
}

#[derive(Clone)]
pub struct Shrinking {
    consts: HashMap<Name, LitHigh>,
    consts_inv: HashMap<LitHigh, Name>,
}

impl Shrinking {
    pub fn new() -> Self {
        Self {
            consts: HashMap::new(),
            consts_inv: HashMap::new(),
        }
    }
}

impl TreePass for Shrinking {
    fn apply(mut self, tree: CpsExpr) -> CpsExpr {
        use BaseCpsExpr::*;

        match tree {
            Const { name, value, body } => {
                if let Some(existing) = self.consts_inv.get(&value) {
                    let nbody = body.subst(Subst::one(name.clone(), existing.clone()));
                    self.apply(nbody)
                } else {
                    self.consts.insert(name.clone(), value.clone());
                    self.consts_inv.insert(value.clone(), name.clone());

                    Const {
                        name,
                        value,
                        body: Box::new(self.apply(*body)),
                    }
                }
            }

            #[rustfmt::skip]
            Prim { name, op, args, body } => {
                let const_args = args.iter().all(|arg| self.consts.contains_key(arg)) && op != Name::new("data");
                if const_args {
                    let args = args.into_iter().map(|arg| self.consts.remove(&arg).unwrap()).collect();
                    let value = eval_op(op, args);
                    Const { name, value, body: Box::new(self.apply(*body)) }
                }
                // else if op == Name::new("id") {
                //     let actual = args[0].clone();
                //     let nbody = body.subst(Subst::one(name.clone(), actual));
                //     self.apply(nbody)
                // }
                else {
                    Prim { name, op, args, body: Box::new(self.apply(*body)) }
                }
            }

            #[rustfmt::skip]
            Cnts { cnts, body } =>
                Cnts {
                    cnts: cnts.into_iter().map(|cnt| {
                        let CntDef { name, args, body } = cnt;
                        CntDef { name, args, body: self.clone().apply(body) }
                    }).collect(),
                    body: Box::new(self.apply(*body)),
                },

            #[rustfmt::skip]
            Funs { funs, body } => 
                Funs {
                    funs: funs.into_iter().map(|fun| {
                            let FunDef { name, args, body, ret} = fun;
                            FunDef { name, args, body: self.clone().apply(body), ret }
                        }).collect(),
                    body: Box::new(self.apply(*body)),
                },

            AppC { cnt, args } => AppC { cnt, args },
            AppF { fun, ret, args } => AppF { fun, ret, args },

            If { op, args, t, f } => {
                let const_args = args.iter().all(|arg| self.consts.contains_key(arg));
                if const_args {
                    let args = args.into_iter().map(|arg| self.consts.remove(&arg).unwrap()).collect();
                    let value = eval_op(op, args);

                    let LitHigh::Int(value) = value;
                    if value > 0 {
                        AppC { cnt: t, args: vec![] }
                    } else {
                        AppC { cnt: f, args: vec![] }
                    }
                } else {
                    If { op, args, t, f }
                }
            },
            Halt(name) => Halt(name),
        }
    }
}

fn eval_op(op: Name, args: Vec<LitHigh>) -> LitHigh {
    let op = op.0;
    let args = args.into_iter().map(|LitHigh::Int(i)| i).collect::<Vec<_>>();

    let out = match op.as_str() {
        "+" => args[0] + args[1],
        "-" => args[0] - args[1],
        "*" => args[0] * args[1],
        "/" => args[0] / args[1],
        "~" => !args[0],
        "==" => (args[0] == args[1]) as i64,
        "!=" => (args[0] != args[1]) as i64,
        "<" => (args[0] < args[1]) as i64,
        ">" => (args[0] > args[1]) as i64,
        "<=" => (args[0] <= args[1]) as i64,
        ">=" => (args[0] >= args[1]) as i64,
        "&&" => (args[0] & args[1]),
        "||" => (args[0] | args[1]),
        "!" => !(args[0] > 0) as i64,
        _ => panic!("unknown op: {}", op),
    };

    LitHigh::Int(out)
}
