use std::env;

use crate::error::ShellError;

pub fn run() -> Result<(), ShellError> {
    let dir = env::current_dir().map_err(|e| ShellError::Builtin(format!("pwd: {}", e)))?;
    println!("{}", dir.display());
    Ok(())
}
