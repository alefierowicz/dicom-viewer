use dicom_object::DefaultDicomObject;
use std::path::Path;

pub struct LoadedFile {
    pub name: String,
    pub obj: DefaultDicomObject,
}

pub fn load_file(path: &Path) -> Result<LoadedFile, String> {
    let obj = dicom_object::open_file(path).map_err(|e| e.to_string())?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    Ok(LoadedFile { name, obj })
}
