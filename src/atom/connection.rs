use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{atom::AtomId, blob::Blob, bundle::BundleId, variable::{AtomConnectionTypeVariable, AtomConnectionVariable, AtomConnectionVariableId}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BundledAtomConnectionTypeVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: AtomConnectionVariableId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AtomConnectionTypeId {
    pub id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct AtomConnectionType {
    variables: HashMap<BundledAtomConnectionTypeVariableId, Blob>
}

impl AtomConnectionType {
    pub fn new(variables: Vec<(&AtomConnectionTypeVariable, BundledAtomConnectionTypeVariableId, Blob)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        Self {
            variables: output_variables,
        }
    }

    pub fn get_variable(&self, bundled_id: BundledAtomConnectionTypeVariableId) -> Option<&Blob> {
        self.variables.get(&bundled_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BundledAtomConnectionVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: AtomConnectionVariableId,
}

#[derive(Serialize, Deserialize)]
pub struct AtomConnection {
    pub atom_1: AtomId,
    pub atom_2: AtomId,
    pub connection_type: BundledAtomConnectionTypeVariableId,
    variables: HashMap<BundledAtomConnectionVariableId, Blob>,
}


impl AtomConnection {
    pub fn new(atom_1: AtomId, atom_2: AtomId, connection_type: BundledAtomConnectionTypeVariableId, variables: Vec<(&AtomConnectionVariable, BundledAtomConnectionVariableId, Blob)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        
        Self { atom_1, atom_2, connection_type, variables: output_variables }
    }

    pub fn set_variable(&mut self, id: (&AtomConnectionVariable, BundledAtomConnectionVariableId), value: Blob) {
        self.variables.insert(id.1, value);
    }

    pub fn get_variable(&self, v: BundledAtomConnectionVariableId) -> Option<&Blob> {
        self.variables.get(&v)
    }
}