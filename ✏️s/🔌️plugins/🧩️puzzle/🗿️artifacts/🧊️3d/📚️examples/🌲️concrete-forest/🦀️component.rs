//! 📚️ Example concrete-forest for 🧊️3d.

use semio_framework_os_kernel::plugin::ExampleSource;
use semio_framework::LocalizedLabel;

pub const ID: &str = "concrete-forest";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Concrete Forest", "Betonwald") }
pub const ICON: &str = "list-tree";
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️tower.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), DSL_TEXT, ICON) }
