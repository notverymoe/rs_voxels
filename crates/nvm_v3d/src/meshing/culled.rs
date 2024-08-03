// Copyright 2024 Natalie Baker // AGPLv3 //

use super::{bit_plane::BitPlane, VisFace, encode_vertex};

pub trait FaceVisibilityProvider {
    fn get_face_visibility_plane(&self, chunk: [usize; 3], face: VisFace, layer: u32) -> BitPlane;
}

pub fn mesh_chunk_plane(vis: &impl FaceVisibilityProvider, chunk: [usize; 3], face: VisFace, layer: u32, dest: &mut [u32]) -> usize {
    assert!(dest.len() >= 8*8);

    let visibility = vis.get_face_visibility_plane(chunk, face, layer).to_raw();
    let offset = face.axis().to_local_usize(chunk).map(|v| (v*8) as u8);

    let mut size = 0;
    let mut i = visibility.trailing_zeros();
    while i < 64 {
        let vis_new = visibility >> i;
        let run = vis_new.trailing_ones();
        for j in 0..run {
            let k = i+j;
            let x = k & 0x07;
            let y = k >> 3;
            dest[size] = encode_vertex(
                offset[0] +     x as u8, 
                offset[1] +     y as u8, 
                offset[2] + layer as u8,
                face
            );
            size += 1;
        }
        i += run;
        if let Some(run) = visibility.checked_shr(i) {
            i += run.trailing_zeros();
        } else {
            break;
        }
    }

    size
}