mod cd;
mod exit;
mod pwd;

use crate::error::ShellError;
use crate::parser::ast::SimpleCommand;
use crate::shell::ShellSignal;

pub fn handle(cmd: &SimpleCommand) -> Result<Option<ShellSignal>, ShellError> {
    match cmd.program.as_str() {
        "cd" => {
            cd::run(cmd)?;
            Ok(Some(ShellSignal::Continue))
        }
        "pwd" => {
            pwd::run()?;
            Ok(Some(ShellSignal::Continue))
        }
        "exit" => {
            let signal = exit::run(cmd)?;
            Ok(Some(signal))
        }
        _ => Ok(None),
    }
}
