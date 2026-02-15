pub mod ast;
mod lexer;

use crate::error::ShellError;
use ast::*;

pub fn parse(input: &str) -> Result<Command, ShellError> {
    let tokens = lexer::tokenize(input);

    if tokens.contains(&"|".to_string()) {
        parse_pipeline(tokens)
    } else {
        Ok(Command::Simple(parse_simple(tokens)?))
    }
}

fn parse_pipeline(tokens: Vec<String>) -> Result<Command, ShellError> {
    let mut segments = Vec::new();
    let mut current = Vec::new();

    for token in tokens {
        if token == "|" {
            segments.push(parse_simple(current)?);
            current = Vec::new();
        } else {
            current.push(token);
        }
    }

    segments.push(parse_simple(current)?);

    Ok(Command::Pipeline(segments))
}

fn parse_simple(tokens: Vec<String>) -> Result<SimpleCommand, ShellError> {
    if tokens.is_empty() {
        return Err(ShellError::Parse("Empty command".into()));
    }

    let program = tokens[0].clone();
    let mut args = Vec::new();
    let mut redirections = Vec::new();

    let mut i = 1;

    while i < tokens.len() {
        match tokens[i].as_str() {
            ">" => {
                redirections.push(Redirection::Stdout(tokens[i + 1].clone()));
                i += 2;
            }
            "<" => {
                redirections.push(Redirection::Stdin(tokens[i + 1].clone()));
                i += 2;
            }
            _ => {
                args.push(tokens[i].clone());
                i += 1;
            }
        }
    }

    Ok(SimpleCommand {
        program,
        args,
        redirections,
    })
}
