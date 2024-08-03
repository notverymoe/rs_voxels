// Copyright 2024 Natalie Baker // AGPLv3 //

use winit::{error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::WindowId};
use wgpu::util::DeviceExt;

use nvm_app::{ActiveApplication, ApplicationShim, WGPUConfig, WGPUState};
use nvm_v3d::{lighting::{light_blocklight_raise_batched, LightStorageWorld}, tiles::TileIdentifier, world::{PosChunk, PosWorld}};

mod pipeline_chunk;
mod wgpu_util;
mod camera;
mod glam_util;
mod texture_group;
mod vox_util;

use camera::{Camera, CameraUniform, ProjectionPerspective, Transform};
use glam::{Vec2, Vec3};
use pipeline_chunk::PipelineChunk;
use texture_group::TextureInfo;
use vox_util::{mesh_chunk, read_vox, write_lighting_data_to_image};

fn main() -> Result<(), EventLoopError> {
    env_logger::init();

    let args: Vec<_> = std::env::args().skip(1).collect();
    let chunk = read_vox(&args[0]);
    let mesh_data = mesh_chunk(&chunk);
    
    let mut light_data = LightStorageWorld::default();
    light_blocklight_raise_batched(
        &mut |pos, _| if chunk.get(pos.to_chunk_and_block().1) == TileIdentifier::DEFAULT { 0 } else { u8::MAX },
        &mut light_data, 
        &[
            (PosWorld::new(12, 12, 15), [31,  0,  0]),
            (PosWorld::new(12, 20, 15), [ 0, 31,  0]),
            (PosWorld::new(20, 12, 15), [ 0,  0, 31]),
            (PosWorld::new(20, 20, 15), [31, 31, 31]),
        ]
    );

    write_lighting_data_to_image("./out.png", &light_data, PosChunk::new(0,0,0), 8);

    let event_loop = EventLoop::new().unwrap();
    let mut app = ApplicationShim::<Application, ApplicationConfig>::new(ApplicationConfig{
        mesh_data,
        light_data: Vec::from(light_data.get_chunk(PosChunk::new(0, 0, 0)).unwrap().get_data()),
    });
    event_loop.run_app(&mut app)
}

pub struct ApplicationConfig {
    pub mesh_data: Vec<u32>,
    pub light_data: Vec<[u8; 4]>,
}

pub struct Application {
    wgpu: WGPUState,

    pipeline_chunk: PipelineChunk,
    
    mesh_buffer: wgpu::Buffer,

    chunk_bind_group: wgpu::BindGroup,

    camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    depth_texture: TextureInfo,

    t: f32,
}

impl ActiveApplication<ApplicationConfig> for Application {
    #[allow(clippy::too_many_lines)]
    async fn new(event_loop: &ActiveEventLoop, config: &ApplicationConfig) -> Self {
        let wgpu = WGPUState::new(event_loop, &mut WGPUConfig::default()).await;

        let mesh_buffer = wgpu.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("mesh_buffer"),
                contents: bytemuck::cast_slice(&config.mesh_data),
                usage: wgpu::BufferUsages::STORAGE,
            }
        );
        let light_buffer = wgpu.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("light_buffer"),
                contents: bytemuck::cast_slice(&config.light_data),
                usage: wgpu::BufferUsages::STORAGE,
            }
        );
        let chunk_bind_group_layout = wgpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("chunk_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let chunk_bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("chunk_bind_group"),
            layout: &chunk_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: mesh_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: light_buffer.as_entire_binding(),
                },
            ],
        });

        let camera = Camera{
            projection: ProjectionPerspective::new(60_f32.to_radians(), (wgpu.size.width as f32)/(wgpu.size.height as f32), 0.1, 100.0),
            view: Transform::looking_at(Vec3::new(32.0, 32.0, 32.0), Vec3::ZERO, Vec3::Y),
        };
        let camera_buffer = wgpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[CameraUniform::from(camera.clone())]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout = wgpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let camera_bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
        });
        
        let pipeline_chunk = PipelineChunk::new(&wgpu.device, wgpu.config.format, camera_bind_group_layout, chunk_bind_group_layout);

        let depth_texture = TextureInfo::create_depth_texture(&wgpu.device, &wgpu.config, "depth_texture");

        Self{
            wgpu,
            pipeline_chunk,
            camera,
            camera_buffer,
            camera_bind_group,
            mesh_buffer,
            chunk_bind_group,
            depth_texture,
            t: 0.0,
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        
        self.wgpu.window.request_redraw();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.wgpu.resize(physical_size);
                self.resize();
            }
            WindowEvent::RedrawRequested => {
                self.update();
                match self.render() {
                    Ok(()) => {}
                    Err(wgpu::SurfaceError::Lost       ) => self.wgpu.resize(self.wgpu.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("{e:?}"),
                }
            }
            _ => {}
        }
    }
}

impl Application {

    pub fn resize(&mut self) {
        self.camera.projection.set_aspect((self.wgpu.size.width as f32)/(self.wgpu.size.height as f32));
        self.depth_texture = TextureInfo::create_depth_texture(&self.wgpu.device, &self.wgpu.config, "depth_texture");
    }

    pub fn update(&mut self) {
        self.t += 1.5_f32;
        let origin = Vec3::new(16.0, 16.0, 16.0);
        let pos = origin + 52.0 * Vec2::from_angle(self.t.to_radians()).extend(0.0);
        self.camera.view = Transform::looking_at(pos, origin, Vec3::Z);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.wgpu.surface.get_current_texture()?;
        let view   = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.wgpu.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[CameraUniform::from(self.camera.clone())]));

        let mut encoder = self.wgpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: &self.depth_texture.view, 
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.pipeline_chunk.pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.chunk_bind_group, &[]);
            render_pass.draw(0..((self.mesh_buffer.size() as u32)*6), 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.wgpu.queue.submit(core::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

}
