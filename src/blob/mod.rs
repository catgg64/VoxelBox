use std::{collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{blob::{function::{Function, FunctionId}, pin::PinBoard}, bundle::BundleId};

pub mod pin;
pub mod function;

#[derive(Serialize, Deserialize)]
pub struct BlobType {
    pub type_params: PinBoard<(bool, BundledBlobTypeId)>,
    pub functions: HashMap<FunctionId, Function>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BlobTypeId {
    pub id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BundledBlobTypeId {
    pub bundle_id: BundleId,
    pub blob_type_id: BlobTypeId,
}

#[derive(Serialize, Deserialize)]
pub struct Blob {
    pub blob_type: BundledBlobTypeId,
    pub params: PinBoard<Blob>,
}