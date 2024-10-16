/* LINKS

src/main.rs
src/lib/
src/lib/filters

*/



//
// created by J. Blackburn - Aug 27 2024
//

use crate::lib::{filters, panel, image_thread};

use eframe::egui;
use image::{ImageBuffer, Rgba};

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};
use std::thread;


type SharedImageBuffer = Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>;
type SharedBoolean     = Arc<Mutex<bool>>;

pub fn view(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) { 
        
        // create shared image buffer
    let shareable_image_buffer: SharedImageBuffer = Arc::new(Mutex::new(image_buffer));;

        // create update switch
    let update_switch: SharedBoolean = Arc::new(Mutex::new(true));

        // do stuff with image buffer in separate thread

    image_thread::start_background_thread(shareable_image_buffer.clone(), update_switch.clone());

        // create egui app with the shareable buffer & update switch
    let mut app = panel::ViewPanel::new(shareable_image_buffer.clone(), update_switch.clone());
        // display image view panel
    let _ = eframe::run_native(
        "Image Filters View Panel", 
        eframe::NativeOptions::default(), 
        Box::new(|_cc| Ok(Box::new(app)))
    );

} // end view method
