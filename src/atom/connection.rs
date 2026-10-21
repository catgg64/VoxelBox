use std::{any::Any, collections::HashMap};

use crate::{atom::AtomId, bundle::BundleId, variable::{AtomConnectionTypeVariable, AtomConnectionVariable, AtomConnectionVariableId}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledAtomConnectionTypeVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: AtomConnectionVariableId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomConnectionTypeId {
    pub id: u16,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledAtomConnectionVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: AtomConnectionVariableId,
}

#[derive(Debug)]
pub struct AtomConnection {
    pub atom_1: AtomId,
    pub atom_2: AtomId,
    pub connection_type: BundledAtomConnectionTypeVariableId,
    variables: HashMap<BundledAtomConnectionVariableId, Box<dyn Any>>,
}


impl AtomConnection {
    pub fn set_variable(&mut self, id: (&AtomConnectionVariable, BundledAtomConnectionVariableId), value: Box<dyn Any>) {
        if id.0.typeid == value.type_id() {
            self.variables.insert(id.1, value);
        }
    }

    pub fn get_variable(&self, v: BundledAtomConnectionVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&v)
    }
}