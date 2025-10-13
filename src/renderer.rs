/*

    Given Scene description and Camera,
    render an image.

    Currently supports:
        - <TODO: type of raytracing here e.g. recursive>


    @date: Oct 11, 2025
    @author: Bartu
*/

use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use tracing::{info, debug, error, warn};

use crate::scene::{Scene};
use crate::numeric::{Vector3, Float};

pub struct ImageData {
    pixels : Vec<Vector3>, // Vector of RGB per pixel
    width : usize,
    height: usize,
}

impl ImageData {
    pub fn flatten(self) -> Vec<Float> {
        // Return [R1, G1, B1, R2, G2, B2, ...] vector
        // where each triplet is RGB color of a pixel.
        self.pixels.into_iter().flat_map(|v| [v.x, v.y, v.z]).collect()
    }
    pub fn to_rgb(self) -> Vec<u8> {
        let rgb_vec = self.flatten().into_iter().map(|x| {
            if x < 0.0 || x > 255.0 {
                warn!("Clamping applied to x={} value for RGB conversion.", x);
            }
            x.clamp(0.0, 255.0) as u8
        }).collect();

        rgb_vec
    } 

    pub fn save_png(self, path: &str){
        // WARNING: Assumes RGB is used (no transparency available atm)
        // DISCLAIMER: This function is based on https://docs.rs/png/0.18.0/png/
        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32); // Width is 2 pixels and height is 1.
    
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        let data = self.to_rgb();
        writer.write_image_data(&data).unwrap(); // Save
    }
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

    let pixels = vec![Vector3::ZERO; width * height];
    let im = ImageData { pixels, width, height };
    Ok(im)

}

