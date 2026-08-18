//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. Atomic cutover (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): `.declare_artifact(…)` (new declaration
/// tree) replaces the old `.artifact_kind(…)`/`.artifact(…)`/`.editor::<>()`/`.viewer::<>()`
/// channel outright — the old channel is NOT kept alongside it (a second parallel registration
/// channel is the compatibility layer this ticket forbids). `crate::artifacts::note::artifact_kind()`
/// itself is UNCHANGED and stays live: it is still read by `crate::editor::note::create_note_app`'s
/// own `Editor::builder(…).artifact_kind(…)` manifest stitch (a different builder entirely) and by
/// the `.activation(…)` call below — only the `PluginBuilder::artifact_kind(…)` CALL is removed
/// (`📓️w4-sequence-report.md`'s identical precedent). `.editor_mutation_roster()`/
/// `.viewer_mutation_roster()` stay: they are an orthogonal, still-supported opt-in
/// (`contributor.list-artifact-mutations`) the new declaration tree's `SurfaceDeclaration.mutation_roster`
/// field does not yet wire live (`📓️w1-c-report.md` openQuestion 3) — not a second registration of
/// the artifact/schema/io itself. `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME E2, `📓️design-abi.md` §3) are this crate's
/// proof-of-migration: the host activates one instance whenever a `"2d.note"` artifact
/// (`crate::artifacts::note::artifact_kind().id`) is opened, this plugin's actor runs `Isolated`
/// (no publisher trust assumed beyond the sandbox default), and it asks the broker for document
/// write access to persist edits.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("note")
        .label("Note")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::note::artifact())
        .editor_mutation_roster::<crate::editor::note::NotePlayApp>()
        .viewer_mutation_roster::<crate::viewer::note::NoteViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::note::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist note edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn note_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::note::NoteViewer>();
    }

    #[test]
    fn note_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::note::NotePlayApp, crate::viewer::note::NoteViewer>();
    }
}
//#endregion 🧪️SurfaceTests
