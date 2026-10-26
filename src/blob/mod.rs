use crate::{Chunk, blob::pin::PinBoard};

pub mod pin;
pub mod function;

pub struct BlobType {
    pub type_params: PinBoard<BlobType>,
    pub functions: ,
}

pub struct Blob {
    pub params: PinBoard<Blob>,
}

pub enum BlobInstruction {
    SetPinXToPinY(u32, u32),
    SetPinXtoInputPinY(u32, u32),
    SetPinToBlob(u32, Blob),

}