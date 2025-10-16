/*

    Provide utilities to parse JSON file in CENG 795 format.

    This format currently assumes:
        - Every field is String (even integers are encapsulated in quotes e.g. "6")
        - Vector3 data fields are in format "<a> <a> <a>" where <a> is integer or float


    The parser is somewhat robust, let <a> be integer or float type,
    in JSON file <a> can be given both in quotes (string) or as is.

    e.g. In JSON file both
    "MaxRecursionDepth": "6" and "MaxRecursionDepth": 6
    works as MaxRecursionDepth: int in source code

    WARNING: It is not robust for handling vec3 types given in brackets 
    e.g. providing [0, 0, 0] for "BackgroundColor" will fail. It is assumed to be
    "BackgroundColor": "0 0 0" for the time being.

    @date: 2 Oct, 2025
    @author: bartu 
*/

use tracing::{error, warn, info, debug};
use std::{str::FromStr, fmt, collections::HashSet};
use serde_json::{Value};
use crate::numeric::{Float, Vector3, Int, Index};
use crate::{dataforms::{From3}};
use crate::scene::{self, Scene, SceneLights};

type BoxedError = Box<dyn std::error::Error>;


pub fn import_scene_json(json_path: &str) -> Result<Scene, Box<dyn std::error::Error>> {
    /*  Return Scene loaded from .json file content.
        
        Example use: 
            // create an empty scene Scene::EMPTY
            // call import_json(path, scene)
        
        WARNING: To import multiple scenes, some object ids also need
        to be merged. This function directly maps a json file to scene.
        In future providing a root scene to aggregate multiple 
        scenes into one might be useful.
    */
    let data = std::fs::read_to_string(json_path)?;
    let json_value: Value = serde_json::from_str(&data)?;
    let scene_value = &json_value["Scene"];
    
    let mut scene = Scene::new();
    let mut handler = JsonHandler::new(scene_value);

    // Scene attributes
    handler.get_optional(&mut scene.max_recursion_depth, "MaxRecursionDepth", parse_integer)?;
    handler.get_optional(&mut scene.background_color , "BackgroundColor", parse_vector3_float)?;
    handler.get_optional(&mut scene.shadow_ray_epsilon, "ShadowRayEpsilon", parse_float)?;
    handler.get_optional(&mut  scene.intersection_test_epsilon, "IntersectionTestEpsilon", parse_float)?;

    // Lights
    if let Some(lights_value) = handler.get_subobject("Lights") {
        let mut lights_handler = JsonHandler::new(lights_value);
        lights_handler.get_optional(&mut scene.lights.ambient,"AmbientLight", parse_vector3_float)?;
        
    }

    handler.warn_extra(); // WARNING: This misses extra subfields, only valid for outer scope
    scene.validate()?;
    Ok(scene)
}


fn parse_scalar<T>(s: &str) -> Result<T, BoxedError> 
where
    T: std::str::FromStr,
    T::Err: std::error::Error + 'static,
{
    s.parse::<T>()
        .map_err(|e| Box::new(e) as BoxedError)
}

/// Helper function: parse a string like "25 25 25" into Vector3
fn parse_vector3<V, F>(s: &str) -> Result<V, String> 
where 
    F: FromStr,
    F::Err: fmt::Display,
    V: From3<F>,
{
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!("Expected 3 values, got {}", parts.len()));
    }
    let x = parts[0].parse::<F>().map_err(|e| e.to_string())?;
    let y = parts[1].parse::<F>().map_err(|e| e.to_string())?;
    let z = parts[2].parse::<F>().map_err(|e| e.to_string())?;
    Ok(V::new(x, y, z))
}


fn parse_vec<T: std::str::FromStr>(s: &str) -> Result<Vec<T>, BoxedError>
where
    T::Err: std::error::Error + 'static
{
    s.split_whitespace()
        .map(|x| x.parse::<T>().map_err(|e| Box::new(e) as BoxedError))
        .collect()
}


// Concrete type wrappers
fn parse_vector3_float(s: &str) -> Result<Vector3, BoxedError> {
    parse_vector3::<Vector3, Float>(s).map_err(|e| e.into())
}

fn parse_float(s: &str) -> Result<Float, BoxedError> {
    parse_scalar::<Float>(s)
}


fn parse_integer(s: &str) -> Result<Int, BoxedError> {
    parse_scalar::<Int>(s)
}

// For debug purposes
fn print_json_keys(v: &Value) {
    if let Some(obj) = v.as_object() {
        info!("Found keys:");
        for key in obj.keys() {
            info!("  - {}", key);
        }
    } else {
        error!("Value is not a JSON object.");
    }
}

struct JsonHandler<'a> {
    value: &'a Value,
    handled_keys: HashSet<String>,
}

impl<'a> JsonHandler<'a> {
    fn new(value: &'a Value) -> Self {
        Self { value, handled_keys: HashSet::new() }
    }

    /// Get an optional field and track it automatically
    fn get_optional<T>(
        &mut self,
        data: &mut Option<T>,
        key: &str,
        parser: fn(&str) -> Result<T, Box<dyn std::error::Error>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(v) = self.value.get(key) {
            let s = v.as_str().ok_or("Expected string")?;
            let parsed = parser(s)?;
            *data = Some(parsed);
            self.handled_keys.insert(key.to_string());
            Ok(())
        } else {
            error!("No field '{}' found in JSON, data is not updated.", key);
            Ok(())
        }
    }

    /// Get a sub-object and track its key
    fn get_subobject(&mut self, key: &str) -> Option<&Value> {
        if let Some(v) = self.value.get(key) {
            self.handled_keys.insert(key.to_string());
            Some(v)
        } else {
            None
        }
    }

    /// Check for extra fields
    fn warn_extra(&self) {
        if let Value::Object(map) = self.value {
            for key in map.keys() {
                if !self.handled_keys.contains(key) {
                    error!("Warning: extra key '{}'", key);
                }
            }
        }
    }
}
