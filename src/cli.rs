use std::str::FromStr;
use clap::error::ErrorKind;
use clap::{Error, Parser};
use rust_decimal::Decimal;

use crate::calculator::Calculator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "help message")]
    subend1: String,

    #[arg(help = "help message")]
    subend2: String,
}

pub fn start() -> Result<(), Error> {
    let args = Args::parse();

    let a = Decimal::from_str(&args.subend1).map_err(|_| clap::Error::new(ErrorKind::InvalidValue))?;
    let b = Decimal::from_str(&args.subend2).map_err(|_| clap::Error::new(ErrorKind::InvalidValue))?;

    let calculator = Calculator::new(a, b);
    println!("{}", calculator.calc());

    Ok(())
}