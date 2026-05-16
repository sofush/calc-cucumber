use crate::{PostfixNotationCalculator, parser::Token};
use anyhow::bail;

#[derive(Default, Debug)]
pub struct Interpreter {
    calc: PostfixNotationCalculator,
}

impl Interpreter {
    pub fn interpret(&mut self, token: &Token) -> anyhow::Result<()> {
        match token {
            Token::Number(n) => self.calc.push(*n),
            Token::Add => self.calc.add(),
            Token::Subtract => self.calc.subtract(),
            Token::Divide => self.calc.divide()?,
            Token::Multiply => self.calc.multiply(),
            Token::Unknown(s) => bail!("Unknown token `{s}`."),
        };

        Ok(())
    }

    pub fn result(&self) -> Vec<i32> {
        self.calc.stack.clone()
    }
}
