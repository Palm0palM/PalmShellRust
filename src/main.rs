use rustyline::DefaultEditor;
use dotenvy::dotenv;
use owo_colors::OwoColorize;

mod parser;
mod builtins;
mod executor;
mod run;
mod error;
mod model_call;

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

    //fs::create_dir_all("chats").unwrap();

    run::main_loop(reader);
}
