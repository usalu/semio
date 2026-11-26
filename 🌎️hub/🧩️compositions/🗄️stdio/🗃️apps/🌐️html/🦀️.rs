//! 🏛️ Mounted app dispatch published from artifact-owned Stdio contributions.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎛️ Mounted app dispatch for this artifact dialect.
    pub enum StdioHtmlApps: PluginApp {
        HtmlEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_html::editor::html::HtmlEditor>>),
        HtmlViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_html::viewer::html::HtmlViewer>>),
    }
}
