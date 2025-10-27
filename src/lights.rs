/*
    Provide a light context to be used in shading 
    computations, that is intended to be used in
    materials, without directly taking light objects
    but working on the required context (e.g. for
    specular shading computation or any other shading
    implementation).

    TODO: How to decouple SceneLights from JSON file? and bring
    Light structs from scene.rs to here?

    @date: 27 Oct, 2025
    @author: Bartu

*/

use crate::scene::{Scene, SceneLights};

struct LightContext {

}

impl LightContext {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn new_from(lights: SceneLights) -> Self {
        Self {

        }
    }
}