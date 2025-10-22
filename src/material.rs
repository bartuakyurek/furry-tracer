

use tracing::{warn, error};
use serde::Deserialize;
use crate::json_parser::*;
use crate::numeric::{Float, Int, Index, Vector3};

// pub enum MaterialEnum {
//     Diffuse,
//     MirrorLike,// 
// }

// impl MaterialEnum {
//     fn radiance(&self) {
//         // TODO: Match self and return radiance 
//     }// 
// }

pub trait Material : std::fmt::Debug + Send + Sync {
    // todo: fn radiance() 
}

pub type BoxedMaterial = Box<dyn Material>;


#[derive(Debug, Deserialize, Clone)]
pub struct DiffuseMaterial {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: Index,
    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient: Vector3,
    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse: Vector3,
    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular: Vector3,
    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exponent: Float,
}

impl DiffuseMaterial {
    pub fn new_from(value: &serde_json::Value) -> Self {
        match serde_json::from_value::<DiffuseMaterial>(value.clone()) {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to parse DiffuseMaterial: {e}. JSON: {value}");
                DiffuseMaterial::default()
            }
        }
    }
}


impl Default for DiffuseMaterial {
    fn default() -> Self {
        DiffuseMaterial {
            _id: 0,
            ambient: Vector3::new(0.0, 0.0, 0.0),
            diffuse: Vector3::new(1.0, 1.0, 1.0),
            specular: Vector3::new(0.0, 0.0, 0.0),
            phong_exponent: 1.0,
        }
    }
}


impl Material for DiffuseMaterial{

}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// ///////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MirrorMaterial {
    pub id: Int,
    pub ambient: Vector3,
    pub diffuse: Vector3,
    pub specular: Vector3,
    pub phong_exp: Float,
    pub mirror: Vector3,
}

impl Material for MirrorMaterial {

}

// TODO: impl Default for Mirrorclear
