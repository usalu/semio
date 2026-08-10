//! 📤️ Export Puzzle2dSnapshot as .stl.

use semio_framework_plugin::{ArtifactCodec, IoError, JsonCodec, MediaFormat};

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::puzzle2d::Puzzle2dSnapshot) -> Result<Vec<u8>, IoError> {
    let value = serde_json::to_value(snapshot).map_err(|e| IoError::Payload(e.to_string()))?;
    JsonCodec.export(&value)
}

pub fn register() {
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    {

        let kind = "2d.block";
        let format = MediaFormat::Stl;
        semio_framework_os::register_os_media_export_handler(kind, format, move |doc| {
            let snapshot: crate::artifacts::puzzle2d::Puzzle2dSnapshot =
                serde_json::from_value(doc.clone()).map_err(|e| e.to_string())?;
            let bytes = export(&snapshot).map_err(|e| e.to_string())?;
            let stem = kind.replace('.', "_");
            semio_framework_os::OsMediaExportResult::from_format_bytes(bytes, format, &stem)
        });
    }
}
//#endregion 🔖️Export
