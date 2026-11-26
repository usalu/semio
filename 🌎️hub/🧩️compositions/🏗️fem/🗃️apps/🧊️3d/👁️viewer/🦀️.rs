//! 👁️ Owns mounted PluginApp dispatch for the FEM 3D viewer.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_s_artifact_fem_3d_viewer::viewer::fem3d::Fem3dViewer;

/// 🧩️ The FEM 3D viewer payload accepted by the artifact declaration.
pub type Fem3dViewerApp = VcsArtifactApp<ViewerApp<Fem3dViewer>>;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🧊️ Dispatch for the FEM 3D viewer.
    pub enum Fem3dViewerApps: PluginApp {
        Fem3dViewer(Fem3dViewerApp),
    }
}
