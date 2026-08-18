//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(register_exports)`
/// escape hatch; `.setup()` itself is gone (W1c) — `ImperativePlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `.editor(…)` below.
/// `.editor(…)`/`.viewer(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the
/// old single `.document_app(…)` call — one surface per role, both bound to the same
/// `crate::artifacts::imperative::IMPERATIVE_DIALECT`. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M4, `📓️design-abi.md`
/// §5/§6) are this crate's proof-of-migration: the host activates one instance whenever a
/// `"computation.imperative"` artifact (`crate::artifacts::imperative::artifact_kind().id`) is
/// opened, this plugin's own actor runs `Isolated` (its 5 `🧩️extensions/` run `Linked` instead —
/// see each extension's own `bundle()`), and it asks the broker for document write access to
/// persist edits.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("imperative")
        .label("Imperative")
        .version("0.1.0")
        .artifact(crate::artifacts::imperative::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::imperative::ImperativePlayApp>(crate::editor::imperative::create_imperative_app())
        .editor_mutation_roster::<crate::editor::imperative::ImperativePlayApp>()
        .viewer::<crate::viewer::imperative::ImperativeViewer>(crate::viewer::imperative::create_imperative_viewer())
        .viewer_mutation_roster::<crate::viewer::imperative::ImperativeViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::imperative::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist imperative graph edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️Tests
/// 🧪️ Contract §2.5 surface-testkit assertions, canonical versions (framework SDK, w0-f gap 2
/// closure) — no local stand-ins needed, unlike the `📐️cad` pilot which predated their landing.
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn imperative_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::imperative::ImperativeViewer>();
    }

    #[test]
    fn imperative_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::imperative::ImperativePlayApp, crate::viewer::imperative::ImperativeViewer>();
    }
}
//#endregion 🧪️Tests
