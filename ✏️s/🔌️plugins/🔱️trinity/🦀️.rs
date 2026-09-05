//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for both Trinity artifact surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum TrinityApps: PluginApp {
        JackEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::jack::TrinityJackPlayApp>>),
        JackViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::jack::TrinityJackViewer>>),
        RewritingEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::rewriting::TrinityRewritingPlayApp>>),
        RewritingViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::rewriting::TrinityRewritingViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. Atomic cutover (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, fleet-trinity-recipe): `.declare_artifact(…)`
/// (new declaration tree) replaces `.artifact(declaration())`/`.editor::<>()`/`.viewer::<>()` for
/// BOTH owned artifacts outright — the old channel is NOT kept alongside it (a second parallel
/// registration channel is the compatibility layer this ticket forbids), same cutover `🗒️note`/
/// `🖍️draw` already made. `.editor_mutation_roster()`/`.viewer_mutation_roster()` stay: they are an
/// orthogonal, still-supported opt-in (`contributor.list-artifact-mutations`) the new declaration
/// tree's `SurfaceDeclaration.mutation_roster` does not yet wire live — not a second registration of
/// the artifact/schema/io itself. `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining, `📓️design-abi.md` §3/§6) are this
/// crate's migration proof: one `OnArtifactKind` event per owned kind, read live from each artifact's
/// own `artifact_kind().id`, `Isolated` execution, one `documents.write` ask covering both editors.
pub fn plugin() -> Result<Plugin<TrinityApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<TrinityApps>::builder("trinity")
        .label("Trinity")
        .version("0.1.0")
        .package_id("semio:trinity")
        .declare_artifact(crate::artifacts::jack::artifact())
        .declare_artifact(crate::artifacts::rewriting::artifact())
        .editor_mutation_roster::<crate::editor::jack::TrinityJackPlayApp>()
        .viewer_mutation_roster::<crate::viewer::jack::TrinityJackViewer>()
        .editor_mutation_roster::<crate::editor::rewriting::TrinityRewritingPlayApp>()
        .viewer_mutation_roster::<crate::viewer::rewriting::TrinityRewritingViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::jack::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::rewriting::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist trinity jack/rewriting edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn trinity_jack_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::jack::TrinityJackViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_jack_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::jack::TrinityJackPlayApp, crate::viewer::jack::TrinityJackViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_rewriting_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::rewriting::TrinityRewritingViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_rewriting_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::rewriting::TrinityRewritingPlayApp, crate::viewer::rewriting::TrinityRewritingViewer>();
    }
}
//#endregion 🧪️SurfaceTests
