use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::ops::Range;

// TODO: tests
// TODO: consider functional approaches where possible
// TODO: come to a single naming policy - tile or segment, not both

pub fn pixelize(image: &mut DynamicImage, tile_size: u32) -> Result<(), ValidationError> {
    return match validate(image, tile_size) {
        Some(e) => Result::Err(e),
        None => {
            pixelize_no_validation(image, tile_size);
            Result::Ok(())
        }
    };
}

fn pixelize_no_validation(input: &mut DynamicImage, tile_size: u32) {
    for segment in split_image_by_segments(input, tile_size) {
        pixelize_segment(input, segment);
    }
}

fn split_image_by_segments(input: &DynamicImage, tile_size: u32) -> Vec<ImageSegmentIterator> {
    let mut segments = Vec::new();
    for column_segment_number in 0..(input.width() / tile_size) {
        for row_segment_number in 0..(input.height() / tile_size) {
            segments.push(ImageSegmentIterator {
                column_segment_number,
                row_segment_number,
                segment_size: tile_size,
            });
        }
    }
    segments
}

fn validate(image: &DynamicImage, tile_size: u32) -> Option<ValidationError> {
    if (image.width() % tile_size != 0) || (image.height() % tile_size != 0) {
        Option::Some(ValidationError::image_cant_be_segmented(
            image.width(),
            image.height(),
            tile_size,
        ))
    } else {
        Option::None
    }
}

// TODO: can be launched in parallel
fn pixelize_segment(image: &mut DynamicImage, image_segment_iterator: ImageSegmentIterator) {
    let rgba = calc_average_rgba_pixel(image, &image_segment_iterator);
    for point in image_segment_iterator.get_points() {
        image.put_pixel(point.x, point.y, rgba);
    }
}

fn calc_average_rgba_pixel(
    image: &mut DynamicImage,
    image_segment_iterator: &ImageSegmentIterator,
) -> Rgba<u8> {
    let mut sum_red: u64 = 0;
    let mut sum_green: u64 = 0;
    let mut sum_blue: u64 = 0;
    let mut sum_alpha: u64 = 0;

    let points = image_segment_iterator.get_points();

    for point in &points {
        let rgba_content = image.get_pixel(point.x, point.y).0;
        sum_red += rgba_content[0] as u64;
        sum_green += rgba_content[1] as u64;
        sum_blue += rgba_content[2] as u64;
        sum_alpha += rgba_content[3] as u64;
    }

    let total_pixels = points.len() as u64;

    Rgba([
        (sum_red / total_pixels) as u8,
        (sum_green / total_pixels) as u8,
        (sum_blue / total_pixels) as u8,
        (sum_alpha / total_pixels) as u8,
    ])
}

// TODO: encapsulate get_columns/get_rows/struct fields by moving out of lib.rs
struct ImageSegmentIterator {
    column_segment_number: u32,
    row_segment_number: u32,
    segment_size: u32,
}

impl ImageSegmentIterator {
    fn get_points(&self) -> Vec<Point> {
        let mut elements = Vec::new();
        for column in self.get_columns() {
            for row in self.get_rows() {
                elements.push(Point { x: column, y: row });
            }
        }
        elements
    }

    fn get_columns(&self) -> Range<u32> {
        let start_column = self.column_segment_number * self.segment_size;
        start_column..(start_column + self.segment_size)
    }

    fn get_rows(&self) -> Range<u32> {
        let start_row = self.row_segment_number * self.segment_size;
        start_row..(start_row + self.segment_size)
    }
}

struct Point {
    x: u32,
    y: u32,
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
