/*

    Declare Scene consisting of all cameras, lights,
    materials, vertex data, and objects to be rendered.

    This declaration is meant to be compatible with 
    CENG 795's JSON file formats.

    @date: 2 Oct, 2025
    @author: Bartu
*/

use serde_json;
use serde::{Deserialize};

use crate::material::{Material, DiffuseMaterial, MirrorMaterial};
use crate::numeric::{Int, Float, Vector3, Index};
use crate::shapes::{Triangle, Sphere, Plane};
use crate::camera::{Cameras};
use crate::json_parser::*;
use crate::dataforms::{SingleOrVec, DataField};


#[derive(Debug, Default)]
pub struct Scene {
    // If anything is changed here please see
    // new( ) and json parser to be compatible
    pub max_recursion_depth: Option<Int>,
    pub background_color: Option<Vector3>,
    pub shadow_ray_epsilon: Option<Float>,
    pub intersection_test_epsilon: Option<Float>,
    pub cameras: Cameras,
    pub lights: SceneLights,
    pub materials: SceneMaterials,
    //pub vertex_data: DataField<Vector3>, 
    //pub objects: SceneObjects,
}

impl Scene {
    pub fn new() -> Self {
        // Initialize everything to default
         Self {
            max_recursion_depth: None,            
            background_color: Some(Vector3::new(0.0, 0.0, 0.0)), 
            shadow_ray_epsilon: Some(0.0001),        
            intersection_test_epsilon: Some(0.0001),  
            cameras: Cameras::default(),
            lights: SceneLights::default(),
            materials: SceneMaterials::default(),
            // vertex_data: DataField::new_empty(),
            //objects: SceneObjects::new(),
        }
    }
    
    
}


#[derive(Debug, Default, Deserialize, Clone)]
pub struct SceneLights {
    #[serde(rename = "AmbientLight", default, deserialize_with = "deser_vec3_opt")]
    pub ambient_light: Option<Vector3>,

    #[serde(rename = "PointLight", default)]
    pub point_lights: Option<SingleOrVec<PointLight>>, 
}


#[derive(Debug, Deserialize, Clone)]
pub struct PointLight {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int, 

    #[serde(rename = "Position", deserialize_with = "deser_vec3")]
    pub position: Vector3,

    #[serde(rename = "Intensity", deserialize_with = "deser_vec3")]
    pub rgb_intensity: Vector3,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct SceneMaterials {
    #[serde(rename = "Material", default)]
    pub raw_materials: SingleOrVec<MaterialRaw>,
}


pub trait SceneObject {
    //fn id(&self) -> Int;
    //fn as_any(&self) -> &dyn std::any::Any; 
}


impl SceneObject for Triangle {
    //fn id(&self) -> Int { self.id }
    //fn as_any(&self) -> &dyn std::any::Any { self }
}

impl SceneObject for Sphere {
    //fn id(&self) -> Int { self.id }
    //fn as_any(&self) -> &dyn std::any::Any { self }
}

impl SceneObject for Mesh {

}

impl SceneObject for Plane {

}


// Same for Plane, Mesh, etc.


#[derive(Debug, Default, Deserialize)]
pub struct SceneObjects {
    #[serde(rename = "Triangle", default)]
    pub triangles: Option<SingleOrVec<Triangle>>,

    #[serde(rename = "Sphere", default)]
    pub spheres: Option<SingleOrVec<Sphere>>,

    #[serde(rename = "Plane", default)]
    pub planes: Option<SingleOrVec<Plane>>,

    #[serde(rename = "Mesh", default)]
    pub meshes: Option<SingleOrVec<Mesh>>,
}


impl SceneObjects {
    pub fn new() -> Self {
        Self {
            triangles: None,
            spheres: None,
            planes: None,
            meshes: None,
        }
    }

    pub fn all(&self) -> Vec<Box<dyn SceneObject>> {
        let mut result: Vec<Box<dyn SceneObject>> = Vec::new();

        if let Some(triangles) = &self.triangles {
            for t in triangles.all() {
                result.push(Box::new(t.clone()));
            }
        }

        if let Some(spheres) = &self.spheres {
            for s in spheres.all() {
                result.push(Box::new(s.clone()));
            }
        }

        if let Some(planes) = &self.planes {
            for p in planes.all() {
                result.push(Box::new(p.clone()));
            }
        }

        if let Some(meshes) = &self.meshes {
            for m in meshes.all() {
                result.push(Box::new(m.clone()));
            }
        }

        result
    }
}



#[derive(Debug, Deserialize, Clone)]
struct Mesh {
    id: Int,
    material: Int,
    faces: DataField<Index>,
}


#[derive(Debug, Deserialize, Clone)]
pub struct MaterialRaw {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "_type", default)]
    pub mat_type: Option<String>, // e.g. "mirror"

    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3_opt", default)]
    pub ambient_reflectance: Option<Vector3>,

    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3_opt", default)]
    pub diffuse_reflectance: Option<Vector3>,

    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3_opt", default)]
    pub specular_reflectance: Option<Vector3>,

    #[serde(rename = "PhongExponent", deserialize_with = "deser_int_opt", default)]
    pub phong_exponent: Option<Int>,

    #[serde(rename = "MirrorReflectance", deserialize_with = "deser_vec3_opt", default)]
    pub mirror_reflectance: Option<Vector3>,
}
