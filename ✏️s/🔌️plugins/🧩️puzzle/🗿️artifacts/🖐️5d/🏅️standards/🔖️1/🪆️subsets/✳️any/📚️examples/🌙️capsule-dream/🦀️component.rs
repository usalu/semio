//! 📚️ Example `🌙️capsule-dream` for artifact `puzzle5d`.

use std::sync::LazyLock;

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "capsule-dream";

/// 🗣️ Localized picker label.
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Capsule Dream", "Kapseltraum")
}

/// 🖼️ Icon id.
pub const ICON: &str = "building";

/// 🗣️ DSL fixture text.
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️dream.dsl.semio");

/// 🔧️ Op fixture text.
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️dream.op.semio");

/// 🎒️ Pack fixture bytes.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️dream.pack.semio");

/// 📡️ SPR fixture bytes.
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️dream.spr.semio");

/// 🏅 Golden flattened poses from compose Flat design (piece id → pose).
pub const GOLDEN_POSES_JSON: &str = include_str!("🖼️assets/🏅golden-poses.json");

fn document_json() -> String {
    let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(DSL_TEXT).unwrap_or_else(|error| panic!("{ID} example dsl parses: {error}"));
    serde_json::to_string(&projection).expect("serialize example")
}

/// 📚️ Canonical example source for `App::example_source`.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| ExampleSource::new(ID, label(), document_json(), ICON));
