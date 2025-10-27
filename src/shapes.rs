/*

    Declare primitives: Triangle, Sphere, Plane
    

    @date: Oct, 2025
    @author: bartu
*/


use serde::{Deserialize};
use tracing::{info, error};
use crate::geometry::get_tri_normal;
use crate::json_parser::*;
use crate::interval::{Interval};
use crate::dataforms::{VertexData};
use crate::numeric::{Float, Vector3, Matrix3};
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
        
        //info!("todo: convert u,v barycentric to coords");
        if let Some((p, t)) = lengthy_but_simple_intersection(ray, t_interval, self.indices, verts) {
            // TODO: Cache tri normals
            // Normal of the triangle (WARNING: no vertex normal used here)
            let [v1, v2, v3] = self.indices.map(|i| verts[i]);
            let tri_normal = get_tri_normal(&v1, &v2, &v3);
            let front_face = ray.is_front_face(tri_normal);
            Some(HitRecord::new(p, tri_normal, t, self.material_idx, front_face)) 
        }
        else {
            None
        }
        
    }
}

//fn get_tri_area(a: Vector3, b: Vector3, c: Vector3) -> Float { // TODO: This belongs to geometry.rs

//}

fn get_beta_gamma_t(a: Vector3, b: Vector3, c: Vector3, o: Vector3, d: Vector3) -> (Float, Float, Float) { 
    // Helper function for computations in Slides 01_B, p.30
    // a, b, c are triangle corners
    // o, d are ray's origin and direction r(t) = o + d * t
    //
    // TODO: reduce verbosity and unnecessary operations
    // I've written in naive way for the beginning
    let (ax, ay, az) = (a[0], a[1], a[2]);
    let (bx, by, bz) = (b[0], b[1], b[2]);
    let (cx, cy, cz) = (c[0], c[1], c[2]);
    let (ox, oy, oz) = (o[0], o[1], o[2]);
    let (dx, dy, dz) = (d[0], d[1], d[2]);

    // Construct A
    let A_x = Vector3::new(ax - bx, ay - by, az - bz);
    let A_y = Vector3::new(ax - cx, ay - cy, az - cz);
    let A_z = Vector3::new(dx, dy, dz);
    let A = Matrix3::from_cols(A_x, A_y, A_z);
    let A_determinant = A.determinant();

    // Construct beta 
    let beta_x = Vector3::new(ax - ox, ay - oy, az - oz);
    let beta_y = Vector3::new(ax - cx, ay - cy, az - cz);
    let beta_z = Vector3::new(dx, dy, dx);
    let beta_matrix = Matrix3::from_cols(beta_x, beta_y, beta_z);
    let beta = beta_matrix.determinant() / A_determinant;

    // Construct gamma
    let gamma_x = Vector3::new(ax - bx, ay - by, az - bz);
    let gamma_y = Vector3::new(ax - ox, ay - oy, az - oz);
    let gamma_z = Vector3::new(dx, dy, dz);
    let gamma_matrix = Matrix3::from_cols(gamma_x, gamma_y, gamma_z);
    let gamma = gamma_matrix.determinant() / A_determinant;

    let t_x = Vector3::new(ax - bx, ay - by, az - bz);
    let t_y = Vector3::new(ax - cx, ay - cy, az - cz);
    let t_z = Vector3::new(ax - ox, ay - oy, az - oz);
    let t_matrix = Matrix3::from_cols(t_x, t_y, t_z);
    let t = t_matrix.determinant() / A_determinant;

    (beta, gamma, t)
}

fn lengthy_but_simple_intersection(ray: &Ray, t_interval: &Interval, tri_indices: [usize; 3], verts: &VertexData) -> Option<(Vector3, Float)> {
    // Slides 01_B, p.14
    //
    //  n    a  
    //   \  / \
    //     /   \
    //   b ----- c
    let [a, b, c] = tri_indices.map(|i| verts[i]);
    let edge_ba = a - b;
    let edge_ca = a - c;
    let edge_ac = c - a;
    let edge_bc = c - b;
    let edge_cb = b - c;
    let n = (edge_bc).cross(edge_ba); 

    let (beta, gamma, t) = get_beta_gamma_t(a, b, c, ray.origin, ray.direction);

    // Conditions at p.32
    if !t_interval.contains(t) {
        return None;
    }
    if (beta + gamma) <= 1. {
        return None;
    }
    if (0. <= beta) || (0. <= gamma) {
        return None;
    }

    // Construct p from barycentric coords
    let p = a + (beta * (b - a)) + (gamma * (c - a)); // p.27

    // Check for edge BA 
    let vp = (p - b).cross(edge_ba); // TODO: we can use the same vp for other checks, right?
    let vc = (edge_bc).cross(edge_ba);
    if vp.dot(vc) <= 0.0 {
        return None;
    }

    // Check for AC
    let vb = (edge_bc).cross(edge_ac);
    if vp.dot(vb) <= 0.0 {
        return None;
    }

    // Check for CB
    let va = (edge_ca).cross(edge_cb);
    if vp.dot(va) <= 0.0 {
        return None;
    }

    Some((p, t))
}

fn moller_trumbore_intersection(ray: &Ray, t_interval: &Interval, tri_indices: [usize; 3], verts: &VertexData) -> Option<(Vector3, Float)> {
    // Based on Möller-Trumbore algorithm
        //
        //     a (pivot)
        //    / \
        //  b  -  c
        // 
        // WARNING: Assumes given interval has incorporated relevant epsilon e.g.
        // instead of [0.0, inf], [0.0001, inf] is given otherwise there might be
        // floating point errors.
        // TODO: Is there something wrong in this function?
        let tri_coords = tri_indices.map(|i| verts[i]);
        let [tri_pivot, tri_left, tri_right] = tri_coords;        
        let edge_ab = tri_left - tri_pivot;
        let edge_ac = tri_right - tri_pivot;

        // Scalar triple product https://youtu.be/fK1RPmF_zjQ
        debug_assert!(ray.direction.is_normalized());
        let perp = ray.direction.cross(edge_ac);
        let determinant: Float = perp.dot(edge_ab);
        if (determinant > -t_interval.min) && (determinant < t_interval.min) {
            return None;
        }

        let inverse_determinant = 1.0 as Float / determinant;
        let dist = ray.origin - tri_pivot;

        let barycentric_u = dist.dot(perp) * inverse_determinant;
        if (barycentric_u < 0.0) || (barycentric_u > 1.0) {
            return None;
        }

        let another_perp = dist.cross(edge_ab);
        let barycentric_v = ray.direction.dot(another_perp) * inverse_determinant;
        if (barycentric_v < 0.0) || ((barycentric_u + barycentric_v) > 1.0) {
            return None;
        }

        // Get ray t
        let t = edge_ac.dot(another_perp) * inverse_determinant;
        if !t_interval.contains(t) {
            return None;
        }

        // Construct hit point p
        let p = ray.at(t); // TODO: would it be faster to use barycentric u,v here? 
        Some((p, t))
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
            let t2 = (-d_dot_oc - discriminant) / d_dot_d; // t2 < t1 
            
            let t= if t2 > 0.0 {t2} else {t1}; // Pick smaller first
            if !t_interval.contains(t) {
                return None;  // Invalid intersection
            };
            
            let point = ray.at(t); // Note that this computation is done inside new_from as well
            let normal = (point - center).normalize(); // TODO: is this correct?
            
            Some(HitRecord::new_from(ray, normal, t, self.material_idx))
            
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
            // Construct Hit Record
            Some(HitRecord::new_from(ray, self.normal, t, self.material_idx))
        }
        else {
            None // t is not within the limits
        }
    }
}
