//! 📚️ Example `🌲️concrete-forest` for artifact `puzzle3d`.

use std::sync::LazyLock;

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "concrete-forest";

/// 🗣️ Localized picker label.
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Concrete Forest", "Betonwald")
}

/// 🖼️ Icon id.
pub const ICON: &str = "list-tree";

/// 🗣️ DSL fixture text.
pub const DSL_TEXT: &str = include_str!("🖼️assets/🧪️forest/🗣️.dsl.semio");

/// 🔧️ Op fixture text.
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️forest.op.semio");

/// 🎒️ Pack fixture bytes.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️.pack.semio");

/// 📡️ SPR fixture bytes.
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️forest.spr.semio");

fn document_json() -> String {
    let projection = crate::artifacts::puzzle3d::dsl::parse_dsl(DSL_TEXT).unwrap_or_else(|error| panic!("{ID} example dsl parses: {error}"));
    dsl::json::to_json_string(&projection)
}

/// 📚️ Canonical example source for `App::example_source`.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| ExampleSource::new(ID, label(), document_json(), ICON));
