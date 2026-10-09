//! ✒️ Writer plugin — declarative writer play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, per SHAPE V2 —
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST` — so every leaf path carries a `../../` prefix to reach
//! back out to the owner-root tree). The grouping modules carry `#[path = "."]` so their own names
//! are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

//#region 🗿️Artifacts
mod artifacts {
    pub use semio_s_artifact_writer_writer as writer;
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
mod editor {
    pub use semio_s_artifact_writer_writer::editor::*;
}
//#endregion ✏️Editor

//#region 👁️Viewer
mod viewer {
    pub use semio_s_artifact_writer_writer::viewer::*;
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }, plugin::plugin, plugin::WriterApps);

//#endregion 🔖️Plugin
