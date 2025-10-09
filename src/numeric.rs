/*

    Declare numeric types used throughout this repo.

    WARNING: If you like to use f32 instead of f64
    during computations, you need to change both of these:
    pub type Float = f32;
    pub type Vector3 = Vec3;

    TODO: maybe provide Vector3 struct to avoid this
    explicit coupling rather than depending on bevy_math.

    @date: 2 Oct, 2025
    @author: Bartu
*/

use bevy_math::{DVec3, U8Vec3};

pub type RGB = U8Vec3;  // For final RGB colors
pub type Int = i32;
pub type Float = f64;
pub type Vector3 = DVec3; // f64


