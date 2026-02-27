#![allow(unused)]


#[derive(Debug)]
enum MathError {
    DivByZero,
}

impl std::error::Error for MathError {}
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "math error {:?}", self)
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidInt,
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error {:?}", self)
    }
}

fn f1() -> Result<u32, MathError> {
    Err(MathError::DivByZero)
}

fn f2() -> Result<u32, ParseError> {
    Err(ParseError::InvalidInt)
}

fn f3() -> Result<(), Box<dyn std::error::Error>> {
    f1()?;
    f2()?;
    Ok(())
}

fn main() {}