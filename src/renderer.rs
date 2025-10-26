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

use crate::dataforms::VertexData;
use crate::ray::{HitRecord, Ray};
use crate::scene::{Scene};
use crate::numeric::{Float, Int, Vector3};
use crate::image::{ImageData};
use crate::interval::{Interval, FloatConst};
use crate::shapes::{Shape};

type ShapeList = Vec<Rc<dyn Shape>>;

pub fn closest_hit(ray: &Ray, t_interval: &Interval, shapes: &ShapeList, vertex_data: &VertexData, rec: &mut Option<HitRecord>) {

    let mut t_min = FloatConst::INF;
    for shape in shapes.iter() { // TODO: later we'll use acceleration structures instead of checking *all* objects like this
       if let Some(hit_record) = shape.intersects_with(ray, &t_interval, vertex_data){
           // Update if new hit is closer 
           if t_min > hit_record.ray_t { 
               t_min = hit_record.ray_t;
               *rec = Some(hit_record);
           }
       }
   }
}

pub fn get_color(ray: &Ray, scene: &Scene, shapes: &ShapeList) -> Vector3 { // TODO: add depth & check depth > scene.max_recursion_depth
    
   let t_interval = Interval::positive(scene.intersection_test_epsilon);
   let mut color = scene.background_color;
   let mut hit_record = None;
   closest_hit(ray, &t_interval, shapes, &scene.vertex_data, &mut hit_record);
   
   if let Some(hit_record) = hit_record {
        // Update color
        //let n = hit_record.normal;
        //color = 0.5 * (n + Vector3::new(1.0, 1.0, 1.0)); // shift to [0, 1]
        //color = color * 255.0; // scale to [0, 255]

        for point_light in scene.lights.point_lights.all() {
            let pos = point_light.position;
            let intensity = point_light.rgb_intensity;
            let origin = hit_record.point + (hit_record.normal * scene.shadow_ray_epsilon);
            let dir = pos - origin;
            let shadow_ray = Ray::new(origin, dir);
            let mut shadow_hit = None;
            let interval = Interval::NONNEGATIVE;
            closest_hit(&shadow_ray, &interval, shapes,  &scene.vertex_data, &mut shadow_hit);

            if let Some(shadow_hit) = shadow_hit {
                color = Vector3::new(255., 0.,0.);  // TODO
            }
        }

        // TODO add ambient
        let ambient_intensity = scene.lights.ambient_light;
   }
   color
}

pub fn render(scene: &Scene) -> Result<Vec<ImageData>, Box<dyn std::error::Error>>
{
    let mut images: Vec<ImageData> = Vec::new();
    for mut cam in scene.cameras.all() {
        cam.setup(); // TODO: Could this be integrated to deserialization? Because it's easy to forget calling it
        
        let n_pixels: usize =  cam.image_resolution[0] * cam.image_resolution[1];
        let mut pixel_colors = vec![scene.background_color; n_pixels]; // Colors range [0, 255], not [0, 1]

        let eye_rays = cam.generate_primary_rays();
        let shapes: ShapeList = scene.objects.all();
        for (i, ray) in eye_rays.iter().enumerate(){ // TODO: parallelize with rayon, for each pixel 
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

