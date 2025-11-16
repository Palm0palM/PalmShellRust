use std::env;
use std::fs;
use std::io::{Read, Write};

// TODO: 优化错误处理
pub fn builtin_cd(args: Vec<String>, _stdin: &mut dyn Read, _stdout: &mut dyn Write){
    let target_dir = match args.first() {
        Some(path) => path.clone(),
        // 默认移动到HOME路径
        None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    };
    env::set_current_dir(target_dir).unwrap();
}

pub fn builtin_pwd(_args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write){
    let path = env::current_dir().unwrap();
    writeln!(stdout, "{}", path.display()).unwrap();
}

pub fn builtin_echo(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write) {
    let output = args.join(" ");
    writeln!(stdout, "{}", output).unwrap();
}

pub fn builtin_echo_pipe(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write){
    let mut output = args.join(" ");

    // echo会试图从stdin中读取内容
    let mut buffer = String::new();
    if _stdin.read_to_string(&mut buffer).is_ok() {
        if !buffer.is_empty() {
            output.push_str(&buffer.trim());
        }
    }

    writeln!(stdout, "{}", output).unwrap();
}

pub fn builtin_ls(args: Vec<String>, _stdin: &mut dyn Read, stdout: &mut dyn Write){
    let obj_path = match args.first() {
        Some(path) => path.clone(),
        None => ".".into(),
    };

    let paths = fs::read_dir(obj_path.as_str()).unwrap();

    for path in paths{
        writeln!(stdout, "{}", path.unwrap().path().display()).unwrap();
    }
}