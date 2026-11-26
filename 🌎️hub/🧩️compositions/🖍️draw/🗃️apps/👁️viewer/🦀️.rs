//! 🗃️ Mounted application dispatch for the Draw viewer.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_s_artifact_draw_drawing_viewer::DrawingViewer;

/// 🧩️ The mounted viewer payload constructed by the Draw artifact declaration.
pub type DrawViewerApp = VcsArtifactApp<ViewerApp<DrawingViewer>>;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎮️ Closed dispatch for the active Draw viewer.
    pub enum DrawViewerApps: PluginApp {
        Viewer(DrawViewerApp),
    }
}
