use image::ImageResult;
use libc::*;
use std::slice;
use image::{DynamicImage, GenericImageView, ImageBuffer};

#[link(name = "opencv_core")]
#[link(name = "opencv_imgproc")]
extern {
    static allocated_size: libc::size_t;
    fn resize_image(width: uint32_t, height: uint32_t, nwidth: uint32_t, nheight: uint32_t, data: *mut uint8_t) -> *const uint8_t;
    fn free_image(input: *const uint8_t);
}


pub fn resize(old_image: &DynamicImage, nwidth: u32, nheight: u32) -> ImageResult<DynamicImage> {
    let (width, height) = old_image.dimensions();
    let arr = old_image.as_rgb8().expect("Input images were in rgb8!");
    let mut raw = arr.clone().into_raw();
    let vec;
    unsafe {
        let result = resize_image(width, height, nwidth, nheight, raw.as_mut_ptr());
        vec = slice::from_raw_parts(result, allocated_size as usize).to_vec();
        free_image(result);
    }

    let image = ImageBuffer::from_raw(nwidth, nheight, vec).map(DynamicImage::ImageRgb8);
    match image {
        Some(image) => Ok(image),
        None => Err(image::ImageError::DimensionError),
    }
}