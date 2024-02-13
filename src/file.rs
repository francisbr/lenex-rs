use std::{fs::File, io::Read, path::Path};

use zip::ZipArchive;

use crate::{
    error::{Error, FileExtensionError},
    model::Lenex,
    Result,
};

enum SupportedFileExtension {
    Lef,
    Lxf,
}

impl TryFrom<&Path> for SupportedFileExtension {
    type Error = Error;

    fn try_from(value: &Path) -> std::prelude::v1::Result<Self, Self::Error> {
        let ext = value
            .extension()
            .and_then(|e| e.to_str())
            .ok_or(FileExtensionError::UnknownExtension)?;

        Ok(match ext {
            "lef" => Ok(Self::Lef),
            "lxf" => Ok(Self::Lxf),
            e => Err(FileExtensionError::UnsupportedExtension(e.to_string())),
        }?)
    }
}

pub fn open_path(path: &Path) -> Result<Lenex> {
    let mut content = String::new();
    let mut file = File::open(path).unwrap();

    match SupportedFileExtension::try_from(path)? {
        SupportedFileExtension::Lef => {
            let _ = file.read_to_string(&mut content);
        }
        SupportedFileExtension::Lxf => {
            let mut archive = ZipArchive::new(file).unwrap();
            let mut zip_archive = archive.by_index(0).unwrap();
            let _ = zip_archive.read_to_string(&mut content);
        }
    };

    Ok(Lenex::try_from(content).unwrap())
}
