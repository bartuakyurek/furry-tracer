

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
}


//trait StructofArrays_3D {
//    fn get_vec() -> Vec<Vector3> {
//        // Return list of (x, y, z) coordinates
//        // i.e. convert to array of structs 
//    }
//}

fn get_tri_normal(v1: Vector3, v2: Vector3, v3: Vector3) -> Vector3{

    let left = v1 - v2;
    let right = v3 - v2;
    let mut normal = right.cross(left); 
    normal = normal.normalize();
    
    debug_assert_eq!(normal.length(), 1.0);
    normal
}

pub struct TriangleNormals {

    pub xs: Vec<Float>,
    pub ys: Vec<Float>,
    pub zs: Vec<Float>,
}


impl TriangleNormals {
   
    fn compute(triangles: &Vec<Triangle>, vertices: &Vec<Vector3>) -> TriangleNormals {
        // WARNING: Assumes triangle indices are given in counter clockwise order 
        //
        //    v1
        //  /    \
        // v2 —— v3
        //
        let len = triangles.len();
        let mut xs: Vec<Float> = vec![0.; len];
        let mut ys: Vec<Float> = vec![0.; len];
        let mut zs: Vec<Float> = vec![0.; len];

        for (i, tri) in triangles.iter().enumerate()  {
            let v1 = vertices[tri.indices[0]];
            let v2 =  vertices[tri.indices[1]];
            let v3 = vertices[tri.indices[2]];

            let n = get_tri_normal(v1, v2, v3);
            (xs[i], ys[i], zs[i]) = (n[0], n[1], n[2]);
        }
       
        TriangleNormals { xs, ys, zs }
    }
    
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
        let tri = Triangle { id: 0, 
                    indices: vec![0, 1, 2], 
                    material: 0, 
                };

        let triangles = vec![tri; 2];
        let tri_normals = TriangleNormals::compute(&triangles, &verts);
        assert_eq!(tri_normals.xs[0], 0.);
        assert_eq!(tri_normals.ys[0], 0.);
        assert_eq!(tri_normals.zs[0], 1.);
        assert_eq!(tri_normals.xs[1], 0.);
        assert_eq!(tri_normals.ys[1], 0.);
        assert_eq!(tri_normals.zs[1], 1.);
    }
}
