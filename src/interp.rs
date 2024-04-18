use crate::ast::*;
use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type BuiltInFn = fn(Vec<Value>) -> Value;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Unit,
    Data(Name, Vec<Value>),
    Closure(Env, FnDef),
    BuiltIn(BuiltInFn),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Value::*;
        match self {
            Int(n) => write!(f, "{}", n),
            Unit => write!(f, "()"),
            Data(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Closure(_, _) => write!(f, "<closure>"),
            BuiltIn(_) => write!(f, "<builtin>"),
        }
    }
}

macro_rules! infix_fn {
    ($name:ident, $op:tt) => {
        fn $name(args: Vec<Value>) -> Value {
            match args.as_slice() {
                [Value::Int(a), Value::Int(b)] => Value::Int(a $op b),
                _ => panic!("Invalid arguments to {}: {:?}", stringify!($name), args),
            }
        }
    };
}

macro_rules! unary_fn {
    ($name:ident, $op:tt) => {
        fn $name(args: Vec<Value>) -> Value {
            match args.as_slice() {
                [Value::Int(a)] => Value::Int($op a),
                _ => panic!("Invalid arguments to {}: {:?}", stringify!($name), args),
            }
        }
    };
}

infix_fn!(add, +);
infix_fn!(sub, -);
infix_fn!(mul, *);
infix_fn!(div, /);
infix_fn!(mod_, %);
unary_fn!(bnot, !);

fn builtin_println(args: Vec<Value>) -> Value {
    assert!(args.len() == 1);
    println!("{}", args[0]);
    Value::Unit
}

#[derive(Debug, Clone)]
pub struct Env {
    pub data_defs: HashMap<Name, DataDef>,
    bindings: HashMap<Name, Rc<RefCell<Option<Value>>>>, // box with hole, for recursive bindings
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Env {{\n")?;
        for (name, value) in &self.bindings {
            match value.as_ref().borrow().as_ref() {
                Some(value) => write!(f, "  {}: {},\n", name, value)?,
                None => write!(f, "  {}: <hole>,\n", name)?,
            }
        }
        write!(f, "}}")
    }
}

impl Env {
    pub fn new() -> Self {
        Self {
            data_defs: HashMap::new(),
            bindings: HashMap::from([
                (Name("+"), Rc::new(RefCell::new(Some(Value::BuiltIn(add))))),
                (Name("-"), Rc::new(RefCell::new(Some(Value::BuiltIn(sub))))),
                (Name("*"), Rc::new(RefCell::new(Some(Value::BuiltIn(mul))))),
                (Name("/"), Rc::new(RefCell::new(Some(Value::BuiltIn(div))))),
                (Name("%"), Rc::new(RefCell::new(Some(Value::BuiltIn(mod_))))),
                (Name("~"), Rc::new(RefCell::new(Some(Value::BuiltIn(bnot))))),
                (Name("println"), Rc::new(RefCell::new(Some(Value::BuiltIn(builtin_println))))),
            ]),
        }
    }

    pub fn bind(mut self, name: Name, value: Value) -> Env {
        if let Some(entry) = self.bindings.get_mut(&name) {
            let mut entry = entry.borrow_mut();
            *entry = Some(value);
        } else {
            self.bindings.insert(name, Rc::new(RefCell::new(Some(value))));
        }

        self
    }

    pub fn bind_late(mut self, name: Name) -> Env {
        self.bindings.insert(name, Rc::new(RefCell::new(None)));
        self
    }

    pub fn capture(&self, names: &[Name]) -> Self {
        let mut bindings = HashMap::new();
        for name in names {
            if let Some(value) = self.bindings.get(name) {
                bindings.insert(name.clone(), value.clone());
            } else {
                panic!("Unable to capture, does not exist: {}", name);
            }
        }
        Self {
            data_defs: self.data_defs.clone(),
            bindings,
        }
    }
}

fn bound_names_pat(pat: &Pattern) -> Vec<Name> {
    pat.bindings()
        .iter()
        .map(|(name, _)| name.clone())
        .collect()
}

fn free_vars_expr(expr: &Expr) -> Vec<Name> {
    use Expr::*;
    match expr {
        Bind(pat, rhs, body) => {
            let free_rhs = free_vars_simp(rhs);
            let free_body = free_vars_expr(body);

            let nbound = bound_names_pat(pat);

            free_rhs
                .into_iter()
                .chain(free_body.into_iter())
                .filter(|name| !nbound.contains(name))
                .collect()
        }
        Simp(s) => free_vars_simp(s),
    }
}

fn free_vars_simp(simp: &Simp) -> Vec<Name> {
    use Simp::*;
    match simp {
        FnDef(f) => {
            let nargs = f
                .args
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>();

            free_vars_simp(&f.body)
                .into_iter()
                .filter(|name| !nargs.contains(name))
                .collect()
        }
        Match(s, arms) => {
            let free_s = free_vars_simp(s);
            let free_arms = arms.into_iter().flat_map(|(pat, body)| {
                let nbound = bound_names_pat(pat);
                let free_body = free_vars_simp(body);

                free_body
                    .into_iter()
                    .filter(|name| !nbound.contains(name))
                    .collect::<Vec<_>>()
            });

            free_s.into_iter().chain(free_arms).collect()
        }
        FnCall(lhs, rhs) => free_vars_simp(lhs)
            .into_iter()
            .chain(rhs.iter().flat_map(free_vars_simp))
            .collect(),
        Block(expr) => free_vars_expr(expr),
        Ref(name) => vec![name.clone()],
        Data(_, args) => args.into_iter().flat_map(free_vars_simp).collect(),
        Int(_) | Unit => vec![],
    }
}

pub fn eval_prog(program: &Program) -> Value {
    let mut env = Env::new();
    for data_def in &program.data_defs {
        env.data_defs
            .insert(data_def.name.clone(), data_def.clone());
    }

    eval_expr(env, program.expr.as_ref().unwrap())
}

fn eval_expr(env: Env, expr: &Expr) -> Value {
    use Expr::*;
    match expr {
        Bind(pat, rhs, body) => {
            let bound_names = bound_names_pat(pat);
            let env = bound_names
                .iter()
                .fold(env, |nenv, name| nenv.bind_late(name.clone()));

            let value = eval_simp(env.clone(), rhs);
            let nenv = eval_pattern_match(&env, pat, &value)
                .unwrap_or_else(|| panic!("Pattern match failed: {:?} = {:?}", pat, value));

            // println!("nenv: {}", nenv);
            eval_expr(nenv, body)
        }
        Simp(s) => eval_simp(env, s),
    }
}

fn eval_simp(env: Env, simp: &Simp) -> Value {
    use Simp::*;
    match simp {
        FnDef(f) => {
            let free_vars = free_vars_simp(simp);
            let closure_env = env.capture(&free_vars);

            Value::Closure(closure_env, f.clone())
        }
        Match(s, arms) => {
            let value = eval_simp(env.clone(), s);
            for (pat, body) in arms {
                if let Some(nenv) = eval_pattern_match(&env, pat, &value) {
                    return eval_simp(nenv, body);
                }
            }
            panic!(
                "Pattern match failed: {:?} does not match any of {:?}",
                value, arms
            );
        }
        FnCall(lhs, rhs) => {
            let lhs = eval_simp(env.clone(), lhs);
            let arg_vals = rhs
                .iter()
                .map(|arg| eval_simp(env.clone(), arg))
                .collect::<Vec<_>>();

            if let Value::BuiltIn(f) = lhs {
                return f(arg_vals);
            }

            let (fenv, fun) = match lhs {
                Value::Closure(closure_env, f) => (closure_env, f),
                _ => panic!("{:?} is not callable", lhs),
            };

            let fenv = fun
                .args
                .iter()
                .zip(arg_vals)
                .fold(fenv, |nenv, ((name, _), val)| nenv.bind(name.clone(), val));

            eval_simp(fenv, &fun.body)
        }
        Block(expr) => eval_expr(env, expr),
        Ref(name) => env
            .bindings
            .get(name)
            .unwrap_or_else(|| panic!("Unbound name: {}", name))
            .as_ref()
            .borrow()
            .as_ref()
            .unwrap_or_else(|| panic!("Uninitialized late binding: {}", name))
            .clone(),
        Int(n) => Value::Int(*n),
        Unit => Value::Unit,
        Data(name, args) => Value::Data(
            name.clone(),
            args.iter().map(|arg| eval_simp(env.clone(), arg)).collect(),
        ),
    }
}

fn eval_pattern_match(env: &Env, pat: &Pattern, value: &Value) -> Option<Env> {
    match pat {
        Pattern::Var(name, _) => Some(env.clone().bind(name.clone(), value.clone())),
        Pattern::Int(n) => match value {
            Value::Int(m) if n == m => Some(env.clone()),
            _ => None,
        },
        Pattern::Data(_, ltag, pats) => match value {
            Value::Data(rtag, vals) if ltag == rtag => pats
                .iter()
                .zip(vals)
                .try_fold(env.clone(), |nenv, (pat, val)| {
                    eval_pattern_match(&nenv, pat, val)
                }),
            _ => None,
        },
    }
}
