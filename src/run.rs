use std::{env, thread};
use std::process::{Stdio, exit};
use std::io::{self, Read, Write};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use rand::seq::IndexedRandom;
use os_pipe::{pipe, PipeReader, PipeWriter};
use colorgrad::Gradient;

use crate::builtins;
use crate::error::ShellError;
use crate::executor::execute;
use crate::parser::{parse_line, Command};


// 主循环的功能是，不断接受输入调用handle_command解析命令，并处理Ctrl+C Ctrl+D
pub fn main_loop(mut reader: DefaultEditor) {
    loop {
        let read_result = reader.readline(&get_prompt());

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
                        builtins::builtin_echo_piped(args, & mut (*reader), & mut (*writer))
                    } else {
                        builtins::builtin_echo(args, & mut (*reader), & mut (*writer))
                    }
                },
                "ls" => builtins::builtin_ls(args, & mut (*reader), & mut (*writer)),
                "grep" => {
                    if is_piped{
                        builtins::builtin_grep_piped(&mut args, & mut (*reader), & mut (*writer))
                    } else {
                        builtins::builtin_grep(&mut args, & mut (*reader), & mut (*writer))
                    }
                }
                "chat" => builtins::builtin_model_call(&mut args, & mut (*reader), & mut (*writer)),
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
            let (pipe_reader, pipe_writer) = pipe().expect("Failed to create pipe");

            let handle1 = thread::spawn(||{
                handle_command(Ok(*former_command), input, Some(pipe_writer));
            });

            let handle2 = thread::spawn(||{
                handle_command(Ok(*latter_command), Some(pipe_reader), output);
            });

            handle1.join().expect("Failed to join handle");
            handle2.join().expect("Failed to join handle");
        }
        Err(e) => {
            eprintln!("psh: {}", e);
        }
    }
}

// 为文本应用渐变色
// 通过\x01和\x02标记包裹ANSI转义序列，告诉rustyline这些是不可打印字符
fn apply_gradient(text: &str, gradient: &dyn Gradient) -> String {
    if text.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut result = String::new();

    for (i, ch) in chars.iter().enumerate() {
        // 计算当前字符在渐变中的位置 (0.0 到 1.0)
        let t = if len > 1 {
            i as f32 / (len - 1) as f32
        } else {
            0.5
        };

        // 从渐变中获取颜色
        let color = gradient.at(t);
        let rgba = color.to_rgba8();

        // 生成24位真彩色ANSI转义序列
        let color_code = format!("\x1b[38;2;{};{};{}m", rgba[0], rgba[1], rgba[2]);

        // 用rustyline的不可打印字符标记包裹ANSI代码
        // \x01 标记不可打印序列的开始
        // \x02 标记不可打印序列的结束
        result.push_str(&format!("\x01{}\x02{}", color_code, ch));
    }

    // 在末尾添加重置代码
    result.push_str("\x01\x1b[0m\x02");

    result
}

fn get_prompt() -> String {
    // 提示符
    let prompt_choices = ["😀", "😃", "😅", "🥲", "🤯", "😝", "😚", "🤥", "💩", "🤡", "🥱", "😔", "🥳", "🤪", "🥰", "😇"];
    let mut prompt= String::new();
    let username = whoami::username();
    if username == "root".to_string() {
        prompt = "\u{1F680} #".to_string();
    } else {
        let mut rng = rand::rng();
        prompt = prompt_choices.choose(&mut rng).unwrap().to_string() + " $";
    }

    // 用户名&主机
    let hostname = whoami::fallible::hostname().unwrap_or("unknown_hostname".to_string());
    let username_at_hostname = username + "@" + &hostname;

    // 当前路径
    let current_dir_path = env::current_dir().unwrap_or_default();
    let current_dir_str = current_dir_path.to_str().unwrap_or("unknown_current_directory").to_string();
    let display_dir = match env::var("HOME") {
        Ok(home_dir) => current_dir_str.replace(&home_dir, "~"),
        Err(_) => current_dir_str.to_string(),
    };

    // 当前时间
    let now = chrono::Local::now();
    let time = now.format("%d/%m/%Y %H:%M").to_string();

    // 创建渐变颜色
    let gradient = colorgrad::GradientBuilder::new()
        .html_colors(&["#FF6B6B", "#FFA07A", "#FFD93D", "#6BCF7F", "#4ECDC4", "#45B7D1", "#9B59B6", "#E056FD"])
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    // 应用渐变到不同部分
    let styled_username = apply_gradient(&username_at_hostname, &gradient);
    let styled_dir = apply_gradient(&display_dir, &gradient);
    let styled_time = apply_gradient(&time, &gradient);

    // 最终样式:
    // username @ hostname [time]
    // emoji
    format!(
        "{} {} [{}]\n{} ",
        styled_username, styled_dir, styled_time, prompt
    )
}