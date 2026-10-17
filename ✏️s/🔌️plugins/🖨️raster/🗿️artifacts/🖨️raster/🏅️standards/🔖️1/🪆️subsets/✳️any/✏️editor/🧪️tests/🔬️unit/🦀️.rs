pub mod context {
    //! 🧪️ Shared harness for every `editor::raster` node's tests — mirrors TEMPLATE.md §7.
    use super::*;
    use semio_framework_plugin::{artifact_app_laws as artifact_laws, InvocationResult, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    pub type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>;

    use semio_framework_plugin::PluginApp;

    /// 🧹️ A live app fixture that CLOSES itself. `VcsArtifactApp`'s `ArtifactStore` owns an
    /// `ArtifactStoreCursorDisposer` whose `Drop` asserts terminal-empty ownership, so a plainly
    /// dropped fixture panics with "artifact store reached Drop without its exact terminal-empty
    /// shallow-shell witness". Dereferences to the app and drains the exact retained close ladder on
    /// the way out — the same law the runtime uses, and the same shape `mounted_app` already had.
    pub struct RasterAppFixture(RasterApp);

    impl std::ops::Deref for RasterAppFixture {
        type Target = RasterApp;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for RasterAppFixture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for RasterAppFixture {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                artifact_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
            }
        }
    }

    /// ✏️ Adapts `create_raster_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry` still expects — framework test context gap.
    pub fn raster_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_raster_app(), examples: Vec::new() }
    }

    semio_framework_plugin::history_edit_acceptance_law!("raster", RasterPlayApp, raster_app_manifest_for_tests, "../..");

    /// 🧪️ The app instance every test builds — registry-backed, because there is no other kind.
    /// `EditorApp<RasterPlayApp>` publishes a `bounded_first_step_tool_proofs!` roster, and
    /// `with_registry_on_bus` joins that roster against the registry's `Migrated` tool ids
    /// (`AppActionRegistry::validate_tool_job_rows`): an empty registry declares none of them, so the
    /// registry-LESS `artifact_app_laws::new_app` fails construction outright with
    /// `interactive-job.catalog-authority … generated_migrated=false, migrated={}`.
    /// 🔌️ Mounted (`bind_instance_id`): `dispatch_typed` refuses `interactive-job.live-instance` for
    /// every typed command until the app carries the live runtime instance the `ActionMeta` names, so
    /// an unmounted fixture can construct and render but never mutate.
    pub async fn app() -> RasterAppFixture {
        let mut app = artifact_laws::new_app_with_registry::<EditorApp<RasterPlayApp>>(raster_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        app.bind_instance_id(artifact_laws::meta("local").instance_id).await;
        RasterAppFixture(app)
    }

    /// 🪟️ The view state the runtime always carries: the composite window INSTANCE the action is
    /// dispatched in. Without it the app publishes no window-scoped chrome and every measure
    /// projection comes back empty.
    pub fn raster_view_state() -> ViewModel {
        let id = "raster-composite";
        ViewModel {
            window_id: Some(id.into()),
            window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_WINDOW_COMPOSITE.into() }],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        }
    }

    /// 🔌️ A MOUNTED app (`bind_instance_id`) QUEUES every typed command: the dispatch only admits it
    /// and nothing reaches the document until the host continuation settles it. A test that stopped at
    /// `dispatch_typed` read an unchanged document, which made every action test below look like a
    /// silent no-op.
    pub async fn dispatch(app: &mut RasterApp, command: RasterCommand) -> InvocationResult {
        let mut action = artifact_laws::meta("local");
        action.view_state = Some(raster_view_state());
        let mut result = app.dispatch_typed(command, &action, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("dispatch");
        let settled = artifact_laws::settle_registered_typed_operation(app, artifact_laws::meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        result
    }
    
    /// ⏪️ One framework-reserved history verb driven the whole way a shell drives it: the admission
    /// comes back as a `SpawnJob` receipt the caller must commit before the store ever sees
    /// `ArtifactCommand::Undo`, and only then does the typed publication settle. Pressing it with a
    /// bare `handle_action` reads back as a silent no-op on a MOUNTED app.
    pub async fn history(app: &mut RasterApp, action: &str) {
        artifact_laws::settle_history_verb(app, action, artifact_laws::meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    }

    pub async fn render(app: &mut RasterApp, body_key: &str) -> String {
        render_with_view(app, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await
    }
    
    pub async fn render_with_view(app: &mut RasterApp, body_key: &str, view_state: &ViewModel) -> String {
        let tree = app.render(body_key, None, view_state).await.expect("render");
        artifact_laws::project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
    }
    
    pub async fn main_window_measures(app: &mut RasterApp) -> Vec<WindowMeasure> {
        app.window_measures(&raster_view_state()).await.remove(semio_s_artifact_raster_raster_editor::modes::edit::windows::composite::RASTER_PLAY_WINDOW_COMPOSITE).unwrap_or_default()
    }
    
    /// 🧹️ A document envelope a test builds only to print a pack is a TERMINAL SHELL: every nested
    /// owner below it (its `ArtifactVcs`, its history ledger, its initial snapshot — which for a
    /// raster document owns a populated asset pool) asserts in `Drop` that the bounded protocol ran
    /// first, and only a store ever runs it. The artifact's own owner catalog retires it instead.
    pub fn retire_raster_envelope(envelope: store::os_store::store_schema::ArtifactEnvelope<RasterSnapshot, RasterMutation>) {
        semio_s_artifact_raster_raster_model::close_raster_owner_cold(semio_s_artifact_raster_raster_host::raster_envelope_decode_owner_bundle().retire_envelope(envelope));
    }

    pub async fn semio_app() -> RasterAppFixture {
        let mut app = app().await;
        let document = semio_s_artifact_raster_raster_io::testing::raster_image_test_snapshot();
        let envelope = store::os_store::document_materialize::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", document, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let files = store::os_store::text_format::print_document_pack(&envelope).await.expect("print document pack");
        retire_raster_envelope(envelope);
        semio_framework_plugin::artifact_app_laws::load_document(&mut app, &files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("load semio");
        app
    }
}

use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::WindowMeasure;
use context::*;
use semio_s_artifact_raster_raster_editor::*;
use semio_s_artifact_raster_raster_editor::config::RasterConfig;
use semio_s_artifact_raster_raster_editor::modes::edit::windows::composite;
use semio_s_artifact_raster_raster_model::RasterLayerNode;
use semio_framework_pack_json::Value;
use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::UtilityCategory;
use semio_s_artifact_raster_raster_editor::panels::{catalogue, document, inspection, masks};
use semio_s_artifact_raster_raster_model::{layer_name, layer_visible};
use semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_document;
use semio_framework_plugin::{artifact_app_laws, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
use store::os_store::backbone::MemoryBackbone;

include!("📥️envelope-and-scene-laws/🦀️.rs");

include!("🧭️layer-and-camera-laws/🦀️.rs");

include!("🔁️wire-and-action-bridge-laws/🦀️.rs");

include!("🏔️mounted-boot-laws/🦀️.rs");
