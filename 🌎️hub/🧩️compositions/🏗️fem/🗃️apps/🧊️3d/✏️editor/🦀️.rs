//! ✏️ Owns mounted PluginApp dispatch for the FEM 3D editor.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp;

/// 🧩️ The FEM 3D editor payload accepted by the artifact declaration.
pub type Fem3dEditorApp = VcsArtifactApp<EditorApp<Fem3dPlayApp>>;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🧊️ Dispatch for the FEM 3D editor.
    pub enum Fem3dEditorApps: PluginApp {
        Fem3dEditor(Fem3dEditorApp),
    }
}
