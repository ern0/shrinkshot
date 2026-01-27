#![warn(clippy::pedantic)]

use std::path::Path;
use std::env;
use image::{ImageBuffer, Rgba};

const REQUIRED_ARG_NUMBER: usize = 3;

type SideSize = usize;
const CHANNELS: SideSize = 3;
const IGNORED_MARGIN_SIZE: SideSize = 2;
const MINIMUM_SIDE_SIZE: SideSize = 10;
const COMBINED_MARGIN: SideSize = (IGNORED_MARGIN_SIZE * 2) + MINIMUM_SIDE_SIZE;

#[derive(Clone, Copy)]
struct Region {
    position: SideSize,
    length: SideSize,
}

impl Region {

    fn new(position: SideSize) -> Self {
        Self {
            position,
            length: 1,
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let name = Path::new(&args[0])
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("shrinkshot");

    if args.len() != REQUIRED_ARG_NUMBER {
        eprintln!("Usage: {} <source_image> <target_image>", name);
        std::process::exit(1);
    }

    let source_path = &args[1];
    let target_path = &args[2];

    let Ok(img) = image::open(Path::new(source_path)) else {
        eprintln!("{name}: error loading file: {source_path}");
        std::process::exit(1);
    };

    let (original_width, origial_height) = (
        img.width() as SideSize,
        img.height() as SideSize,
    );
    let mut pixels = img.to_rgba8().to_vec();
    let (new_width, new_height) = shrink(&mut pixels, original_width, origial_height);

    println!("Original dimensions: {original_width}x{origial_height}");
    println!("New dimensions: {new_width}x{new_height}");

    let new_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        new_width as u32,
        new_height as u32,
        pixels
    ).expect("Failed to create image from pixel data");

    let Ok(()) = new_img.save(Path::new(target_path)) else {
        eprintln!("{name}: error saving file: {target_path}");
        std::process::exit(1);
    };
}

fn shrink(
        pixels: &mut [u8],
        original_width: SideSize,
        original_height: SideSize
    ) -> (SideSize, SideSize) {

    let (horizontal_keep_vec, new_widht,) = calc_keep(
        pixels,
        original_width,     // outer length
        1,                  // outer stepping
        original_height,    // inner length
        original_width,     // inner stepping
    );

    let (vertical_keep_vec, new_height,) = calc_keep(
        pixels,
        original_height,    // outer length
        original_width,     // outer stepping
        original_width,     // inner length
        1,                  // inner stepping
    );

    // TODO: copy image

    (new_widht, new_height,)
}

fn calc_keep(
        pixels: &mut [u8],
        outer_length: SideSize,
        outer_stepping: SideSize,
        inner_length: SideSize,
        inner_stepping: SideSize,
    ) -> (Vec<Region>, SideSize) {

    let mut result = Vec::new();
    let mut new_size = outer_length;

    let outer_start_position = IGNORED_MARGIN_SIZE + 1;
    let outer_stop_position =  outer_length - IGNORED_MARGIN_SIZE;

    let mut outer_index = outer_start_position * outer_stepping;
    for _outer_position in outer_start_position..outer_stop_position {

        if is_similar_to_prev(
            pixels,
            outer_index,
            inner_length,
            inner_stepping,
            outer_length,      // inner offset
        ) {

        } else {

        }

        outer_index += outer_stepping * CHANNELS;
    }

    (result, new_size,)
}

fn is_similar_to_prev(
        pixels: &[u8],
        starting_index: SideSize,
        length: SideSize,
        step: SideSize,
        offset: SideSize,
    ) -> bool {

    let start_pos = IGNORED_MARGIN_SIZE;
    let end_pos = length - IGNORED_MARGIN_SIZE;

    let mut index = starting_index;
    for _pos in start_pos..end_pos {

        if is_different(pixels, index, index - offset) {
            return false;
        }

        index += step * CHANNELS;
    }

    true
}

fn is_different(pixels: &[u8], a: SideSize, b: SideSize) -> bool {

    for channel in 0..CHANNELS {
        if pixels[a + channel] != pixels[b + channel] {
            return false;
        }
    }

    true
}
