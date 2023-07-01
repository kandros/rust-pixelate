use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn resize(img: &Image, new_dimensions: (u32, u32)) -> Image {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = new_dimensions;

    let mut resized = ImageBuffer::new(new_width, new_height);

    for (new_x, new_y, pixel) in resized.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width as f32 / new_width as f32)) as u32;
        let old_y = (new_y as f32 * (old_height as f32 / new_height as f32)) as u32;

        if let Some(old_pixel) = img.get_pixel_checked(old_x, old_y) {
            *pixel = *old_pixel;
        } else {
            println!("{old_x} -> {new_x}, {old_y} -> {new_y}")
        }
    }

    return resized;
}

fn pixelate(img: &DynamicImage, new_dimensions: (u32, u32)) -> Image {
    let old_dimensions = img.dimensions();

    let img = img.to_rgba8();

    let small = resize(
        &img,
        (
            old_dimensions.0 / new_dimensions.0,
            old_dimensions.1 / new_dimensions.1,
        ),
    );

    let pixelated = resize(&small, old_dimensions);

    return pixelated;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("./image.jpeg")?.decode()?;

    let new_pixel_size = (10, 10);
    let img_ = pixelate(&img, new_pixel_size);
    img_.save("pixelated.png")?;

    return Ok(());
}
