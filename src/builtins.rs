use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{Read, Write};

#[derive(Debug)]
enum BuiltinError{
    ArgsLack(u32),
}
impl Display for BuiltinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinError::ArgsLack(num) => write!(f, "({} arguments lacked)", num)
        }?;
        Ok(())
    }
}
impl std::error::Error for BuiltinError{ }

// TODO: 优化错误处理
pub fn builtin_cd(args: Vec<String>, _stdin: &mut dyn Read, _stdout: &mut dyn Write)-> Result<(), Box<dyn std::error::Error>>{
    let target_dir = match args.first() {
        Some(path) => path.clone(),
        // 默认移动到HOME路径
        None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    };
    env::set_current_dir(target_dir).unwrap();

    Ok(())
}

pub fn builtin_pwd(_args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write)-> Result<(), Box<dyn std::error::Error>>{
    let path = env::current_dir().unwrap();
    writeln!(stdout, "{}", path.display()).unwrap();

    Ok(())
}

pub fn builtin_echo(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>>{
    let output = args.join(" ");
    writeln!(stdout, "{}", output).unwrap();

    Ok(())
}

pub fn builtin_echo_piped(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write)-> Result<(), Box<dyn std::error::Error>>{
    let mut output = args.join(" ");

    // echo会试图从stdin中读取内容
    let mut buffer = String::new();
    if _stdin.read_to_string(&mut buffer).is_ok() {
        if !buffer.is_empty() {
            output.push_str(&buffer.trim());
        }
    }

    writeln!(stdout, "{}", output).unwrap();

    Ok(())
}

pub fn builtin_ls(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>>{
    let obj_path = match args.first() {
        Some(path) => path.clone(),
        None => ".".into(),
    };

    let paths = fs::read_dir(obj_path.as_str()).unwrap();

    for path in paths{
        writeln!(stdout, "{}", path.unwrap().path().display()).unwrap();
    }

    Ok(())
}
pub fn builtin_grep(args: & mut Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write)-> Result<(), Box<dyn std::error::Error>>{
    if args.len() < 2{
        return Err(Box::new(BuiltinError::ArgsLack(2)));
    }
    let mut results = Vec::new();
    let query = args.remove(0);

    let contents = args.join(" ");

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    for line in results{
        writeln!(stdout, "{}\n", line).unwrap();
    }

    Ok(())
}

pub fn builtin_grep_piped(args: & mut Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write)-> Result<(), Box<dyn std::error::Error>>{
    if args.len() < 1{
        return Err(Box::new(BuiltinError::ArgsLack(1)));
    }
    
    let mut results = Vec::new();

    let query = args.remove(0);
    let mut contents = args.join(" ");
    
    let mut buffer = String::new();
    if _stdin.read_to_string(&mut buffer).is_ok() {
        if !buffer.is_empty() {
            contents.push_str(&buffer);
        }
    }
    
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    for line in results{
        writeln!(stdout, "{}\n", line).unwrap();
    }

    Ok(())
}