/* LINKS

src/main.rs
src/lib/

*/



//
// created by J. Blackburn - Aug 27 2024
//

use eframe::egui;
use image::{ImageBuffer, Rgba};

use std::sync::{Arc, Mutex};

type SharedImageBuffer = Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>;
type SharedBoolean     = Arc<Mutex<bool>>;

struct ViewPanel {
    image_buffer: SharedImageBuffer,
    image_texture: Option<egui::TextureHandle>,
    update_switch: SharedBoolean
}

impl ViewPanel {

    fn new( image_buffer: SharedImageBuffer, update_switch: SharedBoolean ) -> Self {
        ViewPanel {
            image_buffer: image_buffer,
            image_texture: None,
            update_switch: update_switch
        }
    }



    fn load_image( ctx: &egui::Context, image_buffer: SharedImageBuffer ) -> egui::TextureHandle {
        
        let locked_image_buffer = image_buffer.lock().unwrap();

        let size = [ locked_image_buffer.width() as usize, locked_image_buffer.height() as usize ];
        let raw_rgba = locked_image_buffer.as_raw();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &raw_rgba);

        // return image texture
        ctx.load_texture("image", color_image, Default::default()) 
    }


}

impl eframe::App for ViewPanel {
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // check if view panel should update
        let mut locked_update_switch = self.update_switch.lock().unwrap();


        if *locked_update_switch == true { // if image buffer is updated or no texture is loaded, load texture

            self.image_texture = Some(ViewPanel::load_image(ctx, self.image_buffer.clone()));
            *locked_update_switch = false; // toggle update switch

        }


        // show the ui
        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_size = ui.available_size();

            if let Some(texture) = &self.image_texture { // if the texture is loaded, draw it

                // scale image texture
                let image_size = texture.size_vec2();
                let scale = (panel_size.x / image_size.x).min(panel_size.y / image_size.y);

                let scaled_size = image_size * scale;

                ui.add(egui::Image::new(texture).fit_to_exact_size(scaled_size));
            }

        });

    }

}


pub fn view(image_buffer: SharedImageBuffer) {

    // create update switch
    let update_switch: SharedBoolean = Arc::new( Mutex::new( true ));
    
    // create egui app with the buffer
    let mut app = ViewPanel::new(image_buffer.clone(), update_switch.clone());

    
    // display image view panel
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Image Filters View Panel", native_options, Box::new(|_cc| Ok(Box::new(app))));

    // do stuff with image buffer

}
