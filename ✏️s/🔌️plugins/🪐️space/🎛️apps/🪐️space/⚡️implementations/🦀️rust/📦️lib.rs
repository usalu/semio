//! 🎛️ S Studio app — document entities (constitutional: general).
//!
//! 🕳️ Deviation from the constitutional split recipe's usual "rs" content (a `#[derive(dsl::DslDocument)]`
//! entity + `X_DOCUMENT_SCHEMA` constant): the Studio app has no document type of its own — its
//! `DocumentApp::Projection`/`Operation` are `semio_framework_os::{WorkflowDocument, WorkflowOperation}`, owned
//! entirely by `framework/product/os/core/rs` (outside this plugin). What this app DOES own is
//! `SpaceWindowCamera` — a tiny per-window camera record, the one domain value `space_engine::SpaceConfig`
//! (its `DocumentApp::Config`, see the `⚙️engine` crate) needs that isn't already declared by os-core —
//! mirrors `shooting`'s own `ShootingCamera` living in its entities crate rather than in `shooting_engine`.

use serde::{Deserialize, Serialize};

//#region 🔖️Constants
// 🕳️ Also a deviation from the usual "rs" content: these manifest/panel identifiers are shared by
// `space_engine` (config defaults) and `space_ui` (manifest window/panel/body wiring, render dispatch)
// — centralized here rather than duplicated in both.
pub const S_PLAY_APP_ID: &str = "studio";
pub const S_PLAY_CONTROLLER_ID: &str = "s-play";
pub const S_PLAY_SURFACE_WORKFLOW: &str = "s.play.workflow";
pub const S_PLAY_SURFACE_MEDIA_VFS: &str = "s.play.media-vfs";
pub const S_PLAY_SURFACE_COMPILED_DAG: &str = "s.play.compiled-dag";
pub const S_PLAY_BODY_WORKFLOW: &str = "s.play.workflow";
pub const S_PLAY_BODY_MEDIA_VFS: &str = "s.play.media-vfs";
pub const S_PLAY_BODY_COMPILED_DAG: &str = "s.play.compiled-dag";
pub const S_PLAY_WINDOW_WORKFLOW: &str = "s-workflow";
pub const S_PLAY_WINDOW_MEDIA_VFS: &str = "s-media-vfs";
pub const S_PLAY_WINDOW_COMPILED_DAG: &str = "s-compiled-dag";
pub const S_PLAY_CATALOGUE_TAB_ID: &str = "s-play-catalogue";
pub const S_PLAY_PARAMETERS_TAB_ID: &str = "s-play-parameters";
pub const S_PLAY_INSPECTOR_TAB_ID: &str = "s-play-inspector";
pub const S_PLAY_CATALOGUE_BODY_KEY: &str = "s.play.catalogue";
pub const S_PLAY_PARAMETERS_BODY_KEY: &str = "s.play.parameters";
pub const S_PLAY_INSPECTOR_BODY_KEY: &str = "s.play.inspector";
pub const S_PLAY_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

pub const S_STUDIO_EXAMPLES: &[(&str, &str)] = &[("demo", "Demo Studio")];
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🎥️ One window-instance's workflow-canvas camera — keyed by window id inside
/// `space_engine::SpaceConfig.camera` (a `BTreeMap<String, SpaceWindowCamera>`, per the Configured Node
/// Apps recipe's "camera/selection/per-window options keyed by window-instance id" rule). Distinct from
/// `semio_framework_os::OsWorkflowCamera` (a plain, non-`dsl`-field data type this crate can't blanket-
/// impl `dsl::DslField` for under the orphan rule) — converts to/from it 1:1 at the render boundary.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SpaceWindowCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for SpaceWindowCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

impl From<semio_framework_os::OsWorkflowCamera> for SpaceWindowCamera {
    fn from(camera: semio_framework_os::OsWorkflowCamera) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}

impl From<SpaceWindowCamera> for semio_framework_os::OsWorkflowCamera {
    fn from(camera: SpaceWindowCamera) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}
//#endregion 🔖️Types
