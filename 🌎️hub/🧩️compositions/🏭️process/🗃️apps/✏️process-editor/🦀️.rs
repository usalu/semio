//! ✏️ Owns the Process 3D editor's original mounted app dispatch.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin_instance_construction::OriginalVcsApp;
use semio_s_artifact_process_process3d_editor::Process3dPlayApp;
use semio_s_artifact_stdio_semio::SemioMembers;

/// 🔐️ The native editor and its retained member authority.
pub type Process3dEditorApp = OriginalVcsApp<EditorApp<Process3dPlayApp>, SemioMembers>;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🧭️ Dispatch for the native Process 3D editor.
    pub enum ProcessEditorApps: PluginApp {
        ProcessEditor(Process3dEditorApp),
    }
}
