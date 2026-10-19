//! 🏙️ BIM plugin - building information model editor and viewer bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. The plugin contract lives in the composition root `🦀️.rs`; every surface comes from the artifact crate.

#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
mod artifacts {
    pub use semio_s_artifact_bim_model as model;
}
//#endregion 🗿️Artifacts

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
pub mod plugin;
//#endregion 🔖️Plugin
