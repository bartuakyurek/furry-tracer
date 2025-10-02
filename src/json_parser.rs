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

use std::fmt;
use bevy_math::{DVec3, Vec3};
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde::de::{self, Visitor, SeqAccess, Deserializer};
use tracing::{debug};
use crate::scene::{RootScene};
use crate::camera::{NearPlane};
use crate::numeric::{Int, Float, Vector3};

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

pub fn deser_vec3<'de, D>(deserializer: D) -> Result<Vec3, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = Vec3;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a Vec3 as a string 'x y z' or an array [x, y, z]")
        }

        fn visit_str<E>(self, value: &str) -> Result<Vec3, E>
        where
            E: de::Error,
        {
            // Use your existing logic for string
            let parts: Vec<&str> = value.split_whitespace().collect();
            if parts.len() != 3 {
                return Err(E::custom("Expected 3 components for Vec3 string"));
            }
            let x = parts[0].parse::<f32>().map_err(|_| E::custom("Failed to parse Vec3 x component"))?;
            let y = parts[1].parse::<f32>().map_err(|_| E::custom("Failed to parse Vec3 y component"))?;
            let z = parts[2].parse::<f32>().map_err(|_| E::custom("Failed to parse Vec3 z component"))?;
            Ok(Vec3::new(x, y, z))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Vec3, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // Deserialize from an array [x, y, z]
            let x: f32 = seq.next_element()?.ok_or_else(|| de::Error::custom("Expected 3 elements in Vec3 array"))?;
            let y: f32 = seq.next_element()?.ok_or_else(|| de::Error::custom("Expected 3 elements in Vec3 array"))?;
            let z: f32 = seq.next_element()?.ok_or_else(|| de::Error::custom("Expected 3 elements in Vec3 array"))?;
            if seq.next_element::<f32>()?.is_some() {
                return Err(de::Error::custom("Expected only 3 elements in Vec3 array"));
            }
            Ok(Vec3::new(x, y, z))
        }
    }

    deserializer.deserialize_any(Vec3Visitor)
}



pub fn deser_dvec3<'de, D>(deserializer: D) -> Result<DVec3, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = DVec3;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a Vec3 as a string 'x y z' or an array [x, y, z]")
        }

        fn visit_str<E>(self, value: &str) -> Result<DVec3, E>
        where
            E: de::Error,
        {
            // Use your existing logic for string
            let parts: Vec<&str> = value.split_whitespace().collect();
            if parts.len() != 3 {
                return Err(E::custom("Expected 3 components for Vec3 string"));
            }
            let x = parts[0].parse::<f64>().map_err(|_| E::custom("Failed to parse Vec3 x component"))?;
            let y = parts[1].parse::<f64>().map_err(|_| E::custom("Failed to parse Vec3 y component"))?;
            let z = parts[2].parse::<f64>().map_err(|_| E::custom("Failed to parse Vec3 z component"))?;
            Ok(DVec3::new(x, y, z))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<DVec3, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // Deserialize from an array [x, y, z]
            let x: f64 = seq.next_element()?.ok_or_else(|| de::Error::custom("Expected 3 elements in Vec3 array"))?;
            let y: f64 = seq.next_element()?.ok_or_else(|| de::Error::custom("Expected 3 elements in Vec3 array"))?;
            let z: f64 = seq.next_element()?.ok_or_else(|| de::Error::custom("Expected 3 elements in Vec3 array"))?;
            if seq.next_element::<f32>()?.is_some() {
                return Err(de::Error::custom("Expected only 3 elements in Vec3 array"));
            }
            Ok(DVec3::new(x, y, z))
        }
    }

    deserializer.deserialize_any(Vec3Visitor)
}


pub fn deser_vec2<'de, D>(deserializer: D) -> Result<[Int; 2], D::Error>
where
    D: Deserializer<'de>,
{
    // String "720 720" or array [720, 720] both accepted
    struct Vec2Visitor;

    impl<'de> serde::de::Visitor<'de> for Vec2Visitor {
        type Value = [Int; 2];

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of 2 integers or a string 'w h'")
        }

        fn visit_str<E>(self, value: &str) -> Result<[Int; 2], E>
        where
            E: de::Error,
        {
            let parts: Vec<&str> = value.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(E::custom("Expected 2 components for Vec2 string"));
            }
            let x = parts[0].parse::<Int>().map_err(|_| E::custom("Failed parsing width"))?;
            let y = parts[1].parse::<Int>().map_err(|_| E::custom("Failed parsing height"))?;
            Ok([x, y])
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<[Int; 2], A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let x: Int = seq.next_element()?.ok_or_else(|| de::Error::custom("Expected 2 elements"))?;
            let y: Int = seq.next_element()?.ok_or_else(|| de::Error::custom("Expected 2 elements"))?;
            if seq.next_element::<Int>()?.is_some() {
                return Err(de::Error::custom("Expected only 2 elements"));
            }
            Ok([x, y])
        }
    }

    deserializer.deserialize_any(Vec2Visitor)
}

pub fn deser_nearplane<'de, D>(deserializer: D) -> Result<NearPlane, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 5 {
        return Err(de::Error::custom("Expected 5 elements for NearPlane"));
    }
    Ok(NearPlane {
        left: parts[0].parse().map_err(|_| de::Error::custom("Failed parsing left"))?,
        right: parts[1].parse().map_err(|_| de::Error::custom("Failed parsing right"))?,
        bottom: parts[2].parse().map_err(|_| de::Error::custom("Failed parsing bottom"))?,
        top: parts[3].parse().map_err(|_| de::Error::custom("Failed parsing top"))?,
        near_distance: parts[4].parse().map_err(|_| de::Error::custom("Failed parsing near_distance"))?,
    })
}

pub fn deserialize_ambient_light<'de, D>(deserializer: D) -> Result<Vec<Vector3>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Unexpected, Visitor};
    use std::fmt;

    struct AmbientLightVisitor;

    impl<'de> Visitor<'de> for AmbientLightVisitor {
        type Value = Vec<Vector3>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string 'r g b' or an array of such strings")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![parse_vec3(v).map_err(de::Error::custom)?])
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(elem) = seq.next_element::<String>()? {
                vec.push(parse_vec3(&elem).map_err(de::Error::custom)?);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_any(AmbientLightVisitor)
}

/// Helper function: parse a string like "25 25 25" into Vector3
/// TODO: Use it in other deserializers 
/// TODO: Make f64 flexible for f32 as well
fn parse_vec3(s: &str) -> Result<Vector3, String> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!("Expected 3 values, got {}", parts.len()));
    }
    let x = parts[0].parse::<f64>().map_err(|e| e.to_string())?;
    let y = parts[1].parse::<f64>().map_err(|e| e.to_string())?;
    let z = parts[2].parse::<f64>().map_err(|e| e.to_string())?;
    Ok(Vector3::new(x, y, z))
}
