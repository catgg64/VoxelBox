use crate::{Point};

pub mod element;

#[derive(Debug, Clone)]
pub struct Atom {
    pub point: Point,
    pub voxel_id: AtomId,
    pub type_id: u8,
    pub bundle: u8,
    pub connections: Vec<AtomId>,
}

impl Atom {
    pub fn create_connection(&mut self, atom: &mut Atom) {
        self.connections.push(atom.voxel_id);
        atom.connections.push(self.voxel_id);
    }

    pub fn break_connection(&mut self, atom: &mut Atom) {
        for connection in 0..self.connections.len() {
            if self.connections[connection] == atom.voxel_id {
                self.connections.remove(connection);
                break;
            }
        }
        for connection in 0..atom.connections.len() {
            if atom.connections[connection] == self.voxel_id {
                atom.connections.remove(connection);
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomId {
    pub id: u16,
}