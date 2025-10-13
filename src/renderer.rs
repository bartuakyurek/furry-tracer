/*

    Given Scene description and Camera,
    render an image.

    Currently supports:
        - <TODO: type of raytracing here e.g. recursive>


    @date: Oct 11, 2025
    @author: Bartu
*/

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::BufWriter;
use tracing::{debug, warn};

use crate::scene::{Scene};
use crate::numeric::{Vector3, Float, Index};

#[derive(Clone)]
pub struct ImageData {
    pixel_colors : Vec<Vector3>, // Vector of RGB per pixel
    pixel_centers: Vec<Vector3>,
    width : usize,
    height: usize,
    resolution: [usize; 2],
    name: String, // TODO: width, height, name info actually is stored under camera as well
                  // is it wise to copy those into ImageData? I thought it is more organized this way.
}

pub fn pixel_centers(width: usize, height: usize, offset: Vector3) -> Vec<Vector3>{
        // set offset to Vector3::ZERO if image is centered to camera (as assumed in this course)
    
        // TODO: fill here.... 
}

impl ImageData {

    
    pub fn new(width: usize, height: usize, resolution: [usize; 2], name: String, background: Vector3) -> Self {
        // Create a new image of specified background color
        // Set background to Vector3::ZERO for black background
        let pixel_colors = vec![background; width * height];
        Self::new_from(width, height, resolution, name, pixel_colors)
    }

    pub fn new_from(width: usize, height: usize, resolution: [usize; 2], name: String, pixel_colors: Vec<Vector3>) -> Self {
        let pixel_centers = pixel_centers(width, height, Vector3::ZERO);
        ImageData {
            pixel_colors,
            pixel_centers,
            width,
            height,
            resolution,
            name,
        }
    }

    

    pub fn flatten_color(self) -> Vec<Float> {
        // Return [R1, G1, B1, R2, G2, B2, ...] vector
        // where each triplet is RGB color of a pixel.
        self.pixel_colors.into_iter().flat_map(|v| [v.x, v.y, v.z]).collect()
    }
    pub fn to_rgb(self) -> Vec<u8> {
        let rgb_vec = self.flatten_color().into_iter().map(|x| {
            if x < 0.0 || x > 255.0 {
                warn!("Clamping applied to x={} value for RGB conversion.", x);
            }
            x.clamp(0.0, 255.0) as u8
        }).collect();

        rgb_vec
    } 

    pub fn check_extension(self, path: &PathBuf, extension: &str) -> bool {
        path.extension().unwrap().to_str().unwrap() == extension
    }

    pub fn get_fullpath(&self, path: &str) -> PathBuf {
        // Check if provided path is a folder 
        // if so, create a .png under this folder
        // otherwise use the provided path as is
        let extension = "png";
        {
            let path = Path::new(path);
            let mut finalpath: PathBuf = path.to_path_buf();
            if path.is_dir() {
                // create <imagename>.png under this directory 
                finalpath = path.join(self.name.clone());
            } 
            if finalpath.set_extension(extension) {
                warn!("Extension changed to .{}", extension);
            }
            finalpath
        }
    }

    pub fn save_png(self, path: &str) -> Result<(), Box<dyn std::error::Error>>{
        // Path is either a folder name or
        // full path including <imagename>.png
        // If full path is not provided it will use 
        // stored image name.
        //
        // WARNING: Assumes RGB is used (no transparency available atm)
        // WARNING: Only png accepted for now, if specified image name has another
        // extension it will be silently converted to .png
        //
        // DISCLAIMER: This function is based on https://docs.rs/png/0.18.0/png/
        let path: PathBuf = self.get_fullpath(path);

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
        writer.write_image_data(&data)?; // Save
        
        Ok(())
    }
}


pub fn render(scene: Scene) -> Result<Vec<ImageData>, Box<dyn std::error::Error>>
{
    let mut images: Vec<ImageData> = Vec::new();
    for mut cam in scene.cameras.all(){
        cam.setup(); // TODO: Could this be integrated to deserialization? Because it's easy to forget calling it
        debug!("{:?}", cam);

        // TODO: Return Vec<ImageData>
        let (width, height) = (cam.image_resolution[0], cam.image_resolution[1]);
        warn!("Use Camera.ImageResolution for width and Height.");

        let pixel_colors = vec![Vector3::ZERO; width * height];
        let pixel_centers = vec![Vector3::ZERO; width * height];

        let im = ImageData { pixel_colors, pixel_centers, width, height, name: cam.image_name };
        
        images.push(im);
    }
    
    Ok(images)
}

