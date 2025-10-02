/*

Declare Scene consisting of all cameras, lights,
materials, vertex data, and objects to be rendered.

This declaration is meant to be compatible with 
CENG 795's JSON file formats.

@date: 2 Oct, 2025
@author: Bartu
*/

use serde::{Deserialize};

use crate::camera::{Cameras};
use crate::numeric::{RGBColor, Int, Float, Vector3};
use crate::json_parser::*;

#[derive(Debug, Deserialize)]
pub struct RootScene {
    #[serde(rename = "Scene")]
    scene: Scene,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    #[serde(rename = "MaxRecursionDepth", deserialize_with = "deser_int")]
    max_recursion_depth: Int,

    #[serde(rename = "BackgroundColor", deserialize_with = "deser_vec3")]
    background_color: RGBColor,

    #[serde(rename = "ShadowRayEpsilon", deserialize_with = "deser_float")]
    shadow_ray_epsilon: Float,

    #[serde(rename = "IntersectionTestEpsilon", deserialize_with = "deser_float")]
    intersection_test_epsilon: Float,

    #[serde(rename = "Cameras")]
    cameras: Cameras,

    #[serde(rename = "Lights")]
    lights: SceneLights,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SceneLights {
    #[serde(rename = "AmbientLight")]
    pub ambient_lights: AmbientLights,

    #[serde(rename = "PointLight")]
    pub point_lights: Vec<PointLight>, 
}


#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)] // treat AmbientLights as directly wrapping Vec<Vector3>
pub struct AmbientLights(
    #[serde(deserialize_with = "deserialize_ambient_light")]
    pub Vec<Vector3>
);

#[derive(Debug, Deserialize, Clone)]
pub struct PointLight {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int, // or String if you prefer

    #[serde(rename = "Position", deserialize_with = "deser_dvec3")]
    pub position: Vector3,

    #[serde(rename = "Intensity", deserialize_with = "deser_dvec3")]
    pub intensity: Vector3,
}