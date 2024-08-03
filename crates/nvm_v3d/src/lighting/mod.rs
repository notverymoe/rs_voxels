// Copyright 2024 Natalie Baker // AGPLv3 //

use crate::world::PosWorld;

mod storage_chunk;
pub use storage_chunk::*;

mod storage_world;
pub use storage_world::*;

pub mod update;

pub fn light_sunlight_raise_batched(
    get_transmission:  &mut impl FnMut(PosWorld, usize) -> u8,
    storage: &mut LightStorageWorld,
    updates: &[(PosWorld, u8)]
) {
    let mut queue = Vec::<PosWorld>::with_capacity(updates.len()); // TODO OPT probably larger
    update::light_channel_raise_batched(3, storage, updates.iter().copied(), &mut queue);
    update::light_channel_raise_propogate(get_transmission, 3, storage, &mut queue);
}

pub fn light_blocklight_raise_batched(
    get_transmission:  &mut impl FnMut(PosWorld, usize) -> u8,
    storage: &mut LightStorageWorld,
    updates: &[(PosWorld, [u8; 3])]
) -> usize {
    let mut update_count = 0;
    let mut queue = Vec::<PosWorld>::with_capacity(updates.len()); // TODO OPT probably larger
    for i in 0..3 {
        update_count += update::light_channel_raise_batched(i, storage, updates.iter().map(|&(pos, value)| (pos, value[i])), &mut queue);
        update_count += update::light_channel_raise_propogate(get_transmission, i, storage, &mut queue);
    }
    update_count
}
