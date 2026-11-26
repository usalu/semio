//! 🗃️ Owns the Demonstrator playground viewer surface's mounted app dispatch.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::declarations::ViewerSurfaceApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎪️ The playground viewer routed by the Demonstrator app fleet.
    pub enum PlaygroundViewerApps: PluginApp {
        PlaygroundViewer(ViewerSurfaceApp<semio_s_artifact_demonstrator_playground::viewer::playground::PlaygroundViewer>),
    }
}
