use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

// TODO: tests

pub fn pixelize(input: &mut DynamicImage, tile_size: u32) -> Result<(), ValidationError> {
    if (input.width() % tile_size != 0) || (input.height() % tile_size != 0) {
        return Result::Err(ValidationError);
    }
    pixelize_no_validation(input, tile_size);
    return Result::Ok(());
}

fn pixelize_no_validation(input: &mut DynamicImage, tile_size: u32) {
    for row_segment_number in 0..(input.height() / tile_size) {
        for column_segment_number in 0..(input.width() / tile_size) {
            pixelize_segment(input, row_segment_number, column_segment_number, tile_size);
        }
    }
}

fn pixelize_segment(
    input: &mut DynamicImage,
    row_segment_number: u32,
    column_segment_number: u32,
    tile_size: u32,
) {
    let start_row = row_segment_number * tile_size;
    let start_column = column_segment_number * tile_size;

    let rgba = calc_average_rgba_pixel(input, start_row, start_column, tile_size);

    for row in start_row..(start_row + tile_size) {
        for column in start_column..(start_column + tile_size) {
            input.put_pixel(row, column, rgba);
        }
    }
}

fn calc_average_rgba_pixel(
    input: &mut DynamicImage,
    start_row: u32,
    start_column: u32,
    tile_size: u32,
) -> Rgba<u8> {
    let mut sum_red: u64 = 0;
    let mut sum_green: u64 = 0;
    let mut sum_blue: u64 = 0;
    let mut sum_alpha: u64 = 0;

    for row in start_row..(start_row + tile_size) {
        for column in start_column..(start_column + tile_size) {
            let rgba_content = input.get_pixel(row, column).0;
            sum_red += rgba_content[0] as u64;
            sum_green += rgba_content[1] as u64;
            sum_blue += rgba_content[2] as u64;
            sum_alpha += rgba_content[3] as u64;
        }
    }

    let total_pixels: u64 = tile_size.pow(2).into();

    return Rgba([
        (sum_red / total_pixels) as u8,
        (sum_green / total_pixels) as u8,
        (sum_blue / total_pixels) as u8,
        (sum_alpha / total_pixels) as u8,
    ]);
}

#[derive(Debug)]
pub struct ValidationError; // TODO: populate with error details
