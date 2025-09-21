use image::{GenericImageView, DynamicImage, ColorType};
use std::path::PathBuf;
use crate::configuration::ImageType;
use std::fs::File;
use std::io::{BufWriter, Write};
use image::codecs::jpeg::JpegEncoder;

pub fn save(images: std::slice::Iter<DynamicImage>, path: &PathBuf, filename: &String, format: &ImageType) {
    for (i, image) in images.enumerate() {
        let mut path = path.clone();

        let filename = format!(
            "{}.{}",
            filename,
            match format {
                ImageType::Png => "png",
                ImageType::Jpeg => "jpg",
            }
        )
        .replace("$", &(i + 1).to_string());

        path.push(filename);

        let (image_width, image_height) = image.dimensions();
        let file = File::create(&path).expect("Unable to create file");
        let mut writer = BufWriter::new(file);
        match format {
            ImageType::Png => {
                let color_type = match image.color() {
                    ColorType::L8 => png::ColorType::Grayscale,
                    ColorType::La8 => png::ColorType::GrayscaleAlpha,
                    ColorType::Rgb8 => png::ColorType::Rgb,
                    ColorType::Rgba8 => png::ColorType::Rgba,
                    _ => png::ColorType::Rgb, // Default fallback
                };
                let bit_depth = png::BitDepth::Eight;

                let mut intermediary_buffer: Vec<u8> = Vec::new();
                {
                    let mut encoder = png::Encoder::new(&mut intermediary_buffer, image_width, image_height);
                    encoder.set_color(color_type);
                    encoder.set_depth(bit_depth);
                    encoder.set_compression(png::Compression::Best);

                    let mut png_writer = encoder.write_header().unwrap();
                    png_writer.write_image_data(&image.to_rgb8()).unwrap();
                }

                let mut options = oxipng::Options::from_preset(6);
                options.interlace = None;
                options.deflate = oxipng::Deflaters::Libdeflater { compression: 12 };

                let png_result = oxipng::optimize_from_memory(&intermediary_buffer, &options);
                match png_result {
                    Ok(result) => {
                        writer.write_all(&result).unwrap();
                    }
                    Err(e) => println!("There has been a fatal error: {}", e),
                }
            }
            ImageType::Jpeg => {
                let mut encoder = JpegEncoder::new_with_quality(&mut writer, 95);
                encoder.encode_image(image).unwrap();
            }
        }
    }
}
