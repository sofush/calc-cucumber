use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(i32),
    Add,
    Subtract,
    Divide,
    Multiply,
    Unknown(String),
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = match s {
            "Add" => Token::Add,
            "Subtract" => Token::Subtract,
            "Multiply" => Token::Multiply,
            "Divide" => Token::Divide,

            token if token.starts_with("Number(") && token.ends_with(')') => {
                let inner = &token[7..token.len() - 1];
                let num = inner.parse().unwrap();

                Token::Number(num)
            }

            token if token.starts_with("Unknown(") && token.ends_with(')') => {
                let inner = &token[8..token.len() - 1];
                Token::Unknown(inner.to_string())
            }
            _ => return Err(format!("Unknown expected token format: {s}")),
        };

        Ok(token)
    }
}

pub fn parse(s: &str) -> Vec<Token> {
    s.split_whitespace().map(interpret_token).collect()
}

fn interpret_token(s: &str) -> Token {
    use Token::*;

    match s {
        "+" => Add,
        "-" => Subtract,
        "/" => Divide,
        "*" => Multiply,
        num => {
            if let Ok(num) = num.parse() {
                Number(num)
            } else {
                Unknown(num.to_string())
            }
        }
    }
}
