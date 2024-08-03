// Copyright 2024 Natalie Baker // AGPLv3 //

use std::{fs::File, io::BufWriter};

use image::RgbImage;
use nvm_v3d::{lighting::LightStorageWorld, meshing::{mesh_chunk_plane, VisFace}, tiles::TileIdentifier, world::{ChunkStorage, PosBlock, PosChunk}};

pub fn read_vox(path: &str) -> ChunkStorage {
    let mut storage = ChunkStorage::new_empty();
    let data = &dot_vox::load(path).unwrap().models[0].voxels;
    for entry in data {
        if entry.x >= 32 || entry.y >= 32 || entry.z >= 32 { continue; }
        storage.update(
            PosBlock::new(entry.x as i16, entry.y as i16, entry.z as i16), 
            TileIdentifier::ONE, 
            true
        );
    }
    storage
}

pub fn mesh_chunk(storage: &ChunkStorage) -> Vec<u32> {
    let mut result = vec![0; 32*32*32*6];
    let mut size = 0;
    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let offset = [x, y, z];
                for i in 0..8 {
                    size += mesh_chunk_plane(storage, offset, VisFace::PosX, i, &mut result[size..]);
                    size += mesh_chunk_plane(storage, offset, VisFace::PosY, i, &mut result[size..]);
                    size += mesh_chunk_plane(storage, offset, VisFace::PosZ, i, &mut result[size..]);
                    size += mesh_chunk_plane(storage, offset, VisFace::NegX, i, &mut result[size..]);
                    size += mesh_chunk_plane(storage, offset, VisFace::NegY, i, &mut result[size..]);
                    size += mesh_chunk_plane(storage, offset, VisFace::NegZ, i, &mut result[size..]);
                }
                
            }
        }
    }
    result.truncate(size);
    result
}

pub fn write_lighting_data_to_image(path: &str, light_data: &LightStorageWorld, chunk: PosChunk, factor: u8) {
    let mut image = RgbImage::new(32, 32*32);
    let chunk = light_data.get_chunk(chunk).unwrap();
    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let idx = PosBlock::new(x as i16, y as i16, z as i16).to_idx();
                image.put_pixel(x, y + z*32, image::Rgb([
                    chunk.get_channel(idx, 0)*factor,
                    chunk.get_channel(idx, 1)*factor,
                    chunk.get_channel(idx, 2)*factor,
                ]));
            }
        }
    }
    image.write_to(&mut BufWriter::new(File::create(path).unwrap()), image::ImageFormat::Png).unwrap();
}