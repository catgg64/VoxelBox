use std::{any::Any, collections::HashMap};

use crate::{Point, bundle::BundleId, variable::{AtomVariable, AtomVariableId}};

pub mod element;
pub mod connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledAtomVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: AtomVariableId,
}

#[derive(Debug)]
pub struct Atom {
    pub point: Point,
    pub voxel_id: AtomId,
    pub type_id: u8,
    pub bundle: u8,
    variables: HashMap<BundledAtomVariableId, Box<dyn Any>>,
}

impl Atom {
    pub fn set_variable(&mut self, id: (&AtomVariable, BundledAtomVariableId), value: Box<dyn Any>) {
        if id.0.typeid == value.type_id() {
            self.variables.insert(id.1, value);
        }
    }

    pub fn get_variable(&self, v: BundledAtomVariableId) -> Option<&Box<dyn Any>> {
        self.variables.get(&v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomId {
    pub id: u16,
}