//! 📚️ Example `demo`.

use crate::RasterImageAsset;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 🖼️ The committed emblem pixels the demo DSL names through `semio-emblem` — media the textual
/// carrier cannot inline, minted on example load the same way `📸️remodel`'s `synthetic-orbit` frames
/// reach their document.
pub fn emblem_image_asset() -> RasterImageAsset {
    RasterImageAsset { mime: "image/png".into(), data: include_bytes!("../../🖼️assets/🎬️demo/🖼️semio-emblem.png").to_vec() }
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
