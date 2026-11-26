//! 🗃️ Mounted application dispatch for the Draw editor.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_s_artifact_draw_drawing_editor::DrawingPlayApp;

/// 🧩️ The mounted editor payload constructed by the Draw artifact declaration.
pub type DrawEditorApp = VcsArtifactApp<EditorApp<DrawingPlayApp>>;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎮️ Closed dispatch for the active Draw editor.
    pub enum DrawEditorApps: PluginApp {
        Editor(DrawEditorApp),
    }
}
