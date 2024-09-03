# Rust Image Filters

cli-based image manipulation program implemented in rust, based around applying various "filters".



### Modes:


"Run"  - Basic implementation, applies the filter and saves the new image.

    Usage: Cargo run -- run [image path] [filter key] [output path]

"View" - GUI implementation, allows user to see contents of the buffer being modified before saving.

    Usage:  Cargo run -- [mode] [image-path] 
