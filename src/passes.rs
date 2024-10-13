use std::{collections::HashMap, ops::Rem};

use crate::{
    ast::Name,
    cps::{CntDef, CpsExpr as BaseCpsExpr, FunDef, LitHigh, Subst, Substitutable},
};
type CpsExpr = BaseCpsExpr<LitHigh>;

pub trait TreePass {
    fn apply(&mut self, tree: CpsExpr) -> CpsExpr;
}

pub struct RemoveDupConsts {
    consts: HashMap<Name, LitHigh>,
    consts_inv: HashMap<LitHigh, Name>,
}

impl RemoveDupConsts {
    pub fn new() -> Self {
        Self {
            consts: HashMap::new(),
            consts_inv: HashMap::new(),
        }
    }
}

impl TreePass for RemoveDupConsts {
    fn apply(&mut self, tree: CpsExpr) -> CpsExpr {
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
            Prim { name, op, args, body } =>
                Prim { name, op, args, body: Box::new(self.apply(*body)) },

            #[rustfmt::skip]
            Cnts { cnts, body } =>
                Cnts {
                    cnts: cnts.into_iter().map(|cnt| {
                        let CntDef { name, args, body } = cnt;
                        CntDef { name, args, body: self.apply(body) }
                    }).collect(),
                    body: Box::new(self.apply(*body)),
                },

            #[rustfmt::skip]
            Funs { funs, body } => 
                Funs {
                    funs: funs.into_iter().map(|fun| {
                            let FunDef { name, args, body, ret} = fun;
                            FunDef { name, args, body: self.apply(body), ret }
                        }).collect(),
                    body: Box::new(self.apply(*body)),
                },

            AppC { cnt, args } => AppC { cnt, args },
            AppF { fun, ret, args } => AppF { fun, ret, args },
            If { op, args, t, f } => If { op, args, t, f },
            Halt(name) => Halt(name),
        }
    }
}
