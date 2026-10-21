use std::{any::Any, collections::HashMap};

use crate::{bundle::BundleId, variable::{VoxelConnectionTypeVariable, VoxelConnectionVariable, VoxelConnectionVariableId}, voxel::VoxelGroupPoint};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledVoxelConnectionTypeVariableId {
    pub bundle_id: BundleId,
    pub voxel_variable_id: VoxelConnectionVariableId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelConnectionTypeId {
    pub id: u16,
}

pub struct VoxelConnectionType {
    variables: HashMap<BundledVoxelConnectionTypeVariableId, Box<dyn Any>>
}

impl VoxelConnectionType {
    pub fn new(variables: Vec<(&VoxelConnectionTypeVariable, BundledVoxelConnectionTypeVariableId, Box<dyn Any>)>) -> Self {
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

    pub fn get_variable(&self, bundled_id: BundledVoxelConnectionTypeVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&bundled_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledVoxelConnectionVariableId {
    pub bundle_id: BundleId,
    pub voxel_variable_id: VoxelConnectionVariableId,
}

#[derive(Debug)]
pub struct VoxelConnection {
    pub voxel_1: VoxelGroupPoint,
    pub voxel_2: VoxelGroupPoint,
    pub connection_type: BundledVoxelConnectionTypeVariableId,
    variables: HashMap<BundledVoxelConnectionVariableId, Box<dyn Any>>,
}


impl VoxelConnection {
    pub fn set_variable(&mut self, id: (&VoxelConnectionVariable, BundledVoxelConnectionVariableId), value: Box<dyn Any>) {
        if id.0.typeid == value.type_id() {
            self.variables.insert(id.1, value);
        }
    }

    pub fn get_variable(&self, v: BundledVoxelConnectionVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&v)
    }
}