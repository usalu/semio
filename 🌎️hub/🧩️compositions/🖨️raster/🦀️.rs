//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the raster editor and viewer surfaces.
    pub enum RasterApps: PluginApp {
        RasterEditor(VcsArtifactApp<EditorApp<crate::editor::raster::RasterPlayApp>>),
        RasterViewer(VcsArtifactApp<ViewerApp<crate::viewer::raster::RasterViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `RasterPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `.editor(...)` below. Ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `.document_app(...)` (single mutation-capable
/// surface) split into `.editor(...)` + `.viewer(...)` (contract §2.4) — the same dialect, two roles.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M1, `📓️design-abi.md` §3/§6): the host
/// activates one instance whenever a `"2d.raster"` artifact
/// (`crate::artifacts::raster::artifact_kind().id`) is opened, this plugin's actor runs `Isolated`
/// (no cross-plugin extension attachment, the SDK default holds), and it asks the broker for
/// document write access because `RasterPlayApp` persists edits back to the open document.
///
/// 📚️ The navbar dropdown reads `RasterPlayApp::examples()`, the artifact catalogue `.editor`
/// stamps onto the manifest.
pub fn plugin() -> Result<Plugin<RasterApps>, PluginAssemblyError> {
    Plugin::<RasterApps>::builder("raster")
        .label("Raster")
        .version("0.1.0")
        .package_id("semio:raster")
        .artifact(crate::artifacts::raster::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<crate::editor::raster::RasterPlayApp>(crate::editor::raster::create_raster_app())
        .editor_mutation_roster::<crate::editor::raster::RasterPlayApp>()
        .viewer::<crate::viewer::raster::RasterViewer>(crate::viewer::raster::create_raster_viewer())
        .viewer_mutation_roster::<crate::viewer::raster::RasterViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::raster::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist raster edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️Tests
