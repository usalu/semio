//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};
use semio_framework_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the process editor and viewer surfaces.
    pub enum ProcessApps: PluginApp {
        Process3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_process_process3d_editor::Process3dPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>),
        Process3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_process_process3d_viewer::Process3dViewer>, semio_s_artifact_stdio_semio::SemioMembers>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M4, `📓️design-abi.md`
/// §5/§6) are this crate's proof-of-migration: the host activates one instance whenever a
/// `"3d.process"` artifact (`semio_s_artifact_process_process3d_model::snapshot::artifact_kind().id`) is opened, this
/// plugin's own actor runs `Isolated` (its 4 `🧩️extensions/` — metal, robotic, concrete, wood — run
/// `Declarative` instead, see each extension's own `bundle()`), and it asks the broker for document
/// write access to persist edits.
pub fn plugin() -> Result<Plugin<ProcessApps>, PluginAssemblyError> {
    PluginBuilder::<_, ProcessApps>::new("process")
        .label("Process")
        .version("0.1.0")
        .package_id("semio:process")
        .artifact(semio_s_artifact_process_process3d::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_process_process3d_editor::Process3dPlayApp>(semio_s_artifact_process_process3d_editor::create_process3d_app())
        .editor_mutation_roster::<semio_s_artifact_process_process3d_editor::Process3dPlayApp>()
        .viewer::<semio_s_artifact_process_process3d_viewer::Process3dViewer>(semio_s_artifact_process_process3d_viewer::create_process3d_viewer())
        .viewer_mutation_roster::<semio_s_artifact_process_process3d_viewer::Process3dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_process_process3d_model::snapshot::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist process3d machine-assignment edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🪢️TaxonomyMounts
#[path = "🎟️capabilities/🦀️.rs"]
pub mod capabilities;
#[path = "📜️manifest/🦀️.rs"]
pub mod manifest;
#[path = "🔧️setup/🦀️.rs"]
pub mod setup;
//#endregion 🪢️TaxonomyMounts
