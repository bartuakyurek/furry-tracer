/*

    Aggregate geometry utilities on Shapes
    and cache them in a struct. WARNING: None
    of the structs are actually used at the moment.
    
    TODO:This module is intended to operate on libigl-like data
    expecting matrices V, F for verts and faces to compute
    normals and other useful cache for ray tracing.
    
    @date: 9 Oct, 2025
    @author: bartu
*/

use std::{ops::Index};
use crate::shapes::{Triangle};
use crate::numeric::{Float, Vector3};

pub fn get_tri_normal(v1: &Vector3, v2: &Vector3, v3: &Vector3) -> Vector3{
    // WARNING: Assumes triangle indices are given in counter clockwise order 
    //
    //    v1
    //  /    \
    // v2 —— v3
    //
    let left = v1 - v2;
    let right = v3 - v2;
    let mut normal = right.cross(left); 
    normal = normal.normalize();
    
    debug_assert_eq!(normal.length(), 1.0);
    normal
}


trait StructofArrays {
    type Item;

    fn vectorize(&self) -> Vec<Self::Item>; // Convert to AoS
    fn len(&self) -> usize;
}

pub struct CoordLike {
    // Struct of Arrays for 3D coordinates-like data
    // Useful for holding vertex coordinates or face normals etc.
    // WARNING: Assumes all fields have equal length
    xs: Vec<Float>,
    ys: Vec<Float>,
    zs: Vec<Float>,
}

impl CoordLike {
    pub fn new_from(coords: &Vec<Vector3>) -> Self {
        let xs = (0..coords.len()).map(|i| coords[i][0]).collect();
        let ys = (0..coords.len()).map(|i| coords[i][1]).collect();
        let zs = (0..coords.len()).map(|i| coords[i][2]).collect();
        Self {
            xs,
            ys,
            zs,
        }
    }
}


//impl Index<usize> for CoordLike {
//    // To access data through indexing like
//    // let some_field = DataField::default()
//    // some_field[i] = ...
//    // instead of some_field._data[i]
//    type Output = [Float; 3];
//    fn index(&self, index: usize) -> &Self::Output {
//        [self.xs[index], self.ys[index], self.xs[index]] // doesnt work because it's temporary value
//    }
//}



impl StructofArrays for CoordLike {
    type Item = Vector3;

    fn vectorize(&self) -> Vec<Self::Item> {
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


#[cfg(test)]
mod tests {
    use super::*; // access to the outer scope

    //#[test]
    //fn test_normals() {
    //    // WARNING: A simple test is provided, does not
    //    // check degenerate cases at this point.
    //    let verts: Vec<Vector3> = vec![
    //            Vector3::new(0., 0., 0.),
    //            Vector3::new(1., 0., 0.),
    //            Vector3::new(0.5, 0.5, 0.),
    //    ];
    //    let tri = Triangle { _id: 0, 
    //                indices: [0, 1, 2], 
    //                material_idx: 0, 
    //                
    //            };
    //
    //    let n_tri: usize = 20;
    //    let triangles = vec![tri; n_tri];
    //    let tri_normals_soa = CoordLike::tri_normals(&triangles, &verts);
    //    let tri_normals_aos = tri_normals_soa.vectorize();
    //    assert_eq!(tri_normals_aos, vec![Vector3::new(0.,0.,1.); n_tri]);
    //}
}