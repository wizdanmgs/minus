use crate::parser::ast::*;
use crate::{error::ShellError, shell::ShellSignal};
use std::process::{Command, Stdio};

pub fn execute_pipeline(stages: Vec<SimpleCommand>) -> Result<ShellSignal, ShellError> {
    let mut previous_stdout = None;
    let mut children = Vec::new();

    for (i, stage) in stages.iter().enumerate() {
        let mut command = Command::new(&stage.program);
        command.args(&stage.args);

        if let Some(stdin) = previous_stdout.take() {
            command.stdin(stdin);
        }

        if i < stages.len() - 1 {
            command.stdout(Stdio::piped());
        }

        let mut child = command.spawn()?;
        previous_stdout = child.stdout.take().map(Stdio::from);
        children.push(child);
    }

    for mut child in children {
        child.wait()?;
    }

    Ok(ShellSignal::Continue)
}
