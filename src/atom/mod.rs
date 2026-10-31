use std::{collections::HashMap};

use crate::{Point, blob::Blob, bundle::BundleId, variable::{AtomVariable, AtomVariableId}};

pub mod element;
pub mod connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundledAtomVariableId {
    pub bundle_id: BundleId,
    pub atom_variable_id: AtomVariableId,
}

pub struct Atom {
    pub point: Point,
    pub atom_id: AtomId,
    pub type_id: u8,
    pub bundle: u8,
    variables: HashMap<BundledAtomVariableId, Blob>,
}

impl Atom {
    pub fn new(point: Point, atom_id: AtomId, type_id: u8, bundle: u8, variables: Vec<(&AtomVariable, BundledAtomVariableId, Blob)>) -> Self {
        let mut output_variables = HashMap::new();
        for inputv in variables {
            output_variables.insert(inputv.1, inputv.2);
        }
        
        Self { point, atom_id, type_id, bundle, variables: output_variables }
    }

    pub fn set_variable(&mut self, id: (&AtomVariable, BundledAtomVariableId), value: Blob) {
        self.variables.insert(id.1, value);
    }

    pub fn get_variable(&self, v: BundledAtomVariableId) -> Option<&Blob> {
        self.variables.get(&v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomId {
    pub id: u16,
}