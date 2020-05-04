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

#[cfg(test)]
mod validation_tests {
    use super::*;

    const DIVISIBLE_DIMENSION: u32 = 10;
    const NON_DIVISIBLE_DIMENSION: u32 = 11;
    const TILE_SIZE: u32 = 2;

    #[test]
    fn perfectly_divisible_square_image_should_be_ok() {
        let image = DynamicImage::new_rgba8(DIVISIBLE_DIMENSION, DIVISIBLE_DIMENSION);
        let error_option = validate(&image, TILE_SIZE);
        assert!(error_option.is_none());
    }

    #[test]
    fn divisible_non_square_image_should_be_ok() {
        let image = DynamicImage::new_rgba8(DIVISIBLE_DIMENSION, DIVISIBLE_DIMENSION * 2);
        let error_option = validate(&image, TILE_SIZE);
        assert!(error_option.is_none());
    }

    #[test]
    fn non_divisible_by_width_image_should_return_error() {
        let image = DynamicImage::new_rgba8(NON_DIVISIBLE_DIMENSION, DIVISIBLE_DIMENSION);
        let error_option = validate(&image, TILE_SIZE);
        assert!(error_option.is_some());
    }

    #[test]
    fn non_divisible_by_height_image_should_return_error() {
        let image = DynamicImage::new_rgba8(DIVISIBLE_DIMENSION, NON_DIVISIBLE_DIMENSION);
        let error_option = validate(&image, TILE_SIZE);
        assert!(error_option.is_some());
    }

    #[test]
    fn non_divisible_by_width_and_height_image_should_return_error() {
        let image = DynamicImage::new_rgba8(NON_DIVISIBLE_DIMENSION, NON_DIVISIBLE_DIMENSION);
        let error_option = validate(&image, TILE_SIZE);
        assert!(error_option.is_some());
    }

    #[test]
    fn calling_panic_on_validation_error_should_panic() {
        let result = std::panic::catch_unwind(|| {
            ValidationError { error: String::from("error message") }.panic();
        });
        assert!(result.is_err());
    }
}
