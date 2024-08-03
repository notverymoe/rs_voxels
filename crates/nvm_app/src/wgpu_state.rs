// Copyright 2024 Natalie Baker // AGPLv3 //

use std::sync::Arc;

use wgpu::{Adapter, Device, Queue, Surface, SurfaceCapabilities, SurfaceConfiguration, TextureFormat};
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window};

pub struct WGPUConfig {
    pub power_preference:  wgpu::PowerPreference,
    pub required_limits:   wgpu::Limits,
    pub required_features: wgpu::Features,
    pub surface_texture_selector: Box<dyn FnMut(&SurfaceCapabilities) -> TextureFormat>,
    pub desired_maximum_frame_latency: u32,
}

impl Default for WGPUConfig {
    fn default() -> Self {
        Self { 
            power_preference:  wgpu::PowerPreference::HighPerformance,
            required_limits:   wgpu::Limits::default(),
            required_features: wgpu::Features::empty(),
            surface_texture_selector: Box::new(|v| v.formats.iter().find(|v| v.is_srgb()).copied().unwrap_or(v.formats[0])),
            desired_maximum_frame_latency: 2,
        }
    }
}

pub struct WGPUState {
    pub window:  Arc<Window>,
    pub adapter: Adapter,
    pub surface: Surface<'static>,
    pub device:  Device,
    pub queue:   Queue,
    pub config:  SurfaceConfiguration,
    pub size:    PhysicalSize<u32>,
}

impl WGPUState {

    pub async fn new(event_loop: &ActiveEventLoop, config: &mut WGPUConfig) -> Self {
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
    
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
    
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: config.power_preference,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: config.required_features,
                required_limits:   config.required_limits.clone(),
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps   = surface.get_capabilities(&adapter);
        let surface_format = (config.surface_texture_selector)(&surface_caps);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width:  size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: config.desired_maximum_frame_latency,
        };

        surface.configure(&device, &config);
    
        Self{
            window,
            adapter,
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

}
