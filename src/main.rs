use pixelizer::pixelize;

// TODO: support cli parameters
// TODO: consider pixelizing even if it's not perfectly divisible
fn main() {
    let mut input_image = image::open("avatar.jpeg").unwrap();
    let tile_size = 20;
    match pixelize(&mut input_image, tile_size) {
        Err(e) => e.panic(),
        _ => {}
    };
    input_image.save("result.jpeg").unwrap();
}
