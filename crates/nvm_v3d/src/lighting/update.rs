// Copyright 2024 Natalie Baker // AGPLv3 //

use crate::world::PosWorld;

use super::LightStorageWorld;

pub fn light_channel_raise_batched(
    channel: usize,
    storage: &mut LightStorageWorld,
    updates: impl IntoIterator<Item = (PosWorld, u8)>,
    queue:   &mut Vec<PosWorld>,
) -> usize {
    let mut update_count = 0;
    for (pos, target) in updates {
        let (pos_chunk, pos_block) = pos.to_chunk_and_block();
        let idx = pos_block.to_idx();
        if storage.get_or_create_chunk(pos_chunk).raise_channel(idx, channel, target) {
            update_count += 1;
            // TODO OPT do we want to filter these?
            queue.extend(
                get_light_neighbourhood_of(pos).iter()
                .filter(|&&neighbour| could_transmit_to(target, storage.get_channel(neighbour, channel)))
            );
        }
    }
    update_count
}

pub fn light_channel_raise_propogate(
    get_transmission:  &mut impl FnMut(PosWorld, usize) -> u8,
    channel: usize,
    storage: &mut LightStorageWorld,
    queue:   &mut Vec<PosWorld>,
) -> usize {
    let mut update_count = 0;
    while let Some(pos) = queue.pop() {
        // println!("Updt: {pos:?}");

        // Get neighbour values
        let neighbourhood = get_light_neighbourhood_of(pos);
        let values = neighbourhood.map(|neighbour| storage.get_channel(neighbour, channel));

        // Recalculate light value from neighbours
        let transmit_cost = (get_transmission)(pos, channel).saturating_add(1); // TODO Get from block transmittance
        let target = values.iter().max().unwrap().saturating_sub(transmit_cost);

        // Update light value
        let (pos_chunk, pos_block) = pos.to_chunk_and_block();
        let chunk      = storage.get_or_create_chunk(pos_chunk);
        let did_update = chunk.raise_channel(pos_block.to_idx(), channel, target);

        // If we changed our value, queue neighbours for update if
        //     they have light values less than our new value.
        if did_update {
            update_count += 1;
            for i in 0..neighbourhood.len() {
                // TODO OPT should we account for target transmittance?
                if !could_transmit_to(target, values[i]) { continue; }
                queue.push(neighbourhood[i]);
            }
        }
    }
    update_count
}

const fn get_light_neighbourhood_of(pos: PosWorld) -> [PosWorld; 6] {
    [
        pos.with_offset( 1,  0,  0),
        pos.with_offset(-1,  0,  0),
        pos.with_offset( 0,  1,  0),
        pos.with_offset( 0, -1,  0),
        pos.with_offset( 0,  0,  1),
        pos.with_offset( 0,  0, -1),
    ]
}

const fn could_transmit_to(from: u8, to: u8) -> bool {
    from > to.saturating_add(1)
}