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
use std::cmp::max;
use tracing::{error};
use serde::Deserialize;
use crate::json_parser::*;
use crate::numeric::{Float, Vector3};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// MATERIAL TRAIT
/// 
/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Material : std::fmt::Debug + Send + Sync  {
    // TODO: fn radiance() 

    fn new_from(value: &serde_json::Value) -> Self 
    where
        Self: Sized + serde::de::DeserializeOwned + Default,
    {
        match serde_json::from_value::<Self>(value.clone()) {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to parse DiffuseMaterial: {e}. JSON: {value}");
                Self::default()
            }
        }
    }

    fn radiance(&self, perp_irradiance: Vector3) -> Vector3;
}

pub type BoxedMaterial = Box<dyn Material>;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// DIFFUSE
/// 
/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
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

    fn diffuse(&self, light_intensity: Vector3, light_distance: Float, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.73)
        let cos_theta = w_i.dot(n).max(0.0);
        let received_irradiance = light_intensity / light_distance.powi(2);
        self.diffuse_rf * cos_theta * received_irradiance
    }

    fn ambient(&self, ambient_radiance: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.75)
        self.ambient_rf * ambient_radiance
    }

    fn specular(&self, n: Vector3, h: Vector3, received_irradiance: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.80)
        let p = self.phong_exponent;
        let cos_a = n.dot(h).max(0.0);
        self.specular_rf * cos_a.powf(p) * received_irradiance
    }   
}


impl Material for DiffuseMaterial{
    fn radiance(&self, perp_irradiance: Vector3) -> Vector3 {
        Vector3::new(0.0, 0., 0.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// MIRROR
/// 
/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
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


impl Material for MirrorMaterial {
    fn radiance(&self, perp_irradiance: Vector3) -> Vector3 {
        Vector3::new(0.0, 0., 0.0)
    }
}

// TODO: impl Default for Mirrorclear
