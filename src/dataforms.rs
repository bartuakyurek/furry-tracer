/*

    Declare some (hopefully) useful data structures
    for this repo.  

    @date: 13 Oct, 2025
    @author: Bartu
*/

use serde::{Deserialize};


// To be used for VertexData and Faces in JSON files
#[derive(Debug, Clone)]
pub struct DataField<T> {
    
    pub(crate) _data: Vec<T>,
    pub(crate) _type: String,
}


// To handle JSON file having a single <object>
// or an array of <object>s 
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum SingleOrVec<T> {
    Single(T),
    Multiple(Vec<T>),
}

impl<T: Clone> SingleOrVec<T>  {
    pub fn all(&self) -> Vec<T> {
        match &self {
            SingleOrVec::Single(t) => vec![t.clone()],
            SingleOrVec::Multiple(vec) => vec.clone(),
        }
    }
}
