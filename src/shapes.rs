/*

    Declare primitives: Triangle, Sphere
    
    @date: Oct, 2025
    @author: bartu
*/

use void::Void;
use std::str::FromStr;
use serde::{Deserialize};

use crate::dataforms::DataField;
use crate::numeric::{Int, Float, Vector3, Index};
use crate::json_parser::*;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}


pub trait Intersectable {
    fn intersects_with(ray: Ray) -> bool;
}


pub type VertexData = DataField<Vector3>; // use CoordLike in geometry_processing.rs?

// DISCLAIMER: This function is taken from
// https://serde.rs/string-or-struct.html
impl FromStr for VertexData {
    // This implementation of `from_str` can never fail, so use the impossible
    // `Void` type as the error type.
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DataField::<Vector3>{
            _data: parse_string_vecvec3(s).unwrap(),
            _type: String::from("xyz"), // Default for VertexData (Note: it would be different from other DataFields)
        })
    }
}

// Raw data deserialized from .JSON file
// it assumes vertex indices start from 1
#[derive(Debug, Deserialize, Clone, Default)]
pub struct TriangleSerde {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "Indices", deserialize_with = "deser_usize_vec")]
    pub indices: Vec<usize>,

    #[serde(rename = "Material", deserialize_with = "deser_int")]
    pub material: Int,
}


#[derive(Debug, Deserialize, Clone, Default)]
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

#[derive(Debug, Deserialize, Clone, Default)]
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