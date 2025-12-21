use std::io;

#[derive(Debug)]
pub enum ShellError {
    ParseError(String),
    BuiltinError(String),
    IoError(io::Error),
    ExecuteError(String),
    LLMError(String),
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShellError::ParseError(msg) => write!(f, "psh: Parse Error: {}", msg),
            ShellError::BuiltinError(msg) => write!(f, "psh: Builtin Error: {}", msg),
            ShellError::IoError(err) => write!(f, "psh: IO Error: {}", err),
            ShellError::ExecuteError(msg) => write!(f, "psh: Execute Error: {}", msg),
            ShellError::LLMError(msg) => write!(f, "psh: LLM Error: {}", msg),
        }
    }
}

impl std::error::Error for ShellError {}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> ShellError {
        ShellError::IoError(err)
    }
}