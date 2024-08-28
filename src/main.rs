//
// created by J. Blackburn - Aug 26 2024
//


use eframe::egui;
use image::{ImageBuffer, Rgba};

// use lib::{Run, View}

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

    match mode_argument {

        // TODO: improve panic message
        Mode::Invalid => { panic!( "Program Mode invalid or not specified: Try 'run' or 'view'." ); }


        // both modes should use the rest of the cli arguments
        Mode::View => {
            println!("Running in view mode");
            // View::view(args)
        }

        Mode::Run  => {
            println!("Running in run mode");
            // Run::run(args)
        }
    }
}
