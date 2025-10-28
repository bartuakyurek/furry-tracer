/*

    Declare Material trait, and store data related to
    different types of materials. Currently supporting:
        - Diffuse
        - Mirror
        - Conductor (TBI)
        - Dielectric (TBI)

    @date: Oct, 2025
    @author: Bartu

*/
use std::fmt::Debug;
use std::cmp::max;
use tracing::{error, info, warn};
use serde::{Deserialize, de::DeserializeOwned};
use crate::json_parser::*;
use crate::numeric::{approx_zero, Float, Vector3};
use crate::ray::{Ray, HitRecord}; // TODO: rename it to light or lighting, not lights?

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// MATERIAL TRAIT
/// 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Material : Debug + Send + Sync  {
    
    fn new_from(value: &serde_json::Value) -> Self 
    where
        Self: Sized + DeserializeOwned + Default,
    {
        match serde_json::from_value::<Self>(value.clone()) {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to parse DiffuseMaterial: {e}. JSON: {value}");
                Self::default()
            }
        }
    }
    fn get_type(&self) -> &str;
    fn diffuse(&self, w_i: Vector3, n: Vector3) -> Vector3;
    fn specular(&self, w_o: Vector3, w_i: Vector3, n: Vector3) -> Vector3;
    fn ambient(&self) -> Vector3; 

    //fn get_attenuiation(&self, ray_in: &Ray, ray_out: &mut Option<Ray>, hit_record: &HitRecord) -> Vector3;
    fn attenuate_reflect(&self, ray_in: &Ray, ray_t: Float) -> Vector3;
    fn attenuate_refract(&self, ray_in: &Ray, ray_t: Float) -> Vector3;
    fn reflect(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray>;
    fn refract(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray>;
}

pub type HeapAllocMaterial = Box<dyn Material>; // Box, Rc, Arc -> Probably will be Arc when we use rayon


////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// DIFFUSE
/// 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct DiffuseMaterial {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient_rf: Vector3,
    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse_rf: Vector3,
    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular_rf: Vector3,
    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exponent: Float,
}


impl Default for DiffuseMaterial {
    fn default() -> Self {
        DiffuseMaterial {
            _id: 0,
            ambient_rf: Vector3::new(0.0, 0.0, 0.0),
            diffuse_rf: Vector3::new(1.0, 1.0, 1.0),
            specular_rf: Vector3::new(0.0, 0.0, 0.0),
            phong_exponent: 1.0,
        }
    }
}

impl DiffuseMaterial {

    
}

impl Material for DiffuseMaterial{


    fn get_type(&self) -> &str {
        "diffuse"
    }

    fn reflect(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray> {
        warn!("Reflect not implemented for Diffuse! Only use shadow rays for now.");
        todo!()
    }

    fn refract(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray> {
        warn!("There is no refract for DiffuseMaterial. If this is intentional please delete this warning.");
        None
    }

    fn attenuate_reflect(&self, ray_in: &Ray, ray_t: Float) -> Vector3 {
        //warn!("Attenuate not implemented for Diffuse! Only use shadow rays for now.");
        Vector3::ONE // No attenuation for diffuse
    }

    fn attenuate_refract(&self, ray_in: &Ray, ray_t: Float) -> Vector3 {
        //warn!("Attenuate not implemented for Diffuse! Only use shadow rays for now.");
        Vector3::ONE // No attenuation for diffuse
    }

    fn ambient(&self) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.75)
        // e.g. for test.json it is [25, 25, 25]
        self.ambient_rf 
    }

    fn diffuse(&self, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.73)
        // TODO: reduce the verbosity here
        
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let cos_theta = w_i.dot(n).max(0.0);
        self.diffuse_rf * cos_theta 
    }

    fn specular(&self, w_o: Vector3, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.80)
        debug_assert!(w_o.is_normalized());
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let h = (w_i + w_o).normalize(); //(w_i + w_o) / (w_i + w_o).norm();
        debug_assert!(h.is_normalized());
        
        let p = self.phong_exponent;
        let cos_a = n.dot(h).max(0.0);
        self.specular_rf * cos_a.powf(p)
    }   
    

}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// MIRROR
/// 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct MirrorMaterial {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient_rf: Vector3,
    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse_rf: Vector3,
    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular_rf: Vector3,
    #[serde(rename = "MirrorReflectance", deserialize_with = "deser_vec3")]
    pub mirror_rf: Vector3,
    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exponent: Float,
}

impl Default for MirrorMaterial {
    fn default() -> Self {
        Self {
            _id: 0,
            ambient_rf: Vector3::new(0.0, 0.0, 0.0),
            diffuse_rf: Vector3::new(0.5, 0.5, 0.5),
            specular_rf: Vector3::new(0.0, 0.0, 0.0),
            mirror_rf: Vector3::new(0.5, 0.5, 0.5),
            phong_exponent: 1.0,
        }
    }
}

impl MirrorMaterial {
 
}

impl Material for MirrorMaterial {

    fn get_type(&self) -> &str {
        "mirror"
    }

    fn attenuate_reflect(&self, ray_in: &Ray, ray_t: Float) -> Vector3 {
        self.mirror_rf 
    }

    fn attenuate_refract(&self, ray_in: &Ray, ray_t: Float) -> Vector3 {
        Vector3::ONE  
    }

    fn reflect(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray> {
        // Reflected ray from Slides 02, p.4 (Perfect Mirror)
        // wr ​= - wo ​+ 2 n (n . wo)
        // WARNING: Assume ray_in.direction = wi = - wo
        let n = hit_record.normal;
        let w_i = ray_in.direction;
        let w_r = w_i - 2. * n * (n.dot(w_i));
        debug_assert!(w_r.is_normalized());

        Some(Ray::new(hit_record.point + (n * epsilon), w_r)) // Always reflects
    }

    fn refract(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<Ray> {
        None // Never refract
    }
    
    fn ambient(&self) -> Vector3 {
        self.ambient_rf  
    }

    fn diffuse(&self, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.73)
        // TODO: reduce the verbosity here
        
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let cos_theta = w_i.dot(n).max(0.0);
        self.diffuse_rf * cos_theta  
    }

    fn specular(&self, w_o: Vector3, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.80)
        debug_assert!(w_o.is_normalized());
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let h = (w_i + w_o).normalize(); //(w_i + w_o) / (w_i + w_o).norm();
        debug_assert!(h.is_normalized());
        
        let p = self.phong_exponent;
        let cos_a = n.dot(h).max(0.0);
        self.specular_rf * cos_a.powf(p)  
    }   
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// DIELECTRIC (GLASS)
/// 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct DielectricMaterial {
    #[serde(deserialize_with = "deser_usize")]
    pub _id: usize,
    #[serde(rename = "AmbientReflectance", deserialize_with = "deser_vec3")]
    pub ambient_rf: Vector3,
    #[serde(rename = "DiffuseReflectance", deserialize_with = "deser_vec3")]
    pub diffuse_rf: Vector3,
    #[serde(rename = "SpecularReflectance", deserialize_with = "deser_vec3")]
    pub specular_rf: Vector3,
    #[serde(rename = "MirrorReflectance", deserialize_with = "deser_vec3")]
    pub mirror_rf: Vector3,
    #[serde(rename = "PhongExponent", deserialize_with = "deser_float")]
    pub phong_exponent: Float,
    #[serde(rename = "AbsorptionCoefficient", deserialize_with = "deser_vec3")]
    pub absorption_coeff: Vector3,
    #[serde(rename = "RefractionIndex", deserialize_with = "deser_float")]
    pub refraction_index: Float,
}

impl Default for DielectricMaterial {
    fn default() -> Self {
        Self {
            _id: 0,
            ambient_rf: Vector3::new(0.0, 0.0, 0.0),
            diffuse_rf: Vector3::new(0.5, 0.5, 0.5),
            specular_rf: Vector3::new(0.0, 0.0, 0.0),
            mirror_rf: Vector3::new(0.5, 0.5, 0.5),
            phong_exponent: 1.0,
            absorption_coeff: Vector3::new(0.01, 0.01, 0.01),
            refraction_index: 1.5,
        }
    }
}

impl DielectricMaterial {

    fn fresnel_reflection_ratio(&self, ray_in: &Ray, hit_record: &HitRecord) ->  Float {
        // returns reflection ratio F_r
        // (for transmissoion use 1 - F_r )
        // see slides 02, p.20 for notation

        // d: incoming normalized ray
        // n: surface normal
        let d = ray_in.direction;
        let n = hit_record.normal;
        debug_assert!(d.is_normalized());
        debug_assert!(n.is_normalized());
        let cos_theta = n.dot(-d);

        let mut n1 = 1.00029 as Float; // Assuming Air in slides 02, p.22
        let mut n2 = self.refraction_index;
        if !hit_record.is_front_face {
            n1 = self.refraction_index;
            n2 = 1.00029 as Float;
        }
        
        let ratio_squared: Float = (n1 / n2).powi(2);
        let one_minus_cossqrd: Float = 1. - (cos_theta.powi(2));
        let inside_of_sqrt: Float = 1. - (ratio_squared * one_minus_cossqrd);

        let cos_phi: Float = if inside_of_sqrt < 0. {
            info!("Total internal reflection occured!");
            return 1.; // TODO No need to compute, right? I assume it is total internal reflection (p.16)
        }
        else {
            inside_of_sqrt.sqrt()
        };

        let n1cos_p = n1 * cos_phi;
        let n2cos_p = n2 * cos_phi;
        let n1cos_t = n1 * cos_theta;
        let n2cos_t = n2 * cos_theta;

        let r_parallel: Float = (n2cos_t - n1cos_p) / (n2cos_t + n1cos_p);
        let r_perp: Float = (n1cos_t - n2cos_p) / (n1cos_t + n2cos_p);
        // TODO: in slides 02, p.20 this ratio has - in the denominator but that makes the ratio = 1
        // I assumed this is a typo and checked Fresnel from wikipedia... but I gotta ask for confirmation

        let f_r = 0.5 * (r_parallel.powi(2) + r_perp.powi(2));
        debug_assert!( (f_r > 1e-20) && (f_r < 1.+1e-20)); // in range [0,1]
        f_r
    }
}


impl Material for DielectricMaterial {

    fn get_type(&self) -> &str {
        "dielectric"
    }

    fn attenuate_reflect(&self, ray_in: &Ray, ray_t: Float) -> Vector3 {
        // Slides 02, p.27, only e^(-Cx) part
        // where C is the absorption coefficient
        // WARNING: ray_in.origin is assumed to be the location of the last hit point
        // i.e. point in p.28 with arrow to L(x)
        self.mirror_rf
    }

    fn attenuate_refract(&self, ray_in: &Ray, ray_t: Float) -> Vector3 {
        // Slides 02, p.27, only e^(-Cx) part
        // where C is the absorption coefficient
        // WARNING: ray_in.origin is assumed to be the location of the last hit point
        // i.e. point in p.28 with arrow to L(x)
        (- self.absorption_coeff * ray_in.distance_at(ray_t)).exp() 
    }

    fn reflect(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<(Ray)> {
        // Fresnel reflection 
        todo!()
        // Don't forget to set attenuation
    }

    fn refract(&self, ray_in: &Ray, hit_record: &HitRecord, epsilon: Float) -> Option<(Ray)> {
        todo!()
        // Don't forget to set attenuation
    }
    
    fn ambient(&self) -> Vector3 {
        self.ambient_rf  
    }

    fn diffuse(&self, w_i: Vector3, n: Vector3) -> Vector3 {
        // TODO: these are copy paste from Diffuse material,
        // should we refactor them into a single function within
        // this crate?
        // Actually a better implementation would be to create a struct
        // for diffuse, specular, ambient, and phong as these four are common
        // and then just store them in material, that way you can move diffuse
        // and other common functions inside Material trait! I believe this is 
        // a Rusty way to implement it but before that I better decouple json
        // parser from these material structs...
        
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let cos_theta = w_i.dot(n).max(0.0);
        self.diffuse_rf * cos_theta  
    }

    fn specular(&self, w_o: Vector3, w_i: Vector3, n: Vector3) -> Vector3 {
        // Returns outgoing radiance (see Slides 01_B, p.80)
        debug_assert!(w_o.is_normalized());
        debug_assert!(w_i.is_normalized());
        debug_assert!(n.is_normalized());

        let h = (w_i + w_o).normalize(); //(w_i + w_o) / (w_i + w_o).norm();
        debug_assert!(h.is_normalized());
        
        let p = self.phong_exponent;
        let cos_a = n.dot(h).max(0.0);
        self.specular_rf * cos_a.powf(p)  
    }   
}