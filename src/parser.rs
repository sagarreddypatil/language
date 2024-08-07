use crate::ast::*;
use crate::lexer::*;
use std::collections::HashMap;

pub struct Parser {
    pub tokens: Vec<Token>,
    position: usize,

    // map from type constructor to data type name
    ty_cons: HashMap<Name, DataDef>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,

            ty_cons: HashMap::new(),
        }
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

    fn accept(&mut self) -> Token {
        let token = *self.tokens.get(self.position).unwrap();
        self.position += 1;

        token
    }

    fn expect(&mut self, token: TokenKind) {
        let next = self.accept();
        if next.kind != token {
            panic!("Expected {:?}, got {:?}", token, next);
        }
    }

    fn expect_name(&mut self) -> Name {
        let token = self.accept();
        match &token.kind {
            TokenKind::Ident(name) => Name(name),
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
                TokenKind::Keyword("data") => program.data_defs.push(self.parse_data_ref()),
                // TokenKind::TypeDef => program.type_defs.push(self.parse_type_def()),
                _ => break,
            }
        }

        for data_def in &program.data_defs {
            for cons in &data_def.cons {
                self.ty_cons.insert(*cons.0, data_def.clone());
            }
        }

        program.expr = Some(self.parse_expr());
        program
    }

    fn parse_otype(&mut self) -> Type {
        match self.peek() {
            TokenKind::Colon => {
                self.accept();
                self.parse_type()
            }
            _ => fresh_tv(),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match self.peek() {
            TokenKind::Keyword("let") => self.parse_let(),
            _ => Expr::Simp(self.parse_simp()),
        }
    }

    fn parse_let(&mut self) -> Expr {
        self.expect(TokenKind::Keyword("let"));
        let pattern = self.parse_pattern();

        self.expect(TokenKind::Keyword("="));
        let rhs = self.parse_simp();

        let body = self.parse_expr();

        Expr::Bind(pattern, rhs, Box::new(body))
    }

    fn parse_pattern(&mut self) -> Pattern {
        match self.peek() {
            TokenKind::Ident(_) => {
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
            TokenKind::IntLit(n) => {
                let n = *n;
                self.accept();
                Pattern::Int(n)
            }
            TokenKind::BoolLit(b) => {
                let b = *b;
                self.accept();
                Pattern::Bool(b)
            }
            _ => panic!("Expected pattern, got {:?}", self.peek_wpos()),
        }
    }

    fn parse_pat_list(&mut self) -> Vec<Pattern> {
        let mut pats = Vec::new();
        match self.peek() {
            TokenKind::POpen => {
                self.accept();
                pats.push(self.parse_pattern());

                loop {
                    match self.peek() {
                        TokenKind::Comma => {
                            self.accept();
                            pats.push(self.parse_pattern());
                        }
                        TokenKind::PClose => {
                            self.accept();
                            break;
                        }
                        _ => panic!("Expected ',' or ')', got {:?}", self.peek_wpos()),
                    }
                }
            }
            _ => {}
        }

        pats
    }

    fn parse_simp(&mut self) -> Simp {
        match self.peek() {
            TokenKind::Keyword("if") => self.parse_if(),
            TokenKind::Keyword("match") => self.parse_match(),
            TokenKind::Keyword("fn") => self.parse_fndef(),
            TokenKind::BOpen => {
                self.accept();
                let expr = self.parse_expr();

                self.expect(TokenKind::BClose);

                Simp::Block(Box::new(expr))
            }
            _ => self.parse_simple_ops(0),
        }
    }

    fn parse_fndef(&mut self) -> Simp {
        self.accept();
        self.expect(TokenKind::POpen);

        // parse name of args, comma separated
        let mut args = Vec::new();
        if self.peek() != &TokenKind::PClose {
            args.push((self.expect_name(), self.parse_otype()));
            loop {
                match self.peek() {
                    TokenKind::Comma => {
                        self.accept();
                        args.push((self.expect_name(), self.parse_otype()));
                    }
                    TokenKind::PClose => {
                        self.accept();
                        break;
                    }
                    _ => panic!("Expected ',' or ')', got {:?}", self.peek_wpos()),
                }
            }
        }

        let ret = self.parse_otype();
        self.expect(TokenKind::Keyword("="));

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
                TokenKind::Ident(name) => Name(name),
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
            TokenKind::POpen => {
                let args = self.parse_simp_list();
                Simp::FnCall(Box::new(lhs), args)
            }
            _ => lhs,
        }
    }

    fn parse_utight(&mut self) -> Simp {
        match self.peek() {
            TokenKind::Ident(name) if Name(name).unary() => {
                let name = self.expect_name();
                let name = Simp::Ref(name);

                let rest = self.parse_tight();
                Simp::FnCall(Box::new(name), vec![rest])
            }
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
            }
            TokenKind::Ident(name) if self.ty_cons.contains_key(&Name(name)) => {
                let name = self.expect_name();
                let vals = self.parse_simp_list();

                Simp::Data(name, vals)
            }
            TokenKind::Ident(_) => {
                let name = self.expect_name();
                Simp::Ref(name)
            }
            TokenKind::IntLit(n) => {
                let n = *n;
                self.accept();
                Simp::Int(n)
            }
            TokenKind::BoolLit(b) => {
                let b = *b;
                self.accept();
                Simp::Bool(b)
            }
            _ => panic!("Expected atom, got {:?}", self.peek_wpos()),
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
                        }
                        TokenKind::PClose => {
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
        self.expect(TokenKind::Keyword("if"));
        let cond = self.parse_simp();
        let then = self.parse_simp();

        let els = match self.peek() {
            TokenKind::Keyword("else") => {
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
        self.expect(TokenKind::Keyword("match"));
        let expr = self.parse_simp();

        let mut cases = Vec::new();
        loop {
            match self.peek() {
                TokenKind::Pipe => {
                    self.accept();
                    let pattern = self.parse_pattern();
                    self.expect(TokenKind::Keyword("=>"));
                    let simp = self.parse_simp();
                    cases.push((pattern, simp));
                }
                _ => break,
            }
        }

        if cases.len() == 0 {
            panic!("Match statement at {} has no cases", self.peek_wpos());
        }

        Simp::Match(Box::new(expr), cases)
    }

    fn parse_data_ref(&mut self) -> DataDef {
        self.expect(TokenKind::Keyword("data"));
        let name = self.expect_name();
        self.expect(TokenKind::Keyword("="));

        let mut cons = HashMap::new();

        let con = self.parse_cons();
        cons.insert(con.0, con.1);

        loop {
            match self.peek() {
                TokenKind::Pipe => {
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
            TokenKind::POpen => self.parse_type_list(),
            _ => Vec::new(),
        };

        (tag, Cons { args })
    }

    // fn parse_type_def(&mut self) -> TypeDef {
    //     self.expect(TokenKind::TypeDef);
    //     let name = self.expect_name();
    //     self.expect(TokenKind::Keyword("="));
    //     let ty = self.parse_type();

    //     TypeDef {
    //         name,
    //         ty,
    //     }
    // }

    fn parse_type(&mut self) -> Type {
        let mut lhs = self.parse_type_list();
        if self.peek() == &TokenKind::Keyword("->") {
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
        if self.peek() == &TokenKind::POpen {
            self.accept();

            let mut types = Vec::new();
            types.push(self.parse_type());

            loop {
                match self.peek() {
                    TokenKind::Comma => {
                        self.accept();
                        types.push(self.parse_type());
                    }
                    TokenKind::PClose => {
                        self.accept();
                        break;
                    }
                    _ => panic!("Expected ',' or ')', got {:?}", self.peek_wpos()),
                }
            }

            types
        } else {
            let name = self.expect_name();
            let ty = match name.0 {
                "Int" => Type::Int,
                "Unit" => Type::Unit,
                "Bool" => Type::Bool,
                _ => Type::UserDef(name),
            };

            vec![ty]
        }
    }
}
