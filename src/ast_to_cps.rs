use std::collections::HashMap;

use crate::{
    ast::{DataDef, Expr, Name, Op, Pattern, Program, Simp},
    cps::{CntDef, FunDef, LitHigh},
};

use crate::cps::CpsExpr as BaseCpsExpr;

pub struct AstToCps {
    data_defs: Vec<DataDef>,
    sym_counts: HashMap<String, i64>,
}

type CpsExpr = BaseCpsExpr<LitHigh>;
type Context = Box<dyn FnOnce(&mut AstToCps, Name) -> CpsExpr>;
type VecContext = Box<dyn FnOnce(&mut AstToCps, Vec<Name>) -> CpsExpr>;

impl AstToCps {
    pub fn convert(program: Program) -> CpsExpr {
        let mut obj = Self {
            data_defs: program.data_defs.clone(),
            sym_counts: HashMap::new(),
        };

        obj.lower_expr(program.expr.unwrap())
    }

    fn fresh(&mut self, sym: String) -> Name {
        let count = self.sym_counts.entry(sym.clone()).or_insert(0);
        *count += 1;
        Name(format!("{}${}", sym, count))
    }

    fn simp_list(&mut self, mut simps: Vec<Simp>, ctx: VecContext, mut acc: Vec<Name>) -> CpsExpr {
        if simps.is_empty() {
            ctx(self, acc)
        } else {
            let head = simps.remove(0);
            self.lower_simp(
                head,
                Box::new(|s, name| {
                    s.simp_list(simps, ctx, {
                        acc.push(name);
                        acc
                    })
                }),
            )
        }
    }

    fn lower_simp(&mut self, simp: Simp, ctx: Context) -> CpsExpr {
        use Simp::*;
        match simp {
            FnDef(f) => {
                let anon = self.fresh("anon".to_string());
                let retc = self.fresh("retc".to_string());
                let args = f.args.iter().map(|(name, _)| name.clone()).collect();

                let lfun = FunDef {
                    name: anon.clone(),
                    ret: retc.clone(),
                    args,
                    body: self.lower_simp(
                        *f.body,
                        Box::new(move |_, ret| CpsExpr::AppC {
                            cnt: retc.clone(),
                            args: vec![ret],
                        }),
                    ),
                };

                CpsExpr::Funs {
                    funs: vec![lfun],
                    body: Box::new(ctx(self, anon)),
                }
            }
            Match(simp, arms) => unimplemented!(),
            FnCall(lhs, rhs) => {
                let lhs = *lhs;

                match lhs {
                    Ref(name) if name.valid() => self.simp_list(
                        rhs,
                        Box::new(|s, rhs| {
                            let n_prim = s.fresh("prim".to_string());
                            CpsExpr::Prim {
                                name: n_prim.clone(),
                                op: name,
                                args: rhs,
                                body: Box::new(ctx(s, n_prim)),
                            }
                        }),
                        vec![],
                    ),
                    _ => {
                        let cont_name = self.fresh("fn_ret_cnt".to_string());
                        let ret_name = self.fresh("fn_ret_val".to_string());

                        let cont = CntDef {
                            name: cont_name.clone(),
                            args: vec![ret_name.clone()],
                            body: ctx(self, ret_name),
                        };

                        CpsExpr::Cnts {
                            cnts: vec![cont],
                            body: Box::new(self.lower_simp(
                                lhs,
                                Box::new(|s, lhs| {
                                    s.simp_list(
                                        rhs,
                                        Box::new(|_, rhs| CpsExpr::AppF {
                                            fun: lhs,
                                            ret: cont_name,
                                            args: rhs,
                                        }),
                                        vec![],
                                    )
                                }),
                            )),
                        }
                    }
                }
            }
            Ref(name) => ctx(self, name),
            Int(n) => {
                let name = self.fresh("int".to_string());
                CpsExpr::Const {
                    name: name.clone(),
                    value: LitHigh::Int(n),
                    body: Box::new(ctx(self, name)),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn lower_pattern_match(&mut self, pat: Pattern, val: Name, body: CpsExpr) -> CpsExpr {
        match pat {
            Pattern::Var(name, _) => CpsExpr::Prim {
                name,
                op: Name("id".to_string()),
                args: vec![val],
                body: Box::new(body),
            },
            Pattern::Int(n) => {
                // create two continuations, one for good match, one for bad
                let good = self.fresh("pm_good".to_string());
                let bad = self.fresh("pm_bad".to_string());

                let good_cnt = CntDef {
                    name: good.clone(),
                    args: vec![],
                    body,
                };

                let bad_cnt = CntDef {
                    name: bad.clone(),
                    args: vec![],
                    body: CpsExpr::Halt(Name("pattern_match_failed".to_string())),
                };

                let disc = self.fresh("pm_disc".to_string());
                CpsExpr::Const {
                    name: disc.clone(),
                    value: LitHigh::Int(n),
                    body: Box::new(CpsExpr::Cnts {
                        cnts: vec![good_cnt, bad_cnt],
                        body: Box::new(CpsExpr::If {
                            op: Name("==".to_string()),
                            args: vec![disc, val],
                            t: good,
                            f: bad,
                        }),
                    }),
                }
            }
            Pattern::Bool(b) => {
                // convert bool to int
                let npat = if b { Pattern::Int(1) } else { Pattern::Int(0) };
                self.lower_pattern_match(npat, val, body)
            }
            // Pattern::Data(data_def, disc, pats) => {
            // }
            _ => unimplemented!(),
        }
    }

    fn lower_expr(&mut self, high: Expr) -> CpsExpr {
        match high {
            Expr::Bind(pat, rhs, body) => self.lower_simp(
                rhs,
                Box::new(|s: &mut Self, rhs| {
                    let body = s.lower_expr(*body);
                    s.lower_pattern_match(pat, rhs, body)
                }),
            ),
            Expr::Simp(simp) => self.lower_simp(simp, Box::new(|_, rhs| CpsExpr::Halt(rhs))),
        }
    }
}
