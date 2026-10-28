use std::collections::HashMap;

pub mod atom;
pub mod voxel;
pub mod bundle;
pub mod variable;
pub mod blob;
pub mod config;

#[derive(Debug)]
pub struct VoxelBoxConstants {
    pub disconnected_voxels_rip_from_voxel_group: bool,
    pub processed_voxel_group_depth_check: bool,
    pub processed_voxel_group_depth_check_value: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkElementId {
    pub id: u16,
}

pub enum ChunkElement {
    VoxelGroup(crate::voxel::VoxelGroup),
    Connection(crate::voxel::connection::VoxelGroupConnection)
}

pub struct Chunk {
    chunk_elements: HashMap<ChunkElementId, ChunkElement>,
    id_head: u16
}

impl Chunk {
    pub fn new(chunk_elements: HashMap<ChunkElementId, ChunkElement>) -> Self {
        Self { chunk_elements, id_head: 0 }
    }

    pub fn set_element(&mut self, element: ChunkElement) -> ChunkElementId {
        self.chunk_elements.insert(ChunkElementId { id: self.id_head }, element);
        self.id_head += 1;
        ChunkElementId { id: self.id_head - 1 }
    }

    pub fn get_element(&mut self, element_id: &ChunkElementId) -> &ChunkElement {
        &self.chunk_elements[element_id]
    }
}

pub struct World {

}
