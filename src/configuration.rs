use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageType {
    Png,
    Jpeg,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Distribution {
    AllInOne {
        columns: u8,
    },
    AllSeperate,
    AllSeperateTreatAsOne,
    DistributeOverNr {
        pages: u8,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportConfig {
    pub import_path: String,
    pub export_path: String,
    pub piles: Vec<Pile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pile {
    pub name: String,
    #[serde(default = "default_filename")]
    pub filename: String,
    pub width: u32,
    pub format: ImageType,
    pub margin: u32,
    pub distribution: Distribution,
    pub header: Option<ImageExtension>,
    pub footer: Option<ImageExtension>,
}

fn default_filename() -> String {
    "file$".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageExtension {
    pub path: String,
    pub once_per_file: bool,
}

impl ExportConfig {
    pub fn new(json_str: &str) -> Result<ExportConfig, impl std::error::Error> {
        serde_json::from_str(json_str)
    }
}
