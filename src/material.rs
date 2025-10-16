
use std::fmt::{Debug};
use tracing::{warn, error};
use crate::numeric::{Float, Int, Vector3};


pub type BoxedMaterial = Box<dyn Material>;

pub trait Material: Send + Sync + Debug {
    // todo: fn radiance() 
}

#[derive(Debug, Clone)]
pub struct DiffuseMaterial {
    pub id: Int,
    pub ambient: Vector3,
    pub diffuse: Vector3,
    pub specular: Vector3,
    pub phong_exp: Float,
}

#[derive(Debug, Clone)]
pub struct MirrorMaterial {
    pub id: Int,
    pub ambient: Vector3,
    pub diffuse: Vector3,
    pub specular: Vector3,
    pub phong_exp: Float,
    pub mirror: Vector3,
}



