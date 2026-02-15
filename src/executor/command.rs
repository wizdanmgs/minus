use std::fs::File;
use std::process::{Command, Stdio};

use crate::builtins;
use crate::error::ShellError;
use crate::parser::ast::*;
use crate::shell::ShellSignal;

pub fn execute_simple(cmd: SimpleCommand) -> Result<ShellSignal, ShellError> {
    // Check builtin first
    if let Some(signal) = builtins::handle(&cmd)? {
        return Ok(signal);
    }

    let mut command = Command::new(&cmd.program);
    command.args(&cmd.args);

    for redir in cmd.redirections {
        match redir {
            Redirection::Stdout(file) => {
                command.stdout(Stdio::from(File::create(file)?));
            }
            Redirection::Stdin(file) => {
                command.stdin(Stdio::from(File::open(file)?));
            }
        }
    }

    let status = command.status()?;

    if !status.success() {
        return Err(ShellError::Exec(format!(
            "Exited with code {:?}",
            status.code()
        )));
    }

    Ok(ShellSignal::Continue)
}
