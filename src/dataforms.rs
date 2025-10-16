/*

    Declare some (hopefully) useful data structures
    for this repo.  

    @date: 13 Oct, 2025
    @author: Bartu
*/

use bevy_math;

// To be used for VertexData and Faces in JSON files
#[derive(Debug, Default, Clone)]
pub struct DataField<T> {
    
    pub(crate) _data: Vec<T>,
    pub(crate) _type: String,
}


// To handle JSON file having a single <object>
// or an array of <object>s 
#[derive(Debug, Clone)]
pub enum SingleOrVec<T> {
    None,
    Single(T),
    Multiple(Vec<T>),
}

impl<T: Clone> SingleOrVec<T>  {
    pub fn all(&self) -> Vec<T> {
        match &self {
            SingleOrVec::None => vec![],
            SingleOrVec::Single(t) => vec![t.clone()],
            SingleOrVec::Multiple(vec) => vec.clone(),
        }
    }
}

impl<T> Default for SingleOrVec<T> {
    // Default is an empty vector 
    fn default() -> Self {
        SingleOrVec::Multiple(Vec::new()) 
    }
}

pub trait From3<T>: Sized {
    fn new(x: T, y: T, z: T) -> Self;
}

impl From3<f32> for bevy_math::Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z)
    }
}

impl From3<f64> for bevy_math::DVec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z)
    }
}