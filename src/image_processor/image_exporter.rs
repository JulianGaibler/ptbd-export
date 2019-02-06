use image::{GenericImageView, DynamicImage};
use std::path::PathBuf;
use crate::configuration::ImageType;
use std::fs::File;
use std::io::BufWriter;
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
                let mut encoder = png::Encoder::new(&mut writer, image_width, image_height);
                encoder.set(ct).set(bits).set(png::Compression::Best);
                let mut writer = encoder.write_header().unwrap();
                writer.write_image_data(image.as_rgb8().unwrap()).unwrap();
            },
            ImageType::Jpeg => {
                let mut encoder = JPEGEncoder::new_with_quality(&mut writer, 95);
                encoder.encode(image.as_rgb8().unwrap(), image_width, image_height, image.color()).unwrap();
            },
        }
    }
}