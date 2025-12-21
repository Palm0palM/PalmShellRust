use std::io;

#[derive(Debug)]
pub enum ShellError {
    ParseError(String),
    BuiltinError(String),
    IoError(io::Error),
    ExecuteError(String),
    LLMError(String),
    RedirectionError(String),
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShellError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            ShellError::BuiltinError(msg) => write!(f, "Builtin Error: {}", msg),
            ShellError::IoError(err) => write!(f, "IO Error: {}", err),
            ShellError::ExecuteError(msg) => write!(f, "Execute Error: {}", msg),
            ShellError::LLMError(msg) => write!(f, "LLM Error: {}", msg),
            ShellError::RedirectionError(msg) => write!(f, "Redirection Error: {}", msg),
        }
    }
}

impl std::error::Error for ShellError {}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> ShellError {
        ShellError::IoError(err)
    }
}