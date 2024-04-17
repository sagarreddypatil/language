use std::collections::HashMap;
use core::fmt;
use crate::ast::*;

#[derive(Clone)]
pub struct TyConstraint(pub Type, pub Type);

impl std::fmt::Display for TyConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} == {}", self.0, self.1)
    }
}

type TyConstraints = Vec<TyConstraint>;

// type TySubst = HashMap<usize, Type>;
pub struct TySubst {
    subst: HashMap<usize, Type>,
}

impl fmt::Display for TySubst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (k, v) in &self.subst {
            s.push_str(&format!("{} -> {}\n", k, v));
        }
        write!(f, "{}", s)
    }
}

impl TySubst {
    pub fn new() -> Self {
        TySubst {
            subst: HashMap::new(),
        }
    }

    pub fn singleton(n: usize, ty: Type) -> Self {
        let mut subst = HashMap::new();
        subst.insert(n, ty);
        TySubst { subst }
    }

    pub fn apply(&self, ty: Type) -> Type {
        use Type::*;
        match ty {
            TyVar(n) =>  {
                let ret = self.subst.get(&n);
                match ret {
                    Some(t) => t.clone(),
                    None => ty,
                }
            }
            Fn(args, ret) => {
                let new_args = args.iter().map(|a| self.apply(a.clone())).collect();
                let new_ret = Box::new(self.apply(*ret));
                Fn(new_args, new_ret)
            }
            _ => ty,
        }
    }

    pub fn apply_const(&self, constraints: TyConstraints) -> TyConstraints {
        constraints.iter().map(|c| TyConstraint(self.apply(c.0.clone()), self.apply(c.1.clone()))).collect()
    }

    pub fn compose(&mut self, other: TySubst) {
        let nother = other.subst.iter().map(|(k, v)| (*k, self.apply(v.clone())));
        let nother = nother.collect::<HashMap<usize, Type>>();

        self.subst.extend(nother);
    }
}

#[derive(Clone)]
struct TyEnv {
    env: HashMap<Name, Type>,
}

impl TyEnv {
    fn new() -> Self {
        let int_infix_op: Type = Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Int));
        let int_unary_op: Type = Type::Fn(vec![Type::Int], Box::new(Type::Int));

        TyEnv {
            env: HashMap::from([
                (Name("+"), int_infix_op.clone()),
                (Name("-"), int_infix_op.clone()),
                (Name("*"), int_infix_op.clone()),
                (Name("/"), int_infix_op.clone()),
                (Name("%"), int_infix_op.clone()),
                (Name("~"), int_unary_op.clone()),
            ]),
        }
    }

    fn insert(&mut self, name: Name, ty: Type) {
        self.env.insert(name, ty);
    }

    fn extend(&mut self, bindings: Vec<(Name, Type)>) {
        for (name, ty) in bindings {
            self.insert(name, ty);
        }
    }

    fn get(&self, name: &Name) -> Type {
        let ret = self.env.get(name);
        match ret {
            Some(ty) => ty.clone(),
            None => panic!("Undefined variable: {}", name),
        }
    }
}

fn ty_in(n: Type, m: Type) -> bool {
    use Type::*;
    match (n, m) {
        (TyVar(n), TyVar(m)) => n == m,
        (Fn(args1, ret1), Fn(args2, ret2)) => {
            args1.len() == args2.len()
                && args1.iter().zip(args2.iter()).all(|(a1, a2)| ty_in(a1.clone(), a2.clone()))
                && ty_in(*ret1, *ret2)
        }
        (TyVar(n), Fn(args, ret)) => {
            args.iter().any(|a| ty_in(TyVar(n), a.clone())) || ty_in(TyVar(n), *ret)
        }
        (Fn(args, ret), TyVar(n)) => {
            args.iter().any(|a| ty_in(a.clone(), TyVar(n))) || ty_in(*ret, TyVar(n))
        }
        (_, _) => false
    }
}


#[allow(dead_code)]
fn unify(constraints: TyConstraints) -> TySubst {
    if constraints.is_empty() {
        return TySubst::new();
    }

    let first = &constraints[0];
    let first = (&first.0, &first.1);

    let rest = constraints[1..].to_vec();

    use Type::*;
    match first {
        (t1, t2) if t1 == t2 => unify(rest),
        (Fn(args1, ret1), Fn(args2, ret2)) => {
            let mut new_constraints = vec![];
            for (a1, a2) in args1.iter().zip(args2.iter()) {
                new_constraints.push(TyConstraint(a1.clone(), a2.clone()));
            }
            new_constraints.push(TyConstraint(*ret1.clone(), *ret2.clone()));
            let mut new_constraints = new_constraints;
            new_constraints.extend(rest);

            unify(new_constraints)
        }
        (TyVar(n), t) => {
            if (ty_in(TyVar(*n), t.clone())) {
                panic!("Type error: recursive type");
            }

            let subst = TySubst::singleton(*n, t.clone());
            let rest = subst.apply_const(rest);
            let mut nsubst = unify(rest);
            nsubst.compose(subst);

            nsubst
        }
        (s, TyVar(n)) => {
            if (ty_in(TyVar(*n), s.clone())) {
                panic!("Type error: recursive type");
            }

            let subst = TySubst::singleton(*n, s.clone());
            let rest = subst.apply_const(rest);
            let mut nsubst = unify(rest);
            nsubst.compose(subst);

            nsubst
        }

        _ => panic!("Type error: cannot unify {} and {}", first.0, first.1),
    }
}

pub struct TypeChecker {
    cons_datadef: HashMap<Name, DataDef>, // from constructor name to DataDef
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            cons_datadef: HashMap::new(),
        }
    }

    pub fn infer(&mut self, program: Program) -> TySubst {
        let constraints = self.infer_constraints(program);
        let subst = unify(constraints);
        subst
    }

    fn infer_constraints(&mut self, program: Program) -> TyConstraints {
        self.cons_datadef = program
            .data_defs
            .iter()
            .fold(HashMap::new(), |mut acc, datadef| {
                for cons in &datadef.cons {
                    acc.insert(cons.0.clone(), datadef.clone());
                }
                acc
            });

        let expr = program.expr.as_ref().unwrap();
        let (_, x) = self.infer_constraints_expr(TyEnv::new(), expr);
        x
    }

    fn infer_constraints_expr(&mut self, mut env: TyEnv, exp: &Expr) -> (Type, TyConstraints) {
        use Expr::*;
        match exp {
            Bind(pat, simp, body) => {
                let bindings = pat.bindings();
                let t_pat = pat.get_type();
                env.extend(bindings);

                let (t_rhs, x_rhs) = self.infer_constraints_simp(env.clone(), simp);
                let (t_body, x_body) = self.infer_constraints_expr(env.clone(), body);

                let mut x = vec![];
                x.extend(x_rhs);
                x.extend(x_body);
                x.push(TyConstraint(t_pat, t_rhs));

                (t_body, x)
            }
            Simp(simp) => self.infer_constraints_simp(env, simp),
        }
    }

    fn infer_constraints_simp(&mut self, mut env: TyEnv, simp: &Simp) -> (Type, TyConstraints) {
        use Simp::*;
        match simp {
            FnDef(f) => {
                env.extend(f.args.clone());
                let (t_body, x_body) = self.infer_constraints_simp(env, &*f.body);
                let t_args = f.args.iter().map(|(_, ty)| ty.clone()).collect();
                let t_fn = Type::Fn(t_args, Box::new(t_body));

                (t_fn, x_body)
            }
            Match(simp, arms) => {
                let (t_simp, x_simp) = self.infer_constraints_simp(env.clone(), simp);
                let mut x_out = x_simp;

                let mut t_arms = vec![];

                for (pat, simp) in arms {
                    let t_pat = pat.get_type();
                    x_out.push(TyConstraint(t_simp.clone(), t_pat.clone()));

                    let mut env_arm = env.clone();
                    let bindings = pat.bindings();
                    env_arm.extend(bindings);

                    let (t_arm, x_arm) = self.infer_constraints_simp(env_arm, simp);
                    x_out.extend(x_arm);
                    t_arms.push(t_arm);
                }

                let t_arm_0 = t_arms[0].clone();
                for t_arm in t_arms {
                    x_out.push(TyConstraint(t_arm_0.clone(), t_arm));
                }

                (t_arm_0, x_out)
            }
            FnCall(lhs, args) => {
                let (t_lhs, x_lhs) = self.infer_constraints_simp(env.clone(), lhs);
                let mut x_out = x_lhs;
                let mut t_args = vec![];

                for arg in args {
                    let (t_arg, x_arg) = self.infer_constraints_simp(env.clone(), arg);
                    x_out.extend(x_arg);
                    t_args.push(t_arg);
                }

                let t_out = fresh_tv();
                let t_fn = Type::Fn(t_args, Box::new(t_out.clone()));
                x_out.push(TyConstraint(t_lhs, t_fn));

                (t_out, x_out)
            }
            Ref(name) => (env.get(name), vec![]),
            Int(_) => (Type::Int, vec![]),
            Unit => (Type::Unit, vec![]),
            Data(name, args) => {
                let df = self.cons_datadef.get(name).unwrap().clone();
                let cons = df.cons.get(name).unwrap();

                if args.len() != cons.args.len() {
                    panic!("Wrong number of arguments for data constructor: {}", name);
                }

                let mut x_out = vec![];
                let mut t_args = vec![];

                for arg in args {
                    let (t_arg, x_arg) = self.infer_constraints_simp(env.clone(), arg);
                    x_out.extend(x_arg);
                    t_args.push(t_arg);
                }

                for (arg, con) in t_args.iter().zip(cons.args.iter()) {
                    x_out.push(TyConstraint(arg.clone(), con.clone()));
                }

                (Type::UserDef(df.name), x_out)
            }
        }
    }
}

pub fn apply_subst_expr(subst: &TySubst, tree: Expr) -> Expr {
    use Expr::*;
    match expr {
        Bind(pat, simp, body) => {
            let new_pat = apply_subst_pat(subst, pat);
            let new_simp = apply_subst_simp(subst, simp);
            let new_body = apply_subst_expr(subst, *body);
            Bind(new_pat, new_simp, Box::new(new_body))
        }
        Simp(simp) => Simp(apply_subst_simp(subst, simp)),
    }
}

fn apply_subst_simp(subst: &TySubst, simp: Simp) -> Simp {
    unimplemented!("apply_subst_simp")
}

fn apply_subst_pat(subst: &TySubst, pat: Pattern) -> Pattern {
    use Pattern::*;
    match pat {
        Var(name, ty) => Var(name, subst.apply(ty)),
        Int(i) => Int(i),
    }
}