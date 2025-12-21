use rustyline::DefaultEditor;
use dotenvy::dotenv;
use std::fs;
mod parser;
mod builtins;
mod executor;
mod run;
mod error;
mod model_call;

fn main() {
    // 初始化Readline
    let reader =  DefaultEditor::new().unwrap();
    dotenv().ok();

    fs::create_dir_all("chats").unwrap();

    run::main_loop(reader);
}


