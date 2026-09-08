//! 🔀️ DAG plugin — declarative DAG play app bundled as a hot-swappable WASM component.

extern crate infinite_canvas as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🗿️Artifacts
mod artifacts { pub use semio_s_artifact_dag_dag as dag; }
//#endregion 🗿️Artifacts

//#region ✏️Editor
mod editor { pub use semio_s_artifact_dag_dag::editor::*; }
//#endregion ✏️Editor

//#region 👁️Viewer
mod viewer { pub use semio_s_artifact_dag_dag::viewer::*; }
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::DagApps;
semio_framework_plugin::plugin_exports!(plugin::plugin, DagApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_dag_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
