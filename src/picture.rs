use anyhow::{Error, Result};
use chrono::Local;
use image::codecs::jpeg::JpegEncoder;
use image::ExtendedColorType;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

pub fn take_picture() -> Result<String> {
    let timestamp = Local::now().format("%Y-%m-%d_%H:%M");
    let device = env::var("DEVICE_NUMBER")?.parse()?;
    let data_dir = env::var("DATA_DIR")?;
    let mut dev = Device::new(device)?;
    let mut fmt = dev.format()?;
    fmt.width = 1920;
    fmt.height = 1080;
    fmt.fourcc = FourCC::new(b"YUYV");
    let fmt = dev.set_format(&fmt)?;

    let mut stream = Stream::with_buffers(&mut dev, Type::VideoCapture, 4)?;

    let (buf, _) = match stream.next() {
        Ok((buf, meta)) => (buf, meta),
        Err(e) => return Err(Error::new(e)),
    };

    let image = convert_yuyv_to_rgb(&buf, fmt.width as u32, fmt.height as u32);
    let filename = format!("{}/{}.jpg", data_dir, timestamp);
    let file = File::create(filename)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = JpegEncoder::new_with_quality(w, 100);
    encoder.encode(&image, fmt.width, fmt.height, ExtendedColorType::Rgb8)?;

    Ok(format!(
        "{}/{}.jpg saved. Resolution: {}x{}",
        data_dir, timestamp, fmt.width, fmt.height
    ))
}

/// Convert YUYV encoded buffer to an RGB image buffer.
fn convert_yuyv_to_rgb(buffer: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut img = Vec::with_capacity((width * height * 3) as usize);
    let chunks = buffer.chunks_exact(4);
    for chunk in chunks {
        let y0 = chunk[0] as i32;
        let u = chunk[1] as i32;
        let y1 = chunk[2] as i32;
        let v = chunk[3] as i32;

        let c = y0 - 16;
        let d = u - 128;
        let e = v - 128;

        let r = (298 * c + 409 * e + 128) >> 8;
        let g = (298 * c - 100 * d - 208 * e + 128) >> 8;
        let b = (298 * c + 516 * d + 128) >> 8;

        img.extend_from_slice(&[
            clamp(r, 0, 255) as u8,
            clamp(g, 0, 255) as u8,
            clamp(b, 0, 255) as u8,
        ]);

        let c = y1 - 16;
        let r = (298 * c + 409 * e + 128) >> 8;
        let g = (298 * c - 100 * d - 208 * e + 128) >> 8;
        let b = (298 * c + 516 * d + 128) >> 8;

        img.extend_from_slice(&[
            clamp(r, 0, 255) as u8,
            clamp(g, 0, 255) as u8,
            clamp(b, 0, 255) as u8,
        ]);
    }
    img
}

/// Helper function to clamp values.
fn clamp(value: i32, min: i32, max: i32) -> i32 {
    max.min(value).max(min)
}
