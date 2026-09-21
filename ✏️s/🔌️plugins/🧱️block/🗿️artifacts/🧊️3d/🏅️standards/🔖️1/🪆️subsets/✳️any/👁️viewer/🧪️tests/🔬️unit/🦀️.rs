use super::*;

#[semio_framework_async_macros::async_test]
async fn create_block3d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_block3d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, BLOCK3D_DIALECT.into());
    assert_eq!(def.breadcrumb, vec!["semio", "block", "3d"]);
    // 🪪️ `AppDefinition` names this path `breadcrumb` (camelCase on the wire); the field was never
    // called `document`, so reading `descriptor["document"]` asserted `Null == Null` would have
    // been vacuous and asserting it against the path is what this ever meant to prove.
    let descriptor = serde_json::to_value(&def).expect("language-neutral app descriptor");
    assert_eq!(descriptor["breadcrumb"], serde_json::json!(["semio", "block", "3d"]));
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Block3dViewer as ArtifactViewer>::DIALECT, BLOCK3D_DIALECT);
}

/// ⚖️ LAW: the viewer boots non-empty — it can never load an example itself, so an empty initial
/// snapshot would leave its `World3d` window blank forever.
/// 👁️ Viewer test harness — the read-only twin of the sibling surface's `context`, importing nothing
/// from it (`policyViewerPurityBreaches`).
///
/// 🪪️ The registryless `artifact_app_laws::new_app` this file used to call mounts NO action registry
/// and binds NO live instance, so every typed command it admitted was refused with
/// `interactive-job.missing-factory` ("typed command 'typed-command' has no exact controller/owner/
/// factory/tool/schema proof") and its `ArtifactStore` reached `Drop` without a close ladder having
/// ever run. The production adapter `VcsArtifactApp<ViewerApp<Block3dViewer>>` under the app's own
/// manifest is what the runtime mounts, so it is what these laws run against.
mod context {
    use super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{PluginApp, VcsArtifactApp, ViewerApp};

    pub type Block3dViewerHarness = VcsArtifactApp<ViewerApp<Block3dViewer>>;

    pub fn block3d_viewer_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_block3d_viewer(), examples: Vec::new() }
    }

    /// 🧹️ A live viewer fixture that CLOSES itself: the document store's `Drop` asserts its exact
    /// terminal-empty witness, so a plainly dropped harness aborts the whole test binary.
    pub struct Block3dViewerFixture(Block3dViewerHarness);

    impl std::ops::Deref for Block3dViewerFixture {
        type Target = Block3dViewerHarness;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for Block3dViewerFixture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for Block3dViewerFixture {
        fn drop(&mut self) {
            if std::thread::panicking() {
                return;
            }
            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
            assert!(self.0.close_terminal_is_empty(), "Block3d viewer fixture did not reach its terminal-empty close witness");
        }
    }

    pub async fn app() -> Block3dViewerFixture {
        let mut app = new_app_with_registry::<ViewerApp<Block3dViewer>>(block3d_viewer_manifest_for_tests).await;
        app.bind_instance_id(meta("local").instance_id).await;
        Block3dViewerFixture(app)
    }
}

/// ⚖️ LAW: the viewer boots non-empty — it can never load an example itself, so an empty initial
/// snapshot would leave its `World3d` window blank forever.
#[semio_framework_async_macros::async_test]
async fn viewer_boots_with_at_least_one_representation() {
    let app = context::app().await;
    let snapshot = app.snapshot().expect("snapshot");
    assert!(!snapshot.representations.is_empty(), "the viewer must boot with a renderable document");
    assert!(snapshot.representations.iter().all(|representation| representation.mesh_url.is_some()));
}

/// ⚖️ LAW: nothing a viewer session can dispatch reaches its document.
///
/// 🪪️ Asserted through the production adapter: a viewer declares NO actions at all, so its manifest
/// carries no command declaration and the live registry REFUSES the typed channel outright
/// (`interactive-job.unknown-key`, "no exact manifest declaration"). That refusal is the law — it is
/// the structural reason a read-only surface can never edit — and the document must come through it
/// untouched. The command's own wire form round-trips, so the refusal is about the manifest, not
/// about a codec.
#[semio_framework_async_macros::async_test]
async fn noop_command_round_trips_and_never_mutates() {
    let mut app = context::app().await;
    let before = app.snapshot().expect("snapshot");
    assert_eq!(protocol::OpBinary::decode_op(&protocol::OpBinary::encode_op(&Block3dViewCommand::Noop).expect("encode")).map(|command: Block3dViewCommand| command), Ok(Block3dViewCommand::Noop), "the sole command round-trips through its own wire form");
    let refused = app.dispatch_typed(Block3dViewCommand::Noop, &semio_framework_plugin::artifact_app_laws::meta("local")).await;
    assert!(refused.is_err(), "a viewer declares no command, so the typed channel must refuse it instead of routing it: {refused:?}");
    assert_eq!(app.snapshot().expect("snapshot"), before, "the viewer's document must come through the refusal untouched");
}
