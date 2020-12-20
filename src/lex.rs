use crate::Value;

use std::{
    // string::{ ToString },
    fmt::{Debug, Display, Formatter, Result as fmt_Result},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    USub,
}
impl Operator {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            'u' => Some(Self::USub),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
            Self::USub => 'u',
        }
    }

    pub fn evaluate(&self, left: Value, right: Value) -> Value {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => {
                if right == 0 {
                    panic!("Divide by zero");
                } else {
                    left / right
                }
            }
            Self::USub => -right,
        }
    }
}
impl FromStr for Operator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Operator, Self::Err> {
        match Operator::from_char(
            s.chars()
                .next()
                .expect("Cannot parse empty string into operator"),
        ) {
            Some(n) => Ok(n),
            None => Err("Unknown operator"),
        }
    }
}
impl ToString for Operator {
    fn to_string(&self) -> String {
        format!("{}", self.to_char())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Paren {
    Left,
    Right,
}
impl Paren {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::Left),
            ')' => Some(Self::Right),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Left => '(',
            Self::Right => ')',
        }
    }
}
impl FromStr for Paren {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Paren, Self::Err> {
        match Paren::from_char(
            s.chars()
                .next()
                .expect("Cannot parse empty string into paren"),
        ) {
            Some(n) => Ok(n),
            None => Err("Unknown literal"),
        }
    }
}
impl ToString for Paren {
    fn to_string(&self) -> String {
        format!("{}", self.to_char())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token {
    Operator(Operator),
    Value(Value),
    Paren(Paren),
}
impl Token {
    #[cfg(test)]
    pub fn new(literal: &str) -> Self {
        match literal.parse::<Token>() {
            Ok(t) => t,
            Err(e) => panic!(e),
        }
    }
}
impl FromStr for Token {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Token, Self::Err> {
        if let Ok(v) = s.parse::<f64>() {
            Ok(Token::Value(v.into()))
        } else if let Ok(op) = s.parse::<Operator>() {
            Ok(Token::Operator(op))
        } else if let Ok(p) = s.parse::<Paren>() {
            Ok(Token::Paren(p))
        } else {
            Err("Unexpected literal")
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", {
            match self {
                Token::Operator(op) => op.to_string(),
                Token::Paren(p) => p.to_string(),
                Token::Value(v) => v.to_string(),
            }
        })
    }
}
impl Into<Value> for Token {
    fn into(self) -> Value {
        match self {
            Token::Value(v) => v,
            _ => panic!("Attempt to coerce non-value Token to Value"),
        }
    }
}

/// Parse the string `s` into a Token stream
/// ```rust
/// let tokens = vec![
///     Token::new("("),
///     Token::new("10"),
///     Token::new("+"),
///     Token::new("5"),
///     Token::new(")"),
/// ];
/// assert!(tokens == tokenize("(10+5)"));
/// ```
pub fn tokenize(s: &str) -> Vec<Token> {
    // /*DEBUG:*/ eprintln!("Begin tokenization");
    let mut buffer = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    let cleaned = s
        .chars()
        .filter(|&c| "1234567890./*-+^()".contains(c))
        .collect::<String>();

    let mut idx = 0;

    while let Some(c) = cleaned.chars().nth(idx) {
        // /*DEBUG:*/ eprint!("C: {}, IDX: {} -> ", c, idx);

        // check for unary operators (will always be first or directly following another operator (thanks greg!))
        // unwrap or will make this evalute true if it's the first item in the expression
        match tokens.last().unwrap_or(&Token::Operator(Operator::Add)) {
            Token::Operator(_) | Token::Paren(Paren::Left) => {
                if buffer.is_empty() && c == '-' && buffer.is_empty() {
                    // /*DEBUG:*/ eprintln!("Unary minus");
                    tokens.push(Token::Operator(Operator::USub));
                    idx += 1;
                    continue;
                }
            }
            _ => (),
        }

        // c is a number (0-9 or .), push it to the buffer
        if c.is_numeric() || c == '.' {
            // /*DEBUG:*/ eprintln!("Number: {}", c);
            buffer.push(c);
        }
        // if c is not a number, but there is something in the buffer, push the buffer to output
        else if !buffer.is_empty() {
            // /*DEBUG:*/ eprintln!("Commit number: {}", buffer);
            tokens.push(
                buffer
                    .parse()
                    .expect(&format!("Failed to parse buffer: {:?}", buffer)),
            );
            buffer = String::new();
            idx -= 1;
        }
        // Handle operators and parens normally
        else if let Some(op) = Operator::from_char(c) {
            // /*DEBUG:*/ eprintln!("Operator: {:?}", op);
            tokens.push(Token::Operator(op));
        } else if let Some(p) = Paren::from_char(c) {
            // /*DEBUG:*/ eprintln!("Paren: {:?}", p);
            tokens.push(Token::Paren(p));
        }

        idx += 1;
    }

    if !buffer.is_empty() {
        tokens.push(buffer.parse().expect("Failed to parse token from buffer"));
    }
    // /*DEBUG*/ eprintln!("End tokenization\n");

    tokens
}

fn precedence(token: &Token) -> u32 {
    match token {
        Token::Operator(o) => match o {
            Operator::Add => 2,
            Operator::Sub => 2,
            Operator::Mul => 3,
            Operator::Div => 3,
            Operator::USub => 5,
        },
        _ => 0,
    }
}

#[derive(Copy, Clone, PartialEq)]
enum OperatorAssociativity {
    Left,
    Right,
}
impl From<Token> for OperatorAssociativity {
    fn from(token: Token) -> Self {
        match token {
            Token::Operator(Operator::USub) => OperatorAssociativity::Right,
            _ => OperatorAssociativity::Left,
        }
    }
}
impl<T> From<&T> for OperatorAssociativity
where
    T: Clone,
    OperatorAssociativity: From<T>,
{
    fn from(v: &T) -> Self {
        Self::from(v.clone())
    }
}

/// Takes an infix notated token stream and converts it to postfix notation
pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    // /*DEBUG:*/ eprintln!("Begin reverse poilsh conversion");
    let mut output: Vec<Token> = Vec::new();
    let mut opstack: Vec<Token> = Vec::new();

    for token in tokens {
        // /*DEBUG:*/ eprintln!("\nCurrent state:\n\tOperator stack: {:?}\n\tOutput: {:?}", opstack, output);
        // /*DEBUG:*/ eprint!("Encountered {:?} -> ", token);
        match token {
            Token::Value(_v) => {
                // /*DEBUG:*/ eprintln!("pushing token with value {} to the output", _v);
                output.push(token);
            }
            Token::Operator(_op) => {
                let p = precedence(&token);
                // /*DEBUG:*/ eprintln!("Operator {:?} -> Popping tokens from stack: ", _op);
                while !opstack.is_empty() {
                    match opstack.last() {
                        Some(&Token::Paren(_)) => {
                            // /*DEBUG:*/ eprintln!("\tEncountered paren, breaking");
                            break;
                        }
                        Some(o) => {
                            // /*DEBUG:*/ eprint!("\tEncountered operator {} -> ", o);
                            if match OperatorAssociativity::from(&token) {
                                OperatorAssociativity::Left => {
                                    // /*DEBUG:*/ eprint!("looking for precedence({}) < {}...", o, p);
                                    precedence(o) < p
                                }
                                OperatorAssociativity::Right => {
                                    // /*DEBUG:*/ eprint!("looking for precedence({}) <= {}...", o, p);
                                    precedence(o) <= p
                                }
                            } {
                                // /*DEBUG:*/ eprintln!("Found! Breaking");
                                break;
                            } else {
                                // /*DEBUG:*/ eprintln!("Not found, popping operator from the stack to the output");
                                output.push(opstack.pop().unwrap());
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                opstack.push(token.clone());
            }
            Token::Paren(p) => {
                // /*DEBUG:*/ eprint!("Encountered paren -> ");
                match p {
                    Paren::Left => {
                        // /*DEBUG:*/ eprintln!("Left paren, push to operator stack");
                        opstack.push(token.clone())
                    }
                    Paren::Right => {
                        // /*DEBUG:*/ eprintln!("Right paren, popping operator stack to output until we see a left paren");
                        while !opstack.is_empty() {
                            if let Some(top) = opstack.pop() {
                                match top {
                                    Token::Paren(Paren::Left) => {
                                        // /*DEBUG:*/ eprintln!("Encountered left paren, breaking");
                                        break;
                                    }
                                    o => {
                                        // /*DEBUG:*/ eprintln!("\tpopping {} to the output", o);
                                        output.push(o)
                                    }
                                }
                            } else {
                                unreachable!()
                            }
                        }
                    }
                }
            }
        }
    }

    // /*DEBUG:*/ eprintln!("Clearing operator stack");
    while let Some(top) = opstack.pop() {
        // /*DEBUG:*/ eprintln!("Popping {} to output", top);
        output.push(top);
    }

    // /*DEBUG:*/ eprintln!("\nEnd reverse poilsh conversion\n");

    output
}

#[test]
fn test_tokenize() {
    // Basic
    let tokens = vec![
        Token::new("("),
        Token::new("10"),
        Token::new("+"),
        Token::new("5"),
        Token::new(")"),
    ];
    assert!(tokens == tokenize("(10+5)"));

    // Complex
    let tokens = vec![
        Token::new("("),
        Token::new("("),
        Token::new("10"),
        Token::new("*"),
        Token::new("2"),
        Token::new(")"),
        Token::new("/"),
        Token::new("4"),
        Token::new("+"),
        Token::new("("),
        Token::new("2"),
        Token::new("*"),
        Token::new("4"),
        Token::new(")"),
        Token::new("*"),
        Token::new("2"),
        Token::new(")"),
    ];
    assert!(tokens == tokenize("((10 * 2) / 4 + (2 * 4) * 2)"));

    // No parens
    let tokens = vec![Token::new("10"), Token::new("+"), Token::new("5")];
    assert!(tokens == tokenize("10 + 5"));

    // Unary minus
    let tokens = vec![
        Token::new("u"),
        Token::new("10"),
        Token::new("+"),
        Token::new("u"),
        Token::new("5"),
    ];
    assert!(tokens == tokenize("-10 + -5"));
}

#[test]
fn test_shunting_yard() {
    let tokens = tokenize("((15 / (7 -(1 + 1))) * 3) - (2 + (1 + 1))");
    let expected = vec![
        Token::new("15"),
        Token::new("7"),
        Token::new("1"),
        Token::new("1"),
        Token::new("+"),
        Token::new("-"),
        Token::new("/"),
        Token::new("3"),
        Token::new("*"),
        Token::new("2"),
        Token::new("1"),
        Token::new("1"),
        Token::new("+"),
        Token::new("+"),
        Token::new("-"),
    ];

    assert_eq!(shunting_yard(tokens), expected);

    // unary minus
    let tokens = tokenize("-10 + 5");
    let expected = vec![
        Token::new("10"),
        Token::new("u"),
        Token::new("5"),
        Token::new("+"),
    ];
    assert_eq!(shunting_yard(tokens), expected);
}

#[test]
fn test_operator_evaluate() {
    assert_eq!(
        Operator::Add.evaluate(1.into(), 10.into()),
        Value::from(1 + 10)
    );
    assert_eq!(
        Operator::Add.evaluate(15.into(), 15.into()),
        Value::from(15 + 15)
    );
    assert_eq!(
        Operator::Add.evaluate(10.into(), 20.into()),
        Value::from(10 + 20)
    );

    assert_eq!(
        Operator::Sub.evaluate(1.into(), 10.into()),
        Value::from(1 - 10)
    );
    assert_eq!(
        Operator::Sub.evaluate(15.into(), 15.into()),
        Value::from(15 - 15)
    );
    assert_eq!(
        Operator::Sub.evaluate(10.into(), 20.into()),
        Value::from(10 - 20)
    );

    assert_eq!(
        Operator::Mul.evaluate(1.into(), 10.into()),
        Value::from(1 * 10)
    );
    assert_eq!(
        Operator::Mul.evaluate(15.into(), 15.into()),
        Value::from(15 * 15)
    );
    assert_eq!(
        Operator::Mul.evaluate(10.into(), 20.into()),
        Value::from(10 * 20)
    );

    assert_eq!(
        Operator::Div.evaluate(1.into(), 10.into()),
        Value::from(1. / 10.)
    );
    assert_eq!(Operator::Div.evaluate(15.into(), 15.into()), Value::from(1));
    assert_eq!(
        Operator::Div.evaluate(10.into(), 20.into()),
        Value::from(10. / 20.).simplify()
    );

    assert_eq!(
        Operator::USub.evaluate(0.into(), 10.into()),
        Value::from(-10)
    );
    assert_eq!(
        Operator::USub.evaluate(0.into(), 15.into()),
        Value::from(-15)
    );
    assert_eq!(
        Operator::USub.evaluate(0.into(), 10.into()),
        Value::from(-10)
    );
}
