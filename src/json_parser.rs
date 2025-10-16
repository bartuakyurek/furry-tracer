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
use std::{str::FromStr, fmt};
use serde_json::{Value};
use crate::numeric::{Float, Vector3, Int, Index};
use crate::{dataforms::{From3}, scene::Scene};

type BoxedError = Box<dyn std::error::Error>;

pub fn import_json(json_path: &str) -> Result<Scene, Box<dyn std::error::Error>>
{
    /*
        Return Scene loaded from .json file content.
        
        Example use: 
            // create an empty scene Scene::EMPTY
            // call import_json(path, scene)
        
        WARNING: To import multiple scenes, some object ids also need
        to be merged. This function directly maps a json file to scene.
        In future providing a root scene to aggregate multiple 
        scenes into one might be useful.
    */
    let mut scene = Scene::new();
    let data = std::fs::read_to_string(json_path)?;
    let json_value: Value = serde_json::from_str(&data)?;
    let v = &json_value["Scene"];
    print_json_keys(v);
    
    // Update attributes only if present in JSON (otherwise let default remain as is)
    set_optional(&mut scene.max_recursion_depth, v, "MaxRecursionDepth", parse_integer);
    set_optional(&mut scene.background_color, v, "BackgroundColor", parse_vector3_float);
    set_optional(&mut scene.shadow_ray_epsilon, v, "ShadowRayEpsilon", parse_float);
    
    // NOTE: More fields from JSON file to be declared below

    scene.validate()?;
    Ok(scene)
}

fn get_optional<T>(
    v: &Value,
    key: &str,
    parser: fn(&str) -> Result<T, BoxedError>,
) -> Option<T> {
    v.get(key)?
        .as_str()
        .and_then(|s| parser(s).ok())
}

fn set_optional<T>(
    field: &mut Option<T>,
    v: &Value,
    key: &str,
    parser: fn(&str) -> Result<T, BoxedError>,
) {
    if let Some(new_value) = get_optional::<T>(v, key, parser) {
        *field = Some(new_value);
        debug!("Key '{}' found in JSON", key);
    }
    else {
        warn!("Key '{}' not found in JSON, keeping default value.", key);
    }
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