use crate::error::ShellError;
// 这个Enum定义了Command的状态
#[derive(Debug)]
pub enum Command {
    Empty,
    Exit,
    Builtin(String, Vec<String>),
    External(String, Vec<String>),
    Background(Box<Command>),
    Pipe(Box<Command>, Box<Command>),
}


// 在这种parse机制的处理逻辑中，& 符号会作用于多个管道连接起来的整体
// 如果在管道连接的命令内部使用&，如 cmd & | cmd & 的形式，会出现解析错误
pub fn parse_line(line: &str) -> Result<Command, ShellError> {
    // 去除空格
    let mut line = line.trim();
    let is_background:bool;

    // 检查是否后台命令
    is_background = if line.ends_with('&') {
        // 移除&符号,再trim一遍
        line = line[..line.len()-1].trim();
        true
    } else {
        false
    };

    // 处理空命令
    if line.is_empty() {
        return Ok(Command::Empty);
    }

    let command = parse_command(line, is_background)?;

    test_background(command, is_background)
}

// 解析命令。单独拿出这个函数是方便递归地嵌套Pipe
fn parse_command(cmd : &str, is_background: bool) -> Result<Command, ShellError>{
    // 如果存在管道符号，那就从从第一个管道处拆分出左右两个子串
    if let Some((first_cmd, other)) = cmd.split_once('|'){
        // 递归地解析两个字串
        let former_command = parse_command(first_cmd, is_background)?;
        let latter_command = parse_command(other, is_background)?;
        // 包裹在Command::Pipe中返回
        Ok(Command::Pipe(Box::new(former_command), Box::new(latter_command)))
    } else {// 如果是不存在管道符号的普通命令

        let cmd = cmd.trim();

        if cmd.is_empty() {
            return Ok(Command::Empty);
        }

        let mut parsed : Vec<String> = cmd
            .split_whitespace()
            .map(|s| expand_env_vars(s)) // 第一步：展开 $变量
            .map(|s| expand_tilde(&s))   // 第二步：展开 ~
            .collect();

        // 分割出命令名和参数
        let cmd_name = parsed.remove(0);
        let args = parsed;

        let command = match cmd_name.as_str() {
            "exit" => Command::Exit,
            "quit" => Command::Empty,
            "cd" | "pwd" | "echo" | "grep" | "chat" | "kill" | "export" | "env" => Command::Builtin(cmd_name, args),
            _ => Command::External(cmd_name, args),
        };

        test_background(command, is_background)
    }
}

// 包装函数。如果是后台命令，返回一个Command::Background包裹的Command
fn test_background(command: Command, is_background: bool) -> Result<Command, ShellError>{
    if is_background {
        match command {
            Command::Exit | Command::Empty => Err(ShellError::ParseError("Cannot run in background".to_string())),
            _ => Ok(Command::Background(Box::new(command))),
        }
    } else {
        Ok(command)
    }
}

// 检查并展开参数中的 ~
fn expand_tilde(arg: &str) -> String {
    if let Ok(home) = std::env::var("HOME") {
        let mut expanded = if arg == "~" || arg.starts_with("~/") {
            arg.replacen('~', &home, 1)
        } else {
            arg.to_string()
        };
        
        expanded = expanded.replace("=~/", &format!("={}/", home));
        expanded = expanded.replace(":~/", &format!(":{}/", home));

        return expanded;
    }
    arg.to_string()
}

fn expand_env_vars(arg: &str) -> String {
    let mut result = String::new();
    let mut chars = arg.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            let mut var_name = String::new();
            // 向后读取由字母、数字或下划线组成的变量名
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    var_name.push(next_c);
                    chars.next(); // 消耗该字符
                } else {
                    break;
                }
            }
            // 尝试从环境中获取该变量的值，如果有就替换，没有就留空
            if let Ok(val) = std::env::var(&var_name) {
                result.push_str(&val);
            }
        } else {
            result.push(c);
        }
    }
    result
}