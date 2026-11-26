//! 🗃️ Owns the Demonstrator process viewer surface's mounted app dispatch.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::declarations::ViewerSurfaceApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎪️ The process viewer routed by the Demonstrator app fleet.
    pub enum ProcessViewerApps: PluginApp {
        ProcessViewer(ViewerSurfaceApp<semio_s_artifact_process_process3d_viewer::Process3dViewer>),
    }
}
