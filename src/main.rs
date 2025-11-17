use rustyline::DefaultEditor;

mod parser;
mod builtins;
mod executor;
mod run;
mod error;

fn main() {
    // 初始化Readline
    let reader =  DefaultEditor::new().unwrap();

    // 调用main_loop
    run::main_loop(reader);
}


