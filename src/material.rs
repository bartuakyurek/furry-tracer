/*

    Declare Material trait, and store data related to
    different types of materials. Currently supporting:
        - Diffuse
        - Mirror
        - Conductor (TBI)
        - Dielectric (TBI)

    @date: Oct, 2025
    @author: Bartu

*/
use std::fmt::Debug;
use std::cmp::max;
use tracing::{error, info};
use serde::{Deserialize, de::DeserializeOwned};
use crate::json_parser::*;
use crate::numeric::{approx_zero, Float, Vector3};
use crate::lights::LightContext;
use crate::ray::{Ray, HitRecord}; // TODO: rename it to light or lighting, not lights?

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// MATERIAL TRAIT
/// 
/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Material : Debug + Send + Sync  {
    
    fn new_from(value: &serde_json::Value) -> Self 
    where
        Self: Sized + DeserializeOwned + Default,
    {
        match serde_json::from_value::<Self>(value.clone()) {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to parse DiffuseMaterial: {e}. JSON: {value}");
                Self::default()
            }
        }
    }
    fn ambient_radiance(&self, ambient_light: Vector3) -> Vector3; // TODO: whould ambient_shade be a better name? 
    fn radiance(&self, light_context: &LightContext) -> Vector3;
    fn get_type(&self) -> &str;
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3, rays_out: &mut Vec<Ray>) -> bool;
}

pub type HeapAllocMaterial = Box<dyn Material>; // Box, Rc, Arc -> Probably will be Arc when we use rayon

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// DIFFUSE
/// 
/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct DiffuseMaterial {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient_rf: Vector3,
    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse_rf: Vector3,
    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular_rf: Vector3,
    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exponent: Float,
}


impl Default for DiffuseMaterial {
    fn default() -> Self {
        DiffuseMaterial {
            _id: 0,
            ambient_rf: Vector3::new(0.0, 0.0, 0.0),
            diffuse_rf: Vector3::new(1.0, 1.0, 1.0),
            specular_rf: Vector3::new(0.0, 0.0, 0.0),
            phong_exponent: 1.0,
        }
    }
}

impl DiffuseMaterial {

    fn diffuse(&self, light_context: &LightContext) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.73)
        // TODO: reduce the verbosity here
        let w_i = light_context.out_dir;
        debug_assert!(w_i.is_normalized());
        let n = light_context.normal;
        debug_assert!(n.is_normalized());

        let cos_theta = w_i.dot(n).max(0.0);
        self.diffuse_rf * cos_theta * light_context.irradiance
    }

    fn specular(&self, light_context: &LightContext) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.80)
        let w_o = light_context.view_dir;
        let w_i = light_context.out_dir;
        debug_assert!(w_o.is_normalized());
        debug_assert!(w_i.is_normalized());

        let h = (w_i + w_o).normalize(); //(w_i + w_o) / (w_i + w_o).norm();
        let n = light_context.normal;
        debug_assert!(h.is_normalized());
        debug_assert!(n.is_normalized());

        let p = self.phong_exponent;
        let cos_a = n.dot(h).max(0.0);
        self.specular_rf * cos_a.powf(p) * light_context.irradiance
    }   
}


impl Material for DiffuseMaterial{

    fn get_type(&self) -> &str {
        "diffuse"
    }

    fn radiance(&self, light_context: &LightContext) -> Vector3 {
        self.diffuse(light_context) + self.specular(light_context)
    }

    fn ambient_radiance(&self, ambient_light: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.75)
        // e.g. for test.json it is [25, 25, 25]
        self.ambient_rf * ambient_light // * 10. -> this was to debug there exists ambient light
    }

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3, rays_out: &mut Vec<Ray>) -> bool {
        // TODO: shadow rays here
        false
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// MIRROR
/// 
/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct MirrorMaterial {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient_rf: Vector3,
    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse_rf: Vector3,
    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular_rf: Vector3,
    #[serde(rename = "MirrorReflectance", deserialize_with = "deser_vec3")]
    pub mirror_rf: Vector3,
    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exponent: Float,
}

impl Default for MirrorMaterial {
    fn default() -> Self {
        Self {
            _id: 0,
            ambient_rf: Vector3::new(0.0, 0.0, 0.0),
            diffuse_rf: Vector3::new(0.5, 0.5, 0.5),
            specular_rf: Vector3::new(0.0, 0.0, 0.0),
            mirror_rf: Vector3::new(0.5, 0.5, 0.5),
            phong_exponent: 1.0,
        }
    }
}

// TODO: apologies for the duplicate code here (copy pasted from Diffuse Material above)
impl MirrorMaterial {

    fn diffuse(&self, light_context: &LightContext) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.73)
        // TODO: reduce the verbosity here
        let w_i = light_context.out_dir;
        debug_assert!(w_i.is_normalized());
        let n = light_context.normal;
        debug_assert!(n.is_normalized());

        let cos_theta = w_i.dot(n).max(0.0);
        self.diffuse_rf * cos_theta * light_context.irradiance
    }

    fn specular(&self, light_context: &LightContext) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.80)
        let w_o = light_context.view_dir;
        let w_i = light_context.out_dir;
        debug_assert!(w_o.is_normalized());
        debug_assert!(w_i.is_normalized());

        let h = (w_i + w_o).normalize(); //(w_i + w_o) / (w_i + w_o).norm();
        let n = light_context.normal;
        debug_assert!(h.is_normalized());
        debug_assert!(n.is_normalized());

        let p = self.phong_exponent;
        let cos_a = n.dot(h).max(0.0);
        self.specular_rf * cos_a.powf(p) * light_context.irradiance
    }

    fn reflected_radiance(&self, light_context: &LightContext) -> Vector3 {
        // TODO: irradiance is inferred from LightContext but wouldn't it be better if Material
        // was responsible from generating the new rays given incoming ray and surface normal?
        self.mirror_rf * light_context.irradiance
    }
}

impl Material for MirrorMaterial {

    fn get_type(&self) -> &str {
        "mirror"
    }
    
    fn ambient_radiance(&self, ambient_light: Vector3) -> Vector3 {
        //info!("Computing ambient radiance for Mirror ...");
        self.ambient_rf * ambient_light 
    }

    fn radiance(&self, light_context: &LightContext) -> Vector3 {
        //info!("Computing outgoing radiance for Mirror ...");
        self.diffuse(light_context) + self.specular(light_context) + self.reflected_radiance(light_context)
    }

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3, rays_out: &mut Vec<Ray>) -> bool {
        
        let w_o = -ray_in.direction;  // TODO: This is also computed in lightcontext... 
        let w_r = -w_o + 2. * hit_record.normal * (hit_record.normal.dot(w_o));
        let new_ray = Ray::new(hit_record.point, w_r.normalize()); // TODO: is normalize necessary?
        true
    }
}

