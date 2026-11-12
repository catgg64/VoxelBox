use std::{any::Any, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{blob::{function::{Function, FunctionId}, pin::PinBoard}, bundle::{BundleContext, BundleId, RelativeBundleId}};

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
    pub relative_bundle_id: RelativeBundleId,
    pub context: &BundleContext,
    pub blob_type_id: BlobTypeId,
}

#[derive(Serialize, Deserialize)]
pub struct Blob {
    pub blob_type: BundledBlobTypeId,
    pub params: PinBoard<serde_json::Value>,
}