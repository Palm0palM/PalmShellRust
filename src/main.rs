use rustyline::DefaultEditor;
use dotenvy::dotenv;
use owo_colors::OwoColorize;
use std::env;
use std::fs;
use std::path::PathBuf;

mod parser;
mod builtins;
mod executor;
mod run;
mod error;
mod model_call;
mod prompt;
mod args_analysis;

fn main() {
    // Banner
    let banner = r#"
  _____      _            _____ _          _ _
 |  __ \    | |          / ____| |        | | |
 | |__) |_ _| |_ __ ___ | (___ | |__   ___| | |
 |  ___/ _` | | '_ ` _ \ \___ \| '_ \ / _ \ | |
 | |  | (_| | | | | | | |____) | | | |  __/ | |
 |_|   \__,_|_|_| |_| |_|_____/|_| |_|\___|_|_|

             Welcome to PalmShell!

"#;
    // 用渐变色打印Banner
    for (i, line) in banner.lines().enumerate() {
        let r = (i * 30) as u8;
        let g = 0;
        let b = 255;
        println!("{}", line.truecolor(r, g, b).bold());
    }

    // 初始化Readline
    let reader =  DefaultEditor::new().unwrap();
    dotenv().ok();

    // 配置自定义二进制目录并添加到 PATH
    setup_shell_environment();

    run::main_loop(reader);
}

fn setup_shell_environment() {
    // 设定目录为 ~/.palm_shell/bin
    if let Ok(home) = env::var("HOME") {
        let mut bin_path = PathBuf::from(&home);
        bin_path.push(".palm_shell");
        bin_path.push("bin");

        // 如果目录不存在则创建
        if !bin_path.exists() {
            let _ = fs::create_dir_all(&bin_path);
        }

        // 将其添加到 PATH 环境变量的最前面
        if let Ok(current_path) = env::var("PATH") {
            let new_path = format!("{}:{}", bin_path.display(), current_path);
            unsafe{env::set_var("PATH", new_path);}
        }
    }
}
