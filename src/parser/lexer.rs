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
pub fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("|", " | ")
        .replace(">", " > ")
        .replace("<", " < ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
