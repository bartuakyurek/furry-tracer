

use serde::{Deserialize};
use crate::numeric::{Int, Float, Vector3};
use crate::json_parser::*;


#[derive(Debug, Deserialize, Clone)]
pub struct Triangle {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "Indices", deserialize_with = "deser_usize_vec")]
    pub indices: Vec<usize>,

    #[serde(rename = "Material", deserialize_with = "deser_int")]
    pub material: Int,

    #[serde(skip)]
    pub normal: Vector3,
}

impl Triangle {
    fn compute_normal(&mut self, vertices: &Vec<Vector3>) -> Vector3 {
        // WARNING: Assumes triangle indices are given in counter clockwise order 
        //
        //    v1
        //  /    \
        // v2 —— v3
        //
        let v1 = vertices[self.indices[0]];
        let v2 =  vertices[self.indices[1]];
        let v3 = vertices[self.indices[2]];

        let left = v1 - v2;
        let right = v3 - v2;
        let mut normal = right.cross(left); 
        normal = normal.normalize();
        debug_assert_eq!(normal.length(), 1.0);

        self.normal = normal;
        return self.normal;
    }
}


//trait StructofArrays_3D {
//    fn get_vec() -> Vec<Vector3> {
//        // Return list of (x, y, z) coordinates
//        // i.e. convert to array of structs 
//    }
//}

//pub struct TriangleNormals {
//
//    pub xs: Vec<Float>,
//    pub ys: Vec<Float>,
//    pub zs: Vec<Float>,
//}


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
