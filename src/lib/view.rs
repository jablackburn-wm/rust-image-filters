//
// created by J. Blackburn - Aug 27 2024
//

// view synchronizes 2 threads: the view panel and image thread
// view panel handles displaying the contents of image buffer, 
// image thread prints menu options to the terminal and handles user input

use crate::lib::{panel, image_thread};

use eframe::egui;
use image::{ImageBuffer, Rgba};

use std::sync::{Arc, Mutex};


type SharedImageBuffer = Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>;
type SharedBoolean     = Arc<Mutex<bool>>;

pub fn view(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) { 


            // TODO:
        // rebuild syncing threads to use atomic bools instead of arc mutex


        
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

} 
