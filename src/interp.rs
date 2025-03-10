use crate::ast::*;
use crate::builtins::*;
use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type BuiltInFn = fn(Vec<Value>) -> Value;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Unit,
    Data(Name, Vec<Value>),
    Closure(Env, Rc<FnDef>),
    BuiltIn(BuiltInFn),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Value::*;
        match self {
            Int(n) => write!(f, "{}", n),
            Bool(b) => write!(f, "{}", b),
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
                (
                    Name(String::from("+")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(add)))),
                ),
                (
                    Name(String::from("-")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(sub)))),
                ),
                (
                    Name(String::from("*")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(mul)))),
                ),
                (
                    Name(String::from("/")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(div)))),
                ),
                (
                    Name(String::from("%")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(mod_)))),
                ),
                (
                    Name(String::from("~")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(bnot)))),
                ),
                (
                    Name(String::from("==")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(eq)))),
                ),
                (
                    Name(String::from("!=")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(neq)))),
                ),
                (
                    Name(String::from("<")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(lt)))),
                ),
                (
                    Name(String::from(">")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(gt)))),
                ),
                (
                    Name(String::from("<=")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(le)))),
                ),
                (
                    Name(String::from(">=")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(ge)))),
                ),
                (
                    Name(String::from("&&")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(and)))),
                ),
                (
                    Name(String::from("||")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(or)))),
                ),
                (
                    Name(String::from("!")),
                    Rc::new(RefCell::new(Some(Value::BuiltIn(not)))),
                ),
            ]),
        }
    }

    pub fn bind(mut self, name: Name, value: Value) -> Env {
        if let Some(entry) = self.bindings.get_mut(&name) {
            let mut entry = entry.borrow_mut();
            *entry = Some(value);
        } else {
            self.bindings
                .insert(name, Rc::new(RefCell::new(Some(value))));
        }

        self
    }

    pub fn bind_late(mut self, name: Name) -> Env {
        self.bindings.insert(name, Rc::new(RefCell::new(None)));
        self
    }

    pub fn capture(self, names: &[Name]) -> Self {
        let mut bindings = HashMap::new();
        for name in names {
            if let Some(value) = self.bindings.get(name) {
                bindings.insert(name.clone(), value.clone());
            } else {
                panic!("Unable to capture, does not exist: {}", name);
            }
        }
        Self {
            data_defs: self.data_defs,
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
        FnDef(f, body) => {
            let args = f
                .args
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>();
            let fun_free = free_vars_simp(&f.body)
                .into_iter()
                .filter(|name| !args.contains(name));

            let body_free = free_vars_expr(body);
            let nbound = f.name.clone();

            body_free
                .into_iter()
                .filter(|name| name != &nbound)
                .chain(fun_free)
                .collect()
        }
        Simp(s) => free_vars_simp(s),
    }
}

fn free_vars_simp(simp: &Simp) -> Vec<Name> {
    use Simp::*;
    match simp {
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
        Int(_) | Bool(_) | Unit => vec![],
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
            let nenv = eval_pattern_match(env, pat, &value)
                .unwrap_or_else(|| panic!("Pattern match failed: {:?} = {:?}", pat, value));

            eval_expr(nenv, body)
        }
        FnDef(f, body) => {
            let free_vars = free_vars_simp(&f.body)
                .into_iter()
                .filter(|name| {
                    f.args.iter().all(|(arg_name, _)| arg_name != name) && f.name != *name
                })
                .collect::<Vec<_>>();

            let closure_env = env.clone().capture(&free_vars).bind_late(f.name.clone());
            let f_rc = Rc::new(f.clone());

            let mut closure = Value::Closure(closure_env, f_rc);
            let closure_clone = closure.clone();
            match &mut closure {
                Value::Closure(closure_env, _) => {
                    *closure_env.bindings.get_mut(&f.name).unwrap().borrow_mut() =
                        Some(closure_clone);
                }
                _ => unreachable!(),
            }

            let nenv = env.bind(f.name.clone(), closure);

            eval_expr(nenv, body)
        }
        Simp(s) => eval_simp(env, s),
    }
}

fn eval_simp(env: Env, simp: &Simp) -> Value {
    use Simp::*;
    match simp {
        Match(s, arms) => {
            let value = eval_simp(env.clone(), s);
            for (pat, body) in arms {
                if let Some(nenv) = eval_pattern_match(env.clone(), pat, &value) {
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
        Bool(b) => Value::Bool(*b),
        Unit => Value::Unit,
        Data(name, args) => Value::Data(
            name.clone(),
            args.iter().map(|arg| eval_simp(env.clone(), arg)).collect(),
        ),
    }
}

fn eval_pattern_match(env: Env, pat: &Pattern, value: &Value) -> Option<Env> {
    match pat {
        Pattern::Var(name, _) => Some(env.bind(name.clone(), value.clone())),
        Pattern::Int(n) => match value {
            Value::Int(m) if n == m => Some(env),
            _ => None,
        },
        Pattern::Bool(b) => match value {
            Value::Bool(c) if b == c => Some(env),
            _ => None,
        },
        Pattern::Data(_, ltag, pats) => match value {
            Value::Data(rtag, vals) if ltag == rtag => pats
                .iter()
                .zip(vals)
                .try_fold(env.clone(), |nenv, (pat, val)| {
                    eval_pattern_match(nenv, pat, val)
                }),
            _ => None,
        },
    }
}
