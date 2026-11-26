//! 🗃️ Owns the Demonstrator playground editor surface's mounted app dispatch.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::declarations::EditorSurfaceApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎪️ The playground editor routed by the Demonstrator app fleet.
    pub enum PlaygroundEditorApps: PluginApp {
        PlaygroundEditor(EditorSurfaceApp<semio_s_artifact_demonstrator_playground::editor::playground::PlaygroundEditor>),
    }
}
