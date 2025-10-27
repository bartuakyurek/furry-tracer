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
    pub view_dir : Vector3, // w_o in slides TODO: these could be references but that requires lifetime annotations
    pub out_dir: Vector3, // TODO: Heap allocation?
    pub normal: Vector3,
    pub irradiance: Vector3, // Cache for 1 / distance_squared
}

impl LightContext {
   
    pub fn from_shadow(point_light: &PointLight, light_distance: Float, shadow_dir: Vector3, eye_dir: Vector3, normal: Vector3) -> Self {
        // w_o direction of - eye ray
        // w_i direction of shadow ray
        // see slides 01_B for the notation
        debug_assert!(shadow_dir.is_normalized());
        debug_assert!(eye_dir.is_normalized());

        let light_intensity = point_light.rgb_intensity;
        let irradiance = light_intensity / light_distance.powi(2);
        Self {
            view_dir: - eye_dir,
            out_dir: shadow_dir,
            normal,
            irradiance,
        }
    }

    pub fn from_mirror(eye_dir: Vector3, normal: Vector3, irradiance: Vector3) -> Self {
        // See Slides 02, p.4
        // WARNING: eye_dir is assumed to be the incoming view ray, s.t. w_o = -eye_dir
        let w_o = -eye_dir;  
        let w_r = -w_o + 2. * normal * (normal.dot(w_o));
        debug_assert!(eye_dir.is_normalized());
        Self {
            view_dir: w_o,
            out_dir: w_r,
            normal,
            irradiance,
        }
    }
}