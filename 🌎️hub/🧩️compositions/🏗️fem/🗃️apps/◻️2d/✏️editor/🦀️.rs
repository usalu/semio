//! ✏️ Owns mounted PluginApp dispatch for the FEM 2D editor.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp;

/// 🧩️ The FEM 2D editor payload accepted by the artifact declaration.
pub type Fem2dEditorApp = VcsArtifactApp<EditorApp<Fem2dPlayApp>>;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// ◻️ Dispatch for the FEM 2D editor.
    pub enum Fem2dEditorApps: PluginApp {
        Fem2dEditor(Fem2dEditorApp),
    }
}
