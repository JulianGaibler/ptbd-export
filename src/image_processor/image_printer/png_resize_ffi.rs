use fast_image_resize::{images::Image, PixelType, Resizer, ResizeOptions};
use image::{DynamicImage, ImageBuffer, Rgb, ImageError, ImageResult};

pub fn resize(old_image: &DynamicImage, nwidth: u32, nheight: u32) -> ImageResult<DynamicImage> {
    let rgb_image = old_image.to_rgb8();
    let (width, height) = rgb_image.dimensions();

    // Create source image
    let src_image = Image::from_vec_u8(
        width.try_into().map_err(|_| {
            ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            ))
        })?,
        height.try_into().map_err(|_| {
            ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            ))
        })?,
        rgb_image.into_raw(),
        PixelType::U8x3,
    )
    .map_err(|_| {
        ImageError::Limits(image::error::LimitError::from_kind(
            image::error::LimitErrorKind::DimensionError,
        ))
    })?;

    // Create destination image
    let mut dst_image = Image::new(
        nwidth.try_into().map_err(|_| {
            ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            ))
        })?,
        nheight.try_into().map_err(|_| {
            ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            ))
        })?,
        PixelType::U8x3,
    );

    // Resize with high-quality algorithm
    let mut resizer = Resizer::new();
    let resize_options = ResizeOptions::new().resize_alg(fast_image_resize::ResizeAlg::Convolution(
        fast_image_resize::FilterType::Lanczos3,
    ));
    resizer
        .resize(&src_image, &mut dst_image, Some(&resize_options))
        .map_err(|_| {
            ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            ))
        })?;

    // Convert back to DynamicImage
    let buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(nwidth, nheight, dst_image.into_vec())
        .ok_or_else(|| {
            ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            ))
        })?;

    Ok(DynamicImage::ImageRgb8(buffer))
}
