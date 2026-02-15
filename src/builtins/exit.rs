use crate::error::ShellError;
use crate::parser::ast::SimpleCommand;
use crate::shell::ShellSignal;

pub fn run(cmd: &SimpleCommand) -> Result<ShellSignal, ShellError> {
    // Support: exit <code>
    let code = match cmd.args.get(0) {
        Some(arg) => arg
            .parse::<i32>()
            .map_err(|_| ShellError::Builtin("exit: invalid exit code".into()))?,
        None => 0,
    };

    Ok(ShellSignal::Exit(code))
}
