//! 📚️ Example `🌲️concrete-forest` for artifact `puzzle2d`.

use std::sync::LazyLock;

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "concrete-forest";

/// 🗣️ Localized picker label.
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Concrete Forest", "Betonwald")
}

/// 🖼️ Icon id.
pub const ICON: &str = "list-tree";

/// 🗣️ DSL fixture text.
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️forest.dsl.semio");

/// 🔧️ Op fixture text.
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️forest.op.semio");

/// 🎒️ Pack fixture bytes.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️forest.pack.semio");

/// 📡️ SPR fixture bytes.
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️forest.spr.semio");

async fn document_json() -> String {
    let projection = crate::artifacts::puzzle2d::dsl::parse_dsl(DSL_TEXT).unwrap_or_else(|error| panic!("{ID} example dsl parses: {error}"));
    let mut value = serde_json::to_value(&projection).expect("serialize example");
    if let Some(object) = value.as_object_mut() {
        object.remove("camera");
    }
    serde_json::to_string(&value).expect("re-serialize example")
}

/// 📚️ Canonical example source for `App::example_source`.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| ExampleSource::new(ID, label(), document_json(), ICON));
