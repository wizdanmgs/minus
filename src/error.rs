use std::fmt;

#[derive(Debug)]
pub enum ShellError {
    Io(std::io::Error),
    Parse(String),
    Exec(String),
    Builtin(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Io(e) => write!(f, "IO error: {}", e),
            ShellError::Parse(e) => write!(f, "Parse error: {}", e),
            ShellError::Exec(e) => write!(f, "Execution error: {}", e),
            ShellError::Builtin(e) => write!(f, "Builtin error: {}", e),
        }
    }
}

impl From<std::io::Error> for ShellError {
    fn from(e: std::io::Error) -> Self {
        ShellError::Io(e)
    }
}

