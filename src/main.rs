use eframe::egui;
use image::{ImageBuffer, Rgba};

mod lib;

use std::env;
use std::sync::{Arc, Mutex};



// main function should start by determining if the first cmd line 
// argument is "run", "view", or invalid
//
// the perfect trivial example for using enums

enum Mode { // TODO: maybe rename Mode enum ???
    View,
    Run, 
    Invalid
}

fn main() {

    let args: Vec<String> = env::args().collect();

    // turn 1st argument into Mode enum variant
    // TODO: maybe rename first_arg ???
    let first_arg = match args.get(1).map(String::as_str) {
        Some("view") => Mode::View,
        Some("run")  => Mode::Run,
        _            => Mode::Invalid
    };

    match first_arg {
        Mode::Invalid => { panic!( "Program Mode invalid or not specified: Try 'run' or 'view'." ); }

        Mode::View => {

            println!("Running in view mode");

        }

        Mode::Run  => {

            println!("Running in run mode");

        }
    }
}
