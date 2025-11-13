use std::env;
use std::thread;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod parser;
mod builtins;
mod executor;

use parser::{parse_line, Command};
use executor::{execute_command, execute_background_command};

fn main() {
    // 初始化Readline
    let mut reader =  DefaultEditor::new().unwrap();

    loop {
        let read_result = reader.readline(&get_prompt());

        match read_result {
            Ok(line) => {// 正常读取的情况
                reader.add_history_entry(line.as_str())
                    .expect("Failed to add history");

                match parse_line(&line) {
                    // 处理空行
                    Ok(Command::Empty) => {
                        continue;
                    }
                    // 退出
                    Ok(Command::Exit) => {
                        println!("Exiting...");
                        break;
                    }
                    // 处理内置命令
                    Ok(Command::Builtin(cmd, args)) => {
                        if !builtins::handle_builtin(&cmd, &args) {
                            eprintln!("Error executing builtin: {}", cmd);
                        }
                    }
                    // 处理外置命令
                    Ok(Command::External(program, args)) => {
                        if let Err(e) = execute_command(&program, args) {
                            eprintln!("Error executing command: {}", e);
                        }
                    }
                    // 处理后台命令
                    Ok(Command::Background(boxed_command)) => {
                        match *boxed_command {
                            Command::External(program, args) => {
                                match execute_background_command(&program, args) {
                                    Ok(child) => {
                                        println!("[1] {}", child.id());
                                    }
                                    Err(e) => {
                                        eprintln!("Error executing background command: {}", e);
                                    }
                                }
                            }
                            Command::Builtin(cmd, args) => {
                                thread::spawn(move || {
                                    if !builtins::handle_builtin(&cmd, &args) {
                                        eprintln!("Error executing builtin in background: {}", cmd);
                                    }
                                });
                            }
                            _ => {
                                // 实际上这行报错不可能运行到
                                eprintln!("my_shell: invalid background command");
                            }
                        }
                    }  
                    Err(e) => {
                        eprintln!("my_shell: parse error: {}", e);
                    }
                }
            }

            // 按下了Ctrl + C
            Err(ReadlineError::Interrupted) => {
                continue;
            }

            // 按下了Ctrl + D
            Err(ReadlineError::Eof) => {
                break;
            }
            
            // 其他未知错误
            Err(err) => {
                println!("Error: {:?}" , err);
                break;
            }
        }
    }
}

fn get_prompt() -> String {
    let username = whoami::username();

    // 获取主机名
    let hostname = whoami::fallible::hostname().unwrap_or("unknown_hostname".to_string());

    let username_at_hostname = (username + "@" +&hostname).on_blue();

    // 获取当前路径
    let current_dir_path = env::current_dir().unwrap_or_default();
    let current_dir_str = current_dir_path.to_str().unwrap_or("unknown_current_directory").to_string();

    // 把Home路径简写为~
    let display_dir = match env::var("HOME") {
        Ok(home_dir) => current_dir_str.replace(&home_dir, "~"),
        Err(_) => current_dir_str.to_string(),
    } .green();

    // 获取时间
    let now = chrono::Local::now();
    let time = now.format("%d/%m/%Y %H:%M").to_string().yellow();

    // 组合出Prompt
    // 用户名@主机名 当前路径 [日/月 小时/分] $
    format!(
        "{} {} [{}]\n$ ",
        username_at_hostname, display_dir, time
    )
}
