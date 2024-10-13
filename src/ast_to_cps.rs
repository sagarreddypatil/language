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
        Name(format!("{}_{}", sym, count))
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

    fn match_arms(&mut self, val: Name, mut arms: Vec<(Pattern, Simp)>, ret: Name) -> CpsExpr {
        // match val with arms, then call continuation ret with the result of the expression
        assert!(arms.len() > 0);

        if arms.len() == 1 {
            // no match now is invalid
            let no_match = Name("halt".to_string());
            let (pat, simp) = arms.remove(0);

            let body = self.lower_simp(
                simp,
                Box::new(move |_, simp| CpsExpr::AppC {
                    cnt: ret.clone(),
                    args: vec![simp],
                }),
            );
            self.lower_pattern_match(pat, val, body, no_match)
        } else {
            let (pat, simp) = arms.remove(0);

            let no_match = self.fresh("m_alt".to_string());
            let no_match_cont = CntDef {
                name: no_match.clone(),
                args: vec![],
                body: self.match_arms(val.clone(), arms, ret.clone()),
            };

            let body = self.lower_simp(
                simp,
                Box::new(move |_, simp| CpsExpr::AppC {
                    cnt: ret.clone(),
                    args: vec![simp],
                }),
            );

            CpsExpr::Cnts {
                cnts: vec![no_match_cont],
                body: Box::new(self.lower_pattern_match(pat, val, body, no_match)),
            }
        }
    }

    fn lower_simp(&mut self, simp: Simp, ctx: Context) -> CpsExpr {
        use Simp::*;
        match simp {
            FnDef(f) => {
                let anon = self.fresh("fn".to_string());
                let retc = self.fresh("rc".to_string());
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
            Match(simp, arms) => self.lower_simp(
                *simp,
                Box::new(|s, simp| {
                    let match_after = s.fresh("match_after".to_string());
                    let matched = s.fresh("matched".to_string());

                    let match_after_cont = CntDef {
                        name: match_after.clone(),
                        args: vec![matched.clone()],
                        body: ctx(s, matched),
                    };

                    CpsExpr::Cnts {
                        cnts: vec![match_after_cont],
                        body: Box::new(s.match_arms(simp, arms, match_after)),
                    }
                }),
            ),
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
                        let cont_name = self.fresh("rc".to_string());
                        let ret_name = self.fresh("rv".to_string());

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
                let name = Name(format!("c{}", n));
                CpsExpr::Const {
                    name: name.clone(),
                    value: LitHigh::Int(n),
                    body: Box::new(ctx(self, name)),
                }
            }
            Bool(b) => {
                let n = if b { 1 } else { 0 };
                self.lower_simp(Int(n), ctx)
            }
            Data(name, args) => {
                let data_def = self
                    .data_defs
                    .iter()
                    .find(|def| def.cons.contains_key(&name))
                    .unwrap();

                let tag = data_def.cons.iter().position(|(n, _)| n == &name).unwrap() as i64;
                let desc = Name(format!("d{}", tag));
                let data = self.fresh(format!("data_{}", name));

                self.simp_list(
                    args,
                    Box::new(move |s, args| {
                        CpsExpr::Const {
                            name: desc.clone(),
                            value: LitHigh::Int(tag),
                            body: Box::new(CpsExpr::Prim {
                                name: data.clone(),
                                op: Name("data".to_string()),

                                // args is desc, ...args
                                args: vec![desc].into_iter().chain(args).collect(),
                                body: Box::new(ctx(s, data)),
                            }),
                        }
                    }),
                    vec![],
                )
            }
            _ => unimplemented!(),
        }
    }

    fn pat_list(
        &mut self,
        mut pats: Vec<Pattern>,
        mut vals: Vec<Name>,
        body: CpsExpr,
        no_match: Name,
    ) -> CpsExpr {
        assert!(pats.len() > 0);
        assert!(pats.len() == vals.len());

        let pat = pats.remove(0);
        let val = vals.remove(0);

        if pats.len() == 0 {
            self.lower_pattern_match(pat, val, body, no_match)
        } else {
            let remaining = self.pat_list(pats, vals, body, no_match.clone());
            self.lower_pattern_match(pat, val, remaining, no_match)
        }
    }

    fn data_fields(
        &mut self,
        data: Name,
        num_fields: usize,
        ctx: VecContext,
        mut acc: Vec<Name>,
    ) -> CpsExpr {
        if acc.len() == num_fields {
            ctx(self, acc)
        } else {
            let field = self.fresh(format!("f{}", acc.len()));
            let idx = Name(format!("i{}", acc.len()));
            acc.push(field.clone());

            CpsExpr::Const {
                name: idx.clone(),
                value: LitHigh::Int((acc.len() - 1) as i64),
                body: Box::new(CpsExpr::Prim {
                    name: field.clone(),
                    op: Name("field".to_string()),
                    args: vec![data.clone(), idx],
                    body: Box::new(self.data_fields(data, num_fields, ctx, acc)),
                }),
            }
        }
    }

    fn lower_pattern_match(
        &mut self,
        pat: Pattern,
        val: Name,
        body: CpsExpr,
        no_match: Name,
    ) -> CpsExpr {
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

                let good_cnt = CntDef {
                    name: good.clone(),
                    args: vec![],
                    body,
                };

                let desc = Name(format!("p{}", n));
                CpsExpr::Const {
                    name: desc.clone(),
                    value: LitHigh::Int(n),
                    body: Box::new(CpsExpr::Cnts {
                        cnts: vec![good_cnt],
                        body: Box::new(CpsExpr::If {
                            op: Name("==".to_string()),
                            args: vec![desc, val],
                            t: good,
                            f: no_match,
                        }),
                    }),
                }
            }
            Pattern::Bool(b) => {
                // convert bool to int
                let npat = if b { Pattern::Int(1) } else { Pattern::Int(0) };
                self.lower_pattern_match(npat, val, body, no_match)
            }
            Pattern::Data(data_def, tag, pats) => {
                let tag_idx = data_def
                    .cons
                    .iter()
                    .position(|(name, _)| name == &tag)
                    .unwrap();

                let tag = tag_idx as i64;

                let desc = Name(format!("d{}", tag));
                let good = self.fresh("pm_good".to_string());

                let no_match_2 = no_match.clone();
                let good_cnt = CntDef {
                    name: good.clone(),
                    args: vec![],
                    // if good, we need to check subpatterns
                    body: if pats.len() == 0 {
                        body
                    } else {
                        self.data_fields(
                            val.clone(),
                            pats.len(),
                            Box::new(move |s, fields| s.pat_list(pats, fields, body, no_match_2)),
                            vec![],
                        )
                    },
                };

                // otherwise, we directly jump to no_match
                let val_desc = self.fresh("desc".to_string());
                CpsExpr::Const {
                    name: desc.clone(),
                    value: LitHigh::Int(tag),
                    body: Box::new(CpsExpr::Prim {
                        name: val_desc.clone(),
                        op: Name("desc".to_string()),
                        args: vec![val.clone()],
                        body: Box::new(CpsExpr::Cnts {
                            cnts: vec![good_cnt],
                            body: Box::new(CpsExpr::If {
                                op: Name("==".to_string()),
                                args: vec![desc, val_desc],
                                t: good,
                                f: no_match,
                            }),
                        }),
                    }),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn lower_expr(&mut self, high: Expr) -> CpsExpr {
        match high {
            Expr::Bind(pat, rhs, body) => self.lower_simp(
                rhs,
                Box::new(|s: &mut Self, rhs| {
                    let body = s.lower_expr(*body);
                    s.lower_pattern_match(pat, rhs, body, Name("halt".to_string()))
                }),
            ),
            Expr::Simp(simp) => self.lower_simp(simp, Box::new(|_, rhs| CpsExpr::Halt(rhs))),
        }
    }
}
