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
use crate::lights::LightContext;
use crate::ray::{HitRecord, Ray};
use crate::scene::{PointLight, Scene};
use crate::numeric::{Float, Int, Vector3};
use crate::image::{ImageData};
use crate::interval::{Interval, FloatConst};
use crate::shapes::{Shape};

type ShapeList = Vec<Rc<dyn Shape>>;

pub fn closest_hit(ray: &Ray, t_interval: &Interval, shapes: &ShapeList, vertex_data: &VertexData) -> Option<HitRecord>{
    // Refers to p.91 of slide 01_b, lines 3-7
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

pub fn get_shadow_ray(point_light: &PointLight, hit_record: &HitRecord, epsilon: Float) -> (Ray, Interval) { // TODO: Should we box hitrecord here?
    let light_pos = point_light.position;
    let ray_origin = hit_record.point + (hit_record.normal * epsilon);
    let distance_vec = light_pos - ray_origin;
    let distance_squared = distance_vec.norm_squared();
    let distance = distance_squared.sqrt();
    let dir = distance_vec / distance;
    let shadow_ray = Ray::new(ray_origin, dir);
    let interval = Interval::new(0.0, distance + epsilon); // TODO: Is it correct add epsilon here as well?
    (shadow_ray, interval)
}

pub fn get_color(ray: &Ray, scene: &Scene, shapes: &ShapeList) -> Vector3 { // TODO: add depth & check depth > scene.max_recursion_depth
   // TODO: Shouldn't we box the scene or even Rc<scene> here? otherwise it lives on the stack
   // and it's a huge struct, isn't it?
   let t_interval = Interval::positive(scene.intersection_test_epsilon);
   //let mut color = Vector3::new(0.,0., 0.); // No background color here, otw it'll offset additional colors 
   if let Some(hit_record) = closest_hit(ray, &t_interval, shapes, &scene.vertex_data) {
        let mat = scene.materials.materials[hit_record.material - 1];
        let mut color = mat.ambient_radiance(scene.lights.ambient_light);
        for point_light in scene.lights.point_lights.all() {
            
            let (shadow_ray, interval) = get_shadow_ray(&point_light, &hit_record, scene.shadow_ray_epsilon);
            if !any_hit(&shadow_ray, &interval, shapes, &scene.vertex_data) {
            
                let light_context = LightContext::new_from(&point_light, &hit_record);
                color += mat.radiance(&light_context); 
            }
        }
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

