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
use bevy_math::NormedVectorSpace;
use tracing::span::Record;
use tracing::{debug, info, warn};

use crate::dataforms::VertexData;
use crate::ray::{HitRecord, Ray};
use crate::scene::{Scene};
use crate::numeric::{Float, Int, Vector3};
use crate::image::{ImageData};
use crate::interval::{Interval, FloatConst};
use crate::shapes::{Shape};

type ShapeList = Vec<Rc<dyn Shape>>;

pub fn closest_hit(ray: &Ray, t_interval: &Interval, shapes: &ShapeList, vertex_data: &VertexData) -> Option<HitRecord>{
    let mut rec = None;
    let mut t_min = FloatConst::INF;
    for shape in shapes.iter() { // TODO: later we'll use acceleration structures instead of checking *all* objects like this
       if let Some(hit_record) = shape.intersects_with(ray, &t_interval, vertex_data){
           // Update if new hit is closer 
           if t_min > hit_record.ray_t { 
               t_min = hit_record.ray_t;
               rec = Some(hit_record);
           }
       }
   }
   rec
}

pub fn any_hit(ray: &Ray, t_interval: &Interval, shapes: &ShapeList, vertex_data: &VertexData) -> bool {
    // Check if ray intersects with any shape in the scene
    for shape in shapes.iter() { // TODO: later we'll use acceleration structures instead of checking *all* objects like this
       if let Some(_) = shape.intersects_with(ray, &t_interval, vertex_data){
           return true;
       }
   }
   false
}

pub fn get_color(ray: &Ray, scene: &Scene, shapes: &ShapeList) -> Vector3 { // TODO: add depth & check depth > scene.max_recursion_depth
    
   let t_interval = Interval::positive(scene.intersection_test_epsilon);
   let mut color = Vector3::new(0.,0., 0.); // No background color here, otw it'll offset additional colors 
   if let Some(hit_record) = closest_hit(ray, &t_interval, shapes, &scene.vertex_data) {
        // Generate shadow rays 
        for point_light in scene.lights.point_lights.all() {
            let light_pos = point_light.position;
            let ray_origin = hit_record.point + (hit_record.normal * scene.shadow_ray_epsilon);
            let distance_vec = light_pos - ray_origin;
            let distance_squared = distance_vec.norm_squared();
            let distance = distance_squared.sqrt();
            let dir = distance_vec / distance;
            let shadow_ray = Ray::new(ray_origin, dir);
            //let mut shadow_hit = None;
            let interval = Interval::new(0.0, distance);
            //closest_hit(&shadow_ray, &interval, shapes,  &scene.vertex_data, &mut shadow_hit);

            if !any_hit(&shadow_ray, &interval, shapes, &scene.vertex_data) {
            
                let light_intensity = point_light.rgb_intensity;
                let perp_irradiance = Vector3::new(0., 0., 0.);
                color += scene.materials.materials[hit_record.material - 1].radiance(perp_irradiance); 
            }
        }

        // TODO add ambient
        let ambient_intensity = scene.lights.ambient_light;
        color
   }
   else {
        scene.background_color // no hit
   }
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

