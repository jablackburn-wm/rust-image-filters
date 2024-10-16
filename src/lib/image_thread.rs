// LINKS
/*

src/main.rs
src/lib/filters
src/lib

*/

use crate::lib::filters;
use image::{ImageBuffer, Rgba};

use std::sync::{Arc, Mutex};

use std::path::Path;
use std::io::{stdin, stdout, Write};
use std::thread;

type SharedImageBuffer = Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>;
type SharedBoolean     = Arc<Mutex<bool>>;

pub fn start_background_thread(image_buffer: SharedImageBuffer, update_switch: SharedBoolean) {
    
    let background_thread = thread::spawn(move || { 

            // TODO:
        // add shared quit signal

            // initialize menu option enum
        #[derive(PartialEq)]
        enum MenuOption {
            Empty, 
            Filter, 
            Reset, 
            Save, 
            Quit,
        }

        let mut menu_option = MenuOption::Empty;

            // loop while menu option not quit
        while menu_option != MenuOption::Quit {

                // print CLI menu options
            println!(
    "
    ******************* Image Filters Menu ******************* 

    f - FILTER image buffer          r - RESET image to source      

    s - SAVE image to path           q - QUIT

    **********************************************************

    Enter Option:
    "
            );

                // get user input
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Error: could not read user input");
            let _ = stdout().flush();

                // match against input for MenuOptions enum variants
            menu_option = match input.trim() {
                "f" => MenuOption::Filter,
                "r" => MenuOption::Reset,
                "s" => MenuOption::Save,
                "q" => MenuOption::Quit,
                 _  => MenuOption::Empty
            };

                // match against menu_option for corresponding code branch
            match menu_option {
                MenuOption::Empty => {
                    println!("Option invalid - try again.");
                    continue;
                }

                MenuOption::Save  => {
                    print!("Saving image - Specify output path:");
                    let _ = stdout().flush();

                        // get / create output path
                    let mut output_path = String::new();
                    stdin().read_line(&mut output_path).expect("Error: could not read user input");
                    let _ = stdout().flush();

                        // save to output path
                    let locked_image_buffer = image_buffer.lock().unwrap();
                    locked_image_buffer.save(Path::new(output_path.trim())); 

                    continue;
                }

                MenuOption::Reset => {
                    println!("loading original image");
                    println!("This actually does nothing right now");

                        // pass control to resetting method?

                        // toggle update switch to trigger reload
                    let mut locked_update_switch = update_switch.lock().unwrap();
                    *locked_update_switch = true; 

                    continue;
                }

                MenuOption::Filter => {

                        // Ex. of filter usage: filters::invert(image_buffer.clone());
                        //
                        // how to avoid matching for every individual filter possibility? -> move to filters
                        // module, passing filter name argument
                        // extra parameters handled in filters? 
                        //
                        // pass to filter handler method?
                    
                    print!("Enter filter name: ");
                    let _ = stdout().flush();

                    let mut filter_name = String::new();
                    stdin().read_line(&mut filter_name).expect("Error: could not read user input");
                    let _ = stdout().flush();

                    filters::invert(image_buffer.clone());

                        // toggle update switch to trigger reload
                    let mut locked_update_switch = update_switch.lock().unwrap();
                    *locked_update_switch = true; 

                    continue;
                }

                MenuOption::Quit  => { continue; }

            } // end match against menu option

        } // end while loop

    }); // end background thread
}