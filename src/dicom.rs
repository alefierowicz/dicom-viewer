use crate::volume::Volume;
use dicom_core::{DataDictionary, VR};
use dicom_dictionary_std::{StandardDataDictionary, tags::PIXEL_DATA};
use dicom_object::{DefaultDicomObject, mem::InMemElement};
use std::path::Path;
pub struct LoadedFile {
    pub name: String,
    pub obj: DefaultDicomObject,
    pub metadata: Vec<MetaRow>, // i know its a duplicate but its better than re-rendering metadata every tick
    // todo: workaround
    pub volume: Volume,
    pub min_max: (f64, f64),
}

pub struct MetaRow {
    pub tag: String,
    pub name: String,
    pub value: String,
}

fn is_binary(vr: VR) -> bool {
    matches!(
        vr,
        VR::OB | VR::OW | VR::OD | VR::OF | VR::OL | VR::OV | VR::UN
    )
}

pub fn display_value(element: &InMemElement) -> Option<String> {
    if let Some(items) = element.items() {
        return Some(format!("[{} items]", items.len()));
    } else if is_binary(element.vr()) {
        return Some(match element.header().len.get() {
            // description of the binary since we cant really display it
            Some(n) => format!("[binary, {} bytes]", n),
            None => "[binary]".to_string(),
        });
    } else {
        match element.to_str() {
            Ok(val) => {
                if val.trim().is_empty() {
                    return None; // none if empty
                }
                Some(val.trim().to_string())
            }
            Err(_) => None,
        }
    }
}

pub fn collect_metadata(file: &DefaultDicomObject) -> Vec<MetaRow> {
    let elements = file.iter();
    let mut out: Vec<MetaRow> = Vec::<MetaRow>::new();

    for element in elements {
        let tag = element.header().tag;
        if tag == PIXEL_DATA {
            continue;
        }

        let value = match display_value(element) {
            Some(val) => val,
            None => continue,
        };

        let name = match StandardDataDictionary.by_tag(tag) {
            Some(entry) => entry.alias.to_string(),
            None => String::from("Unknown"),
        };

        // println!("{} | {} | {}", tag, name, value);
        out.push(MetaRow {
            tag: tag.to_string(),
            name,
            value,
        });
    }
    out
}

pub fn load_file(path: &Path) -> Result<LoadedFile, String> {
    let obj: dicom_object::FileDicomObject<dicom_object::InMemDicomObject> =
        dicom_object::open_file(path).map_err(|e| e.to_string())?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let metadata = collect_metadata(&obj);

    let volume =
        Volume::build(&obj).map_err(|e| -> String { format!("failed to build volume: {e}") })?;
    let val_range = volume.value_range();

    if let Some(pixels) = volume.render_frame(0, val_range.0, val_range.1) {
        println!(
            "rendered {} bytes, min {:?}, max {:?}",
            pixels.len(),
            pixels.iter().min(),
            pixels.iter().max()
        );
    }
    Ok(LoadedFile {
        name,
        obj,
        metadata,
        min_max: val_range,
        volume,
    })
}
