use std::fs;
use image::DynamicImage;


pub fn from_dir(directory: &String) -> Result<Vec<DynamicImage>, Box<dyn std::error::Error>> {
    let valid_extensions = vec!["png", "jpeg", "jpg"];
    let mut paths = Vec::new();

    for p in fs::read_dir(directory)? {
        let path = p?.path();
        let ext = path.extension();

        if let None = ext {
            continue;
        }
        let ext = ext.unwrap();

        let compstr = ext.to_string_lossy();
        if valid_extensions.contains(&compstr.as_ref()) {
            paths.push(path.into_os_string().into_string().unwrap());
        }
    }
    paths.sort();

    from_paths(&paths)
}

pub fn from_paths(paths: &Vec<String>) -> Result<Vec<DynamicImage>, Box<dyn std::error::Error>> {
    let mut images = Vec::new();

    for path in paths {
        images.push(image::open(&path)?);
    }

    Ok(images)
}