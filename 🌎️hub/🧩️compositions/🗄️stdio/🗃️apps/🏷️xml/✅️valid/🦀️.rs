//! 🏛️ Mounted app dispatch published from artifact-owned Stdio contributions.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎛️ Mounted app dispatch for this artifact dialect.
    pub enum StdioValidXmlApps: PluginApp {
        XmlValidEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor>>),
        XmlValidViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer>>),
    }
}
