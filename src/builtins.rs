use std::env;
use std::fs;
use std::io::Write;
use std::str::FromStr;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use crate::error::ShellError;
use crate::model_call::llm_call;
use crate::prompt;

pub fn builtin_cd(args: Vec<String>, _piped_input: Option<String>, _stdout: &mut dyn Write) -> Result<(), ShellError> {
    let target_dir = match args.first() {
        Some(path) => path.clone(),
        // 默认移动到HOME路径
        None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    };
    env::set_current_dir(target_dir)?;

    Ok(())
}

pub fn builtin_pwd(_args: Vec<String>, _piped_input: Option<String>, stdout: &mut dyn Write) -> Result<(), ShellError> {
    let path = env::current_dir()?;
    writeln!(stdout, "{}", path.display())?;

    Ok(())
}

pub fn builtin_echo(args: Vec<String>, piped_input: Option<String>, stdout: &mut dyn Write) -> Result<(), ShellError> {
    // 如果有管道输入，将其附加到 args 后面
    let mut parts = args;
    if let Some(input) = piped_input {
        parts.push(input.trim_end().to_string());
    }

    let output = parts.join(" ");
    writeln!(stdout, "{}", output)?;

    Ok(())
}

pub fn builtin_grep(mut args: Vec<String>, piped_input: Option<String>, stdout: &mut dyn Write) -> Result<(), ShellError> {
    // 获取搜索模式
    let pattern = match args.first() {
        Some(p) => p.clone(),
        None => return Err(ShellError::BuiltinError("grep requires a pattern".to_string())),
    };
    args.remove(0);

    // 确定搜索内容来源
    let content = if let Some(input) = piped_input {
        // 优先使用管道输入
        input
    } else if !args.is_empty() {
        // 否则将剩余 args 当作内容（或文件路径）
        args.join(" ")
    } else {
        return Err(ShellError::BuiltinError("grep requires input (from pipe or arguments)".to_string()));
    };

    // 执行搜索
    for line in content.lines() {
        if line.contains(&pattern) {
            writeln!(stdout, "{}", line)?;
        }
    }

    Ok(())
}

pub fn builtin_model_call(args: Vec<String>, _piped_input: Option<String>, stdout: &mut dyn Write) -> Result<(), ShellError> {
    if args.is_empty() {
        return Err(ShellError::BuiltinError("chat requires a message".to_string()));
    }

    writeln!(stdout, "\n{} Thinking...", prompt::get_emoji())?;

    let rt = tokio::runtime::Runtime::new()?;

    let response = rt.block_on(llm_call(args.join(" ")))?;
    writeln!(stdout, "{}", response)?;

    Ok(())
}

pub fn builtin_kill(args: Vec<String>, _piped_input: Option<String>, _stdout: &mut dyn Write) -> Result<(), ShellError> {
    if args.is_empty() {
        return Err(ShellError::BuiltinError("kill usage: kill [-s signal] pid".to_string()));
    }

    let mut signal = Signal::SIGTERM;
    let mut pid_str = None;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "-s" {
             if let Some(sig_arg) = iter.next() {
                 signal = parse_signal(sig_arg)?;
             } else {
                 return Err(ShellError::BuiltinError("kill: option requires an argument -- 's'".to_string()));
             }
        } else {
            if pid_str.is_some() {
                 return Err(ShellError::BuiltinError("kill: too many arguments".to_string()));
            }
            pid_str = Some(arg);
        }
    }

    let pid_str = pid_str.ok_or_else(|| ShellError::BuiltinError("kill: usage: kill [-s signal] pid".to_string()))?;
    let pid_num = pid_str.parse::<i32>().map_err(|_| ShellError::BuiltinError(format!("kill: illegal pid: {}", pid_str)))?;
    let pid = Pid::from_raw(pid_num);

    match signal::kill(pid, signal) {
        Ok(_) => Ok(()),
        Err(e) => Err(ShellError::BuiltinError(format!("kill: {}", e))),
    }
}

pub fn builtin_export(args: Vec<String>, _piped_input: Option<String>, _stdout: &mut dyn Write) -> Result<(), ShellError> {
    for arg in args {
        // 处理 KEY=VALUE 格式
        if let Some((key, value)) = arg.split_once('=') {
            if !key.is_empty() {
                unsafe{ env::set_var(key, value); }
            }
        }
    }
    Ok(())
}

pub fn builtin_env(_args: Vec<String>, _piped_input: Option<String>, stdout: &mut dyn Write) -> Result<(), ShellError> {
    for (key, value) in env::vars() {
        writeln!(stdout, "{}={}", key, value)?;
    }
    Ok(())
}

fn parse_signal(sig_str: &str) -> Result<Signal, ShellError> {
    // 数字的情况
    if let Ok(num) = sig_str.parse::<i32>() {
        return Signal::try_from(num).map_err(|_| ShellError::BuiltinError(format!("kill: invalid signal number: {}", num)));
    }

    // 信号全名的情况
    let upper = sig_str.to_uppercase();
    if let Ok(s) = Signal::from_str(&upper) {
        return Ok(s);
    }

    // 信号后缀的情况
    if !upper.starts_with("SIG") {
        let with_sig = format!("SIG{}", upper);
        if let Ok(s) = Signal::from_str(&with_sig) {
            return Ok(s);
        }
    }

    Err(ShellError::BuiltinError(format!("kill: unknown signal: {}", sig_str)))
}
