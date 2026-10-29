use crate::config::ConfigScript;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionId {
    pub id: usize,
}

pub struct Function {
    pub script: ConfigScript,
}