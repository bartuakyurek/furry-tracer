/*

    Declare primitives: Triangle, Sphere
    
    @date: Oct, 2025
    @author: bartu
*/


use serde::{Deserialize};
use crate::numeric::{Int, Float, Vector3, Index};
use crate::json_parser::*;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}


pub trait Intersectable {
    fn intersects_with(ray: Ray) -> bool;
}


// Raw data deserialized from .JSON file
// it assumes vertex indices start from 1
#[derive(Debug, Deserialize, Clone)]
pub struct Triangle {
    pub id: Int,
    pub indices: Vec<usize>,
    pub material: Int,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Sphere {
    pub id: Int,
    pub center: Index,
    pub radius: Float,
    pub material: Int,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Plane {
    pub id: Int,
    pub point: Index,
    pub normal: Vector3,
    pub material: Int,
}