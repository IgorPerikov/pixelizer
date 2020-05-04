use crate::segments::{split_image_by_segments, ImageSegmentIterator};
use crate::validation::{validate, ValidationError};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use itertools::Itertools;

mod segments;
mod validation;

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
        pixelize_segment(input, &segment);
    }
}

// TODO: can be launched in parallel
fn pixelize_segment(image: &mut DynamicImage, image_segment_iterator: &ImageSegmentIterator) {
    let segment_pixels = get_pixels(image, image_segment_iterator);
    let average_pixel = calc_average_rgba_pixel(segment_pixels);
    for point in image_segment_iterator.get_points() {
        image.put_pixel(point.x, point.y, average_pixel);
    }
}

fn get_pixels(
    image: &DynamicImage,
    image_segment_iterator: &ImageSegmentIterator,
) -> Vec<Rgba<u8>> {
    return image_segment_iterator
        .get_points()
        .iter()
        .map(|point| image.get_pixel(point.x, point.y))
        .collect_vec();
}

fn calc_average_rgba_pixel(segment_pixels: Vec<Rgba<u8>>) -> Rgba<u8> {
    let mut sum_red: u64 = 0;
    let mut sum_green: u64 = 0;
    let mut sum_blue: u64 = 0;
    let mut sum_alpha: u64 = 0;

    let total_pixels = segment_pixels.len() as u64;

    for pixel in segment_pixels {
        let rgba_content = pixel.0;
        sum_red += rgba_content[0] as u64;
        sum_green += rgba_content[1] as u64;
        sum_blue += rgba_content[2] as u64;
        sum_alpha += rgba_content[3] as u64;
    }

    Rgba([
        (sum_red / total_pixels) as u8,
        (sum_green / total_pixels) as u8,
        (sum_blue / total_pixels) as u8,
        (sum_alpha / total_pixels) as u8,
    ])
}

#[cfg(test)]
mod average_pixel_test {
    use super::*;

    #[test]
    fn average_pixel_should_consider_all_colors_and_alpha_channel() {
        let average_rgba_pixel = calc_average_rgba_pixel(vec![
            Rgba([130, 85, 60, 0]),
            Rgba([110, 80, 50, 10]),
            Rgba([90, 70, 50, 40]),
            Rgba([70, 65, 40, 50]),
        ]);
        assert_eq!(Rgba([100, 75, 50, 25]), average_rgba_pixel);
    }
}
