//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the raster editor and viewer surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum RasterApps: PluginApp {
        RasterEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::raster::RasterPlayApp>>),
        RasterViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::raster::RasterViewer>>),
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
pub async fn plugin() -> Result<Plugin<RasterApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<RasterApps>::builder("raster")
        .label("Raster")
        .version("0.1.0")
        .artifact(crate::artifacts::raster::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::raster::RasterPlayApp>(crate::editor::raster::create_raster_app())
        .editor_mutation_roster::<crate::editor::raster::RasterPlayApp>()
        .viewer::<crate::viewer::raster::RasterViewer>(crate::viewer::raster::create_raster_viewer())
        .viewer_mutation_roster::<crate::viewer::raster::RasterViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::raster::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist raster edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️Tests
#[cfg(test)]
mod surface_tests {
    //! 🧪️ Contract §2.5 surface laws, now the real framework functions (`📓️w0-f-report.md` Gap 2) —
    //! no local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn raster_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::raster::RasterViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::raster::RasterPlayApp, crate::viewer::raster::RasterViewer>();
    }
}
//#endregion 🧪️Tests
