/*

    Declare primitives: Triangle, Sphere
    
    @date: Oct, 2025
    @author: bartu
*/


use serde::{Deserialize};
use crate::numeric::{Int, Float, Vector3, Index};
use crate::json_parser::*;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}


pub trait Intersectable {
    fn intersects_with(ray: Ray) -> bool;
}


// Raw data deserialized from .JSON file
// it assumes vertex indices start from 1
#[derive(Debug, Deserialize, Clone)]
pub struct TriangleSerde {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "Indices", deserialize_with = "deser_usize_vec")]
    pub indices: Vec<usize>,

    #[serde(rename = "Material", deserialize_with = "deser_int")]
    pub material: Int,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Sphere {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    // JSON uses a *vertex index* instead of a 3D vector for center
    #[serde(rename = "Center", deserialize_with = "deser_usize")]
    pub center: Index,

    #[serde(rename = "Radius", deserialize_with = "deser_float")]
    pub radius: Float,

    #[serde(rename = "Material", deserialize_with = "deser_int")]
    pub material: Int,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Plane {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "Point", deserialize_with = "deser_usize")]
    pub point: Index,

    #[serde(rename = "Normal", deserialize_with = "deser_vec3")]
    pub normal: Vector3,

    #[serde(rename = "Material", deserialize_with = "deser_int")]
    pub material: Int,
}