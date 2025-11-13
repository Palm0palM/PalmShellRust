use std::process::{Command, Child, ExitStatus};

pub fn execute_command(executable: &str, args: Vec<String>) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    let mut child = Command::new(executable)
        .args(&args)
        .spawn()?;

    let status = child.wait()?;
    Ok(status)
}

pub fn execute_background_command(executable: &str, args: Vec<String>) -> Result<Child, Box<dyn std::error::Error>> {
    let child = Command::new(executable)
        .args(&args)
        .spawn()?;
    Ok(child)
}