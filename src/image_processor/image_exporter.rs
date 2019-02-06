use image::{GenericImageView, DynamicImage};
use std::path::PathBuf;
use crate::configuration::ImageType;
use std::fs::File;
use std::io::{BufWriter, Write};
use image::jpeg::JPEGEncoder;

pub fn save(mut images : std::slice::Iter<DynamicImage>, path: &PathBuf, format: &ImageType) {
    for (i, image) in images.enumerate() {
        let mut path = path.clone();
        path.push(format!("file{}.{}",i, match format {
            Png => "png",
            Jpeg => "jpg",
        }));
        let (image_width, image_height) = image.dimensions();
        let file = File::create(path).expect("Unable to create file");
        let mut writer = BufWriter::new(file);
        match format {
            Png => {
                // Todo
            },
            Jpeg => {
                let mut encoder = JPEGEncoder::new_with_quality(&mut writer, 100);
                encoder.encode(image.as_rgb8().unwrap(), image_width, image_height, image.color());
            },
        }
    }
}