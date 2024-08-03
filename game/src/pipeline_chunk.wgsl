// // ///////////// // //
// // Vertex Shader // //
// // ///////////// // //

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) colour: vec3<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(1) @binding(0) var<storage, read> faces: array<u32>;
@group(1) @binding(1) var<storage, read> light: array<u32>;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    let face_index = in_vertex_index / 6;
    let vert_index = in_vertex_index % 6;

    let face_data  = decode_voxel_face_data(face_index);
    let uv         = get_voxel_face_uv(face_data.face, vert_index);
    let basis      = get_voxel_face_basis(face_data.face);
    let vertex_pos = calc_voxel_face_vertex(face_data, basis, uv);

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(vec3<f32>(vertex_pos), 1.0);
    out.normal = vec3<f32>(basis[2]);

    // out.colour = vec3<f32>(face_data.pos_block)/32.0;

    let light_pos = face_data.pos_block - basis[2];
    let light_x = u32(light_pos.x);
    let light_y = u32(light_pos.y);
    let light_z = u32(light_pos.z);
    if (light_x < 0 || light_x >= 32 || light_y < 0 || light_y >= 32 || light_z < 0 || light_z >= 32) {
        out.colour = vec3<f32>(0.0, 0.0, 0.0);
    } else {
        let light_idx = light_x + (light_y << 5) + (light_z << 10);
        let light_val = light[light_idx];
        let r = f32((light_val >>  0) & 0xFF)/32.0;
        let g = f32((light_val >>  8) & 0xFF)/32.0;
        let b = f32((light_val >> 16) & 0xFF)/32.0;
        // let lum = f32(light_idx)/(32.0*32.0*32.0);
        out.colour = vec3<f32>(r, g, b);
    }
    
    return out;
}

// // /////////////// // //
// // Voxel Face Data // //
// // /////////////// // //

struct VoxelFaceData {
    pos_face:  vec3<i32>,
    pos_block: vec3<i32>,
    face:      u32,
}

fn decode_voxel_face_data(face_index: u32) -> VoxelFaceData {
    let face = faces[face_index];

    var result: VoxelFaceData;
    result.pos_face = vec3<i32>(
        i32( face        & 0x1F),
        i32((face >>  5) & 0x1F),
        i32((face >> 10) & 0x1F)
    );
    result.face = (face >> 15) & 0x0F;

    var pos_block = array<vec3<i32>, 6>(
        result.pos_face.zyx,
        result.pos_face.xzy,
        result.pos_face,
        result.pos_face.zyx,
        result.pos_face.xzy,
        result.pos_face,
    );
    result.pos_block = pos_block[result.face];

    return result;
}

fn get_voxel_face_uv(face: u32, vert_index: u32) -> vec2<i32> {
    var uvs = array<vec2<i32>, 6>(
        vec2<i32>(0, 0),
        vec2<i32>(1, 0),
        vec2<i32>(0, 1),
        vec2<i32>(1, 1),
        vec2<i32>(0, 1),
        vec2<i32>(1, 0),
    );

    return select(uvs[vert_index].xy, uvs[vert_index].yx, face >= 3);
}

fn get_voxel_face_basis(face: u32) -> array<vec3<i32>, 3> {
    var u_axes = array<vec3<i32>, 6>(
        vec3<i32>( 0, 0, -1),
        vec3<i32>( 1, 0,  0),
        vec3<i32>( 1, 0,  0),
        vec3<i32>( 0, 0,  1),
        vec3<i32>(-1, 0,  0),
        vec3<i32>(-1, 0,  0),
    );

    var v_axes = array<vec3<i32>, 6>(
        vec3<i32>(0,  1,  0),
        vec3<i32>(0,  0, -1),
        vec3<i32>(0,  1,  0),
        vec3<i32>(0, -1,  0),
        vec3<i32>(0,  0,  1),
        vec3<i32>(0, -1,  0),
    );

    var n_axes = array<vec3<i32>, 6>(
        vec3<i32>( 1,  0,  0),
        vec3<i32>( 0,  1,  0),
        vec3<i32>( 0,  0,  1),
        vec3<i32>(-1,  0,  0),
        vec3<i32>( 0, -1,  0),
        vec3<i32>( 0,  0, -1),
    );

    return array<vec3<i32>, 3>(
        u_axes[face],
        v_axes[face],
        n_axes[face]
    );
}

fn calc_voxel_face_vertex(face_data: VoxelFaceData, basis: array<vec3<i32>, 3>, uv: vec2<i32>) -> vec3<i32> {
    let face_offset = abs(basis[0])*face_data.pos_face.x + abs(basis[1])*face_data.pos_face.y + abs(basis[2])*face_data.pos_face.z;
    let offset = vec3<i32>(1, 1, 1) - component_max_3(basis[0] + basis[1] + basis[2], 0);
    return face_offset + offset + basis[0]*uv.x + basis[1]*uv.y;
}

fn component_max_3(a: vec3<i32>, b: i32) -> vec3<i32> {
    return vec3<i32>(
        max(a.x, b),
        max(a.y, b),
        max(a.z, b)
    );
}

// // /////////////// // //
// // Fragment Shader // //
// // /////////////// // //

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let factor = dot(
        normalize(vec3<f32>(1.0, 0.5, 0.25)), 
        abs(normalize(in.normal))
    );

    return vec4<f32>(
        pow((factor * 0.02) + in.colour.x*(1.0 - factor * 0.02), 2.2), 
        pow((factor * 0.02) + in.colour.y*(1.0 - factor * 0.02), 2.2), 
        pow((factor * 0.02) + in.colour.z*(1.0 - factor * 0.02), 2.2), 
        1.0
    );
}

