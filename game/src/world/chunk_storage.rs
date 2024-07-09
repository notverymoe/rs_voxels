// Copyright 2024 Natalie Baker // AGPLv3 //s

use crate::{meshing::{BitPlane, VisAxis, VisFace, FaceVisibilityProvider}, tiles::TileIdentifier};
use super::PosBlock;

pub const CHUNK_COORD_BITS:  usize = 6;
pub const CHUNK_SIZE:        usize = 1 << CHUNK_COORD_BITS;
pub const CHUNK_LENGTH:      usize = CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE;
pub const CHUNK_LENGTH_MASK: usize = CHUNK_LENGTH - 1;

pub const CHUNK_SOLID_MASK: usize = CHUNK_SIZE*CHUNK_SIZE;

pub const CHUNK_VIS_SIZE:   usize = CHUNK_SIZE/8;
pub const CHUNK_VIS_LENGTH: usize = CHUNK_VIS_SIZE * CHUNK_VIS_SIZE * CHUNK_VIS_SIZE;

pub struct ChunkStorage {
    identifiers: Box<[TileIdentifier; CHUNK_LENGTH    ]>,
    vis_data:    Box<[BitPlane; 3*CHUNK_VIS_SIZE*CHUNK_VIS_SIZE*CHUNK_SIZE]>,
}

impl ChunkStorage {

    #[must_use]
    pub fn new_empty() -> Self {
        Self{
            identifiers: vec![TileIdentifier::DEFAULT; CHUNK_LENGTH].into_boxed_slice().try_into().unwrap(),
            vis_data:    vec![BitPlane::DEFAULT; 3*CHUNK_VIS_SIZE*CHUNK_VIS_SIZE*CHUNK_SIZE].into_boxed_slice().try_into().unwrap(),
        }
    }

    pub fn update(&mut self, pos: PosBlock, id: TileIdentifier, solid: bool) {
        self.identifiers[pos.to_idx()] = id;
        let [x_idx, x_vis] = Self::get_vis_idx(pos, VisAxis::X);
        if self.vis_data[x_vis].get(x_idx as u32) != solid {
            let [y_idx, y_vis] = Self::get_vis_idx(pos, VisAxis::Y);
            let [z_idx, z_vis] = Self::get_vis_idx(pos, VisAxis::Z);
            self.vis_data[x_vis].set(x_idx as u32, solid);
            self.vis_data[y_vis].set(y_idx as u32, solid);
            self.vis_data[z_vis].set(z_idx as u32, solid);
        }
    }

    #[must_use]
    pub const fn get(&self, pos: PosBlock) -> TileIdentifier {
        self.identifiers[pos.to_idx()]
    }

}

impl ChunkStorage {

    const fn get_vis_idx(pos: PosBlock, axis: VisAxis) -> [usize; 2] {
        let [blk_idx, layer, vis_idx] = Self::get_vis_idx_and_layer(pos, axis);
        [blk_idx, vis_idx + layer]
    }

    const fn get_vis_idx_and_layer(pos: PosBlock, axis: VisAxis) -> [usize; 3] {
        let [x, y, layer] = axis.to_local([pos.x as u32, pos.y as u32, pos.z as u32]);
        let x = x as usize;
        let y = y as usize;
        let start = (axis as usize) * CHUNK_VIS_SIZE*CHUNK_VIS_SIZE*CHUNK_SIZE;
        let off_xy = (x >> 3)*CHUNK_SIZE + (y >> 3)*CHUNK_VIS_SIZE*CHUNK_SIZE;
        [
            (x & 0x07) | ((y & 0x07) << 3), 
            layer as usize, 
            start + off_xy
        ]
    }

}

impl FaceVisibilityProvider for ChunkStorage {

    fn get_face_visibility_plane(&self, chunk: [usize; 3], face: VisFace, layer: u32) -> BitPlane {
        let pos = PosBlock::new((chunk[0]*8) as i16, (chunk[1]*8) as i16, (chunk[2]*8) as i16);

        let [_, base_layer, base_vis] = Self::get_vis_idx_and_layer(pos, face.axis());

        let layer = base_layer + layer as usize;
        let base_vis_plane = self.vis_data[base_vis + layer];

        #[allow(clippy::match_bool)]
        let cull_vis_plane = match face < VisFace::NegX {
            true  if layer > 0            => self.vis_data[base_vis + layer - 1],
            false if layer < CHUNK_SIZE-1 => self.vis_data[base_vis + layer + 1],
            _ => BitPlane::DEFAULT
        };

        base_vis_plane & !cull_vis_plane
    }

}
