use crate::ast::*;
use core::fmt;

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Type::*;
        match self {
            Int => write!(f, "Int"),
            Unit => write!(f, "Unit"),
            Fn(args, ret) => {
                write!(f, "(")?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ") -> {}", ret)
            }
            UserDef(name) => write!(f, "{}", name.0),
            TyVar(i) => write!(f, "T{}", i),
        }
    }

}

impl fmt::Display for TypeDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type {} = {}", self.name.0, self.ty)
    }
}

impl fmt::Display for DataDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "data {} = ", self.name.0)?;
        for (i, con) in self.cons.iter().enumerate() {
            write!(f, "{}{}", con.0, con.1)?;
            if i < self.cons.len() - 1 {
                write!(f, " | ")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Cons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.args.is_empty() {
            write!(f, "(")?;
            for (i, arg) in self.args.iter().enumerate() {
                write!(f, "{}", arg)?;
                if i < self.args.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Pattern::*;
        match self {
            Var(name, ty) => write!(f, "({name}: {ty})"),
            Int(n) => write!(f, "{}", n),
            Data(_, name, pats) => {
                write!(f, "{}(", name)?;
                for (i, pat) in pats.iter().enumerate() {
                    write!(f, "{}", pat)?;
                    if i < pats.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expr::*;
        match self {
            Bind(pat, simp, expr) => write!(f, "let {} = {};\n{}", pat, simp, expr),
            Simp(simp) => write!(f, "{}", simp),
        }
    }
}

impl fmt::Display for Simp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Simp::*;
        match self {
            FnDef(fn_def) => write!(f, "{}", fn_def),
            Match(expr, arms) => {
                write!(f, "match {} {{\n", expr)?;
                for (i, (pat, simp)) in arms.iter().enumerate() {
                    write!(f, "{} => {}", pat, simp)?;
                    if i < arms.len() - 1 {
                        write!(f, "\n")?;
                    }
                }
                write!(f, "}}")
            }
            FnCall(fn_name, args) => {
                write!(f, "{}(", fn_name)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Block(expr) => write!(f, "{{\n{}\n}}", expr),
            Ref(name) => write!(f, "({})", name),
            Int(n) => write!(f, "{}", n),
            Unit => write!(f, "()"),
            Data(name, args) => {
                if args.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}(", name)?;
                    for (i, arg) in args.iter().enumerate() {
                        write!(f, "{}", arg)?;
                        if i < args.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, ")")
                }
            }
        }
    }
}

impl fmt::Display for FnDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn(")?;
        for (i, (name, ty)) in self.args.iter().enumerate() {
            write!(f, "{}: {}", name, ty)?;
            if i < self.args.len() - 1 {
                write!(f, ", ")?;
            }
        };
        // indent body
        let body = format!("{}", self.body);
        let body = body.replace("}", "\n}");

        write!(f, ") -> {} {}", self.ret, body)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for data_def in self.data_defs.iter() {
            write!(f, "{}\n", data_def)?;
        }
        // for type_def in self.type_defs.iter() {
        //     write!(f, "{}\n", type_def)?;
        // }
        if let Some(expr) = &self.expr {
            write!(f, "{}", expr)?;
        }
        Ok(())
    }
}