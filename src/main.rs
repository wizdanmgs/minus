use std::process::Command;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        print!("smash> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        let mut parts  = input.split_whitespace();

        if input == "exit" {
            break;
        }

        let cmd = match parts.next() {
            Some(c) => c,
            None => continue,
        };

        let args: Vec<&str> = parts.collect();

        if let Err(e) = run_command(cmd, &args) {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}

fn run_command(cmd: &str, args: &[&str]) -> Result<(), String>  {
    let status = Command::new(cmd).args(args).status().map_err(|e| e.to_string())?;

    if !status.success() {
        Err(format!("Command {} failed: {}", cmd, status))
    } else {
        Ok(())
    }

}
