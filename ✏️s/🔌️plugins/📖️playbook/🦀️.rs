//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the playbook editor and viewer surfaces.
    pub enum PlaybookApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<crate::editor::playbook::PlaybookPlayApp>>),
        Viewer(VcsArtifactApp<ViewerApp<crate::viewer::playbook::PlaybookViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.declare_artifact(…)` (ticket
/// `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, `terra-descriptors` packet, following the
/// `terra-fleet-trinity-recipe` recipe) replaces the old `.artifact(declaration())`/`.editor()`/
/// `.viewer()` triad — one registration channel for schema/io/viewer/editor rows.
/// `.editor_mutation_roster()`/`.viewer_mutation_roster()` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4) stay orthogonal opt-ins, not a
/// second registration channel. `.activation(…)`/`.execution(…)`/`.requests(…)`
/// (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M4, `📓️design-abi.md` §5/§6) are this
/// crate's proof-of-migration: the host activates one instance whenever a `"text.playbook"`
/// artifact (`crate::artifacts::playbook::artifact_kind().id`) is opened, this plugin's own actor
/// runs `Isolated` (its one `🧩️extensions/🌀️procedural` runs `Declarative` instead — see that
/// extension's own `bundle()`), and it asks the broker for document write access to persist edits.
pub fn plugin() -> Result<Plugin<PlaybookApps>, PluginAssemblyError> {
    Plugin::<PlaybookApps>::builder("playbook")
        .label("Playbook")
        .version("0.1.0")
        .package_id("semio:playbook")
        .declare_artifact(crate::artifacts::playbook::artifact())
        .editor_mutation_roster::<crate::editor::playbook::PlaybookPlayApp>()
        .viewer_mutation_roster::<crate::viewer::playbook::PlaybookViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::playbook::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist playbook step-list edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
