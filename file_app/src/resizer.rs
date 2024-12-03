use std::{fs::File, io::Cursor};

use image::{ImageReader, RgbImage};

pub fn resize(image: &[u8]) {
    let decoded_image = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .expect("Cursor io never fails")
        .decode()
        .expect("Failed to decode image");

    let rgb_img = decoded_image.to_rgb8();

    let (data, width, height) = resize_nearest_neighbor(
        &rgb_img.as_raw(),
        (&decoded_image.height(), &decoded_image.width()),
        180,
    );

    let resized_img = RgbImage::from_raw(width, height, data)
        .expect("Error while creating the redimensionned image");

    let mut output_file =
        File::create("temp/resized_image.jpg").expect("Fail to create file in /temp");
    
    let _ = resized_img.write_to(&mut output_file, image::ImageFormat::Jpeg);

    log::info!("Image successfully redimensionned")
}

pub fn resize_nearest_neighbor(
    data: &[u8],
    (width, height): (&u32, &u32),
    new_width: u32,
) -> (Vec<u8>, u32, u32) {
    // Example : 1920 * 1080; new_width = 720 => ratio = 1,5
    let ratio: f32 = (*width / new_width) as f32;
    let max_height = (*height as f32 / ratio).ceil() as usize;
    
    let mut output: Vec<u8> = vec![0; (new_width as usize * max_height * 3) as usize];
    for x in 0..new_width {
        for y in 0..max_height {
            // Get the similar pixel
            let (base_x, base_y) = ((x as f32 * ratio) as usize, (y as f32 * ratio) as usize);

            // Get the idx of the pixel
            // To get the pixel on a x y axis, in a buffer, the formula is (y * width + x) * 3
            // *3 to map the RGB channel, represented with this 3 values.
            let base_idx = (base_y * *width as usize + base_x) * 3;
            let dist_idx = (y * new_width as usize + x as usize) * 3;

            // Copy it in the output vec
            output[dist_idx..dist_idx + 3].copy_from_slice(&data[base_idx..base_idx + 3]);
        }
    }

    return (output, max_height as u32, new_width as u32);
}
