//
// created by J. Blackburn - Aug 27 2024
//

// run function should get the second argument (the image argument) 
// and print an error message if it is not specified, otherwise continue

pub fn run(args: Vec<String>) {

    let image_argument: Option<&str> = args.get(2).map(String::as_str);

    let image_path: &str = image_argument.expect("Error: no image path specified"); // TODO: improve error message

    println!("Image path exists, continuing in run mode with image: {}", image_path);
}
