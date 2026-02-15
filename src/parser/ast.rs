#[derive(Debug, Clone)]
pub enum Redirection {
    Stdout(String),
    Stdin(String),
}

#[derive(Debug, Clone)]
pub struct SimpleCommand {
    pub program: String,
    pub args: Vec<String>,
    pub redirections: Vec<Redirection>,
}

#[derive(Debug, Clone)]
pub enum Command {
    Simple(SimpleCommand),
    Pipeline(Vec<SimpleCommand>),
}
