//! 🚪️ 🔌️jack IO facet — declared MediaFormat table + OS handler registration.

use semio_framework_plugin::{ArtifactIo, IoFormatSpec, MediaFormat};

//#region 🔖️Formats
pub fn format_specs() -> &'static [IoFormatSpec] {
    &[
        IoFormatSpec { format: MediaFormat::Csv, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Json, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Md, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Png, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Svg, import: true, export: true }
    ]
}
//#endregion 🔖️Formats

//#region 🔖️ArtifactIo
/// 🚪️ jack artifact IO registration surface.
pub struct Io;

impl ArtifactIo for Io {
    fn formats() -> &'static [IoFormatSpec] { format_specs() }
    fn register() {
        super::csv::export::register();
        super::csv::import::register();
        super::json::export::register();
        super::json::import::register();
        super::md::export::register();
        super::md::import::register();
        super::png::export::register();
        super::png::import::register();
        super::svg::export::register();
        super::svg::import::register();
    }
}

pub fn register() {
    <Io as ArtifactIo>::register();
}
//#endregion 🔖️ArtifactIo
