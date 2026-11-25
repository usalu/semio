//! 🛂️ Demonstrator plugin manifest — its own `🎪️playground` editor/viewer surfaces plus the
//! `sourcing` and `process` editor+viewer surface registrations (six surfaces in all). The cad,
//! gis, puzzle and generation3d panes run on their owners' own components.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{Plugin, PluginApp};
use semio_plugin::declarations::{EditorSurfaceApp, ViewerSurfaceApp};
use semio_plugin::PluginBuilder;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};

use semio_s_artifact_process_process3d_editor::{create_process3d_app, Process3dPlayApp};
use semio_s_artifact_process_process3d_viewer::{create_process3d_viewer, Process3dViewer};
use semio_s_artifact_sourcing_curation::editor::sourcing::{create_sourcing_curation_app, SourcingCurationApp};
use semio_s_artifact_sourcing_curation::viewer::sourcing::{create_sourcing_viewer, SourcingViewer};

const PLUGIN_ID: &str = "demonstrator";
const PLUGIN_LABEL: &str = "Entwerfen mit Bestand";
/// 🔢️ The version this bundle and every foreign plugin it composes are built at: all of them are members of one
/// workspace (`version.workspace = true`), and one tree is one catalog.
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

//#region 🔌️Plugin
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the demonstrator's owned and bundled surfaces — every variant
    /// spelled through `EditorSurfaceApp`/`ViewerSurfaceApp`, so each bundled app brings its OWN member
    /// roster (`ArtifactEditor::Members`) instead of this bundle guessing one.
    pub enum DemonstratorApps: PluginApp {
        PlaygroundEditor(EditorSurfaceApp<crate::editor::playground::PlaygroundEditor>),
        PlaygroundViewer(ViewerSurfaceApp<crate::viewer::playground::PlaygroundViewer>),
        SourcingEditor(EditorSurfaceApp<SourcingCurationApp>),
        SourcingViewer(ViewerSurfaceApp<SourcingViewer>),
        ProcessEditor(EditorSurfaceApp<Process3dPlayApp>),
        ProcessViewer(ViewerSurfaceApp<Process3dViewer>),
    }
}

/// 🔌️ Builds the concrete demonstrator bundle: declares its owned playground artifact, registers
/// its own native editor+viewer surfaces over that artifact, then the `sourcing` and `process`
/// editor+viewer pairs.
pub fn plugin() -> Result<Plugin<DemonstratorApps>, PluginAssemblyError> {
    PluginBuilder::<_, DemonstratorApps>::new(PLUGIN_ID)
        .label(PLUGIN_LABEL)
        .version(PLUGIN_VERSION)
        .package_id("semio:demonstrator")
        .depends_on("process", semio_framework::tree_pin!())
        .depends_on("sourcing", semio_framework::tree_pin!())
        .artifact(crate::artifacts::playground::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<crate::editor::playground::PlaygroundEditor>(crate::editor::playground::create_playground_editor())
        .editor_mutation_roster::<crate::editor::playground::PlaygroundEditor>()
        .viewer::<crate::viewer::playground::PlaygroundViewer>(crate::viewer::playground::create_playground_viewer())
        .viewer_mutation_roster::<crate::viewer::playground::PlaygroundViewer>()
        .editor::<SourcingCurationApp>(create_sourcing_curation_app())
        .editor_mutation_roster::<SourcingCurationApp>()
        .viewer::<SourcingViewer>(create_sourcing_viewer())
        .viewer_mutation_roster::<SourcingViewer>()
        .editor::<Process3dPlayApp>(create_process3d_app())
        .editor_mutation_roster::<Process3dPlayApp>()
        .viewer::<Process3dViewer>(create_process3d_viewer())
        .viewer_mutation_roster::<Process3dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::playground::artifact_kind().id })
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist process3d and sourcing editor edits to the open document".into(), optional: false })
        .try_build()
}
//#endregion 🔌️Plugin

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🪞️StandaloneParityTests
#[cfg(test)]
#[path = "🧪️tests/🔬️standalone-parity/🦀️.rs"]
mod standalone_parity_tests;
//#endregion 🪞️StandaloneParityTests
