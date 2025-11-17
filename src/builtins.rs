use std::env;
use std::fs;
use std::io::{Read, Write};
use crate::error::BuiltinError;


// TODO: 优化错误处理
pub fn builtin_cd(args: Vec<String>, _stdin: &mut dyn Read, _stdout: &mut dyn Write)-> Result<(), Box<dyn std::error::Error>>{
    let target_dir = match args.first() {
        Some(path) => path.clone(),
        // 默认移动到HOME路径
        None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    };
    env::set_current_dir(target_dir)?;

    Ok(())
}

pub fn builtin_pwd(_args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write)-> Result<(), Box<dyn std::error::Error>>{
    let path = env::current_dir()?;
    writeln!(stdout, "{}", path.display())?;

    Ok(())
}

pub fn builtin_echo(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>>{
    let output = args.join(" ");
    writeln!(stdout, "{}", output)?;

    Ok(())
}

pub fn builtin_ls(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>>{
    let obj_path = match args.first() {
        Some(path) => path.clone(),
        None => ".".into(),
    };

    let paths = fs::read_dir(obj_path.as_str())?;

    for path in paths{
        writeln!(stdout, "{}", path.unwrap().path().display())?;
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
        writeln!(stdout, "{}\n", line)?;
    }

    Ok(())
}


pub fn read_from_pipe(args: & mut Vec<String>, _stdin: &mut dyn Read){
    let mut buffer = String::new();
    if _stdin.read_to_string(&mut buffer).is_ok() {
        if !buffer.is_empty() {
            args.push(buffer);
        }
    }
}