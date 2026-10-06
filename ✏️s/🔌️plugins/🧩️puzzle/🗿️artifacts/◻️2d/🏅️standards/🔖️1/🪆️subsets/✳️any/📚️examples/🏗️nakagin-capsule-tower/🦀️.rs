//! 📚️ Example `🏗️nakagin-capsule-tower` for artifact `puzzle2d`.

use std::sync::LazyLock;

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "nakagin-capsule-tower";

/// 🗣️ Localized picker label.
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm")
}

/// 🖼️ Icon id.
pub const ICON: &str = "building";

/// 🗣️ DSL snapshot text.
pub const DSL_TEXT: &str = include_str!("🖼️assets/🏢️tower/🗣️.dsl.semio");

/// 🔧️ Op snapshot text.
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️tower.op.semio");

/// 🎒️ Pack snapshot bytes.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️.pack.semio");

/// 📡️ SPR snapshot bytes.
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️tower.spr.semio");

fn document_json() -> String {
    let projection = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(DSL_TEXT).unwrap_or_else(|error| panic!("{ID} example dsl parses: {error}"));
    semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&projection))
}

/// 📚️ Canonical example source for `App::example_source` — DEFERRED: parsing [`DSL_TEXT`] eagerly
/// here ran while the bundle was assembled, i.e. before `describe()` built anything, and is what
/// put this package over its own guest epoch (`📓️a3-descriptor-regeneration.md` §3). The descriptor
/// declares this example by the authored DSL's size and SHA-256, which needs no parse.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| ExampleSource::deferred(ID, label(), ICON, ".dsl.semio", DSL_TEXT.as_bytes(), document_json));
