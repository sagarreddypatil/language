use crate::tokenizer::*;
use crate::ast::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn end(&self) -> bool {
        // last token is always EOF
        self.position >= self.tokens.len() - 1
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.position).unwrap()
    }

    fn accept(&mut self) -> &Token {
        let token = self.tokens.get(self.position).unwrap();
        self.position += 1;

        token
    }

    fn expect(&mut self, token: Token) {
        let next = self.accept();
        if next != &token {
            panic!("Expected {:?}, found {:?}", token, next);
        }
    }

    fn expect_name(&mut self) -> Name {
        let token = self.accept();
        match token {
            Token::Name(name) => {
                let name = Box::new(name.clone());
                let string = Box::leak(name).as_str();

                Name(string)
            },
            _ => panic!("Expected Name, found {:?}", token),
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
                Token::Endl => { self.accept(); },
                Token::DataDef => program.data_defs.push(self.parse_data_ref()),
                Token::TypeDef => program.type_defs.push(self.parse_type_def()),
                _ => break,
            }
        }

        program
    }

    pub fn parse_data_ref(&mut self) -> DataDef {
        self.expect(Token::DataDef);
        let name = self.expect_name();
        self.expect(Token::Eq);

        let mut cons = Vec::new();
        cons.push(self.parse_cons());

        loop {
            match self.peek() {
                Token::Pipe => {
                    self.accept();
                    cons.push(self.parse_cons());
                },
                Token::Endl => { self.accept(); },
                _ => break,
            }
        }

        DataDef {
            name,
            cons,
        }
    }

    pub fn parse_cons(&mut self) -> Cons {
        let tag = self.expect_name();
        let args = match self.peek() {
            Token::POpen => self.parse_type_list(),
            _ => Vec::new(),
        };

        Cons {
            tag,
            args,
        }
    }

    pub fn parse_type_def(&mut self) -> TypeDef {
        self.expect(Token::TypeDef);
        let name = self.expect_name();
        self.expect(Token::Eq);
        let ty = self.parse_type();

        TypeDef {
            name,
            ty,
        }
    }

    pub fn parse_type(&mut self) -> Type {
        let mut lhs = self.parse_type_list();
        if self.peek() == &Token::Arrow {
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

    pub fn parse_type_list(&mut self) -> Vec<Type> {
        if self.peek() == &Token::POpen {
            self.accept();

            let mut types = Vec::new();
            types.push(self.parse_type());

            loop {
                match self.peek() {
                    Token::Comma => {
                        self.accept();
                        types.push(self.parse_type());
                    },
                    Token::PClose => {
                        self.accept();
                        break;
                    },
                    _ => panic!("Expected ',' or ')', found {:?}", self.peek()),
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