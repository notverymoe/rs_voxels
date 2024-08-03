// Copyright 2024 Natalie Baker // AGPLv3 //

pub const PRIMITIVE_STATE_TRIANGLES: wgpu::PrimitiveState = wgpu::PrimitiveState{
    topology: wgpu::PrimitiveTopology::TriangleList,
    strip_index_format: None,
    front_face: wgpu::FrontFace::Ccw,
    cull_mode: Some(wgpu::Face::Back),
    polygon_mode: wgpu::PolygonMode::Fill,
    unclipped_depth: false,
    conservative: false,
};

pub trait ShaderModuleExt {

    fn create_vertex_state<'a>(
        &'a self,
        entry_point: &'a str,
        targets: &'a [wgpu::VertexBufferLayout<'a>],
    ) -> wgpu::VertexState<'a>;

    fn create_fragment_state<'a>(
        &'a self,
        entry_point: &'a str,
        targets: &'a [Option<wgpu::ColorTargetState>],
    ) -> wgpu::FragmentState<'a>;

}

impl ShaderModuleExt for wgpu::ShaderModule {

    fn create_vertex_state<'a>(
        &'a self,
        entry_point: &'a str,
        buffers: &'a [wgpu::VertexBufferLayout<'a>],
    ) -> wgpu::VertexState<'a> {
        wgpu::VertexState {
            module: self,
            entry_point,
            buffers,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }
    }

    fn create_fragment_state<'a>(
        &'a self,
        entry_point: &'a str,
        targets: &'a [Option<wgpu::ColorTargetState>],
    ) -> wgpu::FragmentState<'a> {
        wgpu::FragmentState {
            module: self,
            entry_point,
            targets,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }
    }

}