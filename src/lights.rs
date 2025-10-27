/*
    Provide a light context to be used in shading 
    computations, that is intended to be used in
    materials, without directly taking light objects
    but working on the required context (e.g. for
    specular shading computation or any other shading
    implementation).

    TODO: How to decouple SceneLights from JSON file? and bring
    Light structs from scene.rs to here?

    @date: 27 Oct, 2025
    @author: Bartu

*/

use crate::ray::{HitRecord, Ray};
use crate::scene::{PointLight, Scene, SceneLights};
use crate::numeric::{Float, Vector3};

pub struct LightContext {
    pub eye_ray : Ray, // TODO: these could be references but that requires lifetime annotations
    pub shadow_ray: Ray, // TODO: Heap allocation?
    pub light_distance: Float,
    pub light_intensity: Vector3, // RGB
    pub normal: Vector3,
}

impl LightContext {
    //pub fn new() -> Self {
    //    Self {
    //        
    //    }
    //}

    pub fn new_from(point_light: &PointLight, hit_record: &HitRecord) -> Self {
        Self {

        }
    }
}