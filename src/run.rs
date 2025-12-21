use std::thread;
use std::process::{Stdio, exit};
use std::io::{self, Read, Write};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use os_pipe::{pipe, PipeReader, PipeWriter};

use crate::builtins;
use crate::error::ShellError;
use crate::executor::execute;
use crate::parser::{parse_line, Command};


// 主循环的功能是，不断接受输入调用handle_command解析命令，并处理Ctrl+C Ctrl+D
pub fn main_loop(mut reader: DefaultEditor) {
    loop {
        let read_result = reader.readline(&crate::prompt::get_prompt());

        match read_result {
            Ok(line) => {
                reader.add_history_entry(line.as_str())
                    .expect("Failed to add history");
                handle_command(parse_line(&line), None, None);
            }

            // Ctrl + C
            // 默认行为为重新接收命令
            Err(ReadlineError::Interrupted) => continue,

            // Ctrl + D
            // 默认行为为退出程序
            Err(ReadlineError::Eof) => exit(0),
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

// input 和 output 表示命令的输入输出流
// 如果默认用标准流输入输出（而不Pipe设置的流）的话，二者会被设置为None
fn handle_command(
    cmd: Result<Command, ShellError>,
    input: Option<PipeReader>,
    output: Option<PipeWriter>,
) {
    match cmd {
        Ok(Command::Empty) => return,
        Ok(Command::Exit) => {
            println!("Exiting...");
            exit(0);
        }

        Ok(Command::Builtin(cmd, args)) => {
            // 预先读取管道输入（如果有的话）
            let piped_input = if let Some(mut pipe_reader) = input {
                let mut buffer = String::new();
                match pipe_reader.read_to_string(&mut buffer) {
                    Ok(_) => Some(buffer),
                    Err(e) => {
                        eprintln!("psh: Failed to read from pipe: {}", e);
                        None
                    }
                }
            } else {
                None
            };

            // 准备输出流
            let mut writer: Box<dyn Write> = output
                .map_or(Box::new(io::stdout()), |p| Box::new(p));

            // 调用统一的内置命令接口
            let result = match cmd.as_str() {
                "cd" => builtins::builtin_cd(args, piped_input, &mut *writer),
                "pwd" => builtins::builtin_pwd(args, piped_input, &mut *writer),
                "echo" => builtins::builtin_echo(args, piped_input, &mut *writer),
                "ls" => builtins::builtin_ls(args, piped_input, &mut *writer),
                "grep" => builtins::builtin_grep(args, piped_input, &mut *writer),
                "chat" => builtins::builtin_model_call(args, piped_input, &mut *writer),
                _ => return,
            };

            if let Err(e) = result {
                eprintln!("psh: {}", e);
            }
        }

        Ok(Command::External(program, args)) => {
            // 解析input和output。如果是None，map_or会父进程的io流，实际上就是Stdio
            let stdin = input.map_or(Stdio::inherit(), Stdio::from);
            let stdout = output.map_or(Stdio::inherit(), Stdio::from);

            match execute(&program, args, stdin, stdout) {
                Ok(mut child) => {
                    if let Err(e) = child.wait() {
                        eprintln!("psh: failed to wait on process: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("psh: {}", e);
                }
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
            let (pipe_reader, pipe_writer) = pipe().expect("psh: Failed to create pipe");

            let handle1 = thread::spawn(||{
                handle_command(Ok(*former_command), input, Some(pipe_writer));
            });

            let handle2 = thread::spawn(||{
                handle_command(Ok(*latter_command), Some(pipe_reader), output);
            });

            handle1.join().expect("psh: Failed to join handle");
            handle2.join().expect("psh: Failed to join handle");
        }
        Err(e) => {
            eprintln!("psh: {}", e);
        }
    }
}