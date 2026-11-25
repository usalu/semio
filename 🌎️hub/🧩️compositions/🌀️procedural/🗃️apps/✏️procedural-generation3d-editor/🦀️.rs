//! 🗃️ The closed runtime app fleet of the generation3d editor surface: one crate per app so the app machinery of the composition is never instantiated in a single compilation unit.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The one-app fleet of the generation3d editor surface, routed by the composition's own fleet enum.
    pub enum Generation3dEditorApps: PluginApp {
        Generation3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_procedural_3d_editor::Generation3dPlayApp>>),
    }
}
