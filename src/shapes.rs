

use serde::{Deserialize};
use crate::numeric::{Int, Float};
use crate::json_parser::*;


#[derive(Debug, Deserialize, Clone)]
pub struct Triangle {
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

    // JSON uses a *vertex index* instead of a raw vector
    #[serde(rename = "Center", deserialize_with = "deser_int")]
    pub center: Int,

    #[serde(rename = "Radius", deserialize_with = "deser_float")]
    pub radius: Float,

    #[serde(rename = "Material", deserialize_with = "deser_int")]
    pub material: Int,
}

