use image::{GenericImageView, DynamicImage};
use std::path::PathBuf;
use crate::configuration::ImageType;
use std::fs::File;
use std::io::{BufWriter, Write};
use image::jpeg::JPEGEncoder;
use png::HasParameters;

pub fn save(images : std::slice::Iter<DynamicImage>, path: &PathBuf, format: &ImageType) {
    for (i, image) in images.enumerate() {
        let mut path = path.clone();
        path.push(format!("file{}.{}",i+1, match format {
            ImageType::Png => "png",
            ImageType::Jpeg => "jpg",
        }));
        let (image_width, image_height) = image.dimensions();
        let file = File::create(&path).expect("Unable to create file");
        let mut writer = BufWriter::new(file);
        match format {
            ImageType::Png => {
                let (ct, bits) = image.color().into();
                let mut intermediary_buffer: Vec<u8> = Vec::new();
                {
                    let mut encoder = png::Encoder::new(&mut intermediary_buffer, image_width, image_height);
                    encoder.set(ct).set(bits);
                    let mut png_writer = encoder.write_header().unwrap();
                    png_writer.write_image_data(image.as_rgb8().unwrap()).unwrap();
                }

                //println!("VEC: {:?}", intermediary_buffer);

                let mut options = oxipng::Options::from_preset(6);
                options.interlace = Some(1);
                //println!("{:?}", options);

                let png_result = oxipng::optimize_from_memory(&intermediary_buffer, &options);
                match png_result {
                    Ok(result) => {
                        writer.write(&result).unwrap();
                    },
                    Err(e) => println!("{}", e),
                }

            },
            ImageType::Jpeg => {
                let mut encoder = JPEGEncoder::new_with_quality(&mut writer, 95);
                encoder.encode(image.as_rgb8().unwrap(), image_width, image_height, image.color()).unwrap();
            },
        }
    }
}