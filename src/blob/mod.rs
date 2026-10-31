use std::{collections::HashMap, sync::Arc};

use crate::blob::{function::{Function, FunctionId}, pin::PinBoard};

pub mod pin;
pub mod function;

pub struct BlobType {
    pub type_params: PinBoard<Arc<BundledBlobTypeId>>,
    pub functions: HashMap<FunctionId, Function>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobTypeId {
    pub id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledBlobTypeId {
    pub bundle_id: u16,
    pub blob_type_id: u32,
}

pub struct Blob {
    pub blob_type: BlobType,
    pub params: PinBoard<Blob>,
}