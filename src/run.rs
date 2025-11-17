use std::{env, thread};
use colored::Colorize;
use std::process::{Stdio, exit};
use std::io::{self, Read, Write};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::builtins;
use crate::executor::execute;
use crate::parser::{parse_line, Command};
use os_pipe::{pipe, PipeReader, PipeWriter};

// 主循环的功能是，不断接受输入调用handle_command解析命令，并处理Ctrl+C Ctrl+D
pub fn main_loop(mut reader: DefaultEditor) {
    loop {
        let read_result = reader.readline(&get_prompt());

        match read_result {
            Ok(line) => {
                reader.add_history_entry(line.as_str())
                    .expect("psh: failed to add history");
                handle_command(parse_line(&line), None, None);
            }

            // Ctrl + C
            // 默认行为为重新接收命令
            Err(ReadlineError::Interrupted) => continue,

            // Ctrl + D
            // 默认行为为退出程序
            Err(ReadlineError::Eof) => exit(0),
            Err(err) => {
                println!("psh: error when reading command: {:?}", err);
                break;
            }
        }
    }
}

// input 和 output 表示命令的输入输出流
// 如果默认用标准流输入输出（而不Pipe设置的流）的话，二者会被设置为None
fn handle_command(
    cmd: Result<Command, String>,
    input: Option<PipeReader>,
    output: Option<PipeWriter>,
) {
    match cmd {
        Ok(Command::Empty) => return,
        Ok(Command::Exit) => {
            println!("psh: exiting...");
            exit(0);
        }

        Ok(Command::Builtin(cmd, mut args)) => {
            let mut reader: Box<dyn Read> ;
            let is_piped;
            // 解析input和output。如果是None，map_or会返回指向默认的标准流句柄的指针
            match input {
                Some(pipe_reader) => {
                    reader= Box::new(pipe_reader);
                    is_piped = true;
                }
                None => {
                    reader = Box::new(io::stdin());
                    is_piped = false;
                }
            }

            let mut writer: Box<dyn Write> = output
                .map_or(Box::new(io::stdout()), |p| Box::new(p));


            let result = match cmd.as_str() {
                "cd" => builtins::builtin_cd(args, & mut (*reader), & mut (*writer)),
                "pwd" => builtins::builtin_pwd(args, & mut (*reader), & mut (*writer)),
                "echo" => {
                    if is_piped{
                        builtins::read_from_pipe(& mut args, & mut (*reader));
                        builtins::builtin_echo(args, & mut (*reader), & mut (*writer))
                    } else {
                        builtins::builtin_echo(args, & mut (*reader), & mut (*writer))
                    }
                },
                "ls" => builtins::builtin_ls(args, & mut (*reader), & mut (*writer)),
                "grep" => {
                    if is_piped{
                        builtins::read_from_pipe(& mut args, & mut (*reader));
                        builtins::builtin_grep(&mut args, & mut (*reader), & mut (*writer))
                    } else {
                        builtins::builtin_grep(&mut args, & mut (*reader), & mut (*writer))
                    }
                }
                _ => return,
            };

            match result {
                Ok(()) => return,
                Err(e) => println!("psh: error when executing builtin: {}", e),
            };
        }

        Ok(Command::External(program, args)) => {
            // 解析input和output。如果是None，map_or会父进程的io流，实际上就是Stdio
            let stdin = input.map_or(Stdio::inherit(), Stdio::from);
            let stdout = output.map_or(Stdio::inherit(), Stdio::from);

            if let Ok(mut child) = execute(&program, args, stdin, stdout) {
                // 默认行为是等待子进程完成
                if let Err(e) = child.wait() {
                    eprintln!("psh: failed to wait on process: {}", e);
                }
            } else {
                eprintln!("psh: error executing command: {}", program);
            }
        }
        Ok(Command::Background(boxed_command)) => {
            // 直接生成一个子进程递归调用handle_command但是不等待。
            // 如果内部的Command是External，那么子进程会生成另一个子进程用来执行命令。
            // 这实际上造成了进程冗余，但是为了设计简洁姑且如此。
            thread::spawn(move || {
                handle_command(Ok(*boxed_command), input, output);
            });
        }
        Ok(Command::Pipe(former_command, latter_command)) => {
            let (pipe_reader, pipe_writer) = pipe().expect("psh: failed to create pipe");

            let handle1 = thread::spawn(||{
                handle_command(Ok(*former_command), input, Some(pipe_writer));
            });

            let handle2 = thread::spawn(||{
                handle_command(Ok(*latter_command), Some(pipe_reader), output);
            });

            handle1.join().expect("psh: failed to join handle of first command in pipe");
            handle2.join().expect("psh: failed to join handle of first command in pipe");
        }
        Err(e) => {
            eprintln!("psh: parse error: {}", e);
        }
    }
}

fn get_prompt() -> String {
    let username = whoami::username();
    let hostname = whoami::fallible::hostname().unwrap_or("unknown_hostname".to_string());
    let username_at_hostname = (username + "@" + &hostname).on_blue();
    let current_dir_path = env::current_dir().unwrap_or_default();
    let current_dir_str = current_dir_path.to_str().unwrap_or("unknown_current_directory").to_string();
    let display_dir = match env::var("HOME") {
        Ok(home_dir) => current_dir_str.replace(&home_dir, "~"),
        Err(_) => current_dir_str.to_string(),
    }.green();

    let now = chrono::Local::now();
    let time = now.format("%d/%m/%Y %H:%M").to_string().yellow();

    // 最终样式:
    // username @ hostname [time] $
    format!(
        "{} {} [{}]\n$ ",
        username_at_hostname, display_dir, time
    )
}