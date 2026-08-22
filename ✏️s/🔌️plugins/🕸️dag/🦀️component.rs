//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the declaration-owned DAG surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum DagApps: PluginApp {}
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.declare_artifact(…)` (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1/§2) is the ONLY registration
/// channel for `s.dag.dag` — it walks artifact→standard→subset and registers schema, io,
/// viewer/editor surfaces and examples in one pass, replacing the old `.artifact(declaration())`
/// + `.editor::<E>(AppDefinition)` + `.viewer::<V>(AppDefinition)` triple atomically (no dual
/// registration — that is a forbidden compatibility layer, already rejected once on this ticket).
/// `.editor_mutation_roster()`/`.viewer_mutation_roster()` are KEPT — an orthogonal opt-in
/// (`contributor.list-artifact-mutations`) `SurfaceDeclaration.mutation_roster` does not yet wire
/// live, so keeping them is not a second registration of the artifact/schema/io itself (same
/// reasoning the `🎬️sequence` W4 pass documented). `.activation()`/`.execution()`/`.requests()`
/// are unrelated microkernel-actor-runtime wiring (ticket MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
/// live peer) — untouched by this pass.
pub async fn plugin() -> Result<Plugin<DagApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<DagApps>::builder("dag")
        .label("DAG")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::dag::artifact())
        .editor_mutation_roster::<crate::editor::dag::DagPlayApp>()
        .viewer_mutation_roster::<crate::viewer::dag::DagViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::dag::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist dag edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5's
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
    //! (W0-F gap 2) are used directly here — no local stand-ins, unlike the pilot packet which
    //! predated their landing.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn dag_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::dag::DagViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn dag_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::dag::DagPlayApp, crate::viewer::dag::DagViewer>();
    }
}
//#endregion 🧪️SurfaceTests
