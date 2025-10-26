


use bevy_math::curve::derivatives;

use crate::dataforms::VertexData;
use crate::material;
use crate::numeric::{Vector3, Float};
use crate::interval::{Interval};


#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn at(&self, t: Float) -> Vector3 {
        self.origin + self.direction * t // r(t) = o + dt
    }

    #[inline]
    pub fn is_front_face(&self, normal: Vector3) -> bool {
         self.direction.dot(normal) <= 0.0 
    }
}


// Question: Couldn't we just use point to see which point is closer?
// but this is relative to camera, and t is a single scalar that encaptures
// which HitRecord is closer to the - actually not to camera (for primary rays only it is camera)
// but ray origin so t=0 is at ray origin, smaller t is, closer the object is.  
//
// DISCLAIMER: This struct is based on the approach presented in Ray Tracing in One Weekend book.
#[derive(Debug)]  
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub ray_t: Float,  // To check which HitRecord has smaller t 
    pub material: usize, // TODO: Should we hold the index of material or actually Option<Rc<dyn Material>> as in here https://the-ray-tracing-road-to-rust.vercel.app/9-metal? Or Arc instead of Rc if we use rayon in future.
    pub is_front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vector3, normal: Vector3, ray_t: Float, material: usize, is_front_face: bool) -> Self {
        Self {
            point,
            normal,
            ray_t,
            material,
            is_front_face,
        }
    }
    pub fn new_from(ray: &Ray, n: Vector3, t: Float, material: usize) -> Self {
        let is_front_face = ray.is_front_face(n);
        Self {
            point: ray.at(t),
            normal: if is_front_face {n} else {-n}, // TODO: is this correct?
            ray_t: t,
            material: material, 
            is_front_face,
        }
    }
}


pub fn ray_triangle_intersection(ray: &Ray, t_interval: &Interval, tri_indices: [usize; 3], verts: &VertexData) -> Option<(Vector3, Float)> {

    // Based on Möller-Trumbore algorithm
        //
        //     a (pivot)
        //    / \
        //  b  -  c
        // 
        // WARNING: Assumes given interval has incorporated relevant epsilon e.g.
        // instead of [0.0, inf], [0.0001, inf] is given otherwise there might be
        // floating point errors.
        let tri_coords = tri_indices.map(|i| verts[i]);
        let [tri_pivot, tri_left, tri_right] = tri_coords;        
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

        // Get ray t
        let t = edge_ac.dot(another_perp) * inverse_determinant;
        if !t_interval.contains(t) {
            return None;
        }
        
        // Construct hit point p
        let p = ray.at(t); // TODO: would it be faster to use barycentric u,v here? 

        Some((p, t))
}