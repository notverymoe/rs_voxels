// Copyright 2024 Natalie Baker // AGPLv3 //

use glam::{Vec3, Mat3, Quat};

pub trait QuatExt {
    fn looking_along(forward: Vec3, up: Vec3) -> Quat;

}

impl QuatExt for Quat {

    fn looking_along(forward: Vec3, up: Vec3) -> Quat {
        let s = forward.cross(up).try_normalize().unwrap_or_else(|| up.any_orthogonal_vector());
        let u = s.normalize().cross(forward);
        Quat::from_mat3(&Mat3::from_cols(-s, u, forward))
    }
}

