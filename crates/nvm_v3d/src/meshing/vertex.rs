// Copyright 2024 Natalie Baker // AGPLv3 //

use glam::IVec3;

use super::VisFace;

#[must_use]
pub const fn encode_vertex(
    x:     u8,
    y:     u8,
    layer: u8,
    face:  VisFace,
) -> u32 {
    ((    x & 0x1F) as u32)       |
    ((    y & 0x1F) as u32) <<  5 |
    ((layer & 0x1F) as u32) << 10 |
    (         face  as u32) << 15
}


#[must_use]
pub const fn decode_vertex(vert: u32) -> (
    /*x:       */ u8,
    /*y:       */ u8,
    /*layer:   */ u8,
    /*face:    */ VisFace,
) {
    (
        ( vert        & 0x1F) as u8,
        ((vert >>  5) & 0x1F) as u8,
        ((vert >> 10) & 0x1F) as u8,
        VisFace::from_raw(((vert >> 15) & 0x0F) as u16).unwrap(),
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

pub fn create_quad_for_vertex(x: u8, y: u8, layer: u8, face: VisFace) -> [IVec3; 6] {

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