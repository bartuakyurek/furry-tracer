/*

    Declare Scene consisting of all cameras, lights,
    materials, vertex data, and objects to be rendered.

    This declaration is meant to be compatible with 
    CENG 795's JSON file formats.

    @date: 2 Oct, 2025
    @author: Bartu
*/


use crate::camera::{Camera};
use crate::numeric::{Int, Float, Vector3, Index};
//use crate::material::{*};
//use crate::shapes::{Triangle, Sphere, Plane};
//use crate::dataforms::{SingleOrVec, DataField};


#[derive(Debug, Default)]
pub struct Scene {
    pub max_recursion_depth: Option<Int>,
    pub background_color: Option<Vector3>,
    pub shadow_ray_epsilon: Option<Float>,
    pub intersection_test_epsilon: Option<Float>,
    pub cameras: Vec<Camera>,
    //pub lights: SceneLights,
    //pub materials: SceneMaterials,
    //pub vertex_data: DataField<Vector3>, 
    //pub objects: SceneObjects,
}

impl Scene {
    
}
