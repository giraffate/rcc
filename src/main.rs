use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::iter::Iterator;
use std::rc::Rc;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("invalid arguments!");
    }

    args.next().unwrap();
    let tokens = tokenize(args.next().unwrap());
    let node = parse(tokens);

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

type TokenList = Rc<RefCell<VecDeque<Token>>>;

fn tokenize(s: String) -> TokenList {
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
    Rc::new(RefCell::new(tokens))
}

#[derive(Clone, Debug)]
enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Num(u32),
}

fn parse(tokens: TokenList) -> Node {
    expr(tokens)
}

// expr = mul ("+" mul | "-" mul)
fn expr(tokens: TokenList) -> Node {
    let mut node = mul(tokens.clone());

    loop {
        match (*tokens).borrow_mut().front() {
            Some(Token::Reserved(s)) if matches!(s.as_str(), "+" | "-") => {}
            _ => break,
        }

        let token = (*tokens).borrow_mut().pop_front();
        match token {
            Some(Token::Reserved(s)) if matches!(s.as_str(), "+") => {
                node = Node::Add(Box::new(node), Box::new(mul(tokens.clone())))
            }
            Some(Token::Reserved(s)) if matches!(s.as_str(), "-") => {
                node = Node::Sub(Box::new(node), Box::new(mul(tokens.clone())))
            }
            _ => break,
        }
    }

    node
}

// mul = primary ("*" primary | "/" primary)
fn mul(tokens: TokenList) -> Node {
    let mut node = primary(tokens.clone());

    loop {
        match (*tokens).borrow_mut().front() {
            Some(Token::Reserved(s)) if matches!(s.as_str(), "*" | "/") => {}
            _ => break,
        }

        let token = (*tokens).borrow_mut().pop_front();
        match token {
            Some(Token::Reserved(s)) if matches!(s.as_str(), "*") => {
                node = Node::Mul(Box::new(node), Box::new(primary(tokens.clone())))
            }
            Some(Token::Reserved(s)) if matches!(s.as_str(), "/") => {
                node = Node::Div(Box::new(node), Box::new(primary(tokens.clone())))
            }
            _ => break,
        }
    }

    node
}

// primary = num | "(" expr ")"
fn primary(tokens: TokenList) -> Node {
    match (*tokens).borrow_mut().front() {
        Some(Token::Reserved(s)) if matches!(s.as_str(), "(") => {}
        Some(Token::Num(_)) => {}
        _ => panic!("unexpected token in primary"),
    }

    let token = (*tokens).borrow_mut().pop_front().unwrap();
    let node = match token {
        Token::Reserved(s) if matches!(s.as_str(), "(") => {
            let node = expr(tokens.clone());
            match (*tokens).borrow_mut().pop_front() {
                Some(Token::Reserved(s)) if matches!(s.as_str(), ")") => node,
                _ => panic!("unexpected token in primary"),
            }
        }
        Token::Num(n) => Node::Num(n),
        _ => panic!("unexpected token in primary"),
    };
    node
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
        let tokens = (*tokenize(input)).take();
        assert_eq!(tokens, vec![Token::Num(3)]);

        let input = "13".to_string();
        let tokens = (*tokenize(input)).take();
        assert_eq!(tokens, vec![Token::Num(13)]);

        let input = "3+14".to_string();
        let tokens = (*tokenize(input)).take();
        assert_eq!(
            tokens,
            vec![
                Token::Num(3),
                Token::Reserved("+".to_string()),
                Token::Num(14)
            ]
        );

        let input = "3 + 14 - 1".to_string();
        let tokens = (*tokenize(input)).take();
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
        let tokens = (*tokenize(input)).take();
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
