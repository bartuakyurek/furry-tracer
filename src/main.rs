/*

    A simple ray tracer implemented for CENG 795 course.

    @date: Oct, 2025
    @author: Bartu

*/

use std::{self, env, time::Instant};
use tracing::{info, warn, error, debug};
use tracing_subscriber;

mod ray;
mod image;
mod scene;
mod camera;
mod shapes;
mod numeric;
mod interval;
mod material;
mod renderer;
mod geometry;
mod dataforms;
mod json_parser;
use crate::{json_parser::parse_json795};


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
    info!("Loading scene from {}...", json_path);
    let mut root =  match parse_json795(json_path) {
        Ok(root) => {
            root
        }
        Err(e) => {
            error!("Failed to load scene: {}", e);
            return;
        }
    };
    root.scene.setup_after_json(); // TODO: This created actural structs for materials etc. but should be done in a different way
    debug!("Scene is setup successfully.\n {:#?}", root);
    let root = root; // Shadow mutatability before render

    // Render image and return array of RGB
    let start = Instant::now();
    let images = match renderer::render(&root.scene) {
        Ok(image_data) => {info!("Render completed."); image_data}
        Err(e) => {error!("Failed to render scene: {}", e); return;}
    };
    info!("Rendering of {} image(s) took: {:?}", images.len(), start.elapsed()); 

    // Write images to .png files
    for im in images.into_iter() {
        let imagefolder = "./"; // Save to current folder 
        if let Err(e) = im.save_png(&imagefolder) {
            eprintln!("Failed to save {}: {}", imagefolder, e);
        }
    }
    info!("Finished execution.");
}
