use std::fmt;

#[derive(Debug)]
pub enum BuiltinError{
    ArgsLack(u32),
}
impl fmt::Display for BuiltinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltinError::ArgsLack(num) => write!(f, "({} arguments lacked)", num),
        }?;
        Ok(())
    }
}
impl std::error::Error for BuiltinError{ }