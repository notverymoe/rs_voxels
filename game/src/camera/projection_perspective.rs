// Copyright 2024 Natalie Baker // AGPLv3 //

use core::cell::Cell;

use glam::Mat4;

#[derive(Debug, Clone)]
pub struct ProjectionPerspective {
    fov:    f32,
    aspect: f32,
    near:   f32,
    far:    f32,
    matrix: Cell<Option<Mat4>>,
}

impl ProjectionPerspective {
    pub const fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fov,
            aspect,
            near,
            far,
            matrix: Cell::new(None),
        }
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.matrix.set(None);
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.matrix.set(None);
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.matrix.set(None);
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.matrix.set(None);
    }

    pub const fn fov(&self) -> f32 {
        self.fov
    }

    pub const fn aspect(&self) -> f32 {
        self.aspect
    }

    pub const fn near(&self) -> f32 {
        self.near
    }

    pub const fn far(&self) -> f32 {
        self.far
    }

    pub fn matrix(&self) -> Mat4 {
        if let Some(value) = self.matrix.get() {
            value
        } else {
            let value = Mat4::perspective_lh(self.fov, self.aspect, self.near, self.far);
            self.matrix.set(Some(value));
            value
        }
    }
}

