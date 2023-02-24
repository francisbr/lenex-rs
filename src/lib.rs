use std::{fs::File, io::Read, path::Path};

use model::lenex::Lenex;
use zip::ZipArchive;

pub mod model;
mod util;

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

    return Ok(Lenex::try_from(content).unwrap());
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_open_file() {
        let path = Path::new("examples/inscription.lef");
        let _result = open_path(path);
    }

    #[test]
    #[ignore]
    fn lenex_to_xml() {
        let lenex = Lenex::new();
        println!("{:#?}", lenex);

        let content = lenex.xml().unwrap();

        println!("{content}");
    }
}
