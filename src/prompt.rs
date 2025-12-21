use std::env;
use colorgrad::Gradient;
use rand::prelude::IndexedRandom;

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

pub fn get_prompt() -> String {
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