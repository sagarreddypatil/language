use crate::tokenizer::*;
use crate::ast::*;
use std::collections::HashMap;

pub struct Parser {
    pub tokens: Vec<Token>,
    position: usize,

    // map from type constructor to data type name
    ty_cons: HashMap<Name, Name>,
    tvc: usize, // type variable counter
}

macro_rules! expect_fail {
    ($expected:expr, $found:expr) => {
        panic!("Expected {:?}, found {:?} at {}", $expected, $found.kind, $found.pos)
    };
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,

            ty_cons: HashMap::new(),
            tvc: 0,
        }
    }

    fn fresh_tv(&mut self) -> Type {
        self.tvc += 1;
        Type::Var(self.tvc)
    }

    fn end(&self) -> bool {
        // last token is always EOF
        self.position >= self.tokens.len() - 1
    }

    fn peek(&self) -> &TokenKind {
        &self.tokens.get(self.position).unwrap().kind
    }

    fn peek_wpos(&self) -> &Token {
        self.tokens.get(self.position).unwrap()
    }

    fn accept(&mut self) -> &Token {
        let token = self.tokens.get(self.position).unwrap();
        self.position += 1;

        token
    }

    fn expect(&mut self, token: TokenKind) {
        let next = self.accept();
        if next.kind != token {
            expect_fail!(token, next);
        }
    }

    fn expect_name(&mut self) -> Name {
        let token = self.accept();
        match &token.kind {
            TokenKind::Name(name) => {
                Name::from_string_ref(&name)
            },
            _ => expect_fail!("Name", token),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            data_defs: Vec::new(),
            type_defs: Vec::new(),
            expr: None,
        };

        while !self.end() {
            match self.peek() {
                TokenKind::Endl => { self.accept(); },
                TokenKind::DataDef => program.data_defs.push(self.parse_data_ref()),
                TokenKind::TypeDef => program.type_defs.push(self.parse_type_def()),
                _ => break,
            }
        }

        for data_def in &program.data_defs {
            for cons in &data_def.cons {
                self.ty_cons.insert(cons.tag, data_def.name);
            }
        }

        program.expr = Some(self.parse_expr());
        program
    }

    fn parse_expr(&mut self) -> Expr {
        match self.peek() {
            TokenKind::Let => {
                self.parse_let()
            },
            _ => Expr::Simp(self.parse_simp()),
        }
    }

    fn parse_let(&mut self) -> Expr {
        self.expect(TokenKind::Let);
        let pattern = self.parse_pattern();
        self.expect(TokenKind::Eq);
        let rhs = self.parse_simp();
        self.expect(TokenKind::Endl);

        let body = self.parse_expr();
        Expr::Bind(pattern, rhs, Box::new(body))
    }

    fn parse_pattern(&mut self) -> Pattern {
        match self.peek() {
            TokenKind::Name(_) => Pattern::Var(self.expect_name()),
            TokenKind::Number(n) => {
                let n = *n;
                self.accept();
                Pattern::Int(n)
            },
            _ => expect_fail!("Pattern", self.peek_wpos()),
        }
    }

    fn parse_simp(&mut self) -> Simp {
        match self.peek() {
            TokenKind::Endl => {
                self.accept();
                self.parse_simp()
            },
            TokenKind::Match => self.parse_match(),
            TokenKind::FnDef => self.parse_fndef(),
            _ => self.parse_simple_ops(0),
        }
    }

    fn parse_fndef(&mut self) -> Simp {
        self.accept();
        self.expect(TokenKind::POpen);

        // parse name of args, comma separated
        let mut args = Vec::new();
        if self.peek() != &TokenKind::PClose {
            args.push((self.expect_name(), self.fresh_tv()));
            loop {
                match self.peek() {
                    TokenKind::Comma => {
                        self.accept();
                        args.push((self.expect_name(), self.fresh_tv()));
                    },
                    TokenKind::PClose => {
                        self.accept();
                        break;
                    },
                    _ => expect_fail!("',' or ')'", self.peek_wpos()),
                }
            }
        }

        Simp::FnDef(FnDef {
            args,
            body: Box::new(self.parse_simp()),
            ret: self.fresh_tv(),
        })

    }

    fn parse_simple_ops(&mut self, min_prec: i32) -> Simp {
        let mut lhs = self.parse_utight();
        loop {
            let name = match self.peek() {
                TokenKind::Name(name) => {
                    Name::from_string_ref(name)
                },
                _ => break,
            };

            if !name.valid() || name.prec() < min_prec {
                break;
            }

            self.accept();
            let new_min = name.prec() + name.assoc();
            let rhs = self.parse_simple_ops(new_min);

            lhs = Simp::FnCall(Box::new(Simp::Ref(name)), vec![lhs, rhs]);
        }

        lhs
    }

    fn parse_tight(&mut self) -> Simp {
        let lhs = self.parse_atom();

        match self.peek() {
            TokenKind::POpen => {
                let args = self.parse_simp_list();
                Simp::FnCall(Box::new(lhs), args)
            },
            _ => lhs
        }
    }

    fn parse_utight(&mut self) -> Simp {
        match self.peek() {
            TokenKind::Name(name) if Name::from_string_ref(name).unary() => {
                let name = self.expect_name();
                let name = Simp::Ref(name);

                let rest = self.parse_tight();
                Simp::FnCall(Box::new(name), vec![rest])
            },
            _ => self.parse_tight(),
        }
    }

    fn parse_atom(&mut self) -> Simp {
        match self.peek() {
            TokenKind::POpen => {
                self.accept();
                if self.peek() == &TokenKind::PClose {
                    self.accept();
                    Simp::Unit
                } else {
                    let simp = self.parse_simp();
                    self.expect(TokenKind::PClose);

                    simp
                }
            },
            TokenKind::Name(name)
            if self.ty_cons.contains_key(&Name::from_string_ref(name)) => {
                let name = self.expect_name();
                let vals = self.parse_simp_list();

                Simp::Data(name, vals)
            },
            TokenKind::Name(_) => {
                let name = self.expect_name();
                Simp::Ref(name)
            },
            TokenKind::Number(n) => {
                let n = *n;
                self.accept();
                Simp::Int(n)
            },
            _ => expect_fail!("Atom", self.peek_wpos()),
        }
    }

    fn parse_simp_list(&mut self) -> Vec<Simp> {
        let mut slist = Vec::new();
        match self.peek() {
            TokenKind::POpen => {
                self.accept();
                slist.push(self.parse_simp());

                loop {
                    let token = self.peek_wpos();
                    match token.kind {
                        TokenKind::Comma => {
                            self.accept();
                            slist.push(self.parse_simp());
                        },
                        TokenKind::PClose => {
                            self.accept();
                            break;
                        },
                        _ => expect_fail!("',' or ')'", token),
                    }
                }
            }
            _ => {}
        }

        slist
    }

    fn parse_match(&mut self) -> Simp {
        self.expect(TokenKind::Match);
        let expr = self.parse_simp();

        self.expect(TokenKind::BOpen);

        let mut cases = Vec::new();
        loop {
            match self.peek() {
                TokenKind::Endl => { self.accept(); continue; },
                TokenKind::BClose => { self.accept(); break; },
                _ => {
                    let pattern = self.parse_pattern();
                    self.expect(TokenKind::Colon);
                    let simp = self.parse_simp();
                    cases.push((pattern, simp));
                },
            }
        }

        Simp::Match(Box::new(expr), cases)
    }

    fn parse_data_ref(&mut self) -> DataDef {
        self.expect(TokenKind::DataDef);
        let name = self.expect_name();
        self.expect(TokenKind::Eq);

        let mut cons = Vec::new();
        cons.push(self.parse_cons());

        loop {
            match self.peek() {
                TokenKind::Pipe => {
                    self.accept();
                    cons.push(self.parse_cons());
                },
                TokenKind::Endl => { self.accept(); },
                _ => break,
            }
        }

        DataDef {
            name,
            cons,
        }
    }

    fn parse_cons(&mut self) -> Cons {
        let tag = self.expect_name();
        let args = match self.peek() {
            TokenKind::POpen => self.parse_type_list(),
            _ => Vec::new(),
        };

        Cons {
            tag,
            args,
        }
    }

    fn parse_type_def(&mut self) -> TypeDef {
        self.expect(TokenKind::TypeDef);
        let name = self.expect_name();
        self.expect(TokenKind::Eq);
        let ty = self.parse_type();

        TypeDef {
            name,
            ty,
        }
    }

    fn parse_type(&mut self) -> Type {
        let mut lhs = self.parse_type_list();
        if self.peek() == &TokenKind::Arrow {
            self.accept();
            let rhs = self.parse_type();

            Type::Fn(lhs, Box::new(rhs))
        }
        else {
            if lhs.len() > 1 {
                unimplemented!("Tuples")
            }
            else {
                let ty = lhs.pop().unwrap();
                ty
            }
        }
    }

    fn parse_type_list(&mut self) -> Vec<Type> {
        if self.peek() == &TokenKind::POpen {
            self.accept();

            let mut types = Vec::new();
            types.push(self.parse_type());

            loop {
                match self.peek() {
                    TokenKind::Comma => {
                        self.accept();
                        types.push(self.parse_type());
                    },
                    TokenKind::PClose => {
                        self.accept();
                        break;
                    },
                    _ => expect_fail!("',' or ')'", self.peek_wpos()),
                }
            }

            types
        } else {
            let name = self.expect_name();
            let ty = match name.0 {
                "Int" => Type::Int,
                "Unit" => Type::Unit,
                _ => Type::UserDef(name),
            };

            vec![ty]
        }
    }
}