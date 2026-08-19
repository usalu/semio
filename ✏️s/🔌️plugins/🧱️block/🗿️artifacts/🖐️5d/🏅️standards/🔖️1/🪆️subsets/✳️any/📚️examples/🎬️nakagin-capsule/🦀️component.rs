//! 📚️ Example `nakagin-capsule`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "nakagin-capsule";
pub async fn label() -> LocalizedLabel { LocalizedLabel::native("Nakagin Capsule", "Nakagin Capsule") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️nakagin-capsule.dsl.semio");
pub async fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
