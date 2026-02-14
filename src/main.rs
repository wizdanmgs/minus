use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Entry point of the shell.
/// Starts the REPL loop.
fn main() -> io::Result<()> {
    repl()
}

/// REPL = Read → Eval → Print → Loop
///
/// This is the core loop of every shell:
/// 1. Print prompt
/// 2. Read user input
/// 3. Parse it
/// 4. Execute it
/// 5. Repeat forever
fn repl() -> io::Result<()> {
    loop {
        // Print prompt
        print!("mini-shell> ");
        io::stdout().flush()?; // Flush is required because print! does not auto-flush

        // Read input from stdin
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Remove trailing newline and whitespace
        let input = input.trim();

        // Skip empty input
        if input.is_empty() {
            continue;
        }

        // Execute command safely
        if let Err(e) = run_command(input) {
            eprintln!("Error: {}", e);
        }
    }
}

/// Main command dispatcher.
/// Decides whether:
/// - Builtin
/// - Pipeline
/// - Redirection
/// - Simple command
fn run_command(input: &str) -> Result<(), String> {
    // Convert string into tokens
    let tokens = tokenize(input);

    // Split tokens into pipeline segments
    let pipeline = parse_pipeline(&tokens);

    // If we have multiple pipeline stages, execute as pipe
    if pipeline.len() > 1 {
        execute_pipeline(pipeline)?;
    } else {
        let cmd = &pipeline[0];

        // Try built-in command first
        if handle_builtin(cmd)? {
            return Ok(());
        }

        // Check for redirection symbols
        if cmd.contains(&">".to_string()) || cmd.contains(&"<".to_string()) {
            handle_redirection(cmd)?;
        } else {
            // Normal external command
            execute_simple(&cmd[0], &cmd[1..])?;
        }
    }

    Ok(())
}

/// Splits input into tokens.
///
/// Example:
/// "ls | grep txt > out.txt"
///
/// Becomes:
/// ["ls", "|", "grep", "txt", ">", "out.txt"]
///
/// We manually add spaces around special operators
/// to ensure they are split properly.
fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("|", " | ")
        .replace(">", " > ")
        .replace("<", " < ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

/// Splits tokens into pipeline stages.
///
/// Example:
/// ["ls", "|", "grep", "txt"]
///
/// Becomes:
/// [
///   ["ls"],
///   ["grep", "txt"]
/// ]
///
/// Each inner Vec<String> is a command stage.
fn parse_pipeline(tokens: &[String]) -> Vec<Vec<String>> {
    let mut pipeline = Vec::new();
    let mut current = Vec::new();

    for token in tokens {
        if token == "|" {
            pipeline.push(current);
            current = Vec::new();
        } else {
            current.push(token.clone());
        }
    }

    if !current.is_empty() {
        pipeline.push(current);
    }

    pipeline
}

/// Executes a simple command (no pipe, no redirection).
///
/// Uses std::process::Command which internally:
/// - On Unix calls fork()
/// - Then calls execvp()
///
/// Rust abstracts that complexity away.
fn execute_simple(cmd: &str, args: &[String]) -> Result<(), String> {
    let status = Command::new(cmd)
        .args(args)
        .status() // Wait for process to finish
        .map_err(|e| e.to_string())?;

    // Check exit code
    if !status.success() {
        return Err(format!("Command exited with {:?}", status.code()));
    }

    Ok(())
}

/// Executes a pipeline of commands.
///
/// Example:
/// cat file.txt | grep hello | sort
///
/// What happens internally:
/// - Create pipe1
/// - fork cat
/// - redirect stdout -> pipe1 write end
///
/// - Create pipe2
/// - fork grep
/// - stdin <- pipe1 read end
/// - stdout -> pipe2 write end
///
/// - fork sort
/// - stdin <- pipe2 read end
///
/// Then wait for all processes.
fn execute_pipeline(commands: Vec<Vec<String>>) -> Result<(), String> {
    let mut previous_stdout = None;
    let mut children = Vec::new();

    for (i, cmd) in commands.iter().enumerate() {
        let mut command = Command::new(&cmd[0]);
        command.args(&cmd[1..]);

        // If this is NOT the first command,
        // connect stdin to previous pipe's stdout
        if let Some(stdin) = previous_stdout.take() {
            command.stdin(stdin);
        }

        // If this is NOT the last command,
        // create a new pipe for stdout
        if i < commands.len() - 1 {
            command.stdout(Stdio::piped());
        }

        // Spawn child process
        let mut child = command.spawn().map_err(|e| e.to_string())?;

        // Save its stdout for next stage
        previous_stdout = child.stdout.take().map(Stdio::from);

        children.push(child);
    }

    // Wait for all children to finish
    for mut child in children {
        child.wait().map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Handles input/output redirection.
///
/// Example:
/// ls > out.txt
/// cat < input.txt
///
/// Redirection works by replacing file descriptors:
/// STDIN  (fd 0)
/// STDOUT (fd 1)
///
/// Rust does this via Stdio::from(File).
fn handle_redirection(cmd: &[String]) -> Result<(), String> {
    let mut command = Command::new(&cmd[0]);
    let mut args = Vec::new();

    let mut i = 1;
    while i < cmd.len() {
        match cmd[i].as_str() {
            ">" => {
                // Redirect stdout to file (truncate)
                let file = File::create(&cmd[i + 1]).map_err(|e| e.to_string())?;
                command.stdout(Stdio::from(file));
                i += 2;
            }
            "<" => {
                // Redirect stdin from file
                let file = File::open(&cmd[i + 1]).map_err(|e| e.to_string())?;
                command.stdin(Stdio::from(file));
                i += 2;
            }
            _ => {
                args.push(cmd[i].clone());
                i += 1;
            }
        }
    }

    command.args(args);

    command.status().map_err(|e| e.to_string())?;

    Ok(())
}

/// Handles built-in commands.
///
/// Builtins must run inside the shell process,
/// not in a child process.
///
/// Why?
/// Because `cd` must change THIS process directory.
/// If executed in child, parent directory stays same.
fn handle_builtin(cmd: &[String]) -> Result<bool, String> {
    match cmd[0].as_str() {
        "cd" => {
            let path = cmd.get(1).ok_or("Missing path")?;
            env::set_current_dir(path).map_err(|e| e.to_string())?;
            Ok(true)
        }
        "pwd" => {
            let dir = env::current_dir().map_err(|e| e.to_string())?;
            println!("{}", dir.display());
            Ok(true)
        }
        "exit" => {
            println!("Exiting shell...");
            std::process::exit(0);
        }
        _ => Ok(false),
    }
}
