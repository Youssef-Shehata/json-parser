use std::fmt::{self };

pub enum Errors {
    CorruptFile,
    UnbalancedBrackets,
}
pub fn bail<T>(e: Errors) -> anyhow::Result<T> {
    anyhow::bail!("{}", e)
}
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::CorruptFile => write!(f, " Json File Corrupted"),
            Errors::UnbalancedBrackets => write!(f, " Unbalanced brackets in file"),
        }
    }
}
