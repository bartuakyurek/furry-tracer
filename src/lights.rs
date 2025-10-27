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

use crate::ray::{self, HitRecord, Ray};
use crate::scene::{PointLight, Scene, SceneLights};
use crate::numeric::{Float, Vector3};
use crate::interval::{Interval};

pub struct LightContext {
    pub eye_dir : Vector3, // TODO: these could be references but that requires lifetime annotations
    pub shadow_dir: Vector3, // TODO: Heap allocation?
    //pub distance: Float,
    //pub intensity: Vector3, // RGB
    pub normal: Vector3,
    pub irradiance: Vector3, // Cache for 1 / distance_squared
}

impl LightContext {
    //pub fn new() -> Self {
    //    Self {
    //        
    //    }
    //}

    pub fn new_from(point_light: &PointLight, light_distance: Float, w_i: Vector3, w_o: Vector3, normal: Vector3) -> Self {
        // w_o direction of eye ray
        // w_i direction of shadow ray
        // see slides 01_B for the notation
        debug_assert!(w_i.is_normalized());
        debug_assert!(w_o.is_normalized());

        let light_intensity = point_light.rgb_intensity;
        let irradiance = light_intensity / light_distance.powi(2);
        Self {
            eye_dir: w_o,
            shadow_dir: w_i,
            //distance: light_distance,
            //intensity: light_intensity,
            normal,
            irradiance,
        }
    }
}