use std::env;
use std::path::Path;

pub fn handle_builtin(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "cd" => builtin_cd(args),
        "pwd" => builtin_pwd(),
        "echo" => builtin_echo(args),
        _ => false,
    }
}

// cd命令
fn builtin_cd(args: &[String]) -> bool {
    let target_dir = if let Some(path) = args.get(0) {
        path.clone()
    } else {
        // 默认移动到HOME
        match env::var("HOME") {
            Ok(home_dir) => home_dir,
            Err(_) => {
                eprintln!("cd: HOME variable not set");
                return false;
            }
        }
    };

    let path = Path::new(&target_dir);
    if let Err(e) = env::set_current_dir(path) {
        eprintln!("cd: {}: {}", target_dir, e);
        return false;
    }
    true
}

// pwd命令
fn builtin_pwd() -> bool {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            true
        }
        Err(e) => {
            eprintln!("pwd: {}", e);
            false
        }
    }
}

// echo命令
fn builtin_echo(args: &[String]) -> bool {
    let output = args.join(" ");
    println!("{}", output);
    true
}
