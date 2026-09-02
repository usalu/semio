//! 📚️ Example `hexagonal-cut-concrete-forest-right`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "hexagonal-cut-concrete-forest-right";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Hexagonal Cut Concrete Forest Right", "Hexagonal Cut Concrete Forest Right")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🧪️hexagonal-cut-concrete-forest-right/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
