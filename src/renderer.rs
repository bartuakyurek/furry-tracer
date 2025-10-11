/*

    Given Scene description and Camera,
    render an image.

    Currently supports:
        - <TODO: type of raytracing here e.g. recursive>


    @date: Oct 11, 2025
    @author: Bartu
*/


use tracing::{info, debug, error, warn};

use crate::scene::{Scene};
use crate::numeric::{Vector3};

pub struct ImageData {
    pixels : Vec<Vec<Vector3>>,
    width : usize,
    height: usize,
}

pub fn render(scene: Scene) -> Result<ImageData, Box<dyn std::error::Error>>
{
    
    for mut cam in scene.cameras.all(){
        cam.setup(); // TODO: Could this be integrated to deserialization? Because it's easy to forget calling it
        debug!("{:?}", cam);
    }
    
    // TODO: Return Vec<ImageData>
    let width= 640;
    let height =  640;
    warn!("Use Camera.ImageResolution for width and Height.");

    let pixels = vec![vec![Vector3::ZERO; width]; height];
    let im = ImageData { pixels, width, height };
    Ok(im)
}