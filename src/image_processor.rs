use crate::configuration::Pile;
use std::fs;
use image::DynamicImage;
use std::path::PathBuf;
use colored::*;

mod print_info;
mod print_instructions;
mod image_printer;
mod image_exporter;

pub struct ImageResources<'a> {
    tiles: &'a Vec<DynamicImage>,
    header: Option<DynamicImage>,
    footer: Option<DynamicImage>,
}

pub fn process_pile(p: &Pile, tiles: &Vec<DynamicImage>, mut path: PathBuf) -> Result<(), Box<std::error::Error>> {

    let resources = load_all_resources(&p, &tiles)?;

    let print_infos = print_info::generate(p, tiles.len());

    println!("{}", p.name.yellow());

    println!("Stage: print_instructions::generate");
    let file_info = print_instructions::generate(&print_infos, &resources);

    println!("Stage: image_printer::generate");
    let generated_images = image_printer::generate(&file_info);

    path.push(&p.name);
    fs::create_dir_all(&path)?;

    println!("Stage: save_images");
    image_exporter::save(generated_images.iter(), &path, &p.filename, &p.format);

    Ok(())
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
