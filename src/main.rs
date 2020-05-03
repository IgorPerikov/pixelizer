use image::GenericImageView;
use pixelizer::pixelize;

// TODO: support cli parameters
// TODO: consider pixelizing even if it's not perfectly divisible
fn main() {
    let mut input_image = image::open("avatar.jpeg").unwrap();
    let tile_size = 5;
    match pixelize(&mut input_image, tile_size) {
        Err(_) => panic!(
            "Validation error: {}x{} image cannot be pixelized into tiles of size {}",
            input_image.height(),
            input_image.width(),
            tile_size
        ),
        _ => {}
    };
    input_image.save("result.jpeg").unwrap();
}
