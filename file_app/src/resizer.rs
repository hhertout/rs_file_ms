use std::{fs::File, io::Cursor};

use image::{ImageReader, RgbImage};
use uuid::Uuid;

pub enum ResizeMode {
    Nearest,
    Interpolated,
}

impl ToString for ResizeMode {
    fn to_string(&self) -> String {
        match &self {
            ResizeMode::Interpolated => return String::from("interpoled"),
            ResizeMode::Nearest => return String::from("near"),
        }
    }
}

pub fn resize(mode: ResizeMode, image: &[u8]) {
    let decoded_image = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .expect("Cursor io never fails")
        .decode()
        .expect("Failed to decode image");

    let __resize_ratio = 2;

    let rgb_img = decoded_image.to_rgb8();

    log::debug!(
        "Original image dimensions: {}x{}",
        decoded_image.width(),
        decoded_image.height()
    );

    let (data, new_width, new_height) = match mode {
        ResizeMode::Interpolated => resize_interpolated_pixels(
            &rgb_img.as_raw(),
            (decoded_image.width(), decoded_image.height()),
            decoded_image.width() / __resize_ratio,
            decoded_image.height() / __resize_ratio,
        ),
        ResizeMode::Nearest => resize_nearest_neighbor(
            &rgb_img.as_raw(),
            (decoded_image.width(), decoded_image.height()),
            decoded_image.width() / __resize_ratio,
            decoded_image.height() / __resize_ratio,
        ),
    };

    log::debug!("New image dimensions: {}x{}", new_width, new_height);

    let resized_img = RgbImage::from_raw(new_width, new_height, data)
        .expect("Error while creating the resized image");

    let file_name = format!(
        "temp/{}_{}.jpg",
        mode.to_string(),
        Uuid::new_v4().to_string()
    );

    std::fs::create_dir_all("temp").expect("Failed to create /temp directory");

    let mut output_file = File::create(file_name).expect("Failed to create file in /temp");

    image::DynamicImage::ImageRgb8(resized_img)
        .write_to(&mut output_file, image::ImageFormat::Jpeg)
        .expect("Failed to write image to file");

    log::info!("Image successfully resized");
}

fn resize_nearest_neighbor(
    data: &[u8],
    (width, height): (u32, u32),
    new_width: u32,
    new_height: u32,
) -> (Vec<u8>, u32, u32) {
    log::info!("Resizing the image with nearest neighbor algorithm");
    log::debug!("Resizing to dimensions: {}x{}", new_width, new_height);

    let x_ratio = width as f32 / new_width as f32;
    let y_ratio = height as f32 / new_height as f32;

    let mut output: Vec<u8> = vec![0; new_width as usize * new_height as usize * 3];

    for y in 0..new_height {
        for x in 0..new_width {
            let base_x = (x as f32 * x_ratio) as usize;
            let base_y = (y as f32 * y_ratio) as usize;

            let base_idx = (base_y * width as usize + base_x) * 3;
            let dist_idx = (y as usize * new_width as usize + x as usize) * 3;

            output[dist_idx..dist_idx + 3].copy_from_slice(&data[base_idx..base_idx + 3]);
        }
    }

    (output, new_width, new_height)
}

fn resize_interpolated_pixels(
    data: &[u8],
    (width, height): (u32, u32),
    new_width: u32,
    new_height: u32,
) -> (Vec<u8>, u32, u32) {
    log::info!("Resizing the image with interpolated algorythm");

    let x_ratio = width as f32 / new_width as f32;
    let y_ratio = height as f32 / new_height as f32;

    let mut output: Vec<u8> = vec![0; new_width as usize * new_height as usize * 3];

    for y in 0..new_height {
        for x in 0..new_width {
            // Base image coord
            let x_coord = x as f32 * x_ratio;
            let y_coord = y as f32 * y_ratio;

            let rounded_x = x_coord.floor() as usize;
            let rounded_y = y_coord.floor() as usize;

            // Horizontal ratio
            let dx = x_coord - rounded_x as f32;
            // Vertical ratio
            let dy = y_coord - rounded_y as f32;

            let idx_tl = (rounded_y * width as usize + rounded_x) * 3;
            let idx_tr =
                (rounded_y * width as usize + (rounded_x + 1).clamp(0, width as usize - 1)) * 3;
            let idx_bl =
                ((rounded_y + 1).clamp(0, height as usize - 1) * width as usize + rounded_x) * 3;
            let idx_br = ((rounded_y + 1).clamp(0, height as usize - 1) * width as usize
                + (rounded_x + 1).clamp(0, width as usize - 1))
                * 3;

            // Extract RGB values
            let q11 = [data[idx_tl], data[idx_tl + 1], data[idx_tl + 2]];
            let q21 = [data[idx_tr], data[idx_tr + 1], data[idx_tr + 2]];
            let q12 = [data[idx_bl], data[idx_bl + 1], data[idx_bl + 2]];
            let q22 = [data[idx_br], data[idx_br + 1], data[idx_br + 2]];

            // Bilinear interpolation calculation
            let interpolated_pixel = bilinear_interpolation(q11, q12, q21, q22, dx, dy);

            let dist_idx = ((y * new_width + x) * 3) as usize;
            output[dist_idx..dist_idx + 3].copy_from_slice(&interpolated_pixel);
        }
    }

    (output, new_width as u32, new_height)
}

fn bilinear_interpolation(
    q11: [u8; 3], // Top Left Pixel (R, G, B)
    q12: [u8; 3], // Bottom Left Pixel (R, G, B)
    q21: [u8; 3], // Top Right Pixel (R, G, B)
    q22: [u8; 3], // Bottom Right (R, G, B)
    tx: f32,      // Horizontal ratio Ratio between 0 & 1
    ty: f32,      // Vertical Ratio vertical between 0 & 1
) -> [u8; 3] {
    let mut result = [0u8; 3];
    for i in 0..3 {
        let i1 = (1.0 - tx) * q11[i] as f32 + tx * q21[i] as f32;
        let i2 = (1.0 - tx) * q12[i] as f32 + tx * q22[i] as f32;
        result[i] = ((1.0 - ty) * i1 + ty * i2) as u8;
    }
    result
}
