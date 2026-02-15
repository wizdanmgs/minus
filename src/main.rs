mod shell;
mod repl;
mod parser;
mod executor;
mod builtins;
mod error;

fn main() -> std::io::Result<()> {
    repl::start()
}
