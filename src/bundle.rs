use std::collections::HashMap;

use crate::{atom::{connection::{AtomConnectionType, AtomConnectionTypeId}, element::{Element, ElementId}}, variable::{AtomConnectionTypeVariable, AtomConnectionTypeVariableId, AtomConnectionVariable, AtomConnectionVariableId, AtomVariable, AtomVariableId, ElementVariable, ElementVariableId, VoxelGroupVariable, VoxelGroupVariableId, VoxelVariable, VoxelVariableId}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BundleId {
    pub id: u16,
}

pub struct Bundle {
    pub elements: HashMap<ElementId, Element>,
    pub element_variables: HashMap<ElementVariableId, ElementVariable>,
    pub atom_variables: HashMap<AtomVariableId, AtomVariable>,
    pub atom_connection_types: HashMap<AtomConnectionTypeId, AtomConnectionType>,
    pub atom_connection_type_variables: HashMap<AtomConnectionTypeVariableId, AtomConnectionTypeVariable>,
    pub atom_connection_variables: HashMap<AtomConnectionVariableId, AtomConnectionVariable>,
    pub voxel_variables: HashMap<VoxelVariableId, VoxelVariable>,
    pub voxel_group_variables: HashMap<VoxelGroupVariableId, VoxelGroupVariable>,
}