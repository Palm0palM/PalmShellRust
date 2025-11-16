use std::env;
use std::io::{self, Read, Write};

// cd command
pub fn builtin_cd(args: Vec<String>, _stdin: &mut dyn Read, _stdout: &mut dyn Write) -> io::Result<()> {
    let target_dir = match args.first() {
        Some(path) => path.clone(),
        // 默认移动到HOME路径
        None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    };
    env::set_current_dir(target_dir)
}

// pwd command
pub fn builtin_pwd(_args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write) -> io::Result<()> {
    let path = env::current_dir()?;
    writeln!(stdout, "{}", path.display())
}

// echo command
pub fn builtin_echo(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write) -> io::Result<()> {
    let output = args.join(" ");
    writeln!(stdout, "{}", output)
}

pub fn builtin_echo_pipe(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write) -> io::Result<()> {
    let mut output = args.join(" ");

    // echo会试图从stdin中读取内容
    let mut buffer = String::new();
    if _stdin.read_to_string(&mut buffer).is_ok() {
        if !buffer.is_empty() {
            output.push_str(&buffer.trim());
        }
    }

    writeln!(stdout, "{}", output)
}