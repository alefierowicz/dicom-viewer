use dicom_dictionary_std::tags::PIXEL_DATA;
use dicom_object::DefaultDicomObject;
use std::path::Path;

pub struct LoadedFile {
    pub name: String,
    pub obj: DefaultDicomObject,
    pub metadata: Vec<MetaRow>, // i know its a duplicate but its better than re-rendering metadata every tick
                                // todo: workaround
}

pub struct MetaRow {
    tag: String,
    name: String,
    value: String,
}

pub fn collect_metadata(file: &DefaultDicomObject) -> Vec<MetaRow> {
    let elements = file.iter();
    let mut out: Vec<MetaRow> = Vec::<MetaRow>::new();

    for element in elements {
        let header = element.header();
        if header.tag == PIXEL_DATA {
            continue;
        }
        let tag = header.tag.to_string();

        let value = match element.to_str() {
            Ok(val) => {
                if val.trim().is_empty() {
                    continue;
                };
                val.trim().to_string()
            }
            Err(_) => continue,
        };
        println!("{} | {}", &tag, &value);
        out.push(MetaRow {
            tag,
            name: String::from(""),
            value,
        });
    }
    out
}

pub fn load_file(path: &Path) -> Result<LoadedFile, String> {
    let obj = dicom_object::open_file(path).map_err(|e| e.to_string())?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let metadata = collect_metadata(&obj);
    Ok(LoadedFile {
        name,
        obj,
        metadata,
    })
}
