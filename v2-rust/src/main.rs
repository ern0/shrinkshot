#![warn(clippy::pedantic)]

use std::path::Path;
use std::env;
use image::{ImageBuffer, Rgba};

const REQUIRED_ARG_NUMBER: usize = 3;

type SideSize = usize;
const IGNORED_MARGIN_SIZE: SideSize = 2;
const MINIMUM_SIDE_SIZE: SideSize = 10;

#[derive(Clone, Copy)]
struct Region {
    position: SideSize,
    length: SideSize,
}

impl Region {

    fn new(position: SideSize, length: SideSize) -> Self {
        Self {
            position,
            length,
        }
    }
}

struct BarsDefinition<'bd> {
    pixels: &'bd [u8],
    outer_length: SideSize,
    outer_stepping: SideSize,
    inner_length: SideSize,
    inner_stepping: SideSize,
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

    println!("Original dimensions: {original_width}x{origial_height}");

    let mut pixels = img.to_rgba8().to_vec();
    let (new_width, new_height,) = shrink(&mut pixels, original_width, origial_height);

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

    if original_width < MINIMUM_SIDE_SIZE || original_height < MINIMUM_SIDE_SIZE {
        return (original_width, original_height,);
    }

// println!("---- horizontal ----");

    let horizontal_keep_vec = calc_keep_bars(
        BarsDefinition {
            pixels,
            outer_length: original_width,
            outer_stepping: 1,
            inner_length: original_height,
            inner_stepping: original_width,
        }
    );

// println!("---- vertical ----");

    let vertical_keep_vec = calc_keep_bars(
        BarsDefinition {
            pixels,
            outer_length: original_height,
            outer_stepping: original_width,
            inner_length: original_width,
            inner_stepping: 1,
        }
    );

    let new_width = horizontal_keep_vec.iter().map(|region| region.length).sum();
    let new_height = vertical_keep_vec.iter().map(|region| region.length).sum();

    if new_width == original_width && new_height == original_height {
        return (original_width, original_height,);
    }

    let mut destination_index = 0;

    for vertical_region in &vertical_keep_vec {
        for line_offset in 0..=vertical_region.length {

            let source_line_index = (vertical_region.position + line_offset) * original_width * 4;

            for horizontal_region in &horizontal_keep_vec {
                let mut source_column_index = source_line_index + (horizontal_region.position * 4);

                for _columns in 0..=horizontal_region.length {
                    for channel in 0..4 {
                        pixels[destination_index + channel] =
                            pixels[source_column_index + channel];
                    }
                    source_column_index += 4;
                    destination_index += 4;
                }

            }
        }

    }

    (new_width, new_height,)
}

fn calc_keep_bars(bars_def: BarsDefinition) -> Vec<Region> {

// println!("calc_keep_bars(): outer_length={outer_length} outer_stepping={outer_stepping} inner_length={inner_length} inner_stepping={inner_stepping}");

    let mut keep_list = Vec::new();

    if IGNORED_MARGIN_SIZE > 0 {
        keep_list.push(Region::new(0, IGNORED_MARGIN_SIZE));
    }

    let outer_start_position = IGNORED_MARGIN_SIZE + 1;   // +1 to skip first item
    let outer_stop_position = bars_def.outer_length - IGNORED_MARGIN_SIZE;
    let mut outer_index =
        (outer_start_position * bars_def.outer_stepping * 4) +
        (IGNORED_MARGIN_SIZE * bars_def.inner_stepping * 4);

    let mut region = Region::new(outer_start_position - 1, 1);
    let mut same_stroke = false;

    for outer_position in outer_start_position..outer_stop_position {

        if neighbour_bars_are_identical(
            bars_def.pixels,
            outer_index,                // starting index
            bars_def.inner_length,      // length
            bars_def.inner_stepping,    // step
            bars_def.outer_stepping,    // neighbour offset
        ) {
            if !same_stroke {

                same_stroke = true;
                keep_list.push(region.clone());

                region.position = outer_position;
                region.length = 0;
            }
        } else {
            same_stroke = false;
            region.length += 1;
        }

        outer_index += bars_def.outer_stepping * 4;
    }

    if region.length > 0 {
        keep_list.push(region);
    }

    if IGNORED_MARGIN_SIZE > 0 {
        keep_list.push(Region::new(bars_def.outer_length - IGNORED_MARGIN_SIZE, IGNORED_MARGIN_SIZE));
    }

    keep_list
}

fn neighbour_bars_are_identical(
        pixels: &[u8],
        starting_index: SideSize,
        length: SideSize,
        step: SideSize,
        neighbour_offset: SideSize,
    ) -> bool {

    let start_pos = IGNORED_MARGIN_SIZE;
    let end_pos = length - IGNORED_MARGIN_SIZE;

// println!("neighbour_bars_are_identical(): starting_index={starting_index} length={length} step={step} neighbour_offset={neighbour_offset} start_pos={start_pos} end_pos={end_pos}");

    let mut index = starting_index;

    for _pos in start_pos..end_pos {

        if !pixels_are_identical(pixels, index, index - neighbour_offset*4) {
            return false;
        }

        index += step * 4;
    }

    true
}

fn pixels_are_identical(pixels: &[u8], index1: SideSize, index2: SideSize) -> bool {

    for channel in 0..3 {

        if pixels[index1 + channel] != pixels[index2 + channel] {
            return false;
        }
    }

    true
}
