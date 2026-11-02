use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::blob::{function::{Function, FunctionId}, pin::PinBoard};

pub mod pin;
pub mod function;

#[derive(Serialize, Deserialize)]
pub struct BlobType {
    pub type_params: PinBoard<BundledBlobTypeId>,
    pub functions: HashMap<FunctionId, Function>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BlobTypeId {
    pub id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BundledBlobTypeId {
    pub bundle_id: u16,
    pub blob_type_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Blob {
    pub blob_type: BlobType,
    pub params: PinBoard<Blob>,
}