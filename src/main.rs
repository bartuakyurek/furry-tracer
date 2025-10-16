/*

    A simple ray tracer implemented for CENG 795 course.

    @date: Oct, 2025
    @author: Bartu

*/

use std::{self, env};
use tracing::{info, warn, error};
use tracing_subscriber;

mod scene;
mod camera;
mod shapes;
mod numeric;
mod material;
mod renderer;
mod dataforms;
mod json_parser;
mod geometry_processing;
use crate::{json_parser::import_json, scene::Scene};


fn main() {

    // Logging on console
    tracing_subscriber::fmt::init(); 

    // Parse args
    let args: Vec<String> = env::args().collect();
    let json_path: &String = if args.len() == 1 {
        warn!("No arguments were provided, setting default scene path...");
        &String::from("./assets/bunny.json")
    } else if args.len() == 2 {
        &args[1]
    } else {
        error!("Usage: {} <filename>.json", args[0]);
        std::process::exit(1);
    };
    
    // Parse JSON
    let scn: Scene = match import_json(&json_path.as_str()) {
        Ok(s) => {
            info!("Scene loaded successfully.\n {:#?}", s); 
            s
        }
        Err(e) => {
            error!("Failed to load scene: {}", e);
            return;
        }
    };

    // Render image and return array of RGB
    let images = match renderer::render(scn) {
        Ok(image_data) => {info!("Render completed."); image_data}
        Err(e) => {error!("Failed to render scene: {}", e); return;}
    };

    // Write images to .png files
    for im in images.into_iter() {
        let imagefolder = "./"; // Save to current folder 
        if let Err(e) = im.save_png(&imagefolder) {
            eprintln!("Failed to save {}: {}", imagefolder, e);
        }
    }
    info!("Finished execution.");
}
