//
// created by J. Blackburn - Sept 13 2024
//

use std::sync::{Arc, Mutex};
use image::{ImageBuffer, Rgba};

type SharedImageBuffer = Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>;

pub fn invert(image_buffer: SharedImageBuffer) {

    let mut locked_image_buffer = image_buffer.lock().unwrap();

    let (width, height) = locked_image_buffer.dimensions();
    for y in 0..height {
        for x in 0..width {
            
            // create inverted pixel
            let pixel = locked_image_buffer.get_pixel(x, y);
            let inverted_pixel = invert_pixel(pixel);

            // Update the pixel in the image
            locked_image_buffer.put_pixel(x, y, inverted_pixel);
        }
    }
}


fn invert_pixel(pixel: &Rgba<u8>) -> Rgba<u8> {

            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

            // Invert the subpixel values
            let inverted_r = 255 - r;
            let inverted_g = 255 - g;
            let inverted_b = 255 - b;

            // Return a new RGBA pixel with inverted values
            Rgba([inverted_r, inverted_g, inverted_b, pixel[3]])
}
