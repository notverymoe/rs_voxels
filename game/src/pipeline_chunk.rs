// Copyright 2024 Natalie Baker // AGPLv3 //

use crate::wgpu_util::{ShaderModuleExt, PRIMITIVE_STATE_TRIANGLES};

pub struct PipelineChunk {
    pub pipeline: wgpu::RenderPipeline,
}

impl PipelineChunk {

    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        camera_bind_group_layout: wgpu::BindGroupLayout,
        vertex_bind_group_layout: wgpu::BindGroupLayout,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("pipeline_chunk.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Chunk Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &vertex_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline Chunk"),
            layout: Some(&render_pipeline_layout),
            vertex: shader.create_vertex_state("vs_main", &[]),
            fragment: Some(shader.create_fragment_state("fs_main", &[Some(wgpu::ColorTargetState {
                format: target_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })])),
            primitive: PRIMITIVE_STATE_TRIANGLES,
            depth_stencil: Some(wgpu::DepthStencilState { 
                format: wgpu::TextureFormat::Depth24Plus, 
                depth_write_enabled: true, 
                depth_compare: wgpu::CompareFunction::Less, 
                stencil: wgpu::StencilState::default(), 
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false, 
            },
            multiview: None,
        });

        Self { 
            pipeline: render_pipeline 
        }
    }

}
