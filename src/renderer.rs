/*

    Given Scene description and Camera,
    render an image.

    Currently supports:
        - <TODO: type of raytracing here e.g. recursive>


    @date: Oct 11, 2025
    @author: Bartu
*/

use std::rc::Rc;
use std::io::{self, Write};
use tracing::{debug, info, warn};

use crate::ray::Ray;
use crate::scene::{Scene};
use crate::numeric::{Vector3, Float};
use crate::image::{ImageData};
use crate::interval::{Interval, FloatConst};
use crate::shapes::{Shape};

type ShapeList = Vec<Rc<dyn Shape>>;

pub fn get_color(ray: &Ray, scene: &Scene, shapes: &ShapeList) -> Vector3 {
    
   let mut t_min = FloatConst::INF;
   let t_interval = Interval::positive(scene.intersection_test_epsilon);
   let mut color = scene.background_color;
   for shape in shapes.iter() {
       if let Some(hit_record) = shape.intersects_with(ray, &t_interval, &scene.vertex_data){
           
           if t_min > hit_record.ray_t { // Only update color if the hit object is closer than previous
               t_min = hit_record.ray_t;
               let n = hit_record.normal;
               color = 0.5 * (n + Vector3::new(1.0, 1.0, 1.0)); // shift to [0, 1]
               color = color * 255.0; // scale to [0, 255]
               
           }
       }
   }
   color
}

pub fn render(scene: &Scene) -> Result<Vec<ImageData>, Box<dyn std::error::Error>>
{
    let mut images: Vec<ImageData> = Vec::new();
    for mut cam in scene.cameras.all() {
        cam.setup(); // TODO: Could this be integrated to deserialization? Because it's easy to forget calling it
        debug!("{:#?}", cam);
        debug!("Nearplane corners are {:#?}", &cam.get_nearplane_corners());
        
        let (width, height) = cam.get_resolution();
        let n_pixels = width * height;
        let mut pixel_colors = vec![scene.background_color; n_pixels]; // Colors range [0, 255], not [0, 1]
        
        // ------------------------ Pixel Colors ------------------------------
        // 1- Generate primary rays from camera center to pixel centers
        let eye_rays = cam.generate_primary_rays();
        let shapes: ShapeList = scene.objects.all();

        // 2- Recursive ray tracing here!
        for (i, ray) in eye_rays.iter().enumerate(){ // TODO: parallelize with rayon, for each pixel 
           // TODO: later we'll use acceleration structures instead of checking *all* objects like this
           //eprint!("\rComputing {} / {}", i + 1, n_pixels); 
           //io::stdout().flush().unwrap(); TODO: how to do it with tracing crate?
            pixel_colors[i] = get_color(ray, scene, &shapes);
        }
       
        // --------------------------------------------------------------------
        
        let im = ImageData::new_from_colors(cam.image_resolution, cam.image_name, pixel_colors);
        images.push(im);
    }
    
    Ok(images)
}

