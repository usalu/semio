use super::{store,protocol};
use super::jack;
use jack::standards::v1::subsets::any::schema::composition::JackComposition;
use jack::standards::v1::subsets::any::schema::memory::Graph;
use jack::standards::v1::subsets::any::schema::snapshot::{JackSnapshot, empty_trinity_graph_snapshot, jack_working_scene};
use jack::standards::v1::subsets::any::schema::content::{JackWorkingScene, drive_retirement_to_terminal, retire_owned_to_terminal, trinity_mounted_owner_policy};
use semio_framework_graph_manifest::PropertyValue;
use jack::standards::v1::subsets::any::schema::graph::{TRINITY_GRAPH_SCHEMA, TRINITY_JACK_DEFAULT_QUERY};
use crate::content::*;
use crate::SemioJackComposition;
use crate::jack::editor::jack::*;
use crate::jack::editor::jack::transient::JackResultsWindowTransientOwner;
use protocol::{OpBinary, OpText};
use semio_framework_plugin::{artifact_app_laws, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel};

/// 🎫️ `artifact_app_laws::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`'s own signature
/// is still `fn(manifest: fn() -> App)`, unchanged for this ticket (SDK gap, see this packet's
/// notes file) — `create_trinity_jack_app` now returns a bare `AppDefinition`, so this tiny local
/// wrapper adapts it back into the `App { definition, examples }` shape those test context fns expect.
fn trinity_jack_manifest_for_tests() -> App {
    App { definition: create_trinity_jack_app(), examples: Vec::new() }
}

/// 🎫️ Permanent wire guard (TEMPLATE.md §7): every `TrinityJackCommand` variant round-trips
/// through both its binary (`OpBinary`, via `#[derive(dsl::DslOps)]`) and text (`OpText`) codecs.
#[semio_framework_async_macros::async_test]
async fn trinity_jack_command_text_and_binary_round_trip() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let commands = vec![
        TrinityJackCommand::LoadDocumentJson { json: "{}".into() },
        TrinityJackCommand::DeleteSelection,
        TrinityJackCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
        TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a".into()), results_window_id: "results".into() },
        TrinityJackCommand::RunQuery { query: None, results_window_id: "results".into() },
        TrinityJackCommand::LoadExampleQuery { query: "MATCH (a:Piece) RETURN a.name".into(), results_window_id: "results".into() },
        TrinityJackCommand::FormatDocument,
        TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() },
        TrinityJackCommand::SetViewport { surface_id: Some("trinity.jack.play".into()), viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 1.0 } },
        TrinityJackCommand::TextSelect { start: 3, end: 9 },
        TrinityJackCommand::SetLodMode { value: "compact".into() },
    ];
    for command in commands {
        let bytes = command.encode_op().expect("encode");
        assert_eq!(TrinityJackCommand::decode_op(&bytes).expect("decode"), command);
        let text = command.print_op();
        assert_eq!(TrinityJackCommand::parse_op(&text).expect("parse"), command);
    }

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
    artifact_app_laws::meta(actor)
}

fn query_windows() -> ViewModel {
    ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: "editor-main".into(), window_kind_id: "trinity-jack-editor".into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-main".into(), window_kind_id: "trinity-jack-results".into() },
        ],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

/// 🕹️ Registry-backed (not the bare `artifact_app_laws::new_app`): `interactionSelect`/`interactionHover`
/// resolve the dispatching app's declared `AppActionRegistry.interactions`, so any test exercising
/// domain "ast" selection needs the real manifest's `.interaction(...)` declaration present.
/// 🧩️ Members-aware (`SemioMembers`, not `NoMembers`): the snapshot composes the `s.stdio.semio` content
/// child `genesis_child_pack` derives, which a `NoMembers` roster can never open.
/// 🔌️ Mounted (`bind_instance_id`): `dispatch_typed` refuses `interactive-job.live-instance` for an
/// unbound app, and the mounted app answers BEFORE its retained typed operation publishes, so a
/// dispatching test settles through `settle(&mut app)` before reading the projection.
/// 🔚️ The law forwards its original caller through paid terminal app close; the guard checks completion.
async fn new_app() -> JackTestApp {
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>(trinity_jack_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()),jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy(),&mut artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(JACK_TEST_INSTANCE).await;
    JackTestApp { app, closed: false }
}

const JACK_TEST_INSTANCE: u32 = 1;
const JACK_LIVE_LOAD_STEP_BYTES: usize = 32 * 1_024;

pub(crate) struct JackTestApp {
    app: VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>,
    closed: bool,
}

impl JackTestApp {
    fn close(&mut self,cx:&mut semio_framework_job::StepContext<'_>) {
        if !self.closed {
            self.closed = true;
            artifact_app_laws::close_registered_fixture_app(&mut self.app, jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy(),cx).expect("original caller paid app close");
        }
    }
}

impl std::ops::Deref for JackTestApp {
    type Target = VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl std::ops::DerefMut for JackTestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl Drop for JackTestApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.closed,"fixture app requires original caller paid closure");
        }
    }
}

/// 📬️ Drives the mounted app's retained typed operation to quiescence (pages ACKed, effects, events
/// and completions drained) — the receipt carries what the host would have seen.
async fn settle(app: &mut JackTestApp,cx:&mut semio_framework_job::StepContext<'_>) -> artifact_app_laws::TypedOperationFixtureReceipt {
    artifact_app_laws::settle_registered_typed_operation(&mut app.app, JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx).await.expect("settle the typed operation")
}

#[path = "🔌️live-load-envelope/🦀️.rs"]
mod live_load_envelope;
use live_load_envelope::*;

#[path = "🖼️scene-rendering-and-text-edit/🦀️.rs"]
mod scene_rendering_and_text_edit;
use scene_rendering_and_text_edit::*;

#[path = "🔭️lod-catalogue-and-examples/🦀️.rs"]
mod lod_catalogue_and_examples;

#[path = "🔎️query-ownership-and-examples/🦀️.rs"]
mod query_ownership_and_examples;
use query_ownership_and_examples::*;

#[path = "🩹️rail-verb-laws/🦀️.rs"]
mod rail_verb_laws;
use rail_verb_laws::*;

#[path = "🤖️agent-lane-and-archive-laws/🦀️.rs"]
mod agent_lane_and_archive_laws;
