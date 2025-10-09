

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

fn get_tri_normal(v1: &Vector3, v2: &Vector3, v3: &Vector3) -> Vector3{

    let left = v1 - v2;
    let right = v3 - v2;
    let mut normal = right.cross(left); 
    normal = normal.normalize();
    
    debug_assert_eq!(normal.length(), 1.0);
    normal
}

trait Vector3StructofArrays {
    fn vectorize(&self) -> Vec<Vector3>; // Convert to AoS
    fn len(&self) -> usize;
}

pub struct CoordLike {
    // Struct of Arrays for 3D coordinates-like data
    // Useful for holding vertex coordinates or face normals etc.
    xs: Vec<Float>,
    ys: Vec<Float>,
    zs: Vec<Float>,
}

impl Vector3StructofArrays for CoordLike {
    fn vectorize(&self) -> Vec<Vector3> {
        (0..self.len()).map(|i| Vector3::new(self.xs[i], self.ys[i], self.zs[i])).collect()
    }

    fn len(&self) -> usize{
        // Check if all vectors have same size
        // Return length of the struct
        assert_eq!(self.xs.len(), self.ys.len());
        assert_eq!(self.xs.len(), self.zs.len());
        assert_eq!(self.ys.len(), self.zs.len());

        self.xs.len()
    }
}

impl CoordLike {
   
    fn tri_normals(triangles: &Vec<Triangle>, vertices: &Vec<Vector3>) -> CoordLike {
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

            let n = get_tri_normal(&v1, &v2, &v3);
            (xs[i], ys[i], zs[i]) = (n[0], n[1], n[2]);
        }
       
        CoordLike { xs, ys, zs }
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
        let tri_normals = CoordLike::tri_normals(&triangles, &verts);
        assert_eq!(tri_normals.xs[0], 0.);
        assert_eq!(tri_normals.ys[0], 0.);
        assert_eq!(tri_normals.zs[0], 1.);
        assert_eq!(tri_normals.xs[1], 0.);
        assert_eq!(tri_normals.ys[1], 0.);
        assert_eq!(tri_normals.zs[1], 1.);
    }
}
