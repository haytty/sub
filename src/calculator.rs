use rust_decimal::Decimal;

pub struct Calculator {
    subend1: Decimal,
    subend2: Decimal,
}

impl Calculator {
    pub fn new(subend1: Decimal, subend2: Decimal) -> Self {
        Self {
            subend1,
            subend2,
        }
    }

    pub fn calc(&self) -> Decimal {
        self.subend1 - self.subend2
    }
}
