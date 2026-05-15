pub enum Token {
    Number(i32),
    Add,
    Subtract,
    Divide,
    Multiply,
    Unknown(String),
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
