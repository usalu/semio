//! 🗃️ The closed runtime app fleet of the raster editor surface: one crate per app so the app machinery of the composition is never instantiated in a single compilation unit.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin_instance_construction::OriginalVcsApp;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The one-app fleet of the raster editor surface, routed by the composition's own fleet enum.
    pub enum RasterEditorApps: PluginApp {
        RasterEditor(OriginalVcsApp<EditorApp<semio_s_artifact_raster_raster_editor::RasterPlayApp>,semio_s_artifact_stdio_semio::SemioMembers>),
    }
}
