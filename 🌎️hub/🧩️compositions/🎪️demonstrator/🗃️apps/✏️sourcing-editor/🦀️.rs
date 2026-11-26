//! 🗃️ Owns the Demonstrator sourcing editor surface's mounted app dispatch.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::declarations::EditorSurfaceApp;
use semio_plugin::plugin_app_close_prelude::*;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🎪️ The sourcing editor routed by the Demonstrator app fleet.
    pub enum SourcingEditorApps: PluginApp {
        SourcingEditor(EditorSurfaceApp<semio_s_artifact_sourcing_curation::editor::sourcing::SourcingCurationApp>),
    }
}
