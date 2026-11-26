//! 🗃️ Owns the Demonstrator sourcing viewer surface's mounted app dispatch.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::declarations::ViewerSurfaceApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎪️ The sourcing viewer routed by the Demonstrator app fleet.
    pub enum SourcingViewerApps: PluginApp {
        SourcingViewer(ViewerSurfaceApp<semio_s_artifact_sourcing_curation::viewer::sourcing::SourcingViewer>),
    }
}
