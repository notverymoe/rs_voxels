// Copyright 2024 Natalie Baker // AGPLv3 //

use crate::meshing::{bit_plane::BitPlane, chunk_vis::{ChunkVis, VisAxis, VisFace}, culled::FaceVisibilityProvider};

pub struct ChunkFaceVisibilityProvider<'a> {
    data: [&'a ChunkVis; 7],
}

impl<'a> ChunkFaceVisibilityProvider<'a> {

    #[must_use]
    pub const fn new(data: [&'a ChunkVis; 7]) -> Self {
        Self{data}
    }
    
}

impl<'a> FaceVisibilityProvider for ChunkFaceVisibilityProvider<'a> {
    fn get_face_visbiility_plane(&self, face: VisFace, layer: u32) -> BitPlane {
        let vis_base = self.get_plane(None, face.axis(), layer);
        let vis_cull = self.get_cull_plane(face, layer);
        vis_base & !vis_cull
    }
}

impl<'a> ChunkFaceVisibilityProvider<'a> {

    #[must_use]
    fn get_plane(&self, face: Option<VisFace>, axis: VisAxis, layer: u32) -> BitPlane {
        let data = self.data[face.map_or(self.data.len() - 1, |v| v as usize)];
        *data.get_plane(axis, layer)
    }

    #[must_use]
    fn get_cull_plane(&self, face: VisFace, layer: u32) -> BitPlane {
        if face < VisFace::NegX { 
            if layer > 0 {
                self.get_plane(None, face.axis(), layer-1)
            } else {
                self.get_plane(Some(face), face.axis(), 7)
            }
        } else { 
            #[allow(clippy::collapsible_else_if)]
            if layer < 7 {
                self.get_plane(None, face.axis(), layer+1)
            } else {
                self.get_plane(Some(face), face.axis(), 0)
            }
        }
    }

}