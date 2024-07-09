// Copyright 2024 Natalie Baker // AGPLv3 //

use core::time::Duration;
use std::{io::{BufWriter, Write}, time::Instant};

use glam::{Quat, Vec3};
use nvm_game::{meshing::{VisFace, mesh_chunk_plane, decode_vertex, create_quad_for_vertex}, tiles::TileIdentifier, world::{ChunkStorage, PosBlock}};

fn main() {

    const LOOP_COUNT: usize = 174_264; // Minecraft 64-chunk render distance

    let mut storage = ChunkStorage::new_empty();
    let mut verts = vec![0; 16*16*16*6];
    let mut vert_count = 0;
    let mut vis_time_total = Duration::ZERO;
    let mut mesh_time_total = Duration::ZERO;
    for i in 0..LOOP_COUNT { 
        let (vis_time, mesh_time, vert_count_iter) = do_loop(&mut verts, &mut storage); 
        vis_time_total  += vis_time;
        mesh_time_total += mesh_time;
        vert_count = vert_count_iter;
        if i % 10_000 == 0 {
            println!("Loop {i}");
        }
    }

    let  vis_time_avg = ( vis_time_total.as_micros() as f64) / (LOOP_COUNT as f64);
    let mesh_time_avg = (mesh_time_total.as_micros() as f64) / (LOOP_COUNT as f64);
    let proc_time_avg = vis_time_avg+mesh_time_avg;

    println!("-----------------------");
    println!("# Looped {LOOP_COUNT} times");
    println!("-----------------------");
    println!("Visibility took: {vis_time_avg:.2}us/iter ({:.0} iter/sec) [{:.0}ms total]",  1e6/vis_time_avg,  vis_time_total.as_millis() as f64);
    println!("Meshing took:    {mesh_time_avg:.2}us/iter ({:.0} iter/sec) [{:.0}ms total]", 1e6/mesh_time_avg, mesh_time_total.as_millis() as f64);
    println!("Processing took: {proc_time_avg:.2}us/iter ({:.0} iter/sec) [{:.0}ms total]", 1e6/proc_time_avg, (vis_time_total.as_millis() + mesh_time_total.as_millis()) as f64);
    println!("-----------------------");

    let args: Vec<_> = std::env::args().skip(1).collect();
    let ((), write_time) = do_time(|| write_obj(&args[0], &verts[..vert_count]));
    println!("Writing took: {}us", write_time.as_micros());
}

fn do_loop(
    verts:   &mut [u32], 
    storage: &mut ChunkStorage
) -> (Duration, Duration, usize) {
    let ((), vis_time) = do_time(|| build_vis(&DEMO_CHUNK, storage));
    let (vert_count, mesh_time) = do_time(|| process_chunk(verts, storage));
    (vis_time, mesh_time, vert_count)
}

fn do_time<T>(mut f: impl FnMut() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let end = Instant::now();
    (result, end.duration_since(start))
}

fn build_vis(chunk: &[bool], storage: &mut ChunkStorage) {
    // Set chunk 1,1,1 to the vis mask, extend outside 1 block for culling
    for z in 0..16 {
        for y in 0..16 {
            for x in 0..16 {
                if !chunk[(x % 8) + (y % 8)*8 + (z % 8)*8*8] {
                    continue;
                }
                storage.update(
                    PosBlock::new(x as i16, y as i16, z as i16), 
                    TileIdentifier::DEFAULT, 
                    true
                );
            }
        }
    }
}

fn process_chunk(result: &mut [u32], storage: &ChunkStorage) -> usize {
    let mut size = 0;
    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
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
    size
}

fn write_obj(path: &str, verts: &[u32]) {
    let mut file = BufWriter::new(std::fs::File::create(path).unwrap());

    let rot = Quat::from_axis_angle(Vec3::X, (-90.0_f32).to_radians());
    for (i, (x, y, z, face, _tex)) in verts.iter().enumerate().map(|(i, v)| (i, decode_vertex(*v))) {
        create_quad_for_vertex(x, y, z, face).map(|vert| rot.mul_vec3(vert.as_vec3()).round().as_ivec3()).map(|vert| {
            writeln!(&mut file, "v {} {} {}", vert.x, vert.y, vert.z).unwrap();
        });
        writeln!(&mut file, "f {} {} {}", i*6+1, i*6+2, i*6+3).unwrap();
        writeln!(&mut file, "f {} {} {}", i*6+4, i*6+5, i*6+6).unwrap();
    }
}

const DEMO_CHUNK: [bool; 512] = [
    // ///////////////////////////////////////////////// //
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
    // ///////////////////////////////////////////////// //
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
     true,  true,  true,  true,  true,  true,  true,  true,
    // ///////////////////////////////////////////////// //
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false,  true,  true,  true,  true, false,  true,
    false, false,  true,  true,  true,  true, false, false,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,  true,
    // ///////////////////////////////////////////////// //
     true,  true,  true,  true,  true,  true,  true,  true,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false,  true,  true,  true,  true, false,  true,
    false, false,  true, false, false,  true, false, false,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,  true,
    // ///////////////////////////////////////////////// //
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false,  true, false, false,  true, false,  true,
    false, false,  true, false, false,  true, false, false,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,  true,
    // ///////////////////////////////////////////////// //
     true,  true,  true,  true,  true,  true,  true,  true,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false,  true, false, false,  true, false,  true,
    false, false,  true,  true,  true,  true, false, false,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,  true,
    // ///////////////////////////////////////////////// //
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false,  true,  true, false,  true, false,  true,
    false, false, false,  true,  true,  true, false, false,
    false, false, false, false, false, false, false,  true,
    false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,  true,
    // ///////////////////////////////////////////////// //
     true,  true,  true,  true,  true,  true,  true,  true,
    false,  true, false,  true,  true, false,  true,  true,
     true, false,  true, false, false,  true, false,  true,
    false,  true, false,  true,  true, false,  true,  true,
    false,  true, false,  true,  true, false,  true, false,
     true, false,  true, false, false,  true, false,  true,
    false,  true, false,  true,  true, false,  true, false,
     true, false,  true, false, false,  true, false,  true,
    // ///////////////////////////////////////////////// //
];