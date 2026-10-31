use std::{any::Any, collections::HashMap};

use crate::{bundle::BundleId, variable::{ElementVariable, ElementVariableId}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BundledElementVariableId {
    bundle_id: BundleId,
    element_variable_id: ElementVariableId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementId {
    pub id: u16,
}

#[derive(Debug)]
pub struct Element {
    variables: HashMap<BundledElementVariableId, Box<dyn Any>>
}

impl Element {
    pub fn new(variables: Vec<(&ElementVariable, BundledElementVariableId, Box<dyn Any>)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        Self {
            variables: output_variables,
        }
    }

    pub fn get_variable(&self, bundled_id: BundledElementVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&bundled_id)
    }
}
