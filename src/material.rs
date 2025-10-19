

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

pub trait Material : std::fmt::Debug + Send + Sync {
    // todo: fn radiance() 
}

pub type BoxedMaterial = Box<dyn Material>;


#[derive(Debug, Deserialize, Clone)]
pub struct DiffuseMaterial {
    pub id: Int,
    pub ambient: Vector3,
    pub diffuse: Vector3,
    pub specular: Vector3,
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
    pub id: Int,
    pub ambient: Vector3,
    pub diffuse: Vector3,
    pub specular: Vector3,
    pub phong_exp: Float,
    pub mirror: Vector3,
}

impl Material for MirrorMaterial {

}

