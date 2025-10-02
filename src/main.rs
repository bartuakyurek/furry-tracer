/*

A simple ray tracer for CENG 795 course.

@date: 2 Oct, 2025
@author: Bartu
*/

use tracing::{info, debug, error, warn};
use tracing_subscriber;

mod scene;
mod numeric;
mod json_parser;
use crate::json_parser::{parse_json795};

fn main() {
    
    // Logging 
    tracing_subscriber::fmt::init(); // logs to console
  
    // Parse JSON
    let json_path: &str = "./assets/test.json";
    
    match parse_json795(json_path) {
        Ok(scene) => info!("Scene loaded successfully.\n {:#?}", scene),
        Err(e) => error!("Failed to load scene: {}", e),
    };

    info!("Furry tracer finished execution.");
}
