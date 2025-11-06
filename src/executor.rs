use std::process::{Command, ExitStatus};

pub fn execute_command(executable: &str, args: Vec<String>) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    let mut child = Command::new(executable)
        .args(&args)
        .spawn()?;

    let status = child.wait()?;
    Ok(status)
}