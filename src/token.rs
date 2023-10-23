use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
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

fn matches_two_chars(chars: &[char], idx: usize) -> bool {
    matches!(
        chars.get(idx..=idx + 1),
        Some(&['=', '=']) | Some(&['!', '=']) | Some(&['>', '=']) | Some(&['<', '='])
    )
}

pub fn tokenize(s: String) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    let chars = s.chars().collect::<Vec<_>>();
    let mut idx = 0;
    while idx < chars.len() {
        let c = chars.get(idx).unwrap();
        match *c {
            '=' | '!' | '>' | '<' if matches_two_chars(&chars, idx) => {
                let s = chars.get(idx..=idx + 1).unwrap().iter().collect::<String>();
                idx += 2;
                tokens.push_back(Token::Reserved(s));
            }
            '+' | '-' | '*' | '/' | '(' | ')' | '<' | '>' => {
                idx += 1;
                tokens.push_back(Token::Reserved(c.to_string()));
            }
            _ if c.is_ascii_digit() => {
                idx += 1;
                let mut n = Vec::new();
                n.push(*c);
                while let Some(next_c) = chars.get(idx) {
                    if !next_c.is_ascii_digit() {
                        break;
                    }
                    idx += 1;
                    n.push(*next_c);
                }
                let n = n.into_iter().collect::<String>().parse::<u32>().unwrap();
                tokens.push_back(Token::Num(n));
            }
            _ if c.is_whitespace() => idx += 1,
            _ => panic!("unexpected input!"),
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_two_char() {
        let chars = "==".chars().collect::<Vec<char>>();
        assert!(matches_two_chars(&chars, 0));

        let chars = "!=".chars().collect::<Vec<char>>();
        assert!(matches_two_chars(&chars, 0));

        let chars = "<=".chars().collect::<Vec<char>>();
        assert!(matches_two_chars(&chars, 0));

        let chars = ">=".chars().collect::<Vec<char>>();
        assert!(matches_two_chars(&chars, 0));

        let chars = "!!".chars().collect::<Vec<char>>();
        assert!(!matches_two_chars(&chars, 0));

        let chars = "<>".chars().collect::<Vec<char>>();
        assert!(!matches_two_chars(&chars, 0));
    }

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
