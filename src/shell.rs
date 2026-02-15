use crate::error::ShellError;
use crate::{executor, parser};

#[derive(Debug)]
pub enum ShellSignal {
    Continue,
    Exit(i32),
}

pub fn run(input: &str) -> Result<ShellSignal, ShellError> {
    let ast = parser::parse(input)?;
    executor::execute(ast)
}
