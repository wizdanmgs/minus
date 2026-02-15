# Minus — A Mini Unix-like Shell

A modular, mini Unix-like shell written in Rust.

---

## Features

### Command Execution

Execute external programs just like a real Unix shell.

```bash
minus> ls -la
```

#### Pipes (|)

```bash
minus> cat file.txt | grep hello | sort
```

Implements multi-stage pipelines using OS-level pipes.

#### Redirection (> and <)

```bash
minus> ls > out.txt
minus> cat < input.txt
```

Uses file descriptor replacement via Stdio.

#### Built-in Commands

```bash
minus> cd
minus> pwd
minus> exit
```

Built-ins are implemented without spawning child processes.

## Architecture

The shell is designed using a layered approach:

REPL
↓
Shell
↓
Parser (Lexer → AST)
↓
Executor (Simple | Pipeline)
↓
Builtins

### Project Structure

src/
├── main.rs
├── repl.rs
├── shell.rs
├── error.rs
├── parser/
│ ├── mod.rs
│ ├── lexer.rs
│ └── ast.rs
├── executor/
│ ├── mod.rs
│ ├── command.rs
│ └── pipeline.rs
└── builtins/
├── mod.rs
├── cd.rs
├── pwd.rs
└── exit.rs

## Getting Started

Clone the Repository

```bash
git clone https://github.com/yourusername/minishell.git
cd minus
```

Build

```bash
cargo build
cargo run
```

### Example Usage

```bash
minus> pwd
/home/user

minus> cd ..
minus> pwd
/home

minus> echo hello | grep h
hello

minus> exit 42
Exiting shell...
```
