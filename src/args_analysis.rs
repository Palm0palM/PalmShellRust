use crate::error::ShellError;

#[derive(Debug)]
pub struct Redirection {
    pub output_file: Option<String>,  // > filename
    pub input_file: Option<String>,   // < filename
}

impl Redirection {
    pub fn new() -> Self {
        Redirection {
            output_file: None,
            input_file: None,
        }
    }
}

/// 分析并移除参数列表中的重定向符号
/// 返回：Redirection { output_file, input_file }
pub fn redirection_analysis(args: &mut Vec<String>) -> Result<Redirection, ShellError> {
    let mut rdr = Redirection::new();
    let mut i = 0;

    while i < args.len() {
        if args[i] == ">" || args[i] == "<" {
            let is_output = args[i] == ">";
            args.remove(i);

            if i < args.len() {
                let filename = args.remove(i);

                if is_output {
                    rdr.output_file = Some(filename);
                } else {
                    rdr.input_file = Some(filename);
                }
            } else {
                return Err(ShellError::RedirectionError(
                    "After redirection operator, there is no filename".to_string()
                ));
            }
        } else {
            i += 1;
        }
    }

    Ok(rdr)
}