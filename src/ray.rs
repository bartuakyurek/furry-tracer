


use bevy_math::curve::derivatives;

use crate::material;
use crate::numeric::{Vector3, Float};
use crate::interval::{Interval};


#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn at(&self, t: Float) -> Vector3 {
        self.origin + self.direction * t // r(t) = o + dt
    }
}


// Question: Couldn't we just use point to see which point is closer?
// but this is relative to camera, and t is a single scalar that encaptures
// which HitRecord is closer to the - actually not to camera (for primary rays only it is camera)
// but ray origin so t=0 is at ray origin, smaller t is, closer the object is.  
//
// DISCLAIMER: This struct is based on the approach presented in Ray Tracing in One Weekend book.
#[derive(Debug, Default)] // TODO: Remove default! 
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub ray_t: Float,  // To check which HitRecord has smaller t 
    pub material: usize, // TODO: Should we hold the index of material or actually Option<Rc<dyn Material>> as in here https://the-ray-tracing-road-to-rust.vercel.app/9-metal? Or Arc instead of Rc if we use rayon in future.
    pub is_front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vector3, normal: Vector3, ray_t: Float, material: usize, is_front_face: bool) -> Self {
        Self {
            point,
            normal,
            ray_t,
            material,
            is_front_face,
        }
    }
    pub fn new_from(ray: &Ray, n: Vector3, t: Float, material: usize) -> Self {
        Self {
            point: ray.at(t),
            normal: n,
            ray_t: t,
            material: material, 
            is_front_face: false,
        }
    }
}