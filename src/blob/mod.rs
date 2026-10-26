use crate::Chunk;

pub mod pin;
pub mod function;

pub enum BlobInstruction {
    SetPinXToPinY(u32, u32),
    SetPinXtoInputPinY(u32, u32),
    SetPinToBlob(u32, Blob),

}