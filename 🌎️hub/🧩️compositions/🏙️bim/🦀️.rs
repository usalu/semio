//! 🔌️ Plugin root contract - typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the BIM model editor and viewer surfaces.
    pub enum BimApps: PluginApp {
        BimEditor(VcsArtifactApp<EditorApp<crate::editor::bim::BimModelApp>>),
        BimViewer(VcsArtifactApp<ViewerApp<crate::viewer::bim::BimModelViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the BIM plugin surface for host registration: one declared artifact tree (schema, io with the IFC, glTF and SVG hops, editor, viewer, examples), activated whenever an
/// `s.bim.model` document opens, isolated execution, and document write access to persist edits.
pub fn plugin() -> Result<Plugin<BimApps>, PluginAssemblyError> {
    Plugin::<BimApps>::builder("bim")
        .label("BIM")
        .version("0.1.0")
        .package_id("semio:bim")
        .declare_artifact(crate::artifacts::model::artifact())
        .editor_mutation_roster::<crate::editor::bim::BimModelApp>()
        .viewer_mutation_roster::<crate::viewer::bim::BimModelViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::model::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist building model edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️tests/🛂️committed-descriptor/🦀️.rs"]
mod committed_descriptor_tests;
