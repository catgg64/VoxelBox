use std::{collections::HashMap};

use crate::{Point, atom::{Atom, connection::AtomConnection}, blob::Blob, bundle::BundleId, variable::{VoxelGroupVariable, VoxelGroupVariableId, VoxelVariable, VoxelVariableId}, voxel::connection::VoxelConnection};

pub mod connection;

#[derive(Debug)]
pub struct VoxelGroupPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledVoxelVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: VoxelVariableId,
}

pub enum VoxelElement {
    Atom(Atom),
    Connection(AtomConnection),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelElementId {
    pub id: u16
}

pub struct Voxel {
    pub atoms: HashMap<VoxelElementId, VoxelElement>,
    pub id_head: u16,
    variables: HashMap<BundledVoxelVariableId, Blob>,
}

impl Voxel {
    pub fn new(variables: Vec<(&VoxelVariable, BundledVoxelVariableId, Blob)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        
        Self { atoms: HashMap::new(), id_head: 0, variables: output_variables }
    }

    pub fn add_element(&mut self, element: VoxelElement) -> u16 {
        self.atoms.insert(VoxelElementId{id: self.id_head}, element);
        self.id_head += 1;
        self.id_head - 1
    }

    pub fn get_element(&self, id: VoxelElementId) -> &VoxelElement {
        &self.atoms[&id]
    }

    pub fn set_variable(&mut self, id: (&VoxelVariable, BundledVoxelVariableId), value: Blob) {
        self.variables.insert(id.1, value);
    }

    pub fn get_variable(&self, v: BundledVoxelVariableId) -> Option<&Blob> {
        self.variables.get(&v)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledVoxelGroupVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: VoxelGroupVariableId,
}

pub enum VoxelGroupElement {
    Voxel(Voxel),
    Connection(VoxelConnection),
}

pub struct VoxelGroup {
    pub voxels: HashMap<VoxelGroupPoint, VoxelGroupElement>, 
    pub position: Point,
    variables: HashMap<BundledVoxelGroupVariableId, Blob>,
}

impl VoxelGroup {
    pub fn new(voxels: HashMap<VoxelGroupPoint, VoxelGroupElement>, position: Point, variables: Vec<(&VoxelGroupVariable, BundledVoxelGroupVariableId, Blob)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        
        Self { voxels, position, variables: output_variables }
    }

    pub fn set_variable(&mut self, id: (&VoxelGroupVariable, BundledVoxelGroupVariableId), value: Blob) {
        self.variables.insert(id.1, value);
    }

    pub fn get_variable(&self, v: BundledVoxelGroupVariableId) -> Option<&Blob> {
        self.variables.get(&v)
    }
}
