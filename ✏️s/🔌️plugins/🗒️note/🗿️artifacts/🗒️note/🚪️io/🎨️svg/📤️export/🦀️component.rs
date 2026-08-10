//! 📤️ Export NoteSnapshot as .svg.

use semio_framework_plugin::{IoError, MediaFormat};

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::note::NoteSnapshot) -> Result<Vec<u8>, IoError> {
    let (svg, _w, _h) = crate::artifacts::note::engine::note_document_to_svg(snapshot);
    Ok(svg.into_bytes())
}

pub fn register() {
    let kind = "2d.note";
    let format = MediaFormat::Svg;
    semio_framework_os::register_os_media_export_handler(kind, format, move |doc| {
        let snapshot: crate::artifacts::note::NoteSnapshot =
            serde_json::from_value(doc.clone()).map_err(|e| e.to_string())?;
        let bytes = export(&snapshot).map_err(|e| e.to_string())?;
        semio_framework_os::OsMediaExportResult::from_format_bytes(bytes, format, "note")
    });
}
//#endregion 🔖️Export
