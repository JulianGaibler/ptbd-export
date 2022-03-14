use crate::image_processor::ImageResources;
use image::{GenericImageView, DynamicImage};
use crate::image_processor::print_info::PrintInfo;
use colored::*;

#[derive(Debug)]
pub struct FileInfo<'a> {
    pub width: u32,
    pub height: u32,
    pub components: Vec<TransformInfo<'a>>,
}

pub struct TransformInfo<'a> {
    pub pos_y: u32,
    pub pos_x: u32,
    pub width: u32,
    pub height: u32,
    pub image: &'a DynamicImage,
}

impl<'a> std::fmt::Debug for TransformInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "\n(pos_y: {}, pos_x: {}, width: {}, height: {})",
                self.pos_y, self.pos_x, self.width, self.height
            )
            .yellow()
        )
    }
}

pub fn generate<'a>(print_infos: &Vec<PrintInfo>, resources: &'a ImageResources) -> Vec<FileInfo<'a>> {
    print_infos
        .iter()
        .map(|info| {
            let mut height = if info.cut_top_margin {
                info.margin / 2
            } else {
                info.margin
            };
            let mut components: Vec<TransformInfo> = Vec::new();

            if info.use_header {
                components.push(add_extension(&resources.header, &info, &mut height));
            }

            let mut largest = 0;
            let nr_tiles = info.range.1 - info.range.0;
            for i in 0..nr_tiles {
                let image_ref = &resources.tiles[(info.range.0 + i)];
                let (img_width, img_height) =
                    resize_dimensions(image_ref.dimensions(), info.tile_width, 500, true);

                let curr_col = i as u8 % info.columns as u8;
                let pos_x = info.margin * (curr_col as u32 + 1) + info.tile_width * curr_col as u32;
                components.push(TransformInfo {
                    pos_y: height,
                    pos_x,
                    width: img_width,
                    height: img_height,
                    image: &image_ref,
                });

                if img_height > largest {
                    largest = img_height
                };
                if curr_col == info.columns - 1 || i == nr_tiles - 1 {
                    height += largest + info.margin;
                    largest = 0;
                }
            }

            if info.use_footer {
                components.push(add_extension(&resources.footer, &info, &mut height));
            }

            if info.cut_bottom_margin {
                height -= info.margin / 2;
            }

            FileInfo {
                width: info.width,
                height,
                components,
            }
        })
        .collect::<Vec<_>>()
}

fn add_extension<'a>(ext: &'a Option<DynamicImage>, info: &PrintInfo, height: &mut u32) -> TransformInfo<'a> {
    let ext_ref = match ext {
        Some(ext_ref) => ext_ref,
        None => panic!("Empty header reference!"),
    };
    let (img_width, img_height) = resize_dimensions(ext_ref.dimensions(), info.tile_width, 0, true);

    // Center horizontally
    let pos_x = ((info.margin as f64 * 0.5 + (info.width - info.margin) as f64 * 0.5)
        - (img_width as f64 / 2.0)) as u32;
    let instr = TransformInfo {
        pos_y: *height,
        pos_x,
        width: img_width,
        height: img_height,
        image: &ext_ref,
    };
    *height += img_height + info.margin;
    instr
}

// Forked from PistonDevelopers/image/src/dynimage.rs
fn resize_dimensions((width, height): (u32, u32), nwidth: u32, nheight: u32, fill: bool) -> (u32, u32) {
    let ratio = u64::from(width) * u64::from(nheight);
    let nratio = u64::from(nwidth) * u64::from(height);

    let use_width = if fill { nratio > ratio } else { nratio <= ratio };
    let intermediate = if use_width {
        u64::from(height) * u64::from(nwidth) / u64::from(width)
    } else {
        u64::from(width) * u64::from(nheight) / u64::from(height)
    };
    if use_width {
        if intermediate <= u64::from(::std::u32::MAX) {
            (nwidth, intermediate as u32)
        } else {
            (
                (u64::from(nwidth) * u64::from(::std::u32::MAX) / intermediate) as u32,
                ::std::u32::MAX,
            )
        }
    } else if intermediate <= u64::from(::std::u32::MAX) {
        (intermediate as u32, nheight)
    } else {
        (
            ::std::u32::MAX,
            (u64::from(nheight) * u64::from(::std::u32::MAX) / intermediate) as u32,
        )
    }
}
