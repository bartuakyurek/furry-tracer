/*

    Declare Camera and its related structs like NearPlane
    
    @date: Oct, 2025
    @author: bartu
*/


use serde::{Deserialize};
use tracing::{info, debug};
use crate::numeric::{Int, Float, Vector3, approx_zero};
use crate::json_parser::*;
use crate::dataforms::{SingleOrVec};

#[derive(Debug, Default, Deserialize)]
pub struct Cameras {
    #[serde(rename = "Camera")]
    camera: SingleOrVec<Camera>, // Allow either single cam (as in test.json) or multiple cams
}

impl Cameras {
    /// Always returns a Vec<Camera> regardless of JSON being a single object or array
    pub fn all(&self) -> Vec<Camera> {
        self.camera.all()
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Camera {
    #[serde(rename = "_id", deserialize_with = "deser_int")]
    id: Int,
    
    #[serde(rename = "Position", deserialize_with = "deser_vec3")]
    position: Vector3,

    #[serde(rename = "Gaze", deserialize_with = "deser_vec3")]
    gaze: Vector3,

    #[serde(rename = "Up", deserialize_with = "deser_vec3")]
    up: Vector3,

    #[serde(rename = "NearPlane", deserialize_with = "deser_nearplane")]
    nearplane: NearPlane,

    #[serde(rename = "NearDistance", deserialize_with = "deser_float")]
    near_distance: Float,

    #[serde(rename = "ImageResolution", deserialize_with = "deser_pair")]
    pub image_resolution: [usize; 2], // TODO: Should be usize instead of Int but deserialization needs modification to handle Int for i32, usized etc. 

    #[serde(rename = "ImageName")]
    pub image_name: String,

    #[serde(rename = "NumSamples", deserialize_with = "deser_int")]
    pub num_samples: Int,

    #[serde(skip)]
    w : Vector3,

    #[serde(skip)]
    v : Vector3,

    #[serde(skip)]
    u : Vector3,

}

impl Camera {
    pub fn new(id: Int, position: Vector3, gaze: Vector3, up: Vector3, nearplane: NearPlane, near_distance: Float, image_resolution: [usize; 2], image_name: String, num_samples: Int) -> Self {
        let mut cam = Camera {
            id,
            position,
            gaze,
            up,
            nearplane,
            near_distance,
            image_resolution,
            image_name,
            num_samples,
            w : Vector3::NAN,
            v : Vector3::NAN,
            u : Vector3::NAN,
        };
        cam.setup();
        cam
    }
    pub fn setup(&mut self) {
        // Compute w, v, u vectors
        // assumes Gaze and Up is already provided during creation
        // corrects Up vector if given Up was not perpendicular to
        // Gaze vector.

        self.w = - self.gaze.normalize();
        self.v = self.up.normalize();
        self.u = self.v.cross(self.w);

        if !approx_zero(self.up.dot(self.gaze)) {
            info!("Gaze and Up vectors are not perpendicular, correcting v...");
            self.v = self.w.cross(self.u);
        }
        debug!("Camera w: {}, v: {}, u: {}", self.w, self.v, self.u);    
        debug_assert!(approx_zero(self.u.dot(self.w))); 
        debug_assert!(approx_zero(self.v.dot(self.w))); 
        debug_assert!(approx_zero(self.v.dot(self.u))); 
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
pub(crate) struct NearPlane {
    #[serde(deserialize_with = "deser_float")]
    pub(crate) left: Float,
    #[serde(deserialize_with = "deser_float")]
    pub(crate) right: Float,
    #[serde(deserialize_with = "deser_float")]
    pub(crate) bottom: Float,
    #[serde(deserialize_with = "deser_float")]
    pub(crate) top: Float,
}


impl NearPlane {
    pub fn new(left: Float, right: Float, bottom: Float, top: Float) -> Self {
        NearPlane { 
            left,
            right,
            bottom,
            top,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; // access to the outer scope

    #[test]
    fn test_setup() {

        let cam = Camera::new(
            1,
            Vector3::new(0., 0., 0.),
            Vector3::new(0., 0.2, -10.), // Not perpendicular to up
            Vector3::new(0., 1., 0.),
            NearPlane::new(-1., 1., -1., 1.),
            10.0,
            [720, 720],
            "test.png".to_string(),
            1,
        );
        assert!(approx_zero(cam.u.dot(cam.v))); 
        assert!(approx_zero(cam.v.dot(cam.w))); 
        assert!(approx_zero(cam.w.dot(cam.u))); 
        // These asserts are redundant with debug_asserts in new( )
        // but keeping them here just for sanity checks.

    }
}