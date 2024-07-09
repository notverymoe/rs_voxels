// Copyright 2024 Natalie Baker // AGPLv3 //

use glam::IVec3;

use super::VisFace;

#[must_use]
pub const fn encode_vertex(
    pos_x: u8,
    pos_y: u8,
    pos_z: u8,
    face: VisFace,
    texture: u16,
) -> u32 {
    ((pos_x & 0x0F) as u32)       |
    ((pos_y & 0x0F) as u32) <<  4 |
    ((pos_z & 0x0F) as u32) <<  8 |
    (         face  as u32) << 12 |
    (       texture as u32) << 16 
}


#[must_use]
pub const fn decode_vertex(vert: u32) -> (
    /*pos_x:   */ u8,
    /*pos_y:   */ u8,
    /*pos_z:   */ u8,
    /*face:    */ VisFace,
    /*texture: */ u16,
) {
    (
        ( vert       & 0x0F) as u8,
        ((vert >> 4) & 0x0F) as u8,
        ((vert >> 8) & 0x0F) as u8,
        VisFace::from_raw(((vert >> 12) & 0x0F) as u16).unwrap(),
        (vert >> 16) as u16,
    )
}


fn get_face_basis(f: VisFace) -> [IVec3; 3] {
    match f {
        VisFace::PosX => [-IVec3::Z,  IVec3::Y,  IVec3::X],
        VisFace::PosY => [ IVec3::X, -IVec3::Z,  IVec3::Y],
        VisFace::PosZ => [ IVec3::X,  IVec3::Y,  IVec3::Z],
        VisFace::NegX => [ IVec3::Z, -IVec3::Y, -IVec3::X],
        VisFace::NegY => [-IVec3::X,  IVec3::Z, -IVec3::Y],
        VisFace::NegZ => [-IVec3::X, -IVec3::Y, -IVec3::Z],
    }
}

pub fn create_quad_for_vertex(x: u8, y: u8, z: u8, face: VisFace) -> [IVec3; 6] {

    let [x, y, layer] = face.axis().to_local([x as u32, y as u32, z as u32]);

    let basis = get_face_basis(face);
    let basis_abs = basis.map(IVec3::abs);

    let offset_corner   = IVec3::ONE - (basis[0] + basis[1] + basis[2]).max(IVec3::ZERO);
    let offset_position = (x as i32)*basis_abs[0] + (y as i32)*basis_abs[1] + (layer as i32)*basis_abs[2];
    let offset_base     = offset_corner + offset_position;

    let p: [IVec3; 4] = [
        offset_base,
        offset_base + basis[0],
        offset_base + basis[1],
        offset_base + basis[0] + basis[1],
    ];

    if basis[2].dot(IVec3::ONE) < 0 {
        [
            p[0], p[1], p[2],
            p[3], p[2], p[1],
        ]
    } else {
        [
            p[0], p[2], p[1],
            p[3], p[1], p[2],
        ]
    }

}