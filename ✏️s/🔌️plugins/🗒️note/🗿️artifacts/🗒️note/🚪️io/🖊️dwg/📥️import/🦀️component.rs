//! 📥️ Import .dwg into NoteSnapshot.

use semio_framework_plugin::{IoError, MediaFormat};

//#region 🔖️Import
pub fn import(bytes: &[u8]) -> Result<crate::artifacts::note::NoteSnapshot, IoError> {
    let drawing = semio_framework_plugin::dwg_from_bytes(bytes).map_err(|e| IoError::Payload(e))?;
    let value = crate::artifacts::note::engine::note_document_json_from_dwg(&drawing)
        .map_err(|e| IoError::Payload(e))?;
    serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string()))
}

pub fn register() {
    let kind = "2d.note";
    let format = MediaFormat::Dwg;
    semio_framework_os::register_os_media_import_handler(kind, format, move |bytes| {
        let snapshot = import(bytes).map_err(|e| e.to_string())?;
        serde_json::to_value(snapshot).map_err(|e| e.to_string())
    });
}
//#endregion 🔖️Import
