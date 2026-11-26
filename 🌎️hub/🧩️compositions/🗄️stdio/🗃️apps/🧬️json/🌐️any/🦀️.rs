//! 🏛️ Mounted app dispatch published from artifact-owned Stdio contributions.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎛️ Mounted app dispatch for this artifact dialect.
    pub enum StdioJsonApps: PluginApp {
        JsonAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>>),
        JsonAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer>>),
    }
}
