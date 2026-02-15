mod command;
mod pipeline;

use crate::error::ShellError;
use crate::parser::ast::Command;
use crate::shell::ShellSignal;

pub fn execute(cmd: Command) -> Result<ShellSignal, ShellError> {
    match cmd {
        Command::Simple(simple) => command::execute_simple(simple),
        Command::Pipeline(stages) => pipeline::execute_pipeline(stages),
    }
}
