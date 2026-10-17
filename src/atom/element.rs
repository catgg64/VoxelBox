use std::{any::Any, collections::HashMap};

use crate::{atom::element, bundle::BundleId, variable::ElementVariableId};


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementId {
    pub id: u16,
}

#[derive(Debug)]
pub struct Element {
    pub variables: HashMap<(BundleId, ElementVariableId), Box<dyn Any>>
}

impl Element {
    pub fn add_variable(&mut self, bundle_id: BundleId, element_variable_id: ElementVariableId, v: Box<dyn Any>) {
        self.variables.insert((bundle_id, element_variable_id), v);
    }

    pub fn get_variable(&mut self, bundle_id: BundleId, element_variable_id: ElementVariableId) -> 
}
