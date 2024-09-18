/* LINKS (for quick navigation in vi)

src/lib/mod.rs
src/lib
src/lib/filters

Cargo.toml

*/



//
// created by J. Blackburn - Aug 26 2024
//


mod lib;
use lib::{Run, View};
use image::{ImageBuffer, Rgba};
use std::env;

// main function should start by determining if the first cmd line 
// argument is "run", "view", or invalid
//
// then open an image buffer of the image 
// and pass control to the view and run methods

    // TODO:
        // improve error messages

enum Mode {
    View,
    Run, 
    Invalid
}

fn main() {

    let args: Vec<String> = env::args().collect();

    // turn 1st argument into Mode enum variant
    let mode_argument = match args.get(1).map(String::as_str) {
        Some("view") => Mode::View,
        Some("run")  => Mode::Run,
        _            => Mode::Invalid
    };


    let image_path: &str = args
                                    .get(2)
                                    .map(String::as_str)
                                    .expect("Error: no image path specified"); 


    println!("Attempting to load input image: {}", image_path); 
    let image_buffer = image::open(image_path)
                        .expect("Error: run mode couldnt open your image") 
                        .to_rgba();


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

        Mode::View => { View::view(image_buffer) }

        Mode::Run  => { Run::run(image_buffer, args) }
    }
}
