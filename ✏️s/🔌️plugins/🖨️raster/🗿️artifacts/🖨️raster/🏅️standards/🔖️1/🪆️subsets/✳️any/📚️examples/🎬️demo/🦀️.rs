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
///
/// The bytes are the repo's own shipped semio emblem raster
/// (`🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖼️raster.png`, 512×512 RGBA): the
/// dark-round variant because this pane's paper background is light, so it is the one a reader can
/// actually see. Until 2026-09-22 this was a 75-byte 2×2 red/green/blue/white swatch against a
/// carrier that declared a 1024×1024 backdrop — real, decodable, and visually indistinguishable from a
/// pane that renders nothing, which is exactly how it masked the missing `paint-2d` lane route for
/// three sessions. `🔬️boot-document`'s own law now pins the asset's size against the carrier's
/// declaration so the two can never drift again.
pub fn emblem_image_asset() -> RasterImageAsset {
    RasterImageAsset { mime: "image/png".into(), data: include_bytes!("../../🖼️assets/🎬️demo/🖼️semio-emblem.png").to_vec() }
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
