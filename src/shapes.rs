

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
    //fn new(i1: usize, i2: usize, i3: usize) -> Triangle {
    //    let tri = Triangle { id: 0, 
    //                indices: vec![i1, i2, i3], 
    //                material: 0, 
    //                normal: Vector3::new(0., 0., 0.) 
    //            }
    //    
    //    
    //}

    fn compute_normal_naive(&mut self, vertices: &Vec<Vector3>) -> Vector3 {
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

#[cfg(test)]
mod tests {
    use super::*; // access to the outer scope

    #[test]
    fn test_normals() {
        // WARNING: A simple test is provided, does not
        // check degenerate cases at this point.
        let verts: Vec<Vector3> = vec![
                Vector3::new(0., 0., 0.),
                Vector3::new(1., 0., 0.),
                Vector3::new(0.5, 0.5, 0.),
        ];
        let mut tri = Triangle { id: 0, 
                    indices: vec![0, 1, 2], 
                    material: 0, 
                    normal: Vector3::new(0., 0., 0.) 
                };

        assert_eq!(tri.compute_normal_naive(&verts), Vector3::new(0., 0., 1.));
    }
}
