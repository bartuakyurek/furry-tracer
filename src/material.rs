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
use tracing::{error, info, warn};
use serde::{Deserialize, de::DeserializeOwned};
use crate::json_parser::*;
use crate::numeric::{approx_zero, Float, Vector3};
use crate::ray::{Ray, HitRecord}; // TODO: rename it to light or lighting, not lights?

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// MATERIAL TRAIT
/// 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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
    fn get_type(&self) -> &str;
    fn diffuse(&self, w_i: Vector3, n: Vector3) -> Vector3;
    fn specular(&self, w_o: Vector3, w_i: Vector3, n: Vector3) -> Vector3;
    fn ambient(&self) -> Vector3; 

    //fn get_attenuiation(&self, ray_in: &Ray, ray_out: &mut Option<Ray>, hit_record: &HitRecord) -> Vector3;
    fn attenuate(&self) -> Vector3;
    fn reflect(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray>;
    fn refract(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray>;
}

pub type HeapAllocMaterial = Box<dyn Material>; // Box, Rc, Arc -> Probably will be Arc when we use rayon

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// DIFFUSE
/// 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

    
}


impl Material for DiffuseMaterial{


    fn get_type(&self) -> &str {
        "diffuse"
    }

    fn reflect(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray> {
        warn!("Reflect not implemented for Diffuse! Only use shadow rays for now.");
        todo!()
    }

    fn refract(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray> {
        warn!("There is no refract for DiffuseMaterial. If this is intentional please delete this warning.");
        None
    }

    fn attenuate(&self) -> Vector3 {
        warn!("Attenuate not implemented for Diffuse! Only use shadow rays for now.");
        todo!()
    }

    fn ambient(&self) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.75)
        // e.g. for test.json it is [25, 25, 25]
        self.ambient_rf 
    }

    fn diffuse(&self, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.73)
        // TODO: reduce the verbosity here
        
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let cos_theta = w_i.dot(n).max(0.0);
        self.diffuse_rf * cos_theta 
    }

    fn specular(&self, w_o: Vector3, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.80)
        debug_assert!(w_o.is_normalized());
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let h = (w_i + w_o).normalize(); //(w_i + w_o) / (w_i + w_o).norm();
        debug_assert!(h.is_normalized());
        
        let p = self.phong_exponent;
        let cos_a = n.dot(h).max(0.0);
        self.specular_rf * cos_a.powf(p)
    }   
    

}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// MIRROR
/// 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

impl MirrorMaterial {

    fn reflected_radiance(&self) -> Vector3 {
        self.mirror_rf 
    }
}

impl Material for MirrorMaterial {

    fn get_type(&self) -> &str {
        "mirror"
    }

    fn attenuate(&self) -> Vector3 {
        self.reflected_radiance()
    }

    fn reflect(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray> {
        // Reflected ray from Slides 02, p.4
        // wr ​= - wo ​+ 2 n (n . wo)
        // WARNING: Assume ray_in.direction = wi = - wo
        let n = hit_record.normal;
        let w_i = ray_in.direction;
        let w_r = w_i - 2. * n * (n.dot(w_i));
        debug_assert!(w_r.is_normalized());

        Some(Ray::new(hit_record.point + (n * epsilon), w_r)) // Always reflects
    }

    fn refract(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray> {
        None // Never refract
    }
    
    fn ambient(&self) -> Vector3 {
        self.ambient_rf  
    }

    fn diffuse(&self, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.73)
        // TODO: reduce the verbosity here
        
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let cos_theta = w_i.dot(n).max(0.0);
        self.diffuse_rf * cos_theta  
    }

    fn specular(&self, w_o: Vector3, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.80)
        debug_assert!(w_o.is_normalized());
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let h = (w_i + w_o).normalize(); //(w_i + w_o) / (w_i + w_o).norm();
        debug_assert!(h.is_normalized());
        
        let p = self.phong_exponent;
        let cos_a = n.dot(h).max(0.0);
        self.specular_rf * cos_a.powf(p)  
    }   
}

