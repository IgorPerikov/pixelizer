use image::{DynamicImage, GenericImageView};

pub fn validate(image: &DynamicImage, tile_size: u32) -> Option<ValidationError> {
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

pub struct ValidationError {
    error: String,
}

impl ValidationError {
    pub fn panic(&self) {
        panic!("Validation failed: {}", self.error);
    }

    fn image_cant_be_segmented(width: u32, height: u32, tile_size: u32) -> ValidationError {
        ValidationError {
            error: format!(
                "{}x{} image cannot be segmented into tiles of size {}",
                width, height, tile_size
            ),
        }
    }
}
