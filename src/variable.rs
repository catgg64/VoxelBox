use serde::{Deserialize, Serialize};

use crate::blob::BundledBlobTypeId;

#[derive(Serialize, Deserialize)]
pub struct ElementVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ElementVariableId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct AtomConnectionTypeVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AtomConnectionTypeVariableId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct AtomConnectionVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AtomConnectionVariableId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct AtomVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AtomVariableId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct VoxelVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelVariableId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct VoxelConnectionTypeVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelConnectionTypeVariableId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct VoxelConnectionVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelConnectionVariableId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct VoxelGroupVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelGroupVariableId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct VoxelGroupConnectionTypeVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelGroupConnectionTypeVariableId {
    pub id: u16,
}

pub struct VoxelGroupConnectionVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelGroupConnectionVariableId {
    pub id: u16,
}