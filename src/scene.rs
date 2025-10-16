/*

    Declare Scene consisting of all cameras, lights,
    materials, vertex data, and objects to be rendered.

    This declaration is meant to be compatible with 
    CENG 795's JSON file formats.

    @date: 2 Oct, 2025
    @author: Bartu
*/

use std::rc::Rc; 
use serde::de::value::Error;
use tracing::{warn, error};

use crate::camera::{Camera};
use crate::numeric::{Int, Float, Vector3, Index};
use crate::material::{BoxedMaterial, Material};
use crate::shapes::{Intersectable, Plane, Sphere, Triangle};
use crate::dataforms::{SingleOrVec, DataField};


#[derive(Debug, Default)]
pub struct Scene {
    pub max_recursion_depth: Option<Int>,
    pub background_color: Option<Vector3>,
    pub shadow_ray_epsilon: Option<Float>,
    pub intersection_test_epsilon: Option<Float>,
    pub cameras: Vec<Camera>,
    pub lights: SceneLights,
    pub materials: Vec<Rc<dyn Material>>,
    pub vertex_data: DataField<Vector3>, 
    pub objects: Vec<Rc<dyn Intersectable>>,
}

impl Scene {
    pub fn validate(&self) -> Result<(), Error> {
        // TODO: check if materials vector has material
        // ids matching with their indices
        // if not, attempt reordering them
        // if reordering is successful print a warning about reorder
        // else return error
        error!("Validate function not implemented yet!");
        Ok(())
    }
}

#[derive(Debug, Default)]
struct SceneLights {
    ambient: Option<Vector3>,
    point_lights: Vec<PointLight>,
}


#[derive(Debug, Default)]
struct PointLight {
    _id: Int,
    position: Vector3,
    intensity: Vector3, // R G B
}

#[derive(Debug, Default)]
struct SceneMaterials {

}
