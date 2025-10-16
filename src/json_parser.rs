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

use std::{error::Error};
use serde_json::{Value};
use crate::scene::{Scene};

pub fn import_json(json_path: &str, scene: &mut Scene) -> Result<(), Box<dyn Error>>{
    /*
        Import JSON file contents to a given scene.
        
        Example use: 
            // create an empty scene Scene::EMPTY
            // call import_json(path, scene)
        
        Intended to be useful for importing multiple json files
        into a single scene.
    */
    let data = std::fs::read_to_string(json_path)?;
    let value: Value = serde_json::from_str(&data)?;
    let scene_json = &value["Scene"];
    
    // TODO: add hashmap contents to scene
    Ok(())
}

// TODO: Parser functions to be declared below