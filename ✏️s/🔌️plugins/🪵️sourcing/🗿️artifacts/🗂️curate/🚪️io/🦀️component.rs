//! 🚪️ 🗂️curate IO facet — declared MediaFormat table + OS handler registration.

use semio_framework_plugin::{ArtifactIo, IoFormatSpec, MediaFormat};

//#region 🔖️Formats
pub fn format_specs() -> &'static [IoFormatSpec] {
    &[
        IoFormatSpec { format: MediaFormat::Glb, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Json, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Obj, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Png, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Stl, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Zip, import: true, export: true }
    ]
}
//#endregion 🔖️Formats

//#region 🔖️ArtifactIo
/// 🚪️ curate artifact IO registration surface.
pub struct Io;

impl ArtifactIo for Io {
    fn formats() -> &'static [IoFormatSpec] { format_specs() }
    fn register() {
        super::glb::export::register();
        super::glb::import::register();
        super::json::export::register();
        super::json::import::register();
        super::obj::export::register();
        super::obj::import::register();
        super::png::export::register();
        super::png::import::register();
        super::stl::export::register();
        super::stl::import::register();
        super::zip::export::register();
        super::zip::import::register();
    }
}

pub fn register() {
    <Io as ArtifactIo>::register();
}
//#endregion 🔖️ArtifactIo
