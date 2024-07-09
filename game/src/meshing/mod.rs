// Copyright 2024 Natalie Baker // AGPLv3 //

mod bit_plane;
pub use bit_plane::*;

mod culled;
pub use culled::*;

mod vertex;
pub use vertex::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisAxis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl VisAxis {

    #[must_use]
    pub const fn to_local(self, world: [u32; 3]) -> [u32; 3] {
        match self {
            VisAxis::X => [world[2], world[1], world[0]],
            VisAxis::Y => [world[0], world[2], world[1]],
            VisAxis::Z => world,
        }
    }

    #[must_use]
    pub const fn to_world(self, local: [u32; 3]) -> [u32; 3] {
        match self {
            VisAxis::X => [local[2], local[1], local[0]],
            VisAxis::Y => [local[0], local[2], local[1]],
            VisAxis::Z => local,
        }
    }

}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisFace {
    PosX = 0,
    PosY = 1,
    PosZ = 2,
    NegX = 3,
    NegY = 4,
    NegZ = 5,
}

impl VisFace {

    #[must_use]
    pub const fn from_raw(raw: u16) -> Option<Self> {
        match raw {
            0 => Some(Self::PosX),
            1 => Some(Self::PosY),
            2 => Some(Self::PosZ),
            3 => Some(Self::NegX),
            4 => Some(Self::NegY),
            5 => Some(Self::NegZ),
            _ => None,
        }
    }

    #[must_use]
    pub const fn reverse(self) -> Self {
        match self {
            VisFace::PosX => VisFace::NegX,
            VisFace::PosY => VisFace::NegY,
            VisFace::PosZ => VisFace::NegZ,
            VisFace::NegX => VisFace::PosX,
            VisFace::NegY => VisFace::PosY,
            VisFace::NegZ => VisFace::PosZ,
        }
    }

    #[must_use]
    pub const fn axis(self) -> VisAxis {
        match self {
            VisFace::PosX => VisAxis::X,
            VisFace::PosY => VisAxis::Y,
            VisFace::PosZ => VisAxis::Z,
            VisFace::NegX => VisAxis::X,
            VisFace::NegY => VisAxis::Y,
            VisFace::NegZ => VisAxis::Z,
        }
    }

}