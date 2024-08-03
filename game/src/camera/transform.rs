// Copyright 2024 Natalie Baker // AGPLv3 //

use glam::{Mat4, Quat, Vec3};

use crate::glam_util::QuatExt;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Transform {

    pub const fn new() -> Self {
        Self{
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn looking_at(origin: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - origin).normalize();
        Self {
            position: origin,
            rotation: Quat::looking_along(forward, up),
        }
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation, self.position)
    }
    
}
