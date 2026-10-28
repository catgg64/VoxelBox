use crate::blob::{function::{Function}, pin::PinBoard};

pub mod pin;
pub mod function;

pub struct BlobType {
    pub type_params: PinBoard<BlobType>,
    pub functions: Vec<Function>,
}

pub struct Blob {
    pub r#type: BlobType,
    pub params: PinBoard<Blob>,
}