/*

    Declare Scene consisting of all cameras, lights,
    materials, vertex data, and objects to be rendered.

    This declaration is meant to be compatible with 
    CENG 795's JSON file formats.

    WARNING: This Scene description is coupled with JSON file descriptions
    and it assumes JSON file fields are named in PascalCase (not camelCase or snake_case)
    TODO: Provide structs to (de)serialize JSON files, and communicate with a separate
    Scene struct that is hopefully decoupled from JSON file descriptions, i.e. to support
    such workflow:
        
        let s Scene::EMPTY
        s.add_some_object()
        s.add_some_light()
        s.center_camera() 
        let js = JSONScene::new_from(s)
        js.serialize(path/to/json)
    
    or
        let js = JSONSCene::new(path/to/json)
        let s = Scene::new_from(js)
        s.do_something_if_you_like()
        render(s)

    @date: 2 Oct, 2025
    @author: Bartu
*/
use std::{rc::Rc};
use serde_json::{self, Value};
use serde::{Deserialize};
use tracing::{warn, error, debug};

use crate::json_parser::{deser_string_or_struct};
use crate::material::{BoxedMaterial, DiffuseMaterial, Material, MirrorMaterial};
use crate::numeric::{Int, Float, Vector3};
use crate::shapes::{Shape, Mesh, Plane, Sphere, Triangle};
use crate::camera::{Cameras};
use crate::json_parser::*;
use crate::dataforms::{SingleOrVec};
use crate::dataforms::{VertexData};

#[derive(Debug, Deserialize)]
pub struct RootScene {
    #[serde(rename = "Scene")]
    pub scene: Scene,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct Scene {
    #[serde(deserialize_with = "deser_int")]
    pub max_recursion_depth: Int,

    #[serde(deserialize_with = "deser_vec3")]
    pub background_color: Vector3,

    #[serde(deserialize_with = "deser_float")]
    pub shadow_ray_epsilon: Float,

    #[serde(deserialize_with = "deser_float")]
    pub intersection_test_epsilon: Float,

    #[serde(deserialize_with = "deser_string_or_struct")]
    pub vertex_data: VertexData, 

    pub cameras: Cameras,
    pub lights: SceneLights,
    pub materials: SceneMaterials,
    pub objects: SceneObjects,
}

impl Scene {
    //pub fn new() {
    //}
    pub fn setup_after_json(&mut self) {
        // Implement required adjustments after loading from a JSON file

        // 1- Convert materials serde_json values to actual structs
        self.materials.finalize();
        for m in &self.materials.materials { // TODO: refactor that ambigious call materials.materials( )
            debug!("Material: {:#?}", m);
        }

        // 2- Fix VertexData if _type is not "xyz" 
        let previous_type = self.vertex_data._type.clone();
        if self.vertex_data.normalize_to_xyz() { warn!("VertexData _type is changed from '{}' to '{}'", previous_type, self.vertex_data._type); }

        // 3- Add a dummy vertex at index 0 because JSON vertex ids start from 1
        self.vertex_data.insert_dummy_at_the_beginning();
        warn!("Inserted a dummy vertex at the beginning to use vertex IDs beginning from 1.")

    }
}


#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SceneLights {
    #[serde(rename = "AmbientLight", deserialize_with = "deser_vec3")]
    pub ambient_light: Vector3,

    #[serde(rename = "PointLight")]
    pub point_lights: SingleOrVec<PointLight>, 
}

impl Default for SceneLights {
    fn default() -> Self {
        Self {
            ambient_light: Vector3::ZERO, // No intensity
            point_lights: SingleOrVec::default(),
            }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PointLight {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int, // or String if you prefer

    #[serde(rename = "Position", deserialize_with = "deser_vec3")]
    pub position: Vector3,

    #[serde(rename = "Intensity", deserialize_with = "deser_vec3")]
    pub rgb_intensity: Vector3,
}


#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct SceneMaterials {
    #[serde(rename = "Material")]
    raw_materials: SingleOrVec<serde_json::Value>, // Parse the json value later separately

    #[serde(skip)]
    materials: Vec<BoxedMaterial>,
}

impl SceneMaterials {
    pub fn finalize(&mut self) {
        self.materials = self.raw_materials
                        .all()
                        .into_iter()
                        .flat_map(parse_material)
                        .collect();
    }

    pub fn all(&mut self) -> &Vec<BoxedMaterial> {
        if self.materials.is_empty() && !self.raw_materials.all().is_empty() {
            warn!("Calling SceneMaterials.finalize() to fully deserialize materials from JSON file...");
            self.finalize(); 
        }
        &self.materials
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)] // If any of the fields below is missing in the JSON, use default (empty vector, hopefully)
// #[serde(rename_all = "PascalCase")] // Do NOT do that here, naming is different in json file
pub struct SceneObjects {
    #[serde(rename = "Triangle")]
    pub triangles: SingleOrVec<Triangle>,
    #[serde(rename = "Sphere")]
    pub spheres: SingleOrVec<Sphere>,
    #[serde(rename = "Plane")]
    pub planes: SingleOrVec<Plane>,
    #[serde(rename = "Mesh")]
    pub meshes: SingleOrVec<Mesh>,
}

impl SceneObjects {

    pub fn all(&self) -> Vec<Rc<dyn Shape>> {
        // Return a vector of all shapes in the scene
        warn!("SceneObjects.all( ) assumes there are only triangles, spheres, planes, and meshes. If there are other Shape trait implementations they are not added yet.");
        let mut shapes: Vec<Rc<dyn Shape>> = Vec::new();

        shapes.extend(self.triangles.all().into_iter().map(|t| Rc::new(t) as Rc<dyn Shape>));
        shapes.extend(self.spheres.all().into_iter().map(|s| Rc::new(s) as Rc<dyn Shape>));
        shapes.extend(self.planes.all().into_iter().map(|p| Rc::new(p) as Rc<dyn Shape>));
        shapes.extend(self.meshes.all().into_iter().map(|m| Rc::new(m) as Rc<dyn Shape>));

        shapes
    }

}

fn parse_single_material(value: serde_json::Value) -> BoxedMaterial {
    
    debug!("Parsing material JSON: {:#?}", value);

    // Check _type field
    let mat_type = value.get("_type").and_then(|v| v.as_str()).unwrap_or("diffuse");

    match mat_type {
        "diffuse" => Box::new(DiffuseMaterial::new_from(&value)),
        "mirror" => Box::new(MirrorMaterial::new_from(&value)),
        // TODO: add more materials here

        other => {
            error!("Unknown material type '{other}', defaulting to DiffuseMaterial");
            Box::new(DiffuseMaterial::new_from(&value))
        }
    }
}

fn parse_material(value: serde_json::Value) -> Vec<BoxedMaterial> {
    match value {
        Value::Array(arr) => arr.into_iter().map(parse_single_material).collect(),
        Value::Object(_) => vec![parse_single_material(value)],
        _ => {
            error!("Invalid material JSON, expected object or array: {value:?}");
            vec![]
        }
    }
}
