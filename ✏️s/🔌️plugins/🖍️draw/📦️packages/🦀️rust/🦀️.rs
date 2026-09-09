//! ✏️ Draw plugin — declarative draw app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, two levels inside the
//! plugin root — hence every leaf `#[path]` below is prefixed `../../`). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
// 🎭️ `fsm::statechart!` (used by `editor::drawing::commands::canvas_pointer_down`'s gesture machine) generates code
// containing `#[cfg(feature = "serde")]` gates meant for `fsm`'s OWN crate; macro hygiene splices
// that cfg check into the CALLING crate's feature list instead (a `fsm`/rustc macro-expansion
// limitation, not a real conditional-compilation bug here) — this crate declares no `serde` feature
// at all (the dependency is always-on), so rustc flags the value as unrecognized. Harmless, but a
// hard error under `-D warnings` without this crate-wide allow.

//#region 🗿️Artifacts
mod artifacts {
    pub use semio_s_artifact_draw_drawing as drawing;
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
mod editor {
    pub use semio_s_artifact_draw_drawing::editor::*;
}
//#endregion ✏️Editor

//#region 👁️Viewer
mod viewer {
    pub use semio_s_artifact_draw_drawing::viewer::*;
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::DrawApps;
semio_framework_plugin::plugin_exports!(plugin::plugin, DrawApps);

//#endregion 🔖️Plugin
