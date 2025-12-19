use std::process::{Child, Command, Stdio};
use crate::error::ShellError;

pub fn execute(
    executable: &str,
    args: Vec<String>,
    stdin: Stdio,
    stdout: Stdio,
) -> Result<Child, ShellError> {
    Command::new(executable)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn()
        .map_err(|e| ShellError::ExecuteError(e.to_string()))
}