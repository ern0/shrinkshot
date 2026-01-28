#![warn(clippy::pedantic)]

use std::path::Path;
use std::env;
use image::{ImageBuffer, Rgba};

const REQUIRED_ARG_NUMBER: usize = 3;

type SideSize = usize;
const IGNORED_MARGIN_SIZE: SideSize = 10;
const MINIMUM_SIDE_SIZE: SideSize = 30;
const MINIMUM_SHRINK_SIZE: SideSize = 15;

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

    let mut pixels = img.to_rgba8().to_vec();
    let (new_width, new_height,) = shrink(&mut pixels, original_width, origial_height);

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
    pixels: &mut Vec<u8>,
    original_width: SideSize,
    original_height: SideSize
) -> (SideSize, SideSize) {

    if original_width < MINIMUM_SIDE_SIZE || original_height < MINIMUM_SIDE_SIZE {
        return (original_width, original_height);
    }

    let mut horizontal_keep_vec = calc_keep_bars(
        BarsDefinition {
            pixels,
            outer_length: original_width,
            outer_stepping: 1,
            inner_length: original_height,
            inner_stepping: original_width,
        }
    );

    let mut vertical_keep_vec = calc_keep_bars(
        BarsDefinition {
            pixels,
            outer_length: original_height,
            outer_stepping: original_width,
            inner_length: original_width,
            inner_stepping: 1,
        }
    );

    eliminate_gaps(&mut horizontal_keep_vec);
    eliminate_gaps(&mut vertical_keep_vec);

    let new_width: SideSize = horizontal_keep_vec.iter().map(|region| region.length).sum();
    let new_height: SideSize = vertical_keep_vec.iter().map(|region| region.length).sum();

    if new_width == original_width && new_height == original_height {
        return (original_width, original_height,);
    }

    let mut dest_idx = 0;

    for vertical_region in &vertical_keep_vec {
        for line_in_region in 0..vertical_region.length {
            let src_line_idx = (vertical_region.position + line_in_region) * original_width * 4;

            for horizontal_region in &horizontal_keep_vec {
                for col_in_region in 0..horizontal_region.length {
                    let src_col_idx = src_line_idx + (horizontal_region.position + col_in_region) * 4;

                    for channel in 0..3 {
                        pixels[dest_idx + channel] = pixels[src_col_idx + channel];
                    }

                    dest_idx += 4;
                }
            }
        }
    }

    (new_width, new_height)
}

fn calc_keep_bars(bars_def: BarsDefinition) -> Vec<Region> {

    let mut keep_list = Vec::new();

    if IGNORED_MARGIN_SIZE > 0 {
        keep_list.push(Region::new(0, IGNORED_MARGIN_SIZE));
    }

    let outer_start_position = IGNORED_MARGIN_SIZE;
    let outer_end_position = bars_def.outer_length.saturating_sub(IGNORED_MARGIN_SIZE);

    let mut current_region_start = outer_start_position;
    let mut current_region_length = 0;

    for outer_position in outer_start_position..outer_end_position {
        let outer_index = outer_position * bars_def.outer_stepping * 4;

        let is_identical_to_neighbor = {
            if outer_position > outer_start_position {
                neighbour_bars_are_identical(
                    bars_def.pixels,
                    outer_index,
                    bars_def.inner_length,
                    bars_def.inner_stepping,
                    bars_def.outer_stepping,
                )
            } else {
                false
            }
        };

        if is_identical_to_neighbor {

            if current_region_length > 0 {
                keep_list.push(Region::new(current_region_start, current_region_length));
                current_region_length = 0;
            }

        } else {

            if current_region_length == 0 {
                current_region_start = outer_position;
            }
            current_region_length += 1;

        }
    }

    if current_region_length > 0 {
        keep_list.push(Region::new(current_region_start, current_region_length));
    }

    if (IGNORED_MARGIN_SIZE > 0) && (bars_def.outer_length > IGNORED_MARGIN_SIZE) {
        let margin_start = bars_def.outer_length - IGNORED_MARGIN_SIZE;
        keep_list.push(Region::new(margin_start, IGNORED_MARGIN_SIZE));
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

    if starting_index < neighbour_offset * 4 {
        return false;
    }

    let start_pos = IGNORED_MARGIN_SIZE;
    let end_pos = length.saturating_sub(IGNORED_MARGIN_SIZE);

    if start_pos >= end_pos {
        return true;
    }

    for inner_pos in start_pos..end_pos {

        let index = starting_index + (inner_pos * step * 4);
        let prev_index = index - (neighbour_offset * 4);

        if !pixels_are_similar(pixels, index, prev_index) {
            return false;
        }
    }

    true
}

fn pixels_are_similar(pixels: &[u8], index1: SideSize, index2: SideSize) -> bool {

    pixels.get(index1..index1 + 3) == pixels.get(index2..index2 + 3)

}

fn eliminate_gaps(region_list: &mut Vec<Region>) {

    for i in 0..region_list.len() - 1 {

        let region_end = {
            let region = region_list[i];
            region.position + region.length
        };
        let next_begin = {
            let next = region_list.get(i + 1).unwrap();
            next.position
        };

        let gap = next_begin - region_end;
        if (1..MINIMUM_SHRINK_SIZE).contains(&gap) {
            region_list[i].length += gap;
        }
    }
}
