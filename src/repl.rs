use crate::shell;
use std::io::{self, Write};

pub fn start() -> io::Result<()> {
    loop {
        print!("minus> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match shell::run(input) {
            Ok(shell::ShellSignal::Continue) => {}
            Ok(shell::ShellSignal::Exit(code)) => {
                println!("Exiting shell...");
                std::process::exit(code);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
