// Copyright 2024 Natalie Baker // AGPLv3 //

mod projection_perspective;
pub use projection_perspective::*;

mod transform;
pub use transform::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub projection: ProjectionPerspective,
    pub view:       Transform,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl From<Camera> for CameraUniform {
    fn from(value: Camera) -> Self {
        let projection = value.projection.matrix();
        let view       = value.view.matrix();
        let view_proj  = projection * view.inverse();
        Self{
            view_proj: view_proj.to_cols_array_2d()
        }
    }
}