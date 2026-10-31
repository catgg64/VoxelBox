use serde::{Deserialize, Serialize};

use crate::config::ConfigScript;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FunctionId {
    pub id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    pub script: ConfigScript,
}