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
use bevy_math::{Vec3};
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde::de::{self, Deserializer};
use tracing::{debug};
use crate::scene::{RootScene};
use crate::numeric::{Int, Float};

pub fn parse_json795(path: &str) -> Result<RootScene, Box<dyn std::error::Error>> {
    /*
        Parse JSON files in CENG 795 format.
    */

    let span = tracing::span!(tracing::Level::INFO, "load_scene");
    let _enter = span.enter();

    // Open file
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    debug!("Reading file from {}", path);
    
    // Parse JSON into Scene
    let root: RootScene = serde_json::from_reader(reader)?;
    Ok(root) 
}


pub fn deser_int<'de, D>(deserializer: D) -> Result<Int, D::Error>
where
    D: Deserializer<'de>,
{
    /*
        Deserialize integer type given as either string or number in JSON
    */
    let s: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match s {
        serde_json::Value::Number(n) => n.as_i64()
            .map(|v| v as Int)
            .ok_or_else(|| de::Error::custom("Invalid integer")),
        serde_json::Value::String(s) => s.parse::<Int>()
            .map_err(|_| de::Error::custom("Failed to parse integer from string")),
        _ => Err(de::Error::custom("Expected int or string")),
    }
}

// Handles floats as string or number
pub fn deser_float<'de, D>(deserializer: D) -> Result<Float, D::Error>
where
    D: Deserializer<'de>,
{
    /*
        Deserialize float type given as either string or number in JSON
    */
    let s: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match s {
        serde_json::Value::Number(n) => n.as_f64()
            .map(|v| v as Float)
            .ok_or_else(|| de::Error::custom("Invalid float")),
        serde_json::Value::String(s) => s.parse::<Float>()
            .map_err(|_| de::Error::custom("Failed to parse float from string")),
        _ => Err(de::Error::custom("Expected float or string")),
    }
}

pub fn deser_vec3_from_str<'de, D>(deserializer: D) -> Result<Vec3, D::Error>
where
    D: Deserializer<'de>,
{
    /*
    
        Deserialize Vec3 type given as string in JSON
    
        Expects JSON file string has format
        "<a> <a> <a>" where <a> is either int or float

        Throws error if there are more than 3 numbers in the string.
    */
    let s = String::deserialize(deserializer)?;
    let parts: Vec<&str> = s.split_whitespace().collect();

    if parts.len() != 3 {
        return Err(de::Error::custom("Expected 3 components for Vec3"));
    }

    let x = parts[0].parse::<f32>().map_err(|_| de::Error::custom("Failed to parse Vec3 x component"))?;
    let y = parts[1].parse::<f32>().map_err(|_| de::Error::custom("Failed to parse Vec3 y component"))?;
    let z = parts[2].parse::<f32>().map_err(|_| de::Error::custom("Failed to parse Vec3 z component"))?;

    Ok(Vec3::new(x, y, z))
}
