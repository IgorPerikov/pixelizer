use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::ops::Range;

// TODO: tests

pub fn pixelize(input: &mut DynamicImage, tile_size: u32) -> Result<(), ValidationError> {
    if (input.width() % tile_size != 0) || (input.height() % tile_size != 0) {
        return Result::Err(ValidationError::image_cant_be_segmented(
            input.width(),
            input.height(),
            tile_size,
        ));
    }
    pixelize_no_validation(input, tile_size);
    return Result::Ok(());
}

fn pixelize_no_validation(input: &mut DynamicImage, tile_size: u32) {
    for row_segment_number in 0..(input.height() / tile_size) {
        for column_segment_number in 0..(input.width() / tile_size) {
            let image_segment_iterator = ImageSegmentIterator {
                row_segment_number,
                column_segment_number,
                segment_size: tile_size,
            };
            pixelize_segment(input, &image_segment_iterator);
        }
    }
}

// TODO: can be launched in parallel
fn pixelize_segment(input: &mut DynamicImage, image_segment_iterator: &ImageSegmentIterator) {
    let rgba = calc_average_rgba_pixel(input, image_segment_iterator);
    for (row, column) in image_segment_iterator.get_elements() {
        input.put_pixel(row, column, rgba);
    }
}

fn calc_average_rgba_pixel(
    input: &mut DynamicImage,
    image_segment_iterator: &ImageSegmentIterator,
) -> Rgba<u8> {
    let mut sum_red: u64 = 0;
    let mut sum_green: u64 = 0;
    let mut sum_blue: u64 = 0;
    let mut sum_alpha: u64 = 0;

    let elements = image_segment_iterator.get_elements();

    for (row, column) in &elements {
        let rgba_content = input.get_pixel(*row, *column).0;
        sum_red += rgba_content[0] as u64;
        sum_green += rgba_content[1] as u64;
        sum_blue += rgba_content[2] as u64;
        sum_alpha += rgba_content[3] as u64;
    }

    let total_pixels = elements.len() as u64;

    return Rgba([
        (sum_red / total_pixels) as u8,
        (sum_green / total_pixels) as u8,
        (sum_blue / total_pixels) as u8,
        (sum_alpha / total_pixels) as u8,
    ]);
}

// TODO: encapsulate get_columns/get_rows/struct fields by moving out of lib.rs
struct ImageSegmentIterator {
    row_segment_number: u32,
    column_segment_number: u32,
    segment_size: u32,
}

impl ImageSegmentIterator {
    fn get_elements(&self) -> Vec<(u32, u32)> {
        let mut elements = Vec::new();
        for row in self.get_rows() {
            for column in self.get_columns() {
                elements.push((row, column));
            }
        }
        return elements;
    }

    fn get_columns(&self) -> Range<u32> {
        let start_column = self.column_segment_number * self.segment_size;
        return start_column..(start_column + self.segment_size);
    }

    fn get_rows(&self) -> Range<u32> {
        let start_row = self.row_segment_number * self.segment_size;
        return start_row..(start_row + self.segment_size);
    }
}

#[derive(Debug)]
pub struct ValidationError {
    error: String,
}

impl ValidationError {
    pub fn image_cant_be_segmented(width: u32, height: u32, tile_size: u32) -> ValidationError {
        ValidationError {
            error: format!(
                "{}x{} image cannot be segmented into tiles of size {}",
                width, height, tile_size
            ),
        }
    }

    pub fn panic(&self) {
        panic!("Validation failed: {}", self.error);
    }
}
