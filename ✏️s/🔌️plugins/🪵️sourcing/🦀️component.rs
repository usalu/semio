//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. Atomic cutover (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): `.declare_artifact(...)` (new declaration
/// tree) replaces `.artifact(...)`/`.editor::<>()`/`.viewer::<>()` outright — the old channel is
/// NOT kept alongside it (a second parallel registration channel is the compatibility layer this
/// ticket forbids). `.editor_mutation_roster()`/`.viewer_mutation_roster()` stay: they are an
/// orthogonal, still-supported opt-in (`contributor.list-artifact-mutations`) the new declaration
/// tree's `SurfaceDeclaration.mutation_roster` field does not yet wire live (`📓️w1-c-report.md`
/// openQuestion 3) — not a second registration of the artifact/schema/io itself.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M2, `📓️design-abi.md` §3): the host activates
/// one instance whenever a `"catalogue.sourcing"` artifact (`crate::artifacts::curate::
/// artifact_kind().id`) is opened, this plugin's actor runs `Isolated` (no publisher trust assumed
/// beyond the sandbox default — nothing in this crate's own effects, all UI-chrome/RPC `Effect`
/// variants with no documented `CapabilityId`, justifies otherwise), and it asks the broker for
/// document write access to persist edits.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("sourcing")
        .label("Sourcing")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::curate::artifact())
        .editor_mutation_roster::<crate::editor::sourcing::SourcingCurateApp>()
        .viewer_mutation_roster::<crate::viewer::sourcing::SourcingViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::curate::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist sourcing catalogue edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️Tests
#[cfg(test)]
mod surface_tests {
    /// 👁️✏️ Editor and viewer must share the exact same `Dialect` — both surfaces address the same
    /// artifact coordinate, only the role differs (contract §2.5).
    #[semio_framework_async_macros::async_test]
    async fn editor_and_viewer_share_the_same_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::sourcing::SourcingCurateApp, crate::viewer::sourcing::SourcingViewer>();
    }

    /// 👁️ Structural + runtime proof the viewer can never mutate the document or draft store
    /// (contract §2.2/§2.5) — dispatches `SourcingViewCommand::default()` through the full
    /// `VcsArtifactApp<ViewerApp<SourcingViewer>>` runtime path.
    #[semio_framework_async_macros::async_test]
    async fn viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::sourcing::SourcingViewer>();
    }
}
//#endregion 🧪️Tests
