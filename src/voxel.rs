use std::{any::Any, collections::HashMap};

use crate::{Point, atom::{Atom, AtomId, connection::AtomConnection}, bundle::BundleId, variable::{VoxelGroupVariable, VoxelGroupVariableId, VoxelVariable, VoxelVariableId}};

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

#[derive(Debug)]
pub enum VoxelElement {
    Atom(Atom),
    Connection(AtomConnection),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoxelElementId {
    pub id: u16
}

#[derive(Debug)]
pub struct Voxel {
    pub atoms: HashMap<VoxelElementId, VoxelElement>,
    pub id_head: u16,
    variables: HashMap<BundledVoxelVariableId, Box<dyn Any>>,
}

impl Voxel {
    pub fn add_element(&mut self, element: VoxelElement) -> u16 {
        self.atoms.insert(VoxelElementId{id: self.id_head}, element);
        self.id_head += 1;
        self.id_head - 1
    }

    pub fn get_element(&self, id: VoxelElementId) -> &VoxelElement {
        &self.atoms[&id]
    }

    pub fn set_variable(&mut self, id: (&VoxelVariable, BundledVoxelVariableId), value: Box<dyn Any>) {
        if id.0.typeid == value.type_id() {
            self.variables.insert(id.1, value);
        }
    }

    pub fn get_variable(&self, v: BundledVoxelVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&v)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledVoxelGroupVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: VoxelGroupVariableId,
}

pub struct VoxelGroup {
    pub voxels: HashMap<VoxelGroupPoint, Voxel>,
    pub position: Point,
    variables: HashMap<BundledVoxelGroupVariableId, Box<dyn Any>>,
}

impl VoxelGroup {
    pub fn set_variable(&mut self, id: (&VoxelGroupVariable, BundledVoxelGroupVariableId), value: Box<dyn Any>) {
        if id.0.typeid == value.type_id() {
            self.variables.insert(id.1, value);
        }
    }

    pub fn get_variable(&self, v: BundledVoxelGroupVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&v)
    }
}