// 这个Enum定义了Command的状态
pub enum Command {
    Empty,
    Exit,
    Builtin(String, Vec<String>),
    External(String, Vec<String>),
}

pub fn parse_line(line: &str) -> Result<Command, String> {
    // 去除空格
    let line = line.trim();

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

    match command_name.as_str() {
        "exit" => Ok(Command::Exit),
        "cd" | "pwd" | "echo" => Ok(Command::Builtin(command_name, args)),
        _ => {
            Ok(Command::External(command_name, args))
        }
    }
}