#[allow(unused)]
#[derive(Debug, Default, Clone)]
pub struct PostfixNotationCalculator {
    stack: Vec<i32>,
}

impl PostfixNotationCalculator {
    pub fn push(&mut self, n: i32) {
        self.stack.push(n);
    }

    pub fn stack(&self) -> &[i32] {
        self.stack.as_slice()
    }

    pub fn add(&mut self) {
        if let Some(n) = self.take(2).map(|v| v.iter().sum()) {
            self.push(n)
        }
    }

    pub fn subtract(&mut self) {
        let Some(numbers) = self.take(2) else {
            return;
        };

        self.push(numbers[0] - numbers[1])
    }

    pub fn multiply(&mut self) {
        let Some(numbers) = self.take(2) else {
            return;
        };

        self.push(numbers[0] * numbers[1])
    }

    pub fn divide(&mut self) -> anyhow::Result<()> {
        let Some(numbers) = self.take(2) else {
            return Ok(());
        };

        let res = numbers[0]
            .checked_div(numbers[1])
            .ok_or(anyhow::anyhow!(""))?;

        self.push(res);
        Ok(())
    }

    fn take(&mut self, n: usize) -> Option<Vec<i32>> {
        if n > self.stack.len() {
            return None;
        }

        Some(self.stack.split_off(self.stack.len() - n))
    }
}
