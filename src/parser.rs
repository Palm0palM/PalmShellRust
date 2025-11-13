// 这个Enum定义了Command的状态
#[derive(Debug)]
pub enum Command {
    Empty,
    Exit,
    Builtin(String, Vec<String>),
    External(String, Vec<String>),
    Background(Box<Command>),
}

pub fn parse_line(line: &str) -> Result<Command, String> {
    // 去除空格
    let mut line = line.trim();

    // 检查是否后台命令
    let is_background = if line.ends_with('&') {
        // 移除&符号
        line = line[..line.len()-1].trim();
        true
    } else {
        false
    };

    // 处理空命令
    if line.is_empty() {
        return Ok(Command::Empty);
    }

    // 分割命令
    let mut words: Vec<String> = line
        .split_whitespace()
        .map(String::from)
        .collect();

    // 分割命令名和参数
    let command_name = words.remove(0);
    let args = words;

    let command = match command_name.as_str() {
        "exit" => Command::Exit,
        "quit" => Command::Empty,
        "cd" | "pwd" | "echo" => Command::Builtin(command_name, args),
        _ => Command::External(command_name, args),
    };

    if is_background {
        match command {
            Command::Exit | Command::Empty => Err("Cannot run in background".to_string()),
            _ => Ok(Command::Background(Box::new(command))),
        }
    } else {
        Ok(command)
    }
}