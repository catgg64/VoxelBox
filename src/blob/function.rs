use crate::{blob::BlobType, config::ConfigScript};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionId {
    pub id: usize,
}

pub struct Function {
    pub input: Option<Vec<BlobType>>,
    pub output: Option<BlobType>,
    pub script: ConfigScript,
}