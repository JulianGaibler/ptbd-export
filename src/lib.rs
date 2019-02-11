mod configuration;
mod image_loader;
mod image_processor;
use std::path::PathBuf;
use std::error::Error;
use std::fs;
pub use configuration::ExportConfig;

pub fn run(config: ExportConfig) -> Result<(), Box<dyn Error>> {
    println!("deserialized = {:?}", config.import_path);

    let tiles = image_loader::from_dir(&config.import_path)?;

    let path = PathBuf::from(&config.export_path);
    fs::create_dir_all(&path)?;

    for p in config.piles {
        image_processor::process_pile(&p, &tiles, path.clone())?;
    }

    return Ok(());
}