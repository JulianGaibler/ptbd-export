use ptbd_export::run;
use std::fs;
use std::env;
use std::process;

fn main() {
    let filename = read_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Problem loading file: {}", err);
        process::exit(1);
    });

    let config = ptbd_export::ExportConfig::new(&contents).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(config).unwrap_or_else(|err| {
        println!("Problem: {}", err);
        process::exit(1);
    });

}

pub fn read_args(mut args: std::env::Args) -> Result<String, &'static str> {
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a config-filename"),
    };

    Ok(filename)
}