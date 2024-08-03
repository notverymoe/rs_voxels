// Copyright 2024 Natalie Baker // AGPLv3 //

use crate::world::CHUNK_LENGTH;

#[derive(Debug)]
pub struct LightStorageChunk(Box<[[u8; 4]; CHUNK_LENGTH]>);

impl LightStorageChunk {
    pub fn raise_channel(&mut self, idx: usize, channel: usize, value: u8) -> bool {
        let current = self.0[idx][channel];
        if current < value {
            self.0[idx][channel] = value;
            true
        } else {
            false
        }
    }

    #[must_use] 
    pub fn get_channel(&self, idx: usize, channel: usize) -> u8 {
        self.0[idx][channel]
    }

    #[must_use] 
    pub const fn get_data(&self) -> &[[u8; 4]; CHUNK_LENGTH] {
        &self.0
    }
}


impl Default for LightStorageChunk {
    fn default() -> Self {
        Self(vec![[0; 4]; CHUNK_LENGTH].into_boxed_slice().try_into().unwrap())
    }
}

