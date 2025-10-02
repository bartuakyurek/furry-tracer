/*

Declare Scene consisting of all cameras, lights,
materials, vertex data, and objects to be rendered.

This declaration is meant to be compatible with 
CENG 795's JSON file formats.

@date: 2 Oct, 2025
@author: Bartu
*/

use serde::{Deserialize};
use crate::numeric::{RGBColor, Int, Float};
use crate::json_parser::{deser_int, deser_float, deser_vec3_from_str};

#[derive(Debug, Deserialize)]
pub struct RootScene {
    Scene: Scene,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    #[serde(deserialize_with = "deser_int")]
    MaxRecursionDepth: Int,

    #[serde(deserialize_with = "deser_vec3_from_str")]
    BackgroundColor: RGBColor,

    #[serde(deserialize_with = "deser_float")]
    ShadowRayEpsilon: Float,

    #[serde(deserialize_with = "deser_float")]
    IntersectionTestEpsilon: Float,

    #[serde(rename = "Cameras")]
    cameras: Cameras,
}


// To handle JSON file having a single camera
// or an array of Cameras 
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum CameraSingleOrVec {
    Single(Camera),
    Multiple(Vec<Camera>),
}

#[derive(Debug, Deserialize)]
struct Cameras {
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


}

