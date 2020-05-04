use crate::segments::ImageSegmentIterator;
use crate::validation::{validate, ValidationError};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

mod segments;
mod validation;

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
            segments.push(ImageSegmentIterator::create(
                column_segment_number,
                row_segment_number,
                tile_size,
            ));
        }
    }
    segments
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
