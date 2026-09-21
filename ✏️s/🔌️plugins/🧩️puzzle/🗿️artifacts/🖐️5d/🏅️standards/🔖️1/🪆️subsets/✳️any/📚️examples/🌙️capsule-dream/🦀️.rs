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
pub const DSL_TEXT: &str = include_str!("🖼️assets/🌙️dream/🗣️.dsl.semio");

/// 🔧️ Op fixture text.
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️dream.op.semio");

/// 🎒️ Pack fixture bytes.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️.pack.semio");

/// 📡️ SPR fixture bytes.
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️dream.spr.semio");

/// 🏅 Golden flattened poses from compose Flat design (piece id → pose).
pub const GOLDEN_POSES_JSON: &str = include_str!("🖼️assets/🔣️.json");

/// 🏭️ Derives the document body from [`DSL_TEXT`]. 3 035 200 B of DSL in, ~3.5 MB of JSON out —
/// this is the producer `ExampleSource::deferred` holds, and NOTHING but an actual request for the
/// document runs it.
fn document_json() -> String {
    let projection = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(DSL_TEXT).unwrap_or_else(|error| panic!("{ID} example dsl parses: {error}"));
    dsl::json::to_json_string(&projection)
}

/// 📚️ Canonical example source for `App::example_source` — DEFERRED, because eagerly parsing
/// [`DSL_TEXT`] here ran while the bundle was assembled and made this package the one plugin in the
/// tree whose `describe` could not finish inside its 1 800 s guest epoch
/// (`📓️a3-descriptor-regeneration.md` §3, `📓️a3b-descriptor-sweep.md` §3). The descriptor declares
/// this example by the authored DSL's own size and SHA-256, which needs no parse.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| ExampleSource::deferred(ID, label(), ICON, ".dsl.semio", DSL_TEXT.as_bytes(), document_json));
