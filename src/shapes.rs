/*

    Declare primitives: Triangle, Sphere
    

    @date: Oct, 2025
    @author: bartu
*/

use serde::{Deserialize};
use crate::json_parser::*;
use crate::interval::{Interval};
use crate::dataforms::{DataField, VertexData};
use crate::numeric::{Float, Vector3, Index};
use crate::ray::{Ray, HitRecord}; // TODO: Can we create a small crate for gathering shapes.rs, ray.rs?

pub trait Intersectable {
    fn intersects_with(ray: &Ray, t_interval: &Interval, verts: &VertexData) -> Option<HitRecord>;
}

// Raw data deserialized from .JSON file
// WARNING: it assumes vertex indices start from 1
// TODO: How to convert this struct into V, F matrices, for both array of triangles and Mesh objects in the scene?
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Triangle {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: Index,
    #[serde(rename = "Indices", deserialize_with = "deser_usize_vec")]
    pub indices: Vec<usize>,
    #[serde(rename = "Material", deserialize_with = "deser_usize")]
    pub material_idx: Index,
}

impl Intersectable for Triangle {
    fn intersects_with(ray: &Ray, t_interval: &Interval, verts: &VertexData) -> Option<HitRecord> {




        None   
    }
}


#[derive(Debug, Deserialize, Clone, Default)]
pub struct Sphere {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: Index,
    #[serde(rename = "Center", deserialize_with = "deser_usize")]
    pub center_idx: Index, // Refers to VertexData
    #[serde(rename = "Radius", deserialize_with = "deser_float")]
    pub radius: Float,
    #[serde(rename = "Material", deserialize_with = "deser_usize")]
    pub material_idx: Index,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Plane {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: Index,
    #[serde(rename = "Point", deserialize_with = "deser_usize")]
    pub point_idx: Index,
    #[serde(rename = "Normal", deserialize_with = "deser_vec3")]
    pub normal: Vector3,
    #[serde(rename = "Material", deserialize_with = "deser_usize")]
    pub material_idx: Index,
}


#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Mesh {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: Index,
    #[serde(rename = "Material", deserialize_with = "deser_usize")]
    material_idx: Index,
    #[serde(rename = "Faces")]
    faces: DataField<Index>,
}

impl Mesh {
    // to_triangles ( )
}
