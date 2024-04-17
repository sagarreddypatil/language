use std::collections::HashMap;

use crate::ast::*;

pub struct TyConstraint(pub Type, pub Type);

impl std::fmt::Display for TyConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} == {}", self.0, self.1)
    }
}

type Constraints = Vec<TyConstraint>;

#[derive(Clone)]
struct TypeEnv {
    env: HashMap<Name, Type>,
}

impl TypeEnv {
    fn new() -> Self {
        let int_infix_op: Type = Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Int));
        let int_unary_op: Type = Type::Fn(vec![Type::Int], Box::new(Type::Int));

        TypeEnv {
            env: HashMap::from(
                [
                    (Name("+"), int_infix_op.clone()),
                    (Name("-"), int_infix_op.clone()),
                    (Name("*"), int_infix_op.clone()),
                    (Name("/"), int_infix_op.clone()),
                    (Name("%"), int_infix_op.clone()),
                    (Name("~"), int_unary_op.clone()),
                ]
            )
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
