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
use tracing::{debug, info, warn, error};

use crate::camera::Camera;
use crate::dataforms::VertexData;
use crate::lights::LightContext;
use crate::material::{DiffuseMaterial, Material, HeapAllocMaterial};
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
    
    debug_assert!(hit_record.normal.is_normalized());
    let ray_origin = hit_record.point + (hit_record.normal * epsilon);
    let distance_vec = point_light.position - ray_origin;
    let distance_squared = distance_vec.norm_squared(); // TODO: Cache?
    let distance = distance_squared.sqrt();
    let dir = distance_vec / distance;
    debug_assert!(dir.is_normalized());
    let shadow_ray = Ray::new(ray_origin, dir);
    let interval = Interval::new(0.0, distance); 
    (shadow_ray, interval)
}

// TODO: Wait why there is both scene and shapes where scene already should contain shapes?
pub fn shade_diffuse(scene: &Scene, shapes: &ShapeList, hit_record: &HitRecord, ray: &Ray, mat: &HeapAllocMaterial) -> Vector3 {
    let mut color = Vector3::ZERO;
    for point_light in scene.lights.point_lights.all() {
            
            let (shadow_ray, interval) = get_shadow_ray(&point_light, hit_record, scene.shadow_ray_epsilon);
            if !any_hit(&shadow_ray, &interval, shapes, &scene.vertex_data) {
                let (shadow_dir, eye_dir) = (shadow_ray.direction, ray.direction); 
                let light_context = LightContext::from_shadow(&point_light, interval.max, shadow_dir, eye_dir, hit_record.normal); // WARNING: w_o is reverse of primary ray, see slide 01_B p.78
                // TODO WARNING: This assumes ray interval has light distance information inside... prone to error. 
                color += mat.radiance(&light_context); 
            }
            //else { // TODO: For debugging shadow acne!! Remove this else part later
            //    color = Vector3::new(255. , 0., 0.);
            //}
    }
    color
}

pub fn get_color(ray: &Ray, scene: &Scene, shapes: &ShapeList, depth: usize) -> Vector3 { // TODO: add depth & check depth > scene.max_recursion_depth
   // TODO: Shouldn't we box the scene or even Rc<scene> here? otherwise it lives on the stack
   // and it's a huge struct, isn't it?
   if depth >= scene.max_recursion_depth {
        return scene.background_color;
   }
   
   let t_interval = Interval::positive(scene.intersection_test_epsilon);
   //let mut color = Vector3::new(0.,0., 0.); // No background color here, otw it'll offset additional colors 
   if let Some(hit_record) = closest_hit(ray, &t_interval, shapes, &scene.vertex_data) {
        
        let mat: &HeapAllocMaterial = &scene.materials.materials[hit_record.material - 1];
        let mut color = mat.ambient_radiance(scene.lights.ambient_light);
        let mat_type = mat.get_type();
        color += match mat_type{ // WARNING: Expecting lowercase material
            "diffuse" => shade_diffuse(scene, shapes, &hit_record, &ray, mat),
            "mirror" => {
               
                let mut attenuation = Vector3::ZERO;
                let mut rays_out: Vec<Ray>;
                if mat.scatter(ray, &hit_record, &mut attenuation, &mut rays_out) {
                    for new_ray in rays_out {
                        get_color(&new_ray, scene, shapes, depth + 1); // L_i
                        let light_context = LightContext::from_mirror(ray.direction, hit_record.normal, received);
                        mat.radiance(&light_context) 
                    }
                }
                else {
                    Vector3::ZERO // TODO: Background?
                }
                
            }, 
            _ => {
                // WARNING: Below does not panic when json has unknown material because parser defaults it to Diffuse (however it does panic if you make a typo or not implement shading function)
                panic!(">> Unknown material type '{}'! Shading function for this material is missing.", mat_type); 
            },
        };
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
            pixel_colors[i] = get_color(ray, scene, &shapes, 0);
        }
       
        // -------------------------------------------------------------------- 
        
        let im = ImageData::new_from_colors(cam.image_resolution, cam.image_name, pixel_colors);
        images.push(im);
    }
    
    Ok(images)
}

