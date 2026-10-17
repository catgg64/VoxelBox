use std::any::TypeId;

#[derive(Debug, Clone, Copy)]
pub struct ElementVariable {
    pub typeid: TypeId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementVariableId {
    pub id: u16,
}