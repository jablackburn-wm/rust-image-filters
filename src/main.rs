//
// created by J. Blackburn - Aug 26 2024
//


use eframe::egui;
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

    match mode_argument {

        // TODO: improve panic message
        Mode::Invalid => { panic!( 
   "
    ******************* Image Filters Error ******************* 

    Program Mode invalid or not specified: Try 'run' or 'view'. 
    ex: cargo run -- view image.png

    ***********************************************************
   " 
        ); }

        Mode::View => {
            View::view(args)
        }

        Mode::Run  => {
            Run::run(args)
        }
    }
}
