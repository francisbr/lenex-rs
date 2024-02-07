use std::{fs::File, io::Read, path::Path};

use crate::model::Lenex;
use zip::ZipArchive;

pub fn open_path(path: &Path) -> Result<Lenex, ()> {
    let ext = match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(ext) => ext,
            None => return Err(()),
        },
        None => return Err(()),
    };

    let mut content = String::new();
    let mut file = File::open(path).unwrap();

    match ext {
        "lef" => {
            let _ = file.read_to_string(&mut content);
        }
        "lxf" => {
            let mut archive = ZipArchive::new(file).unwrap();
            if archive.len() != 1 {
                return Err(());
            }

            let mut zip_archive = archive.by_index(0).unwrap();
            let _ = zip_archive.read_to_string(&mut content);
        }
        _ => return Err(()),
    };

    Ok(Lenex::try_from(content).unwrap())
}
