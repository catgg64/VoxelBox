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
pub struct AtomVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomVariableId {
    pub id: u16,
}