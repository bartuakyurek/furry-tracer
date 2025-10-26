/*

    Declare primitives: Triangle, Sphere
    

    @date: Oct, 2025
    @author: bartu
*/

use bevy_math::FloatOrd;
use serde::{Deserialize};
use tracing::{info, error};
use crate::json_parser::*;
use crate::interval::{Interval};
use crate::dataforms::{DataField, VertexData};
use crate::numeric::{Float, Vector3};
use crate::ray::{Ray, HitRecord}; // TODO: Can we create a small crate for gathering shapes.rs, ray.rs?

pub trait Shape {
    fn intersects_with(&self, ray: &Ray, t_interval: &Interval, verts: &VertexData) -> Option<HitRecord>;
}

// Raw data deserialized from .JSON file
// WARNING: it assumes vertex indices start from 1
// TODO: How to convert this struct into V, F matrices, for both array of triangles and Mesh objects in the scene?
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Triangle {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "Indices", deserialize_with = "deser_usize_array")]
    pub indices: [usize; 3],
    #[serde(rename = "Material", deserialize_with = "deser_usize")]
    pub material_idx: usize,
}

impl Shape for Triangle {
    fn intersects_with(&self, ray: &Ray, t_interval: &Interval, verts: &VertexData) -> Option<HitRecord> {

        // TODO: cache vertex / face normals
        // WARNING: vertex normals are tricky because if the same vertex was used by multiple 
        // meshes, that means there are more vertex normals than the length of vertexdata because
        // connectivities are different. Perhaps it is safe to assume no vertex is used in multiple
        // objects, but there needs to be function to actually check the scene if a vertex in VertexData
        // only referred by a single scene object. 
        // Furthermore, what if there were multiple VertexData to load multiple meshes in the Scene? 
        // this is not handled yet and our assumption is VertexData is the only source of vertices, every
        // shape refers to this data for their coordinates. 
        
        // Based on Möller-Trumbore algorithm
        //
        //     a (pivot)
        //    / \
        //  b  -  c
        // 
        // WARNING: Assumes given interval has incorporated relevant epsilon e.g.
        // instead of [0.0, inf], [0.0001, inf] is given otherwise there might be
        // floating point errors.
        let [tri_pivot, tri_left, tri_right] = self.indices.map(|i| verts[i]);        
        let edge_ab = tri_left - tri_pivot;
        let edge_ac = tri_right - tri_pivot;

        // Scalar triple product https://youtu.be/fK1RPmF_zjQ
        let perp = ray.direction.cross(edge_ac);
        let determinant: Float = perp.dot(edge_ab);
        if determinant > - t_interval.min && determinant < t_interval.min {
            return None;
        }

        let inverse_determinant = 1.0 as Float / determinant;
        let dist = ray.origin - tri_pivot;

        let barycentric_u = dist.dot(perp) * inverse_determinant;
        if barycentric_u < 0.0 || barycentric_u > 1.0 {
            return None;
        }

        let another_perp = dist.cross(edge_ab);
        let barycentric_v = ray.direction.dot(another_perp) * inverse_determinant;
        if barycentric_v < 0.0 || barycentric_u + barycentric_v > 1.0 {
            return None;
        }

        let t = edge_ac.dot(another_perp) * inverse_determinant;
        debug_assert!(t_interval.contains(t));
        //info!("todo: convert u,v barycentric to coords");
        Some(HitRecord::default()) 
    }
}


#[derive(Debug, Deserialize, Clone, Default)]
pub struct Sphere {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "Center", deserialize_with = "deser_usize")]
    pub center_idx: usize, // Refers to VertexData
    #[serde(rename = "Radius", deserialize_with = "deser_float")]
    pub radius: Float,
    #[serde(rename = "Material", deserialize_with = "deser_usize")]
    pub material_idx: usize,
}

impl Shape for Sphere {
    fn intersects_with(&self, ray: &Ray, t_interval: &Interval, verts: &VertexData) -> Option<HitRecord> {
        
        // Based on Slides 01_B, p.11, Ray-Sphere Intersection 
        let center = verts[self.center_idx];
        let o_minus_c = ray.origin - center;
        let d_dot_d: Float = ray.direction.dot(ray.direction);
        let oc_dot_oc: Float = o_minus_c.dot(o_minus_c);
        let d_dot_oc: Float = ray.direction.dot(o_minus_c);
        let discriminant_left: Float = d_dot_oc.powi(2) as Float;
        let discriminant_right: Float = d_dot_d * (oc_dot_oc - self.radius.powi(2)) as Float; // TODO: cache radius squared?
        let discriminant: Float = discriminant_left - discriminant_right;
        if discriminant < 0. { // Negative square root
            None
        }
        else {
            let discriminant = discriminant.sqrt();
            let t1 = (-d_dot_oc + discriminant) / d_dot_d;
            let t2 = (-d_dot_oc - discriminant) / d_dot_d;

            let t = if t1 < t2 {t1} else {t2}; // Take the closer root
            debug_assert!(t_interval.contains(t));
            Some(HitRecord::default()) // TODO: Create the actual hit record!!!
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Plane {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "Point", deserialize_with = "deser_usize")]
    pub point_idx: usize,
    #[serde(rename = "Normal", deserialize_with = "deser_vec3")]
    pub normal: Vector3,
    #[serde(rename = "Material", deserialize_with = "deser_usize")]
    pub material_idx: usize,
}

impl Shape for Plane {
    fn intersects_with(&self, ray: &Ray, t_interval: &Interval, verts: &VertexData) -> Option<HitRecord> {
        None
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Mesh {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "Material", deserialize_with = "deser_usize")]
    material_idx: usize,
    #[serde(rename = "Faces")]
    faces: DataField<usize>,
}

impl Mesh {
    // to_triangles ( )
}

impl Shape for Mesh {
    fn intersects_with(&self, ray: &Ray, t_interval: &Interval, verts: &VertexData) -> Option<HitRecord> {
        
        // TODO: debug_assert!(t_interval.contains(t));
        None
    }
}