use crate::blob::BundledBlobTypeId;

pub struct ElementVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementVariableId {
    pub id: u16,
}

pub struct AtomConnectionTypeVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomConnectionTypeVariableId {
    pub id: u16,
}

pub struct AtomConnectionVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomConnectionVariableId {
    pub id: u16,
}

pub struct AtomVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomVariableId {
    pub id: u16,
}

pub struct VoxelVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelVariableId {
    pub id: u16,
}

pub struct VoxelConnectionTypeVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelConnectionTypeVariableId {
    pub id: u16,
}

pub struct VoxelConnectionVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelConnectionVariableId {
    pub id: u16,
}

pub struct VoxelGroupVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelGroupVariableId {
    pub id: u16,
}

pub struct VoxelGroupConnectionTypeVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelGroupConnectionTypeVariableId {
    pub id: u16,
}

pub struct VoxelGroupConnectionVariable {
    pub typeid: BundledBlobTypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelGroupConnectionVariableId {
    pub id: u16,
}