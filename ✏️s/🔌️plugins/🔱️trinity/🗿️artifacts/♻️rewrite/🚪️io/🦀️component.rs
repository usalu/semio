//! 🚪️ ♻️rewrite IO facet — declared MediaFormat table + OS handler registration.

use semio_framework_plugin::{ArtifactIo, IoFormatSpec, MediaFormat};

//#region 🔖️Formats
pub fn format_specs() -> &'static [IoFormatSpec] {
    &[
        IoFormatSpec { format: MediaFormat::Docx, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Json, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Md, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Pdf, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Txt, import: true, export: true }
    ]
}
//#endregion 🔖️Formats

//#region 🔖️ArtifactIo
/// 🚪️ rewrite artifact IO registration surface.
pub struct Io;

impl ArtifactIo for Io {
    fn formats() -> &'static [IoFormatSpec] { format_specs() }
    fn register() {
        super::docx::export::register();
        super::docx::import::register();
        super::json::export::register();
        super::json::import::register();
        super::md::export::register();
        super::md::import::register();
        super::pdf::export::register();
        super::pdf::import::register();
        super::txt::export::register();
        super::txt::import::register();
    }
}

pub fn register() {
    <Io as ArtifactIo>::register();
}
//#endregion 🔖️ArtifactIo
