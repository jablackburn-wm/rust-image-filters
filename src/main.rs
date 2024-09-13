/* LINKS (for quick navigation in vi)

src/lib/mod.rs
src/lib
src/lib/filters

Cargo.toml

*/



//
// created by J. Blackburn - Aug 26 2024
//

use image::{ImageBuffer, Rgba};

mod lib;
use lib::{Run, View};

use std::env;
use std::sync::{Arc, Mutex};



// main function should start by determining if the first cmd line 
// argument is "run", "view", or invalid
//
// the perfect trivial example for using enums

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

    // Both modes use the image file path, so defining it can be handled in main
    // however view uses an arc mutex of the image buffer, so creating the actual image buffer 
    // can be handled in the following  match statement
    //

    match mode_argument {

        Mode::Invalid => { panic!( 
   "
    ******************* Image Filters Error ******************* 

    Program Mode invalid or not specified: Try 'run' or 'view'. 
    ex: cargo run -- view image.png

    ***********************************************************
   " 
        ); }


        Mode::View => {

            // get image argument
            let image_argument: Option<&str> = args.get(2).map(String::as_str);
            let image_path: &str = image_argument.expect("Error: no image path specified"); // TODO: improve error message

            // load image
            println!("Attempting to load input image: {}", image_path); // TODO: improve info message

            // view mode should take only an image buffer wrapped in arc mutex
            let image_buffer = Arc::new( Mutex::new(
                     image::open(image_path)
                        .expect("Error: view mode couldnt open your image") // TODO: improve error message
                        .to_rgba8()
            ));

            View::view(image_buffer.clone())

        }


        Mode::Run  => {

            // get image argument
            let image_argument: Option<&str> = args.get(2).map(String::as_str);
            let image_path: &str = image_argument.expect("Error: no image path specified"); // TODO: improve error message

            // load image
            println!("Attempting to load input image: {}", image_path); // TODO: improve info message

            // run mode can deal with the raw image buffer without a shared reference, 
            // and needs the rest of the arguments to infer image manipulation strategy
            let image_buffer = image::open(image_path)
                                    .expect("Error: run mode couldnt open your image") // TODO: improve error message
                                    .to_rgba();

            Run::run(image_buffer, args)

        }
    }
}
