/*

Declare Scene consisting of all cameras, lights,
materials, vertex data, and objects to be rendered.

This declaration is meant to be compatible with 
CENG 795's JSON file formats.

@date: 2 Oct, 2025
@author: Bartu
*/

use serde::{Deserialize};
use serde_json::{Value};

use crate::material::{Material, DiffuseMaterial, MirrorMaterial};
use crate::numeric::{RGB, Int, Float, Vector3};
use crate::shapes::{Triangle, Sphere};
use crate::camera::{Cameras};
use crate::json_parser::*;

#[derive(Debug, Deserialize)]
pub struct RootScene {
    #[serde(rename = "Scene")]
    pub scene: Scene,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    #[serde(rename = "MaxRecursionDepth", deserialize_with = "deser_int")]
    max_recursion_depth: Int,

    #[serde(rename = "BackgroundColor", deserialize_with = "deser_vec3")]
    background_color: Vector3,

    #[serde(rename = "ShadowRayEpsilon", deserialize_with = "deser_float")]
    shadow_ray_epsilon: Float,

    #[serde(rename = "IntersectionTestEpsilon", deserialize_with = "deser_float")]
    intersection_test_epsilon: Float,

    #[serde(rename = "Cameras")]
    cameras: Cameras,

    #[serde(rename = "Lights")]
    lights: SceneLights,

    #[serde(rename = "Materials")]
    materials: SceneMaterials,

    #[serde(rename = "VertexData", deserialize_with = "deser_vertex_data")]
    vertex_data: Vec<Vector3>, 
    
    #[serde(rename = "Objects")]
    objects: SceneObjects,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SceneLights {
    #[serde(rename = "AmbientLight", deserialize_with = "deser_vec3")]
    pub ambient_light: Vector3,

    #[serde(rename = "PointLight")]
    pub point_lights: Vec<PointLight>, 
}

#[derive(Debug, Deserialize, Clone)]
pub struct PointLight {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int, // or String if you prefer

    #[serde(rename = "Position", deserialize_with = "deser_vec3")]
    pub position: Vector3,

    #[serde(rename = "Intensity", deserialize_with = "deser_vec3")]
    pub rgb_intensity: Vector3,
}


#[derive(Debug, Deserialize)]
pub struct SceneMaterials {
    #[serde(rename = "Material")]
    pub raw_materials: Vec<Value>, // keep JSON nodes as-is for postprocessing
}

impl SceneMaterials {
    pub fn into_materials(self) -> Vec<Box<dyn Material>> {
        self.raw_materials
            .into_iter()
            .map(|val| {
                if let Some(t) = val.get("_type") {
                    match t.as_str().unwrap() {
                        "mirror" => Box::new(serde_json::from_value::<MirrorMaterial>(val).unwrap()) as Box<dyn Material>,
                        other => panic!("Unknown material type: {}", other),
                    }
                } else {
                    Box::new(serde_json::from_value::<DiffuseMaterial>(val).unwrap()) as Box<dyn Material>
                }
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct SceneObjects {
    #[serde(rename = "Triangle", default)]
    pub triangles: Vec<Triangle>,

    #[serde(rename = "Sphere", default)]
    pub spheres: Vec<Sphere>,
}
