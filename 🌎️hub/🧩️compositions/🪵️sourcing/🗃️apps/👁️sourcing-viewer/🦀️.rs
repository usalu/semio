//! 🗃️ The closed runtime app fleet of the sourcing viewer surface: one crate per app so the app machinery of the composition is never instantiated in a single compilation unit.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The one-app fleet of the sourcing viewer surface, routed by the composition's own fleet enum.
    pub enum SourcingViewerApps: PluginApp {
        Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_sourcing_curation::viewer::sourcing::SourcingViewer>, semio_s_artifact_stdio_semio::SemioMembers>),
    }
}
