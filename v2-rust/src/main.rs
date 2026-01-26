#![warn(clippy::pedantic)]

use std::path::Path;
use std::env;
use image::{ImageBuffer, Rgba};

const COUNT_OF_ARGS_REQUIRED: usize = 3;

fn main() {

    let args: Vec<String> = env::args().collect();

    let name = Path::new(&args[0])
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("shrinkshot");

    if args.len() != COUNT_OF_ARGS_REQUIRED {
        eprintln!("Usage: {} <source_image> <target_image>", name);
        std::process::exit(1);
    }

    let source_path = &args[1];
    let target_path = &args[2];

    let Ok(img) = image::open(Path::new(source_path)) else {
        eprintln!("{name}: error loading file: {source_path}");
        std::process::exit(1);
    };

    let (original_width, origial_height) = (img.width(), img.height());
    let mut pixels = img.to_rgba8().to_vec();
    let (new_width, new_height) = shrink(&mut pixels, original_width, origial_height);

    println!("Original dimensions: {original_width}x{origial_height}");
    println!("New dimensions: {new_width}x{new_height}");

    let new_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        new_width,
        new_height,
        pixels
    ).expect("Failed to create image from pixel data");

    let Ok(()) = new_img.save(Path::new(target_path)) else {
        eprintln!("{name}: error saving file: {target_path}");
        std::process::exit(1);
    };
}

fn shrink(
    pixels: &mut Vec<u8>,
    original_width: u32,
    original_height: u32
) -> (u32, u32) {

    let mut new_widht = original_width;
    let mut new_height = original_height / 2;

    pixels[0] = 0;

    // TODO

    (new_widht, new_height,)
}
