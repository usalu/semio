//! 🗃️ Owns the Demonstrator process editor surface's mounted app dispatch.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::declarations::EditorSurfaceApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎪️ The process editor routed by the Demonstrator app fleet.
    pub enum ProcessEditorApps: PluginApp {
        ProcessEditor(EditorSurfaceApp<semio_s_artifact_process_process3d_editor::Process3dPlayApp>),
    }
}
