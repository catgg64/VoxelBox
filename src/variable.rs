use std::any::TypeId;

#[derive(Debug, Clone, Copy)]
pub struct ElementVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementVariableId {
    pub id: u16,
}
#[derive(Debug, Clone, Copy)]

pub struct AtomConnectionTypeVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomConnectionTypeVariableId {
    pub id: u16,
}

pub struct AtomConnectionVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomConnectionVariableId {
    pub id: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct AtomVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomVariableId {
    pub id: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct VoxelVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelVariableId {
    pub id: u16,
}

pub struct VoxelConnectionTypeVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelConnectionTypeVariableId {
    pub id: u16,
}

pub struct VoxelConnectionVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelConnectionVariableId {
    pub id: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct VoxelGroupVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelGroupVariableId {
    pub id: u16,
}

pub struct VoxelGroupConnectionTypeVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelGroupConnectionTypeVariableId {
    pub id: u16,
}

pub struct VoxelGroupConnectionVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelGroupConnectionVariableId {
    pub id: u16,
}