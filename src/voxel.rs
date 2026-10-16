use std::collections::HashMap;

use crate::{Point, atom::{Atom, AtomId}};

#[derive(Debug)]
pub struct VoxelGroupPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
pub struct Voxel {
    pub atoms: HashMap<AtomId, Atom>,
    pub id_head: u16,
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
}

pub struct VoxelGroup {
    pub voxels: HashMap<VoxelGroupPoint, Voxel>,
    pub position: Point,
}