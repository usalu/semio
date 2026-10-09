//! 🏙️ BIM plugin - building information model editor and viewer bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. The plugin contract lives in the composition root `🦀️.rs`; every surface comes from the artifact crate.

#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
mod artifacts {
    pub use semio_s_artifact_bim_model as model;
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
mod editor {
    pub use semio_s_artifact_bim_model::editor::*;
}
//#endregion ✏️Editor

//#region 👁️Viewer
mod viewer {
    pub use semio_s_artifact_bim_model::viewer::*;
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }, plugin::plugin, plugin::BimApps);
//#endregion 🔖️Plugin
