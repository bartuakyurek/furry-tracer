

use tracing::{warn, error};
use serde::Deserialize;
use crate::json_parser::*;
use crate::numeric::{Float, Int, Vector3};

// pub enum MaterialEnum {
//     Diffuse,
//     MirrorLike,// 
// }

// impl MaterialEnum {
//     fn radiance(&self) {
//         // TODO: Match self and return radiance 
//     }// 
// }

pub trait Material {
    // todo: fn radiance() 
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiffuseMaterial {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient: Vector3,

    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse: Vector3,

    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular: Vector3,

    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exp: Float,
}

impl Material for DiffuseMaterial{

}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// ///////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
pub struct MirrorMaterial {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient: Vector3,

    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse: Vector3,

    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular: Vector3,

    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exp: Float,

    #[serde(rename = "MirrorReflectance", deserialize_with = "deser_vec3")]
    pub mirror: Vector3,
}

impl Material for MirrorMaterial {

}

pub type BoxedMaterial = Box<dyn Material>;

#[derive(Debug, Deserialize)]
pub struct SceneMaterials {
    #[serde(rename = "Material")]
    pub materials: Vec<serde_json::Value>, // parse raw JSON first
}


fn parse_material(value: serde_json::Value) -> BoxedMaterial {
    if let Some(t) = value.get("_id") {
        match t.as_str().unwrap() {
            "1" => Box::new(serde_json::from_value::<DiffuseMaterial>(value).unwrap()),
            "2" => Box::new(serde_json::from_value::<MirrorMaterial>(value).unwrap()),
            _ => {
                error!("Unknown material type with _id = {} encountered! Material reverted to default.", t.as_str().unwrap());
                Box::new(serde_json::from_value::<DiffuseMaterial>(value).unwrap())
             } 
        }
    } else {
        Box::new(serde_json::from_value::<DiffuseMaterial>(value).unwrap())
    }
}
