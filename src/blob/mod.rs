use std::collections::HashMap;

use crate::blob::{function::{Function, FunctionId}, pin::PinBoard};

pub mod pin;
pub mod function;

#[derive(Clone)]
pub struct BlobType {
    pub type_params: PinBoard<BlobType>,
    pub functions: HashMap<FunctionId, Function>,
}

#[derive(Clone)]
pub struct Blob {
    pub blob_type: BlobType,
    pub params: PinBoard<Blob>,
}