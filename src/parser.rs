use crate::lexer::Token;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    EqualEqual,
    Define,
}

impl Op {
    pub fn from_token(tok: Token) -> Option<Self> {
        match tok {
            Token::Plus => Some(Self::Plus),
            Token::Minus =>Some(Self::Minus),
            Token::Star => Some(Self::Star),
            Token::Slash => Some(Self::Slash),
            Token::Power => Some(Self::Power),
            Token::EqualEqual => Some(Self::EqualEqual),
            Token::Define => Some(Self::Define),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Declaration {
        name: String,
        ty: Type,
    },
    Lambda {
        variable: Option<String>,
        body: Box<Expr>,
    },
    Call {
        name: String,
        variable: Box<Expr>,
    },
    BinaryExpr {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    UnaryExpr {
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnexpectedToken,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Number,
    Void,
    Lambda {
        variable: Box<Type>,
        body: Box<Type>,
    },
    Vector(usize),
    Matrix(usize, usize),
}

impl Type {
    pub fn from_string(s: String) -> Result<Self, Error> {
        match s.as_str() {
            "number" => Ok(Self::Number),
            _ => Err(Error::UnexpectedToken)
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    definitions: HashMap<String, Expr>,
    current: usize,
}


impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            definitions: HashMap::new(),
            current: 0,
        }
    }

    pub fn current(&self) -> Result<Token, Error> {
        if let Some(t) = self.tokens.get(self.current) {
            return Ok(t.clone())
        }

        Err(Error::EOF)
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        if let Some(tok) = self.tokens.get(self.current) {
            self.current += 1;
            Ok(tok.clone())
        } else {
            Err(Error::EOF)
        }
    }

    pub fn check(&self, token: Token) -> Result<(), Error> {
        if self.current()? == token {
            return Ok(());
        }

        Err(Error::UnexpectedToken)
    }

    pub fn consume(&mut self, token: Token) -> Result<(), Error> {
        if let Ok(_) = self.check(token) {
            self.current += 1;
            return Ok(());
        }

        Err(Error::UnexpectedToken)
    }

    pub fn expect(&mut self, tokens: Vec<Token>) -> Result<(), Error> {
        for tok in tokens.iter() {
            if let Ok(_) = self.check(tok.clone()) {
                self.current += 1;
                return Ok(());
            }
        }

        Err(Error::UnexpectedToken)
    }

    pub fn previous(&self) -> Result<Token, Error> {
        let tok = self.tokens.get(self.current-1).ok_or(Error::UnexpectedToken)?.clone();

        Ok(tok)
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Expr, Error> {
        self.tokens = tokens;
        let def = self.definition();
        if let Err(_) = def {
            println!("{:?}", self.current());
        } 

        def
    }

    pub fn definition(&mut self) -> Result<Expr, Error> {
        let mut res = self.declaration()?; 
        if let Ok(_) = self.expect(vec![Token::Define]) {
            if let Expr::Declaration { name, .. } = res.clone() {
                let expr = self.lambda()?;
                self.definitions.insert(name, expr.clone());
                res = Expr::BinaryExpr {
                    lhs: Box::new(res),
                    op: Op::Define,
                    rhs: Box::new(expr),
                }; 
            }
        }

        Ok(res)
    }

    pub fn declaration(&mut self) -> Result<Expr, Error> {
        let mut res = self.term()?; 
        if let Ok(_) = self.expect(vec![Token::DefineType]) {
            if let Expr::Identifier(id) = res.clone() {
                res = Expr::Declaration {
                    name: id,
                    ty: self.ty()?,
                }; 
            }
        }

        Ok(res)
    }

    pub fn ty(&mut self) -> Result<Type, Error> {
        if let Expr::Identifier(id) = self.primary()? {
            let ty = Type::from_string(id)?;

            if let Ok(_) = self.expect(vec![Token::Arrow]) {
                let ty = Type::Lambda {
                    variable: Box::new(ty),
                    body: Box::new(self.ty()?),
                }; 
                return Ok(ty);
            }

            return Ok(ty)
        } else if let Ok(_) = self.expect(vec![Token::LeftSquared]) {
            if let Token::Number(n) = self.next()? {
                let mut ty = Type::Void;
                let width = n as usize;
                if let Ok(_) = self.expect(vec![Token::Comma]) {
                    if let Token::Number(n1) = self.next()? {
                        let height = n1 as usize;
                        self.consume(Token::RightSquared)?;
                        ty = Type::Matrix(width, height);
                    }
                } else if let Ok(_) = self.expect(vec![Token::RightSquared]) {
                    ty = Type::Vector(width);
                }

                if let Ok(_) = self.expect(vec![Token::Arrow]) {
                    ty = Type::Lambda {
                        variable: Box::new(ty),
                        body: Box::new(self.ty()?),
                    }; 
                }

                return Ok(ty);
            }
        }

        Err(Error::UnexpectedToken)
    }

    pub fn lambda(&mut self) -> Result<Expr, Error> {
        if let Token::Lambda = self.next()? {
            if let Expr::Identifier(variable) = self.primary()? {
                if let Ok(_) = self.consume(Token::Dot) {
                    let body = Box::new(self.term()?);
                    return Ok(Expr::Lambda {
                        variable: Some(variable),
                        body,
                    });
                }
            }
            if let Ok(_) = self.consume(Token::Dot) {
                let body = Box::new(self.term()?);
                return Ok(Expr::Lambda {
                    variable: None,
                    body,
                });
            }
        }

        self.term()
    }

    pub fn term(&mut self) -> Result<Expr, Error> {
        let mut res = self.factor()?;

        while let Ok(_) = self.expect(vec![Token::Plus, Token::Minus]) {
            let op = Op::from_token(self.previous()?).unwrap();

            let rhs = self.factor()?;

            res = Expr::BinaryExpr {
                lhs: Box::new(res),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(res)
    }

    pub fn factor(&mut self) -> Result<Expr, Error> {
        let mut res = self.unary()?;

        while let Ok(_) = self.expect(vec![Token::Star, Token::Slash]) {
            let op = Op::from_token(self.previous()?).unwrap();

            let rhs = self.unary()?;

            res = Expr::BinaryExpr {
                lhs: Box::new(res),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(res)
    }

    pub fn unary(&mut self) -> Result<Expr, Error> {
        if let Ok(_) = self.expect(vec![Token::Plus, Token::Minus]) {
            let op = Op::from_token(self.previous()?).unwrap();

            let rhs = self.primary()?;
            return Ok(Expr::UnaryExpr {
                op,
                rhs: Box::new(rhs),
            });
        } 

        self.primary()
    }
            
    pub fn primary(&mut self) -> Result<Expr, Error> {
        if let Token::Number(n) = self.next()? {
            return Ok(Expr::Number(n));
        }

        self.current -= 1;

        if let Token::Identifier(id) = self.next()? {
            if let Some(expr) = self.definitions.get(&id) {
                if let Ok(expr) = self.primary() {
                    return Ok(Expr::Call {
                        name: id,
                        variable: Box::new(expr),
                    });
                } 
            }
            return Ok(Expr::Identifier(id));
        }

        self.current -= 1;

        Err(Error::UnexpectedToken)
    }
}
