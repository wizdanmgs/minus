use std::env;
use std::path::Path;

use crate::{error::ShellError, parser::ast::SimpleCommand};

pub fn run(cmd: &SimpleCommand) -> Result<(), ShellError> {
    let target = match cmd.args.get(0) {
        Some(path) if path == "~" => home_dir()?,
        Some(path) => path.clone(),
        None => home_dir()?,
    };

    env::set_current_dir(Path::new(&target))
        .map_err(|e| ShellError::Builtin(format!("cd: {}", e)))?;

    Ok(())
}

fn home_dir() -> Result<String, ShellError> {
    env::var("HOME").map_err(|_| ShellError::Builtin("HOME not set".into()))
}
