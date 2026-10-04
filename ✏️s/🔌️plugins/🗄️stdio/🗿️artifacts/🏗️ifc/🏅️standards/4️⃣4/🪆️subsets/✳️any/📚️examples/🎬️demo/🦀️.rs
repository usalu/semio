//! 🏘️ Authored IFC4 native example and explicit Part21 exchange asset.
use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "demo";
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
pub const PRIMARY_PACK: &[u8] = include_bytes!("🖼️assets/🎒️.pack.semio");
pub const EXCHANGE_TEXT: &str = include_str!("🖼️assets/🧪️example/🏗️.ifc");

pub fn label() -> LocalizedLabel { LocalizedLabel::native("Demo", "Demo") }
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
