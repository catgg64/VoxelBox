use std::{any::Any, collections::HashMap};

use crate::{bundle::BundleId, variable::{VoxelConnectionTypeVariable, VoxelConnectionVariable, VoxelConnectionVariableId, VoxelGroupConnectionTypeVariable, VoxelGroupConnectionVariable, VoxelGroupConnectionVariableId}, voxel::VoxelGroupPoint};

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
            output_variables.insert(inputv.1, inputv.2);
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
    pub fn new(voxel_1: VoxelGroupPoint, voxel_2: VoxelGroupPoint, connection_type: BundledVoxelConnectionTypeVariableId, variables: Vec<(&VoxelConnectionVariable, BundledVoxelConnectionVariableId, Box<dyn Any>)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        Self {
            voxel_1,
            voxel_2,
            connection_type,
            variables: output_variables,
        }
    }

    pub fn set_variable(&mut self, id: (&VoxelConnectionVariable, BundledVoxelConnectionVariableId), value: Box<dyn Any>) {
        self.variables.insert(id.1, value);
    }

    pub fn get_variable(&self, v: BundledVoxelConnectionVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&v)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledVoxelGroupConnectionTypeVariableId {
    pub bundle_id: BundleId,
    pub voxel_variable_id: VoxelGroupConnectionVariableId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelGroupConnectionTypeId {
    pub id: u16,
}

pub struct VoxelGroupConnectionType {
    variables: HashMap<BundledVoxelGroupConnectionTypeVariableId, Box<dyn Any>>
}

impl VoxelGroupConnectionType {
    pub fn new(variables: Vec<(&VoxelGroupConnectionTypeVariable, BundledVoxelGroupConnectionTypeVariableId, Box<dyn Any>)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        Self {
            variables: output_variables,
        }
    }

    pub fn get_variable(&self, bundled_id: BundledVoxelGroupConnectionTypeVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&bundled_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledVoxelGroupConnectionVariableId {
    pub bundle_id: BundleId,
    pub voxel_group_variable_id: VoxelGroupConnectionVariableId,
}

#[derive(Debug)]
pub struct VoxelGroupConnection {
    pub voxel_1: VoxelGroupPoint,
    pub voxel_2: VoxelGroupPoint,
    pub connection_type: BundledVoxelGroupConnectionTypeVariableId,
    variables: HashMap<BundledVoxelGroupConnectionVariableId, Box<dyn Any>>,
}

impl VoxelGroupConnection {
    pub fn new(voxel_1: VoxelGroupPoint, voxel_2: VoxelGroupPoint, connection_type: BundledVoxelGroupConnectionTypeVariableId, variables: Vec<(&VoxelGroupConnectionVariable, BundledVoxelGroupConnectionVariableId, Box<dyn Any>)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        Self {
            voxel_1,
            voxel_2,
            connection_type,
            variables: output_variables,
        }
    }

    pub fn set_variable(&mut self, id: (&VoxelGroupConnectionVariable, BundledVoxelGroupConnectionVariableId), value: Box<dyn Any>) {
        self.variables.insert(id.1, value);
    }

    pub fn get_variable(&self, v: BundledVoxelGroupConnectionVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&v)
    }
}