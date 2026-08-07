//! 📚️ Example `🏗️️nakagin-capsule-tower` for artifact ◻2d.

use semio_framework_os_kernel::plugin::ExampleSource;
use semio_framework::LocalizedLabel;

/// 🏷️ Stable example id for the navbar picker.
pub const ID: &str = "nakagin-capsule-tower";

/// 🗣️ Localized label.
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm")
}

/// 🖼️ Icon id.
pub const ICON: &str = "building";

/// 🗣️ DSL fixture text.
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️tower.dsl.semio");

/// 📚️ Example source for App::example_source.
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), DSL_TEXT, ICON)
}
