use std::{any::Any, collections::HashMap};

use crate::{atom::AtomId, bundle::BundleId, variable::{AtomConnectionTypeVariable, AtomConnectionVariableId}};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledAtomConnectionTypeVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: AtomConnectionVariableId,
}

pub struct AtomConnectionType {
    variables: HashMap<BundledAtomConnectionTypeVariableId, Box<dyn Any>>
}

impl AtomConnectionType {
    pub fn new(variables: Vec<(&AtomConnectionTypeVariable, BundledAtomConnectionTypeVariableId, Box<dyn Any>)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            if inputv.0.typeid == inputv.2.type_id() {
                output_variables.insert(inputv.1, inputv.2);
            }
        }
        Self {
            variables: output_variables,
        }
    }

    pub fn get_variable(&self, bundled_id: BundledAtomConnectionTypeVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&bundled_id)
    }
}

pub struct Connection {
    pub atom_1: AtomId,
    pub atom_2: AtomId,
    pub connection_type: BundledAtomConnectionTypeVariableId,
}