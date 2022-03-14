use crate::configuration::ImageExtension;
use crate::configuration::Pile;
use crate::configuration::Distribution;

#[derive(Debug)]
pub struct PrintInfo {
    pub width: u32,
    pub tile_width: u32,
    pub margin: u32,
    pub columns: u8,
    pub range: (usize, usize),
    pub cut_top_margin: bool,
    pub cut_bottom_margin: bool,
    pub use_header: bool,
    pub use_footer: bool,
}

#[derive(PartialEq)]
enum ExtState {
    NotExist,
    OncePerFile,
    OncePerPile,
}

fn check_ext_use<F>(check: F, state: &ExtState) -> bool
where
    F: Fn() -> bool,
{
    match state {
        ExtState::NotExist => false,
        ExtState::OncePerFile => true,
        ExtState::OncePerPile => check(),
    }
}

fn get_ext_state(ext: &Option<ImageExtension>) -> ExtState {
    match ext {
        Some(ext_info) => match &ext_info.once_per_file {
            true => ExtState::OncePerFile,
            false => ExtState::OncePerPile,
        },
        None => ExtState::NotExist,
    }
}

pub fn generate(p: &Pile, nr_tiles: usize) -> Vec<PrintInfo> {
    // Closure to calculate tile width when there is only one column
    let shrinking_tile_width = || p.width - (p.margin * 2);

    let header_state = get_ext_state(&p.header);
    let footer_state = get_ext_state(&p.footer);

    let base = PrintInfo {
        width: 0,
        tile_width: 0,
        margin: p.margin,
        columns: 1,
        range: (0, nr_tiles),
        cut_top_margin: false,
        cut_bottom_margin: false,
        use_header: header_state != ExtState::NotExist,
        use_footer: footer_state != ExtState::NotExist,
    };

    match p.distribution {
        Distribution::AllInOne { columns } => {
            // calculate actual amount of columns
            let actual_columns = if columns > (nr_tiles as u8) {
                nr_tiles as u8
            } else {
                columns
            };
            // If the column size should have been 1 in the first place,
            // we'll respect the width param, otherwise the image will
            // grow with the amount of columns.
            let (width, tile_width) = if columns == 1 {
                (p.width, shrinking_tile_width())
            } else {
                (
                    p.margin + ((p.width + p.margin) * (actual_columns as u32)),
                    p.width,
                )
            };

            vec![PrintInfo {
                width,
                tile_width,
                columns: actual_columns,
                ..base
            }]
        }
        Distribution::AllSeperate | Distribution::AllSeperateTreatAsOne => {
            let base = PrintInfo {
                width: p.width,
                tile_width: shrinking_tile_width(),
                columns: 1,
                ..base
            };

            (0..nr_tiles)
                .map(|i| {
                    let (cut_top_margin, cut_bottom_margin) = match p.distribution {
                        Distribution::AllSeperateTreatAsOne => (i != 0, i + 1 != nr_tiles),
                        _ => (false, false),
                    };

                    PrintInfo {
                        range: (i, i + 1),
                        use_header: check_ext_use(|| i == 0, &header_state),
                        use_footer: check_ext_use(|| i + 1 == nr_tiles, &footer_state),
                        cut_top_margin,
                        cut_bottom_margin,
                        ..base
                    }
                })
                .collect::<Vec<_>>()
        }
        Distribution::DistributeOverNr { pages } => {
            let mut files_nr = nr_tiles as u8;
            let img_per_file = (nr_tiles as f64 / pages as f64) as u8;
            let files = if img_per_file == 0 { 1 } else { pages };
            let mut images_len = img_per_file;

            let base = PrintInfo {
                width: p.width,
                tile_width: shrinking_tile_width(),
                columns: 1,
                ..base
            };

            (0..files)
                .map(|i| {
                    let start = i * img_per_file;
                    if files_nr - img_per_file > 0 && i + 1 == files {
                        images_len = files_nr
                    };
                    files_nr -= img_per_file;
                    PrintInfo {
                        range: (start as usize, (start + images_len) as usize),
                        use_header: check_ext_use(|| i == 0, &header_state),
                        use_footer: check_ext_use(|| i + 1 == nr_tiles as u8, &footer_state),
                        ..base
                    }
                })
                .collect::<Vec<_>>()
        }
    }
}
