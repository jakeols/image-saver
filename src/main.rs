use kamera::Camera;
use image::{ImageBuffer, DynamicImage};
use std::io;
use chrono::{Local, Datelike, Timelike};
use log::{info, warn};

fn main() -> io::Result<()> {
    let camera: Camera = Camera::new_default_device();
    camera.start();

    if let Some(frame) = camera.wait_for_frame() {
        let (w, h) = frame.size_u32();
        let binding = frame.data();
        let data: &[u32] = binding.data_u32();

        // heap-allocated buffer to for image data
        let mut img_buffer: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);

        // convert u32 buffer to an image + store it buffer
        for value in data.iter() {
            let brightness_adjustment: u8 = 10;
            let red = (*value as u8).saturating_add(brightness_adjustment);
            let green = ((value >> 8) as u8).saturating_add(brightness_adjustment);
            let blue = ((value >> 16) as u8).saturating_add(brightness_adjustment);
            img_buffer.extend_from_slice(&[red, green, blue, 255]);
        }

        // create a `DynamicImage` from the buffer
        let img = DynamicImage::ImageRgba8(ImageBuffer::from_raw(w, h, img_buffer).unwrap());
        let file_path: String = get_timestamp();
        // clone file_path before moving it into the closure
        let file_path_clone: String = file_path.clone();

        if let Err(err) = img.save_with_format(&file_path, image::ImageFormat::Png) {
            // Convert ImageError into io::Error
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Image error: {}", err),
            ));
        }

        info!("Image saved to '{}'", file_path_clone);
    } else {
        warn!("No frame received.");
    }

    camera.stop();
    Ok(())
}

fn get_timestamp() -> String {
    let local_time = Local::now();
    let year = local_time.year();
    let month = local_time.month();
    let day = local_time.day();
    let hour = local_time.hour();
    let minute = local_time.minute();
    let second = local_time.second();
    let formatted_timestamp = format!("{:02}-{:02}-{}_{:02}:{:02}:{:02}_brightened.png", month, day, year, hour, minute, second);
    formatted_timestamp
}
