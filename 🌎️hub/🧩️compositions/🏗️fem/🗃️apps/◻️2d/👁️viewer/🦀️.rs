//! 👁️ Owns mounted PluginApp dispatch for the FEM 2D viewer.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_s_artifact_fem_2d_viewer::viewer::fem2d::Fem2dViewer;

/// 🧩️ The FEM 2D viewer payload accepted by the artifact declaration.
pub type Fem2dViewerApp = VcsArtifactApp<ViewerApp<Fem2dViewer>>;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// ◻️ Dispatch for the FEM 2D viewer.
    pub enum Fem2dViewerApps: PluginApp {
        Fem2dViewer(Fem2dViewerApp),
    }
}
