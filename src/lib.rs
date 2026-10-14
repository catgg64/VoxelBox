pub mod atom;
pub mod voxel;

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
    pub z: f64
}

pub struct Chunk {
    pub voxel_groups: Vec<VoxelBoxConstants>,
}

pub struct World {

}
