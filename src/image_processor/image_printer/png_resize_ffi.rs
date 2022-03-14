use image::ImageResult;

use std::slice;
use image::{DynamicImage, GenericImageView, ImageBuffer};

#[link(name = "opencv_core")]
#[link(name = "opencv_imgproc")]
extern "C" {
    static allocated_size: libc::size_t;
    fn resize_image(width: u32, height: u32, nwidth: u32, nheight: u32, data: *mut u8) -> *const u8;
    fn free_image(input: *const u8);
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
