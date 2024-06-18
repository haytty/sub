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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_integer_calculation() {
        let decimal1 = Decimal::from_str("5").unwrap();
        let decimal2 = Decimal::from_str("3").unwrap();
        let calculator = Calculator::new(decimal1, decimal2);

        assert_eq!(Decimal::from_str("2").unwrap(), calculator.calc());
    }

    #[test]
    fn test_float_calculation() {
        let decimal1 = Decimal::from_str("5.7").unwrap();
        let decimal2 = Decimal::from_str("3.5").unwrap();
        let calculator = Calculator::new(decimal1, decimal2);

        assert_eq!(Decimal::from_str("2.2").unwrap(), calculator.calc());
    }

    #[test]
    fn test_negative_integer_result() {
        let decimal1 = Decimal::from_str("3").unwrap();
        let decimal2 = Decimal::from_str("5").unwrap();
        let calculator = Calculator::new(decimal1, decimal2);

        assert_eq!(Decimal::from_str("-2").unwrap(), calculator.calc());
    }

    #[test]
    fn test_negative_float_result() {
        let decimal1 = Decimal::from_str("3.7").unwrap();
        let decimal2 = Decimal::from_str("5.5").unwrap();
        let calculator = Calculator::new(decimal1, decimal2);

        assert_eq!(Decimal::from_str("-1.8").unwrap(), calculator.calc());
    }
}