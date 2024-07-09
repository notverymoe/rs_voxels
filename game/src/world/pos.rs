// Copyright 2024 Natalie Baker // AGPLv3 //

use glam::IVec3;

use super::{CHUNK_COORD_BITS, CHUNK_SIZE};

macro_rules! impl_world_pos {
    ($name:ident) => {

        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name {
            pub x: i16,
            pub y: i16,
            pub z: i16,
        }

        impl $name {

            #[must_use]
            pub const fn new(x: i16, y: i16, z: i16) -> Self {
                Self{x, y, z}
            }

            #[must_use]
            pub const fn as_ivec3(self) -> IVec3 {
                IVec3::new(self.x as i32, self.y as i32, self.z as i32)
            }

            #[must_use]
            pub const fn from_ivec3(other: IVec3) -> Self {
                Self::new(other.x as i16, other.y as i16, other.z as i16)
            }

        }
    };
}

impl_world_pos!(PosWorld);

impl PosWorld {

    #[must_use]
    pub const fn from_chunk_and_block(chunk: PosChunk, block: PosBlock) -> Self {
        Self::new(
            chunk.x << 6 | block.x,
            chunk.y << 6 | block.y,
            chunk.z << 6 | block.z,
        )
    }

    #[must_use]
    pub const fn to_chunk_and_block(self) -> (PosChunk, PosBlock) {
        (
            PosChunk::new(
                self.x >> CHUNK_COORD_BITS,
                self.y >> CHUNK_COORD_BITS,
                self.z >> CHUNK_COORD_BITS,
            ),
            PosBlock::new_unnormalized(
                self.x,
                self.y,
                self.z,
            )
        )
    }

}

impl_world_pos!(PosChunk);

impl PosChunk {

    pub const COORDINATE_MASK: i16 = (u16::MAX >> CHUNK_COORD_BITS) as i16;

    #[must_use]
    pub const fn normalize(self) -> Self {
        Self::new(
            self.x & Self::COORDINATE_MASK,
            self.y & Self::COORDINATE_MASK,
            self.z & Self::COORDINATE_MASK,
        )
    }

}


impl_world_pos!(PosBlock);

impl PosBlock {

    pub const COORDINATE_MASK: i16 = (CHUNK_SIZE - 1) as i16;

    #[must_use]
    pub const fn new_unnormalized(x: i16, y: i16, z: i16) -> Self {
        Self::new(
            x & Self::COORDINATE_MASK,
            y & Self::COORDINATE_MASK,
            z & Self::COORDINATE_MASK,
        )
    }

    #[must_use]
    pub const fn normalize(self) -> Self {
        Self::new(
            self.x & Self::COORDINATE_MASK,
            self.y & Self::COORDINATE_MASK,
            self.z & Self::COORDINATE_MASK,
        )
    }

    #[must_use]
    pub const fn from_idx(idx: usize) -> Self {
        Self {
            x: (idx          as i16) & Self::COORDINATE_MASK,
            y: ((idx >>  6)  as i16) & Self::COORDINATE_MASK,
            z: ((idx >> 12)  as i16) & Self::COORDINATE_MASK,
        }
    }

    #[must_use]
    pub const fn to_idx(self) -> usize {
        let Self{x, y, z} = self;
        (x as usize) | ((y as usize) << 6) | ((z as usize) << 12)
    }

}

