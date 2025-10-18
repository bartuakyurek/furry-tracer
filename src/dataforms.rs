/*

    Declare some (hopefully) useful data structures
    for this repo.  

    @date: 13 Oct, 2025
    @author: Bartu
*/

use serde::{Deserialize, de::{Deserializer}};
use crate::numeric::{Vector3, Index};
use crate::json_parser::{deser_vertex_data, deser_usize_vec};

// To be used for VertexData and Faces in JSON files
#[derive(Debug, Clone)]
pub struct DataField<T> {
    
    pub(crate) _data: Vec<T>,
    pub(crate) _type: String,
}

impl<'de> Deserialize<'de> for DataField<Vector3> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "_data", deserialize_with = "deser_vertex_data")]
            _data: Vec<Vector3>,
            #[serde(rename = "_type")]
            _type: String,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(DataField {
            _data: helper._data,
            _type: helper._type,
        })
    }
}

impl<'de> Deserialize<'de> for DataField<Index> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "_data", deserialize_with = "deser_usize_vec")]
            _data: Vec<Index>,
            #[serde(rename = "_type")]
            _type: String,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(DataField {
            _data: helper._data,
            _type: helper._type,
        })
    }
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
