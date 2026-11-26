//! 🏛️ Mounted app dispatch published from artifact-owned Stdio contributions.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎛️ Mounted app dispatch for this artifact dialect.
    pub enum StdioXmlApps: PluginApp {
        XmlAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor>>),
        XmlAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer>>),
    }
}
