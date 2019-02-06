#![allow(dead_code)]
#![allow(unused)]

use crate::configuration::ImageExtension;
use crate::configuration::ImageType;
use crate::configuration::Pile;
use crate::configuration::Distribution;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use image::DynamicImage;
use std::path::PathBuf;

mod print_info;
mod print_instructions;
mod image_printer;

pub struct ImageResources<'a> {
    tiles: &'a Vec<DynamicImage>,
    header: Option<DynamicImage>,
    footer: Option<DynamicImage>,
}

pub fn process_pile(p: &Pile, tiles: &Vec<DynamicImage>, mut path: PathBuf) -> Result<(), Box<std::error::Error>> {

    let resources = load_all_resources(&p, &tiles)?;

    let print_infos = print_info::generate(p, tiles.len());

    println!("Stage: print_instructions::generate");
    let file_info = print_instructions::generate(&print_infos, &resources);

    //println!("Pile: {}", p.name);
    //for p in file_info {
    //  println!("{:?}", p);
    //}
    //println!("");

    println!("Stage: image_printer::generate");
    let generated_images = image_printer::generate(&file_info);

    path.push(&p.name);
    fs::create_dir_all(&path)?;

    println!("Stage: save_images");
    save_images(generated_images.iter(), &path, &p.format);

    Ok(())
}

fn save_images(mut images : std::slice::Iter<DynamicImage>, path: &PathBuf, format: &ImageType) {
    for (i, image) in images.enumerate() {
        let mut path = path.clone();
        path.push(format!("file{}.{}",i, match format {
            Png => "png",
            Jpeg => "jpg",
        }));

        image.save(path).unwrap();
    }
}

fn load_all_resources<'a>(p: &Pile, tiles: &'a Vec<DynamicImage>) -> Result<ImageResources<'a>, Box<std::error::Error>> {
let mut resources = ImageResources {
        tiles: &tiles,
        header: None,
        footer: None,
    };

    if let Some(ext_info) = &p.header {
        resources.header = Some(image::open(ext_info.path.clone())?);
    }
    if let Some(ext_info) = &p.footer {
        resources.footer = Some(image::open(ext_info.path.clone())?);
    }
    Ok(resources)
}
