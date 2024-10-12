use logos::Lexer;

use crate::ast::*;
use crate::lexer::*;
use std::collections::HashMap;
use std::iter::Peekable;

pub struct Parser<'a> {
    pub lexer: Peekable<Lexer<'a, Token>>,
    // map from type constructor to data type name
    ty_cons: HashMap<Name, DataDef>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, Token>) -> Self {
        Parser {
            lexer: lexer.peekable(),
            ty_cons: HashMap::new(),
        }
    }

    fn end(&mut self) -> bool {
        let peek = self.lexer.peek();
        let peek = peek.map(|x| x.is_ok());

        match peek {
            Some(true) => false,
            _ => true,
        }
    }

    fn peek(&mut self) -> &Token {
        if self.end() {
            &Token::EOF
        } else {
            self.lexer.peek().unwrap().as_ref().unwrap()
        }
    }

    fn accept(&mut self) -> Token {
        let token = self.lexer.next();
        let token = token.unwrap().unwrap();

        token
    }

    fn expect(&mut self, token: Token) {
        let next = self.accept();

        if next != token {
            panic!("Expected {:?}, got {:?}", token, next);
        }
    }

    fn expect_name(&mut self) -> Name {
        let token = self.accept();

        match token {
            Token::Ident(name) => Name(name),
            _ => panic!("Expected name, got {:?}", token),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            data_defs: Vec::new(),
            // type_defs: Vec::new(),
            expr: None,
        };

        while !self.end() {
            match self.peek() {
                Token::Data => program.data_defs.push(self.parse_data_ref()),
                // Token::TypeDef => program.type_defs.push(self.parse_type_def()),
                _ => break,
            }
        }

        for data_def in &program.data_defs {
            for cons in &data_def.cons {
                self.ty_cons.insert(cons.0.clone(), data_def.clone());
            }
        }

        program.expr = Some(self.parse_expr());
        program
    }

    fn parse_otype(&mut self) -> Type {
        match self.peek() {
            Token::Colon => {
                self.accept();
                self.parse_type()
            }
            _ => fresh_tv(),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match self.peek() {
            Token::Let => self.parse_let(),
            _ => Expr::Simp(self.parse_simp()),
        }
    }

    fn parse_let(&mut self) -> Expr {
        self.expect(Token::Let);
        let pattern = self.parse_pattern();

        self.expect(Token::Eq);
        let rhs = self.parse_simp();

        let body = self.parse_expr();

        Expr::Bind(pattern, rhs, Box::new(body))
    }

    fn parse_pattern(&mut self) -> Pattern {
        match self.peek() {
            Token::Ident(_) => {
                let name = self.expect_name();

                if self.ty_cons.contains_key(&name) {
                    let pats = self.parse_pat_list();
                    let df = self.ty_cons.get(&name).unwrap().clone();
                    Pattern::Data(df, name, pats)
                } else {
                    let ty = self.parse_otype();
                    Pattern::Var(name, ty)
                }
            }
            Token::Int(n) => {
                let n = *n;
                self.accept();
                Pattern::Int(n)
            }
            Token::Bool(b) => {
                let b = *b;
                self.accept();
                Pattern::Bool(b)
            }
            _ => panic!("Expected pattern, got {:?}", self.peek()),
        }
    }

    fn parse_pat_list(&mut self) -> Vec<Pattern> {
        let mut pats = Vec::new();
        match self.peek() {
            Token::POpen => {
                self.accept();
                pats.push(self.parse_pattern());

                loop {
                    match self.peek() {
                        Token::Comma => {
                            self.accept();
                            pats.push(self.parse_pattern());
                        }
                        Token::PClose => {
                            self.accept();
                            break;
                        }
                        _ => panic!("Expected ',' or ')', got {:?}", self.peek()),
                    }
                }
            }
            _ => {}
        }

        pats
    }

    fn parse_simp(&mut self) -> Simp {
        match self.peek() {
            Token::If => self.parse_if(),
            Token::Match => self.parse_match(),
            Token::Fn => self.parse_fndef(),
            Token::BOpen => {
                self.accept();
                let expr = self.parse_expr();

                self.expect(Token::BClose);

                Simp::Block(Box::new(expr))
            }
            _ => self.parse_simple_ops(0),
        }
    }

    fn parse_fndef(&mut self) -> Simp {
        self.accept();
        self.expect(Token::POpen);

        // parse name of args, comma separated
        let mut args = Vec::new();
        if self.peek() != &Token::PClose {
            args.push((self.expect_name(), self.parse_otype()));
            loop {
                match self.peek() {
                    Token::Comma => {
                        self.accept();
                        args.push((self.expect_name(), self.parse_otype()));
                    }
                    Token::PClose => {
                        self.accept();
                        break;
                    }
                    _ => panic!("Expected ',' or ')', got {:?}", self.peek()),
                }
            }
        }

        let ret = self.parse_otype();
        self.expect(Token::Eq);

        Simp::FnDef(FnDef {
            args,
            body: Box::new(self.parse_simp()),
            ret,
        })
    }

    fn parse_simple_ops(&mut self, min_prec: i32) -> Simp {
        let mut lhs = self.parse_utight();
        loop {
            let name = match self.peek() {
                Token::Ident(name) => Name(name.clone()),
                _ => break,
            };

            if !name.valid() || name.prec() < min_prec {
                break;
            }

            self.accept();
            let new_min = name.prec() + name.assoc();
            let rhs = self.parse_simple_ops(new_min);

            let fname = Box::new(Simp::Ref(name));

            lhs = Simp::FnCall(fname, vec![lhs, rhs]);
        }

        lhs
    }

    fn parse_tight(&mut self) -> Simp {
        let lhs = self.parse_atom();

        match self.peek() {
            Token::POpen => {
                let args = self.parse_simp_list();
                Simp::FnCall(Box::new(lhs), args)
            }
            _ => lhs,
        }
    }

    fn parse_utight(&mut self) -> Simp {
        match self.peek() {
            Token::Ident(name) if Name(name.clone()).unary() => {
                let name = self.expect_name();
                let name = Simp::Ref(name);

                let rest = self.parse_tight();
                Simp::FnCall(Box::new(name), vec![rest])
            }
            _ => self.parse_tight(),
        }
    }

    fn parse_atom(&mut self) -> Simp {
        let peeked = self.peek();
        let peeked: Token = peeked.clone();

        match peeked {
            Token::POpen => {
                self.accept();
                if self.peek() == &Token::PClose {
                    self.accept();
                    Simp::Unit
                } else {
                    let simp = self.parse_simp();

                    self.expect(Token::PClose);

                    simp
                }
            }
            Token::Ident(name) if self.ty_cons.contains_key(&Name(name.clone())) => {
                let name = self.expect_name();
                let vals = self.parse_simp_list();

                Simp::Data(name, vals)
            }
            Token::Ident(_) => {
                let name = self.expect_name();
                Simp::Ref(name)
            }
            Token::Int(n) => {
                self.accept();
                Simp::Int(n)
            }
            Token::Bool(b) => {
                self.accept();
                Simp::Bool(b)
            }
            _ => panic!("Expected atom, got {:?}", self.peek()),
        }
    }

    fn parse_simp_list(&mut self) -> Vec<Simp> {
        let mut slist = Vec::new();
        match self.peek() {
            Token::POpen => {
                self.accept();
                slist.push(self.parse_simp());

                loop {
                    let token = self.peek();
                    match token {
                        Token::Comma => {
                            self.accept();
                            slist.push(self.parse_simp());
                        }
                        Token::PClose => {
                            self.accept();
                            break;
                        }
                        _ => panic!("Expected ',' or ')', got {:?}", token),
                    }
                }
            }
            _ => {}
        }

        slist
    }

    fn parse_if(&mut self) -> Simp {
        self.expect(Token::If);
        let cond = self.parse_simp();
        let then = self.parse_simp();

        let els = match self.peek() {
            Token::Else => {
                self.accept();
                self.parse_simp()
            }
            _ => Simp::Unit,
        };

        Simp::Match(
            Box::new(cond),
            vec![(Pattern::Bool(true), then), (Pattern::Bool(false), els)],
        )
    }

    fn parse_match(&mut self) -> Simp {
        self.expect(Token::Match);
        let expr = self.parse_simp();

        let mut cases = Vec::new();
        loop {
            match self.peek() {
                Token::Pipe => {
                    self.accept();
                    let pattern = self.parse_pattern();
                    self.expect(Token::FatArrow);
                    let simp = self.parse_simp();
                    cases.push((pattern, simp));
                }
                _ => break,
            }
        }

        if cases.len() == 0 {
            panic!("Match statement at {:?} has no cases", self.peek());
        }

        Simp::Match(Box::new(expr), cases)
    }

    fn parse_data_ref(&mut self) -> DataDef {
        self.expect(Token::Data);
        let name = self.expect_name();
        self.expect(Token::Eq);

        let mut cons = HashMap::new();

        let con = self.parse_cons();
        cons.insert(con.0, con.1);

        loop {
            match self.peek() {
                Token::Pipe => {
                    self.accept();
                    let con = self.parse_cons();
                    cons.insert(con.0, con.1);
                }
                _ => break,
            }
        }

        DataDef { name, cons }
    }

    fn parse_cons(&mut self) -> (Name, Cons) {
        let tag = self.expect_name();
        let args = match self.peek() {
            Token::POpen => self.parse_type_list(),
            _ => Vec::new(),
        };

        (tag, Cons { args })
    }

    // fn parse_type_def(&mut self) -> TypeDef {
    //     self.expect(Token::TypeDef);
    //     let name = self.expect_name();
    //     self.expect(Token::Keyword("="));
    //     let ty = self.parse_type();

    //     TypeDef {
    //         name,
    //         ty,
    //     }
    // }

    fn parse_type(&mut self) -> Type {
        let mut lhs = self.parse_type_list();
        if self.peek() == &Token::Arrow {
            self.accept();
            let rhs = self.parse_type();

            Type::Fn(lhs, Box::new(rhs))
        } else {
            if lhs.len() > 1 {
                unimplemented!("Tuples")
            } else {
                let ty = lhs.pop().unwrap();
                ty
            }
        }
    }

    fn parse_type_list(&mut self) -> Vec<Type> {
        if self.peek() == &Token::POpen {
            self.accept();

            let mut types = Vec::new();
            types.push(self.parse_type());

            loop {
                match self.peek() {
                    Token::Comma => {
                        self.accept();
                        types.push(self.parse_type());
                    }
                    Token::PClose => {
                        self.accept();
                        break;
                    }
                    _ => panic!("Expected ',' or ')', got {:?}", self.peek()),
                }
            }

            types
        } else {
            let name = self.expect_name();
            let ty = match name.0.as_str() {
                "Int" => Type::Int,
                "Unit" => Type::Unit,
                "Bool" => Type::Bool,
                _ => Type::UserDef(name),
            };

            vec![ty]
        }
    }
}
