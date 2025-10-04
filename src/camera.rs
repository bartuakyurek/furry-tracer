/*



*/



use serde::{Deserialize};
use crate::numeric::{Int, Float, Vector3};
use crate::json_parser::*;

// To handle JSON file having a single camera
// or an array of Cameras 
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum CameraSingleOrVec {
    Single(Camera),
    Multiple(Vec<Camera>),
}

#[derive(Debug, Deserialize)]
pub struct Cameras {
    #[serde(rename = "Camera")]
    camera: CameraSingleOrVec, // Allow either single cam (as in test.json) or multiple cams
}

impl Cameras {
    /// Always returns a Vec<Camera> regardless of JSON being a single object or array
    fn all(&self) -> Vec<Camera> {
        match &self.camera {
            CameraSingleOrVec::Single(cam) => vec![cam.clone()],
            CameraSingleOrVec::Multiple(vec) => vec.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Camera {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    id: Int,
    
    #[serde(rename = "Position", deserialize_with = "deser_vec3")]
    position: Vector3,

    #[serde(rename = "Gaze", deserialize_with = "deser_vec3")]
    gaze: Vector3,

    #[serde(rename = "Up", deserialize_with = "deser_vec3")]
    up: Vector3,

    #[serde(rename = "NearPlane", deserialize_with = "deser_nearplane")]
    nearplane: NearPlane,

    // NOTE: Skipping near distance as nearplane already contains it

    #[serde(rename = "ImageResolution", deserialize_with = "deser_vec2")]
    image_resolution: [Int; 2],

    #[serde(rename = "ImageName")]
    image_name: String,

}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct NearPlane {
    #[serde(deserialize_with = "deser_float")]
    pub(crate) left: Float,
    #[serde(deserialize_with = "deser_float")]
    pub(crate) right: Float,
    #[serde(deserialize_with = "deser_float")]
    pub(crate) bottom: Float,
    #[serde(deserialize_with = "deser_float")]
    pub(crate) top: Float,
    #[serde(deserialize_with = "deser_float")]
    pub(crate) near_distance: Float,
}