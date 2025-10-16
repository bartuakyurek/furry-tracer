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
use crate::material::{Material};
use crate::shapes::{Intersectable};
use crate::dataforms::{DataField};


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
    pub fn new( ) -> Self {
        Scene {
                max_recursion_depth: Some(99),
                background_color: Some(Vector3::new(0.0, 0.0, 0.0)), // black
                shadow_ray_epsilon: Some(0.001),
                intersection_test_epsilon: Some(0.0001),
                // Everything else gets their own Default
                ..Default::default()
            }
    }

    pub fn validate(&self) -> Result<(), Error> {
        // TODO: check if materials vector has material
        // ids matching with their indices
        // if not, attempt reordering them
        // if reordering is successful print a warning about reorder
        // else return error

        // Currently assumes given material ids are in order
        // However it'd fail if we attempt to load multiple .json
        // scenes as two different material can have the same id
        
        // WARNING: Same validation should be done for lights
        error!("Validate function not implemented yet!");
        Ok(())
    }

    pub fn add_material(&mut self, mat: Rc<dyn Material>) {
        self.materials.push(mat);
    }
}





#[derive(Debug, Default)]
pub(crate) struct SceneLights {
    ambient: Option<Vector3>,
    point_lights: Vec<PointLight>,
    // WARNING: Above assumes PointLight._id matches with vector indices!
}


#[derive(Debug, Default)]
struct PointLight {
    _id: Index,
    position: Vector3,
    intensity: Vector3, // R G B
}

