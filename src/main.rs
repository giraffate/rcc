use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::iter::Iterator;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("invalid arguments!");
    }

    args.next().unwrap();
    let tokens = tokenize(args.next().unwrap());
    let mut parser = Parser::new(tokens);
    let node = parser.parse();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Reserved(String),
    Num(u32),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Num(n) => n.fmt(f),
            Token::Reserved(s) => s.fmt(f),
        }
    }
}

impl Token {
    #[allow(dead_code)]
    fn expect_number(&self) -> u32 {
        match self {
            Token::Num(n) => *n,
            _ => panic!("expect number!"),
        }
    }
}

fn tokenize(s: String) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        match c {
            '+' | '-' | '*' | '/' | '(' | ')' => tokens.push_back(Token::Reserved(c.to_string())),
            _ if c.is_ascii_digit() => {
                let mut n = Vec::new();
                n.push(c);
                while let Some(next_c) = iter.peek() {
                    if !next_c.is_ascii_digit() {
                        break;
                    }

                    let next_c = iter.next().unwrap();
                    n.push(next_c);
                }
                let n = n.into_iter().collect::<String>().parse::<u32>().unwrap();
                tokens.push_back(Token::Num(n));
            }
            _ if c.is_whitespace() => {}
            _ => panic!("unexpected input!"),
        }
    }
    tokens
}

#[derive(Clone, Debug)]
enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Num(u32),
}

struct Parser {
    tokens: VecDeque<Token>,
    idx: usize,
}

impl Parser {
    fn new(tokens: VecDeque<Token>) -> Self {
        Parser { tokens, idx: 0 }
    }

    fn parse(&mut self) -> Node {
        self.expr()
    }

    // expr = mul ("+" mul | "-" mul)
    fn expr(&mut self) -> Node {
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

fn gen(node: Node) {
    match node {
        Node::Num(n) => {
            println!("  push {}", n);
            return;
        }
        _ => {}
    }

    match node {
        Node::Add(l, r) => {
            gen(*l);
            gen(*r);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  add rax, rdi");
        }
        Node::Sub(l, r) => {
            gen(*l);
            gen(*r);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  sub rax, rdi");
        }
        Node::Mul(l, r) => {
            gen(*l);
            gen(*r);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  imul rax, rdi");
        }
        Node::Div(l, r) => {
            gen(*l);
            gen(*r);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => {}
    }

    println!("  push rax");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "3".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::Num(3)]);

        let input = "13".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::Num(13)]);

        let input = "3+14".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::Num(3),
                Token::Reserved("+".to_string()),
                Token::Num(14)
            ]
        );

        let input = "3 + 14 - 1".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::Num(3),
                Token::Reserved("+".to_string()),
                Token::Num(14),
                Token::Reserved("-".to_string()),
                Token::Num(1)
            ]
        );

        let input = "3 * 14 / 1".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::Num(3),
                Token::Reserved("*".to_string()),
                Token::Num(14),
                Token::Reserved("/".to_string()),
                Token::Num(1)
            ]
        );
    }
}
