/*

    Declare primitives: Triangle, Sphere
    
    @date: Oct, 2025
    @author: bartu
*/


use std::fmt::Debug;
use crate::numeric::{Int, Float, Vector3, Index};

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}


pub trait Intersectable: Send + Sync + Debug {
    fn intersects_with(&self, ray: Ray) -> bool;
}


// Raw data deserialized from .JSON file
// it assumes vertex indices start from 1
#[derive(Debug, Default, Clone)]
pub struct Triangle {
    pub id: Int,
    pub indices: Vec<usize>,
    pub material: Int,
}


#[derive(Debug, Default, Clone)]
pub struct Sphere {
    pub id: Int,
    pub center: Index,
    pub radius: Float,
    pub material: Int,
}

#[derive(Debug, Default, Clone)]
pub struct Plane {
    pub id: Int,
    pub point: Index,
    pub normal: Vector3,
    pub material: Int,
}