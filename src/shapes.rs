/*

    Declare primitives: Triangle, Sphere
    

    @date: Oct, 2025
    @author: bartu
*/

use bevy_math::FloatOrd;
use serde::{Deserialize};
use tracing::{info, error};
use tracing_subscriber::registry::Data;
use crate::json_parser::*;
use crate::interval::{Interval};
use crate::dataforms::{DataField, VertexData};
use crate::numeric::{Float, Vector3};
use crate::ray::{Ray, HitRecord, ray_triangle_intersection}; // TODO: Can we create a small crate for gathering shapes.rs, ray.rs?

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
        
        //info!("todo: convert u,v barycentric to coords");
        if let Some(t) = ray_triangle_intersection(ray, t_interval, self.indices, verts) {
            Some(HitRecord::default()) 
        }
        else {
            None
        }
        
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
            //Some(HitRecord::new_from(ray, t, self.material_idx)) // TODO: is this correct?
            Some(HitRecord::default())
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
       // Based on Slides 01_B, p.9, Ray-Plane Intersection 
        let a_point_on_plane = verts[self.point_idx];
        let dist = a_point_on_plane - ray.origin;
        let  t = dist.dot(self.normal) / ray.direction.dot(self.normal);

        if t_interval.contains(t) {
            Some(HitRecord::default()) // TODO: construct it 
        }
        else {
            None // t is not within the limits
        }
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

type FaceType = DataField<usize>;
impl FaceType {
    pub fn len(&self) -> usize {
        debug_assert!(self._type == "triangle"); // Only triangle meshes are supported
        (self._data.len() as f64 / 3.) as usize
    }

    pub fn get_indices(&self, i: usize) -> [usize; 3] {
        debug_assert!(self._type == "triangle");
        let start = i * 3;
        [self._data[start], self._data[start + 1], self._data[start + 2]]
    }
}

impl Mesh {
    // to_triangles ( )
}

impl Shape for Mesh {
    fn intersects_with(&self, ray: &Ray, t_interval: &Interval, verts: &VertexData) -> Option<HitRecord> {
        
        // TODO: do not iterate over triangles like this
        if self.faces._type != "triangle" {
            panic!(">> Expected triangle faces in Ray-Mesh intersection, got '{}'.", self.faces._type);
        }
        let n_faces = self.faces.len() ; //TODO: cache it?
        for i in 0..n_faces {
            
            let indices = self.faces.get_indices(i); // Assume triangle!
            if let Some(t) = ray_triangle_intersection(ray, t_interval, indices, verts) {
                return Some(HitRecord::default())  // dont forget to use t
            }
            else {
                return None
            }
            

        }
        None
    }
}