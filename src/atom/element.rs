use std::{collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{blob::BundledBlobTypeId, bundle::BundleId, variable::{ElementVariable, ElementVariableId}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BundledElementVariableId {
    bundle_id: BundleId,
    element_variable_id: ElementVariableId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ElementId {
    pub id: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    variables: HashMap<BundledElementVariableId, BundledBlobTypeId>
}

impl Element {
    pub fn new(variables: Vec<(&ElementVariable, BundledElementVariableId, BundledBlobTypeId)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        Self {
            variables: output_variables,
        }
    }

    pub fn get_variable(&self, bundled_id: BundledElementVariableId) -> Option<BundledBlobTypeId> {
        self.variables.get(&bundled_id).cloned()
    }
}
