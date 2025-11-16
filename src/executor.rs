use std::process::{Child, Command, Stdio};

pub fn execute(
    executable: &str,
    args: Vec<String>,
    stdin: Stdio,
    stdout: Stdio,
) -> Result<Child, Box<dyn std::error::Error>> {
    Command::new(executable)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn()
        .map_err(Into::into)
}