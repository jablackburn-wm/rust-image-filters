//
// created by J. Blackburn - Aug 26 2024
//

// main function should start by determining if the first cmd line 
// argument is "run", "view", or invalid
//
// then open an image buffer of the image 
// and pass control to the view and run methods

mod lib;
use lib::{Run, View};
use image::{ImageBuffer, Rgba};
use std::env;

    // TODO:
        // improve error messages

enum Mode {
    View,
    Run, 
    Invalid
}

fn main() {

        // get cli argument vector
    let args: Vec<String> = env::args().collect();

        // turn first argument into Mode enum variant
    let mode_argument = match args.get(1).map(String::as_str) {
        Some("view") => Mode::View,
        Some("run")  => Mode::Run,
        _            => Mode::Invalid
    };

    match mode_argument {

        Mode::Invalid => { panic!( 
   "
    ******************* Image Filters Error ******************* 

    Program Mode invalid or not specified: Try 'run' or 'view'. 
    ex: cargo run -- view image.png

    ***********************************************************
   " 
        ); }

            // TODO:
                // find a way to not repeat view/run three times in a row

        Mode::View => { View::view(load_image_buffer(&args)) }

        Mode::Run  => { Run::run(load_image_buffer(&args), args) }
    }
}



fn load_image_buffer(args: &Vec<String>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    
    let image_path: &str = args
                            .get(2)
                            .map(String::as_str)
                            .expect("Error: no image path specified"); 

    println!("Attempting to load input image: {}", image_path); 

        // return buffer
    image::open(image_path)
            .expect("Error: run mode couldnt open your image") 
            .to_rgba()
}

