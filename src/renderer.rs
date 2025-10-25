/*

    Given Scene description and Camera,
    render an image.

    Currently supports:
        - <TODO: type of raytracing here e.g. recursive>


    @date: Oct 11, 2025
    @author: Bartu
*/

use tracing::{debug, info, warn};

use crate::ray::Ray;
use crate::scene::{Scene};
use crate::numeric::{Vector3};
use crate::image::{ImageData};
use crate::interval::{Interval};


pub fn render(scene: Scene) -> Result<Vec<ImageData>, Box<dyn std::error::Error>>
{
    let mut images: Vec<ImageData> = Vec::new();
    for mut cam in scene.cameras.all() {
        cam.setup(); // TODO: Could this be integrated to deserialization? Because it's easy to forget calling it
        debug!("{:#?}", cam);
        debug!("Nearplane corners are {:#?}", &cam.get_nearplane_corners());
        
        let (width, height) = cam.get_resolution();
        let pixel_colors = vec![Vector3::ZERO; width * height]; // Colors range [0, 255], not [0, 1]
        
        // ------------------------ Pixel Colors ------------------------------
        // 1- Generate primary rays from camera center to pixel centers
        let rays = cam.generate_primary_rays();
        let shapes = scene.objects.all();

        // 2- Recursive ray tracing here!
        for ray in rays.iter(){ // TODO: parallelize with rayon, for each pixel 
            // TODO: later we'll use acceleration structures instead of checking *all* objects like this
            for shape in shapes.iter() {
                let hit_record = shape.intersects_with(ray, &Interval::NONNEGATIVE, &scene.vertex_data);
            }
        }
       
        // --------------------------------------------------------------------
        
        let im = ImageData::new_from_colors(cam.image_resolution, cam.image_name, pixel_colors);
        images.push(im);
    }
    
    Ok(images)
}

