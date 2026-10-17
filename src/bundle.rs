use std::collections::HashMap;

use crate::{atom::element::{Element, ElementId}, variable::{ElementVariable, ElementVariableId}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BundleId {
    pub id: u16,
}

pub struct Bundle {
    pub elements: HashMap<ElementId, Element>,
    pub element_variables: HashMap<ElementVariableId, ElementVariable>,
}