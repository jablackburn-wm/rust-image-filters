// 
// created by J. Blackburn - Nov 16 2024
//

mod invert;
pub use invert::invert;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use image::{ImageBuffer, Rgba};

type SharedImageBuffer = Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>;





    // filter handler has a set of keys mapping to functions
    // and a reference to the image buffer to pass to the filters
pub struct FilterHandler {
    keys: HashMap<String, fn(SharedImageBuffer)>,
    image_buffer: SharedImageBuffer,
}

impl FilterHandler {

    pub fn new(image_buffer: SharedImageBuffer) -> Self {

            // load key set
        let mut keys: HashMap<String, fn(SharedImageBuffer)> = HashMap::new();
        keys.insert(String::from("invert"), invert::invert);

            // return Filter Handler with loaded key set & image buffer
        FilterHandler { 
            keys, 
            image_buffer: image_buffer.clone(), 
        }
    }

    pub fn apply_filter(&self, key: &str) {

            // if filter is in the keyset, apply it to the image buffer
        if let Some(filter) = self.keys.get(key) {
            filter(self.image_buffer.clone());
        } else {
            panic!("FilterHandler failed to filter image buffer");
        }
    }
}
