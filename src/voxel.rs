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

pub enum VoxelElement {
    Atom(Atom),
    Connection(AtomConnection)
}

#[derive(Debug)]
pub struct Voxel {
    pub atoms: HashMap<AtomId, Atom>,
    pub id_head: u16,
    variables: HashMap<BundledVoxelVariableId, Box<dyn Any>>,
}

impl Voxel {
    pub fn add_atom(&mut self, atom: Atom) -> u16 {
        self.atoms.insert(AtomId{id: self.id_head}, atom);
        self.id_head += 1;
        self.id_head - 1
    }

    pub fn get_atom(&self, id: AtomId) -> &Atom {
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