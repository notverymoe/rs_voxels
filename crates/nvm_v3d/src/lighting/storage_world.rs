// Copyright 2024 Natalie Baker // AGPLv3 //

use std::collections::{hash_map::Entry, HashMap};

use crate::world::{PosChunk, PosWorld};

use super::LightStorageChunk;

#[derive(Debug, Default)]
pub struct LightStorageWorld(HashMap<PosChunk, LightStorageChunk>);

impl LightStorageWorld {

    #[must_use] 
    pub fn get_channel(&self, pos: PosWorld, channel: usize) -> u8 {
        let (pos_chunk, pos_block) = pos.to_chunk_and_block();
        let idx = pos_block.to_idx();
        self.0.get(&pos_chunk).map_or(0, |chunk| chunk.get_channel(idx, channel))
    }

    pub fn get_or_create_chunk(&mut self, pos_chunk: PosChunk) -> &mut LightStorageChunk {
        match self.0.entry(pos_chunk) {
            Entry::Occupied(o) => o.into_mut(),
              Entry::Vacant(v) => v.insert(LightStorageChunk::default()),
        }
    }

    #[must_use] 
    pub fn get_chunk(&self, pos_chunk: PosChunk) -> Option<&LightStorageChunk> {
        self.0.get(&pos_chunk)
    }

}