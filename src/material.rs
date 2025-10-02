use crate::numeric::{RGBColor, Float, Int, Vector3};

pub trait Material {
    fn ambient(&self) -> RGBColor;
    fn diffuse(&self) -> RGBColor;
    fn specular(&self) -> RGBColor;
    fn phong_exponent(&self) -> Float;

    // optional: extend with reflection/refraction later
    fn mirror_reflectance(&self) -> Option<RGBColor> {
        None
    }
}


use serde::Deserialize;
use crate::json_parser::*;

#[derive(Debug, Deserialize, Clone)]
pub struct DiffuseMaterial {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient: RGBColor,

    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse: RGBColor,

    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular: RGBColor,

    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exp: Float,
}

impl Material for DiffuseMaterial {
    fn ambient(&self) -> RGBColor { self.ambient }
    fn diffuse(&self) -> RGBColor { self.diffuse }
    fn specular(&self) -> RGBColor { self.specular }
    fn phong_exponent(&self) -> Float { self.phong_exp }
}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// ///////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
pub struct MirrorMaterial {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    pub id: Int,

    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient: RGBColor,

    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse: RGBColor,

    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular: RGBColor,

    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exp: Float,

    #[serde(rename = "MirrorReflectance", deserialize_with = "deser_vec3")]
    pub mirror: RGBColor,
}

impl Material for MirrorMaterial {
    fn ambient(&self) -> RGBColor { self.ambient }
    fn diffuse(&self) -> RGBColor { self.diffuse }
    fn specular(&self) -> RGBColor { self.specular }
    fn phong_exponent(&self) -> Float { self.phong_exp }
    fn mirror_reflectance(&self) -> Option<RGBColor> { Some(self.mirror) }
}

pub type BoxedMaterial = Box<dyn Material>;

#[derive(Debug, Deserialize)]
pub struct SceneMaterials {
    #[serde(rename = "Material")]
    pub materials: Vec<serde_json::Value>, // parse raw JSON first
}


fn parse_material(value: serde_json::Value) -> BoxedMaterial {
    if let Some(t) = value.get("_type") {
        match t.as_str().unwrap() {
            "mirror" => Box::new(serde_json::from_value::<MirrorMaterial>(value).unwrap()),
            _ => panic!("Unknown material type"),
        }
    } else {
        Box::new(serde_json::from_value::<DiffuseMaterial>(value).unwrap())
    }
}
