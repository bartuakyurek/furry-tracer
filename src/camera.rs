/*

    Declare Camera and its related structs like NearPlane
    
    @date: Oct, 2025
    @author: bartu
*/


use tracing::{info, debug, error};
use crate::numeric::{Int, Index, Float, Vector3, approx_zero};

#[derive(Default, Debug, Clone)]
pub struct Camera {
    pub(crate) _id: Index,
    pub(crate) position: Vector3,
    pub(crate) gaze: Vector3,
    pub(crate) up: Vector3,
    pub(crate) nearplane: NearPlane,
    pub(crate) near_distance: Float,
    pub(crate) image_resolution: [usize; 2],
    pub(crate) image_name: String,
    num_samples: Int,
    w : Vector3,
    v : Vector3,
    u : Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
                image_name: String::from("default.png"), 
                ..Default::default()
            }
    }
    
    pub fn new_from(id: Index, position: Vector3, gaze: Vector3, up: Vector3, nearplane: NearPlane, near_distance: Float, image_resolution: [usize; 2], image_name: String, num_samples: Int) -> Self {
        let mut cam = Camera {
            _id: id,
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

    pub fn get_resolution(&self) -> (usize, usize) {
        (self.image_resolution[0], self.image_resolution[1])
    }
}


#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct NearPlane {
    pub(crate) left: Float,
    pub(crate) right: Float,
    pub(crate) bottom: Float,
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

    pub fn new_from<T>(vec4: Vec<T>) -> Result<Self, String>
    where
        T: Into<Float> + Copy,
    {
        if vec4.len() != 4 {
            error!(
                "Expected 4 elements to construct NearPlane, got {}. Ignoring remaining elements.",
                vec4.len()
            );
        }

        Ok(NearPlane {
            left: vec4[0].into(),
            right: vec4[1].into(),
            bottom: vec4[2].into(),
            top: vec4[3].into(),
        })
    }

}


#[cfg(test)]
mod tests {
    use super::*; // access to the outer scope

    #[test]
    fn test_setup() {

        let cam = Camera::new_from(
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