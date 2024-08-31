//
// created by J. Blackburn - Aug 27 2024
//

use std::sync::{Arc, Mutex};
use image::{ImageBuffer, Rgba};

type SharedImageBuffer = Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>;

pub fn view(image_buffer: SharedImageBuffer) {

    println!("continuing in view mode");

}
