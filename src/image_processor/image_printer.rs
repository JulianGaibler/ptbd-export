use crate::image_processor::print_instructions::FileInfo;
use image::{DynamicImage, GenericImage};
use image::FilterType;

pub fn generate(print_infos: &Vec<FileInfo>) -> Vec<DynamicImage> {
    print_infos.iter().map(|print_info| {
        let mut imgbuf = image::DynamicImage::new_rgb8(print_info.width, print_info.height);
        imgbuf.invert(); // Turns image white
        for component in &print_info.components {
            let resized = component.image.resize_exact(component.width, component.height, FilterType::Gaussian);
            imgbuf.copy_from(&resized, component.pos_x, component.pos_y);
        }
        imgbuf
    }).collect::<Vec<_>>()
}