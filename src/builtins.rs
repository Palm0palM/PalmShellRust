use std::env;
use std::fs;
use std::io::Write;
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

pub fn builtin_ls(args: Vec<String>, _piped_input: Option<String>, stdout: &mut dyn Write) -> Result<(), ShellError> {
    let obj_path = match args.first() {
        Some(path) => path.clone(),
        None => ".".into(),
    };

    let paths = fs::read_dir(obj_path.as_str())?;

    for path in paths {
        writeln!(stdout, "{}", path.unwrap().path().display())?;
    }

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