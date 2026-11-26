//! 👁️ Owns the Process 3D viewer's original mounted app dispatch.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin_instance_construction::OriginalVcsApp;
use semio_s_artifact_process_process3d_viewer::Process3dViewer;
use semio_s_artifact_stdio_semio::SemioMembers;

/// 🔒️ The native viewer and its retained member authority.
pub type Process3dViewerApp = OriginalVcsApp<ViewerApp<Process3dViewer>, SemioMembers>;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗂️ Dispatch for the native Process 3D viewer.
    pub enum ProcessViewerApps: PluginApp {
        ProcessViewer(Process3dViewerApp),
    }
}
