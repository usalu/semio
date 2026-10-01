//! 🔀️ DAG plugin — declarative DAG play app bundled as a hot-swappable WASM component.

//#region 🗿️Artifacts
mod artifacts {
    pub use semio_s_artifact_dag_dag as dag;
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
mod editor {
    pub use semio_s_artifact_dag_dag::editor::*;
}
//#endregion ✏️Editor

//#region 👁️Viewer
mod viewer {
    pub use semio_s_artifact_dag_dag::viewer::*;
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::DagApps;
semio_framework_plugin::plugin_exports!(plugin::plugin, DagApps);

//#endregion 🔖️Plugin
