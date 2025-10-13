/*

    Declare Scene consisting of all cameras, lights,
    materials, vertex data, and objects to be rendered.

    This declaration is meant to be compatible with 
    CENG 795's JSON file formats.

    @date: 2 Oct, 2025
    @author: Bartu
*/

use serde_json;
use serde::{Deserialize, de::{Deserializer}};

use crate::material::{Material, DiffuseMaterial, MirrorMaterial};
use crate::numeric::{Int, Float, Vector3, Index};
use crate::shapes::{TriangleSerde, Sphere, Plane};
use crate::camera::{Cameras};
use crate::json_parser::*;
use crate::dataforms::{SingleOrVec, DataField};

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
    pub cameras: Cameras,

    #[serde(rename = "Lights")]
    lights: SceneLights,

    #[serde(rename = "Materials")]
    materials: SceneMaterials,

    #[serde(rename = "VertexData")]
    vertex_data: DataField<Vector3>, 
    
    #[serde(rename = "Objects")]
    objects: SceneObjects,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SceneLights {
    #[serde(rename = "AmbientLight", deserialize_with = "deser_vec3")]
    pub ambient_light: Vector3,

    #[serde(rename = "PointLight")]
    pub point_lights: SingleOrVec<PointLight>, 
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
    pub raw_materials: SingleOrVec<serde_json::Value>, // keep json value as-is for postprocessing
}

impl SceneMaterials {
    pub fn into_materials(self) -> Vec<Box<dyn Material>> {
        self.raw_materials
            .all()
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
    #[serde(rename = "Triangle")]
    pub triangles: SingleOrVec<TriangleSerde>,

    #[serde(rename = "Sphere")]
    pub spheres: SingleOrVec<Sphere>,

    #[serde(rename = "Plane")]
    pub planes: SingleOrVec<Plane>,

    #[serde(rename = "Mesh")]
    pub meshes: SingleOrVec<Mesh>,
}


impl SceneObjects {
    /// Always returns a Vec<Camera> regardless of JSON being a single object or array
    pub fn all(&self) -> (Vec<TriangleSerde>, Vec<Sphere>, Vec<Plane>, Vec<Mesh>) {
        (self.triangles.all(), self.spheres.all(), self.planes.all(), self.meshes.all())
    }
}


#[derive(Debug, Deserialize, Clone)]
struct Mesh {
 pub    _id: Int,
    material: Int,
    faces: DataField<Index>,
}

