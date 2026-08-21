//! 📚️ Example `🏗️nakagin-capsule-tower` for artifact `puzzle2d`.

use std::sync::LazyLock;

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "nakagin-capsule-tower";

/// 🗣️ Localized picker label.
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm")
}

/// 🖼️ Icon id.
pub const ICON: &str = "building";

/// 🗣️ DSL fixture text.
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️tower.dsl.semio");

/// 🔧️ Op fixture text.
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️tower.op.semio");

/// 🎒️ Pack fixture bytes.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️tower.pack.semio");

/// 📡️ SPR fixture bytes.
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️tower.spr.semio");

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
