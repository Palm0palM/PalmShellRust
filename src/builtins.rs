use std::env;
use std::io::{self, Read, Write};

pub fn handle_builtin(
    cmd: &str,
    args: &[String],
    stdin: &mut dyn Read,
    stdout: &mut dyn Write,
) -> bool {
    let result = match cmd {
        "cd" => builtin_cd(args, stdin, stdout),
        "pwd" => builtin_pwd(args, stdin, stdout),
        "echo" => builtin_echo(args, stdin, stdout),
        _ => return false,
    };

    if let Err(e) = result {
        eprintln!("psh: {}: {}", cmd, e);
        return false;
    }

    true
}

// cd command
fn builtin_cd(args: &[String], _stdin: &mut dyn Read, _stdout: &mut dyn Write) -> io::Result<()> {
    let target_dir = match args.first() {
        Some(path) => path.clone(),
        // 默认移动到HOME路径
        None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    };
    env::set_current_dir(target_dir)
}

// pwd command
fn builtin_pwd(_args: &[String], _stdin: &mut dyn Read, stdout: &mut dyn Write) -> io::Result<()> {
    let path = env::current_dir()?;
    writeln!(stdout, "{}", path.display())
}

// echo command
fn builtin_echo(args: &[String], _stdin: &mut dyn Read, stdout: &mut dyn Write) -> io::Result<()> {
    let mut output = args.join(" ");

    // echo会试图从stdin中读取内容
    let mut buffer = String::new();
    if _stdin.read_to_string(&mut buffer).is_ok() {
        if !buffer.is_empty() {
            output.push(' ');
            output.push_str(&buffer.trim());
        }
    }

    writeln!(stdout, "{}", output)
}
