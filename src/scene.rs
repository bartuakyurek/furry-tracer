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


    //Cameras: Cameras,
}

#[derive(Debug, Deserialize)]
struct Cameras {}

#[derive(Debug, Deserialize)]
struct Camera {}

