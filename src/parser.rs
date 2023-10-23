use std::collections::VecDeque;

use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Eq(Box<Node>, Box<Node>), // ==
    Ne(Box<Node>, Box<Node>), // !=
    Lt(Box<Node>, Box<Node>), // <
    Le(Box<Node>, Box<Node>), // <=
    Num(u32),
}

pub struct Parser {
    tokens: VecDeque<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Parser { tokens, idx: 0 }
    }

    pub fn parse(&mut self) -> Node {
        self.expr()
    }

    // expr = equality
    fn expr(&mut self) -> Node {
        self.equality()
    }

    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> Node {
        let mut node = self.relational();

        loop {
            match self.tokens.get(self.idx) {
                Some(Token::Reserved(s)) if matches!(s.as_str(), "==") => {
                    self.idx += 1;
                    node = Node::Eq(Box::new(node), Box::new(self.relational()));
                }
                Some(Token::Reserved(s)) if matches!(s.as_str(), "!=") => {
                    self.idx += 1;
                    node = Node::Ne(Box::new(node), Box::new(self.relational()))
                }
                _ => break,
            }
        }

        node
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> Node {
        let mut node = self.add();

        loop {
            match self.tokens.get(self.idx) {
                Some(Token::Reserved(s)) if matches!(s.as_str(), "<") => {
                    self.idx += 1;
                    node = Node::Lt(Box::new(node), Box::new(self.add()));
                }
                Some(Token::Reserved(s)) if matches!(s.as_str(), "<=") => {
                    self.idx += 1;
                    node = Node::Le(Box::new(node), Box::new(self.mul()))
                }
                Some(Token::Reserved(s)) if matches!(s.as_str(), ">") => {
                    self.idx += 1;
                    node = Node::Lt(Box::new(self.add()), Box::new(node));
                }
                Some(Token::Reserved(s)) if matches!(s.as_str(), ">=") => {
                    self.idx += 1;
                    node = Node::Le(Box::new(self.add()), Box::new(node))
                }
                _ => break,
            }
        }

        node
    }

    // add = mul ("+" mul | "-" mul)
    fn add(&mut self) -> Node {
        let mut node = self.mul();

        loop {
            match self.tokens.get(self.idx) {
                Some(Token::Reserved(s)) if matches!(s.as_str(), "+") => {
                    self.idx += 1;
                    node = Node::Add(Box::new(node), Box::new(self.mul()));
                }
                Some(Token::Reserved(s)) if matches!(s.as_str(), "-") => {
                    self.idx += 1;
                    node = Node::Sub(Box::new(node), Box::new(self.mul()))
                }
                _ => break,
            }
        }

        node
    }

    // mul = unary ("*" unary | "/" unary)
    fn mul(&mut self) -> Node {
        let mut node = self.unary();

        loop {
            match self.tokens.get(self.idx) {
                Some(Token::Reserved(s)) if matches!(s.as_str(), "*") => {
                    self.idx += 1;
                    node = Node::Mul(Box::new(node), Box::new(self.unary()))
                }
                Some(Token::Reserved(s)) if matches!(s.as_str(), "/") => {
                    self.idx += 1;
                    node = Node::Div(Box::new(node), Box::new(self.unary()))
                }
                _ => break,
            }
        }

        node
    }

    // unary = ("+" | "-")? unary | primary
    fn unary(&mut self) -> Node {
        match self.tokens.get(self.idx) {
            Some(Token::Reserved(s)) if matches!(s.as_str(), "+") => {
                self.idx += 1;
                return self.unary();
            }
            Some(Token::Reserved(s)) if matches!(s.as_str(), "-") => {
                self.idx += 1;
                return Node::Sub(Box::new(Node::Num(0)), Box::new(self.unary()));
            }
            _ => {}
        }

        self.primary()
    }

    // primary = num | "(" expr ")"
    fn primary(&mut self) -> Node {
        let node = match self.tokens.get(self.idx) {
            Some(Token::Reserved(s)) if matches!(s.as_str(), "(") => {
                self.idx += 1;
                let node = self.expr();
                match self.tokens.get(self.idx) {
                    Some(Token::Reserved(s)) if matches!(s.as_str(), ")") => {
                        self.idx += 1;
                        node
                    }
                    _ => panic!("unexpected token in primary"),
                }
            }
            Some(Token::Num(n)) => {
                self.idx += 1;
                Node::Num(*n)
            }
            _ => panic!("unexpected token in primary"),
        };
        node
    }
}
