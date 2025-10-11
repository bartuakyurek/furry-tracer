/*

    A simple ray tracer implemented for CENG 795 course.

    @date: Oct, 2025
    @author: Bartu

*/

use std::{self, env};
use tracing::{info, debug, error, warn};
use tracing_subscriber;

mod scene;
mod camera;
mod shapes;
mod numeric;
mod material;
mod renderer;
mod json_parser;
mod geometry_processing;
use crate::{json_parser::parse_json795, numeric::{Vector3}};


fn main() {

    // Logging 
    tracing_subscriber::fmt::init(); // logs to console

    // Parse args
    let args: Vec<String> = env::args().collect();
    let json_path: &String = if args.len() == 1 {
        debug!("No arguments were provided, setting default scene path...");
        &String::from("./assets/test.json")
    } else if args.len() == 2 {
        &args[1]
    } else {
        error!("Usage: {} <filename>.json", args[0]);
        std::process::exit(1);
    };
    
    // Parse JSON
    info!("Loading scene from {}...", json_path);
    let root =  match parse_json795(json_path) {
        Ok(root) => {
            info!("Scene loaded successfully.\n {:#?}", root);
            root
        }
        Err(e) => {
            error!("Failed to load scene: {}", e);
            return;
        }
    };

    // Render image and return array of RGB
    let image_data = match renderer::render(root.scene) {
        Ok(image_data) => {info!("Render completed."); image_data}
        Err(e) => {error!("Failed to render scene: {}", e); return;}
    };

    // Write image data to file
    // ...
    warn!("Don't forget to write image data to a file.");
    info!("Finished execution.");
}
