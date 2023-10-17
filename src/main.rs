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
    let mut tokens = tokenize(args.next().unwrap());

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", tokens.pop_front().unwrap());
    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();
        match token {
            Token::Reserved(s) => match s.as_str() {
                "+" => {
                    println!("  add rax, {}", tokens.pop_front().unwrap().expect_number());
                }
                "-" => {
                    println!("  sub rax, {}", tokens.pop_front().unwrap().expect_number());
                }
                _ => panic!("unexpected reserved token!"),
            },
            _ => {
                panic!("unexpected token!")
            }
        }
    }
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
            '+' | '-' => tokens.push_back(Token::Reserved(c.to_string())),
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
    }
}
