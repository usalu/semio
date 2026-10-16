//! 🕹️ Puzzle3d editor surface laws — action roster, local interaction reads, operations, command envelopes, inspector, manifest, suggestions and the paged scene carrier.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_editor::*;
use semio_s_artifact_puzzle_3d_retained_action::{PUZZLE3D_RETAINED_PAYLOAD_SCHEMA, Puzzle3dAcceptSuggestionWork, Puzzle3dAddBrushObjectWork, Puzzle3dFocusSelectionWork, Puzzle3dInstanceBinding, Puzzle3dPrecomputeCommandWork, Puzzle3dTransformWork, Puzzle3dWindowCommandWork, puzzle3d_retained_extent, puzzle3d_retained_reduce};
use {semio_s_artifact_puzzle_3d_retained_document::Puzzle3dAddObjectKindWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dCreateAttractionWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dKindWeightWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dPatchInspectorWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dSetActiveExampleWork};
use semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action;
use {semio_s_artifact_puzzle_3d_editor_session::PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_view_session_key};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_DEFAULT_UTILITY, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_KIND, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_HOVER_CHANNEL, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_entity_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_fault_notices, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_action, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_argument, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_kind_ids, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_paste_operations, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_active_utility, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_play_snapshot, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_selected_objects_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_upsert_catalog_rows, semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot};
use semio_s_artifact_puzzle_3d_commands_document::world_relocate;
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime};
use semio_s_artifact_puzzle_3d_editor_modes as edit;
use semio_s_artifact_puzzle_3d_editor_modes::tools::fill as fill_tool;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use semio_s_artifact_puzzle_3d_editor_panels::{catalogue, artifact, inspection, settings as settings_panel};
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsOwner;
use semio_s_artifact_puzzle_3d_editor_presence as presence;
use semio_s_artifact_puzzle_3d_editor_presence::{Puzzle3dPresence, Puzzle3dPresenceMutation};
use semio_s_artifact_puzzle_3d_editor_terminology::{puzzle3d_labels, puzzle3d_localized, puzzle3d_localized_phrase, Puzzle3dLabels};
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, StepContext};
use semio_framework_plugin::panel_tab_element_id;
use semio_framework_plugin::panel_tab_first_draggable_element_id;
use semio_framework_plugin::window_element_id;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractionWrite;
use semio_framework::onboarding::IntroductionDefinition;
use semio_framework::onboarding::IntroductionInteraction;
use semio_framework::onboarding::IntroductionPlacement;
use semio_framework::onboarding::IntroductionStepDefinition;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_2d::compute::EngineHandles;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use semio_framework_plugin::app::{EphemeralEmit, InteractionView};
use std::collections::{HashMap, HashSet};
use std::collections::{BTreeMap};
use semio_s_artifact_puzzle_3d_commands_document::delete_selection;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
#[path = "../🤖️agent-lane/🦀️.rs"]
mod agent_lane_tests;
#[path = "../🧵️retained-wiring/🦀️.rs"]
mod retained_wiring_tests;
#[path = "../../🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
mod modes_layout;
use context::*;
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::PluginApp;

//#region 🕹️ActionRoster
/// 🕹️ Every action DISPATCHABLE in this app, once per id, in declaration order.
///
/// 🪪️ App-level actions are no longer CLONED onto every window kind (that made a package descriptor
/// grow as `apps × window kinds × actions`): `semio_framework::window_kind_actions` resolves a
/// window's own roster plus every app-level row no window claims, at read time. So
/// `WindowKindDefinition::actions` alone now holds only the window's OWN declarations — for this app,
/// the framework-injected interaction verbs — and a law that reads it sees none of the authored
/// vocabulary. This is the roster the shell offers and `dispatch_action` routes.

/// 🪪️ Every DECLARATION of one action id across the manifest — the app-level roster plus every window
/// kind's own roster. A verb declared once app-wide and resolved into N windows is still exactly one
/// declaration, which is what the bijection laws mean by "exactly one manifest declaration".
fn action_declarations<'a>(definition: &'a semio_framework_plugin::AppDefinition, id: &str) -> Vec<&'a ActionDefinition> {
    definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).filter(|action| action.id == id).collect()
}
//#endregion 🕹️ActionRoster

//#region 🕹️LocalInteractionRead
/// 🚧️ Runaway guard for the host's own continuation drain. Far above the host's real 4096-turn
/// budget so a law that fails here fails on the state machine, never on the guard.
const LOCAL_INTERACTION_READ_TURNS: usize = 65_536;

/// 📤️ The exact frame admission `advance_typed_operation_output` hands
/// `publish_local_interaction_query_reply` every continuation turn.
const LOCAL_INTERACTION_READ_FRAMES: usize = 4;

/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: plays the HOST half of one local-interaction
/// read exactly as `plugin_continue_typed_operations` → `advance_typed_operation_output` does —
/// one `advance_typed_operation_publication` unit, then `publish_local_interaction_query_reply`
/// into the same fixed frame admission — and stops draining the moment
/// `has_runnable_typed_operations` reports quiescence. That quiescence point is the ONLY place a
/// page acknowledgement can arrive: the shell's `settlePluginTurn(drainOperations=true)` owns the
/// call until the actor stops reporting more-work, so a query that stays runnable while it waits
/// on the host never sees its own ACK. Returns the exact capture bytes the shell assembles.
async fn read_local_interaction(app: &mut Puzzle3dApp, request_id: u64) -> Vec<u8> {
    read_local_interaction_while_rendering(app, request_id, false).await
}

/// 🖼️ The same host loop with the shell's own render pressure optionally interleaved, because a boot
/// shell renders on every turn while it waits for its surfaces — so a read must terminate while the
/// app is also doing its ordinary per-turn work, not only on an otherwise idle instance.
async fn read_local_interaction_while_rendering(app: &mut Puzzle3dApp, request_id: u64, render: bool) -> Vec<u8> {
    use semio_framework_plugin::PluginApp;
    assert!(app.begin_local_interaction_query(request_id, request_id).is_none(), "a live instance must admit local interaction read {request_id}");
    let (mut capture, mut started, mut closed) = (Vec::new(), false, false);
    let mut awaiting_ack: Option<protocol::LocalInteractionQueryToken> = None;
    let mut turns = 0_usize;
    for _ in 0..LOCAL_INTERACTION_READ_TURNS {
        turns += 1;
        if !app.has_runnable_typed_operations() {
            let Some(token) = awaiting_ack.take() else { break };
            assert!(app.acknowledge_local_interaction_query(&token), "the page the app itself published must accept its own exact token");
            continue;
        }
        if render {
            drop(render_composite(app).await);
        }
        app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("the live-cleanup clock keeps ticking during a read");
        app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.expect("advance one local interaction publication unit");
        let mut frames = Vec::new();
        app.publish_local_interaction_query_reply(&mut frames, LOCAL_INTERACTION_READ_FRAMES);
        for frame in &frames {
            let protocol::AppFrame::LocalInteractionQuery { reply } = protocol::decode_app_frame(frame).await.expect("a published local interaction frame decodes") else {
                panic!("local interaction publication emitted a foreign app frame");
            };
            match reply {
                protocol::LocalInteractionQueryReply::Started { .. } => started = true,
                protocol::LocalInteractionQueryReply::Page { page } => {
                    assert!(awaiting_ack.is_none(), "a second page was published while the first still awaited its acknowledgement");
                    capture.extend_from_slice(&page.bytes);
                    awaiting_ack = Some(protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal });
                }
                protocol::LocalInteractionQueryReply::Closed { cancelled, .. } => {
                    assert!(!cancelled, "an acknowledged read closes uncancelled");
                    closed = true;
                }
                other => panic!("unexpected local interaction reply: {other:?}"),
            }
        }
        if closed {
            break;
        }
    }
    eprintln!("local interaction read {request_id} turns={turns} bytes={}", capture.len());
    assert!(started, "local interaction read {request_id} never published its Started reply");
    assert!(closed, "local interaction read {request_id} never terminated: the actor stayed runnable without ever publishing Closed, or quiesced with nothing left for the host to answer");
    capture
}

/// 🩺️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: reproduces the browser boot stall — the shell's
/// very first `readLocalInteraction` over a document with NO selection, driven through the exact
/// host continuation loop. The actor must reach `Closed` and hand its three captured roots back,
/// and it must never stay runnable while the only thing it waits for is a host acknowledgement the
/// drain cannot deliver.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_of_an_unselected_document_terminates() {
    let mut app = app().await;
    let capture = read_local_interaction(&mut app, 1).await;
    let capture: protocol::LocalInteractionCapture = semio_framework_pack_json::from_json_str(std::str::from_utf8(&capture).expect("a capture is canonical UTF-8 JSON"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("a terminated read yields a whole capture");
    assert!(capture.state.selection.values().all(|selection| selection.ids.is_empty()), "this law's document carries no selection: {:?}", capture.state.selection);
}

/// 🩺️ The boot shape: the shell renders while it waits for its UI surfaces, so the read runs against
/// a busy instance rather than an idle one. A terminal reply withheld while unrelated app-owned
/// maintenance is in flight — while the query keeps reporting runnable — is the livelock the boot
/// hit, and it cannot be seen on an instance that does nothing else.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_terminates_under_concurrent_render_pressure() {
    let mut app = app().await;
    read_local_interaction_while_rendering(&mut app, 4, true).await;
}

/// 🩺️ The shell re-reads the local interaction on every refresh, so the SECOND and third reads of
/// one live instance must terminate exactly like the first — a store lease registry that still
/// reports a reclaimed slot, or a live slot that outlives its own terminal reply, strands them.
#[semio_framework_async_macros::async_test]
async fn repeated_local_interaction_reads_of_one_instance_terminate() {
    let mut app = app().await;
    for request_id in 1..=3 {
        read_local_interaction(&mut app, request_id).await;
    }
}

/// 🩺️ The same read over a LIVE selection: a non-empty capture spans many more fixed pages, so it
/// exercises every intermediate page's acknowledgement round trip as well as the terminal one.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_of_a_selected_document_terminates() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let capture = read_local_interaction(&mut app, 2).await;
    let capture: protocol::LocalInteractionCapture = semio_framework_pack_json::from_json_str(std::str::from_utf8(&capture).expect("a capture is canonical UTF-8 JSON"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("a terminated read yields a whole capture");
    assert!(
        capture.state.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| selection.ids.iter().any(|id| id == &object_id)),
        "the capture carries the live selection: {:?}",
        capture.state.selection
    );
}

/// 🩺️ ticket 26/09/02/PUZZLE-3D-END-TO-END: reproduces `runtime live cleanup faulted for
/// instance 1` — the vortex-picking local interaction query is the only path by which the
/// puzzle3d document store's `snapshot_read()` lease is ever taken and returned, so it is
/// driven to completion here (Started → pages → acknowledge → Closed) exactly as the
/// host does every actor turn, then one plain `maintenance_step` must not fault.
#[semio_framework_async_macros::async_test]
async fn local_interaction_query_return_does_not_fault_the_next_maintenance_step() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    read_local_interaction(&mut app, 1).await;
    semio_s_plugin_puzzle_jobs::testing::maintain(&mut *app, 4096).expect("maintenance step after a returned snapshot read lease must not fault");
}
//#endregion 🕹️LocalInteractionRead

/// 🔁️ Full round-robin sweeps driven after the example lands, so every fixed stage runs its own
/// bounded unit several times (first unit does the real work, later ones prove it stays terminal).
const MAINTENANCE_UNIT_SWEEPS: usize = 8;

/// 🚧️ Runaway guard for the measured host-turn loop; the flagship example loads need thousands of
/// publication turns, so this only has to be far above them, never tight.
const MAINTENANCE_BUDGET_TURNS: usize = 1_048_576;

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave R2: the OS runtime's cooperative-maintenance clock
/// (`RuntimeLiveCleanupJob::step` → `maintenance_step(1, 4096)`) faults its instance with
/// `plugin.internal.interactive-ceiling` the moment one unit overruns
/// `semio_framework_trace::INTERACTIVE_STEP_CEILING_US`, which is exactly how the browser boot died. A unit stays inside the
/// ceiling because it is bounded by its grant — one item and [`RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP`] bytes. This law drives
/// the REAL app — the real typed `setActiveExample` command, the real store, the real store-replacement/envelope registries
/// and the real fixed round robin — over both flagship documents and holds every unit of every stage to its grant, naming
/// the offending stage when it does not. The bound is a count, so the verdict is the same on an idle and a saturated
/// machine. The measured turn deliberately does NOT assert the operation's own result lane: a domain command that faults is
/// a different law's subject, while the clock must stay inside its grant either way.
#[semio_framework_async_macros::async_test]
async fn every_maintenance_unit_stays_inside_the_interactive_step_budget() {
    for example in [PUZZLE3D_EXAMPLE_CONCRETE_FOREST, PUZZLE3D_EXAMPLE_NAKAGIN] {
        let mut app = app().await;
        let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": example })), None).expect("setActiveExample is a declared puzzle3d command");
        let action_meta = app.meta("local");
        app.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("setActiveExample mints its retained whole-document operation");
        for _ in 0..MAINTENANCE_BUDGET_TURNS {
            if !app.has_pending_typed_operations() {
                break;
            }
            measured_host_turn(&mut app).await;
        }
        for _ in 0..MAINTENANCE_UNIT_SWEEPS * semio_framework_plugin::MAINTENANCE_STAGES as usize {
            app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("a live-cleanup maintenance unit never faults");
        }
        assert!(app.maintenance.units() > 0, "{example}: the law drove no maintenance unit");
        assert_eq!(app.maintenance.first_overrun(), None, "{example}: a maintenance unit released more than its grant as (stage, items, bytes, granted items, granted bytes) (per-stage worstItems/worstBytes/units: {})", app.maintenance.report());
    }
}

/// 🔁️ One host actor turn with its cooperative-maintenance unit measured: the same
/// `maintenance_step` → `advance_typed_operation_publication` → present/ACK → drain shape the
/// runtime drives, minus any assertion on the operation's own outcome lane.
async fn measured_host_turn(app: &mut Puzzle3dApp) {
    app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("a live-cleanup maintenance unit never faults");
    app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.expect("advance one typed operation publication unit");
    let instance_id = app.instance_id;
    if let Some(page) = app.take_typed_operation_result_page(instance_id) {
        assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge one presented result page"), "the app's own presented result page must accept its exact token");
    }
    drop(app.take_typed_operation_effect());
    drop(app.take_typed_operation_event());
    drop(app.take_typed_operation_completion().await.expect("take one typed operation completion witness"));
    drop(app.take_typed_operation_ui_scope());
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            assert!(app.acknowledge_local_interaction_query(&token), "the app's own local-interaction page must accept its exact token");
        }
    }
}

#[test]
fn retained_publication_contracts_are_an_exact_nonempty_tool_bijection() {
    let scene_snapshot: Value = parse(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json")).expect("committed test data reads").as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Puzzle3D retained route scene_snapshot");
    assert_eq!(scene_snapshot.get("toolIds"), Some(&Value::Array(PUZZLE3D_RETAINED_TOOL_IDS.iter().map(|id| Value::from(*id)).collect())));
    let manifest = create_puzzle3d_app();
    for tool_id in PUZZLE3D_RETAINED_TOOL_IDS {
        let actions = action_declarations(&manifest, tool_id);
        assert_eq!(actions.len(), 1, "{tool_id} requires exactly one manifest declaration");
        assert_eq!(actions[0].semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{tool_id}");
    }
    let exact = |contracts: &[ArtifactToolPublicationContract]| {
        let ids = contracts.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>();
        ids == PUZZLE3D_RETAINED_TOOL_IDS.iter().copied().collect()
            && ids.len() == contracts.len()
            && contracts.iter().all(|contract| !contract.lanes.is_empty() && (!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1))
    };
    let contracts = <Puzzle3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert!(exact(contracts));
    assert!(!exact(&contracts[..contracts.len() - 1]));
    let mut duplicate = contracts.to_vec();
    let copied = duplicate[1];
    duplicate[0] = copied;
    assert!(!exact(&duplicate));
}

#[test]
fn retained_command_catalog_excludes_framework_owned_shared_actions() {
    assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_TOOL_ACTION_ID));
    assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
}

fn suggestion_and_precompute_routes_are_cursorized(source: &str) -> bool {
    [
        r#""acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default())"#,
        r#"| "targetBrushSuggestions""#,
        "=> Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))",
        "Puzzle3dAcceptSuggestionStage::Target",
        "Puzzle3dAcceptSuggestionStage::Candidate",
        "Puzzle3dAcceptSuggestionStage::Vortices",
        "Puzzle3dAcceptSuggestionStage::ExistingAttractions",
        "Puzzle3dAcceptSuggestionStage::PublishObject",
        "Puzzle3dAcceptSuggestionStage::PublishAttraction",
        "Puzzle3dPrecomputeCommandStage::Objects",
        "Puzzle3dPrecomputeCommandStage::Vortices",
        "Puzzle3dPrecomputeCommandStage::Attractions",
        "Puzzle3dPrecomputeCommandStage::CatalogObjects",
        "Puzzle3dPrecomputeCommandStage::CatalogVortices",
        "Puzzle3dPrecomputeCommandStage::Positions",
        "Puzzle3dPrecomputeCommandStage::Indices",
        "PUZZLE3D_MESH_PAGE_SCAN_CHARS",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains(r#""acceptSuggestion" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""cycleBrushCandidate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""fillBuildTick" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""registerBrushMesh" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""setFillCount" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""targetBrushSuggestions" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn suggestion_and_precompute_hostile_static_law_rejects_one_grant_reducers_and_missing_boundaries() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(suggestion_and_precompute_routes_are_cursorized(source));
    let direct_accept = source.replace(
        r#""acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default())"#,
        r#""acceptSuggestion" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!suggestion_and_precompute_routes_are_cursorized(&direct_accept));
    let direct_precompute = source.replace("=> Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))", "=> Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))");
    assert!(!suggestion_and_precompute_routes_are_cursorized(&direct_precompute));
    for marker in [
        "Puzzle3dAcceptSuggestionStage::Target",
        "Puzzle3dAcceptSuggestionStage::Vortices",
        "Puzzle3dAcceptSuggestionStage::ExistingAttractions",
        "Puzzle3dAcceptSuggestionStage::PublishObject",
        "Puzzle3dAcceptSuggestionStage::PublishAttraction",
        "Puzzle3dPrecomputeCommandStage::Objects",
        "Puzzle3dPrecomputeCommandStage::Vortices",
        "Puzzle3dPrecomputeCommandStage::Attractions",
        "Puzzle3dPrecomputeCommandStage::CatalogObjects",
        "Puzzle3dPrecomputeCommandStage::CatalogVortices",
        "Puzzle3dPrecomputeCommandStage::Positions",
        "Puzzle3dPrecomputeCommandStage::Indices",
        "PUZZLE3D_MESH_PAGE_SCAN_CHARS",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ] {
        assert!(!suggestion_and_precompute_routes_are_cursorized(&source.replace(marker, "cursor-removed")), "missing retained boundary was falsely accepted: {marker}");
    }
}

fn selection_transforms_run_the_transform_tool(source: &str) -> bool {
    source.contains(r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => Box::new(Puzzle3dTransformWork::new(tool_id, request.authoring_seed.clone()))"#)
        && source.contains("Puzzle3dTransformStage::Read")
        && source.contains("Puzzle3dTransformStage::Scan")
        && source.contains("Puzzle3dRelocateScan::begin(document")
        && source.contains("Puzzle3dTransformStage::Commit")
        && source.contains("puzzle3d_transform_tool_commit(self.tool_id")
        && !source.contains(r#""translateSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

/// 🛠️ Every selection transform — the three gumball verbs, the target-volume relocate and the Relocate
/// utility — is ONE staged work that states the gesture and commits it through the transform tool: a
/// one-grant reducer, a missing stage or a bypassed tool fails closed.
#[test]
fn selection_transform_hostile_static_law_rejects_one_grant_reducers_and_bypassed_tools() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(selection_transforms_run_the_transform_tool(source));
    let direct = source.replace(
        r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => Box::new(Puzzle3dTransformWork::new(tool_id, request.authoring_seed.clone()))"#,
        r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!selection_transforms_run_the_transform_tool(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["Puzzle3dTransformStage::Read", "Puzzle3dTransformStage::Scan", "Puzzle3dRelocateScan::begin(document", "Puzzle3dTransformStage::Commit", "puzzle3d_transform_tool_commit(self.tool_id"] {
        assert!(!selection_transforms_run_the_transform_tool(&source.replace(marker, "stage-removed")), "missing transform stage was falsely accepted: {marker}");
    }
}

fn focus_selection_is_cursorized(source: &str) -> bool {
    source.contains(r#""focusSelection" => Box::new(Puzzle3dFocusSelectionWork::default())"#)
        && source.contains("Puzzle3dFocusSelectionStage::Selection")
        && source.contains("Puzzle3dFocusSelectionStage::SumObjects")
        && source.contains("Puzzle3dFocusSelectionStage::DistanceObjects")
        && source.contains("Puzzle3dFocusSelectionStage::Publish")
        && !source.contains(r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn focus_selection_hostile_static_law_rejects_hidden_whole_collection_work() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(focus_selection_is_cursorized(source));
    let direct = source
        .replace(r#""focusSelection" => Box::new(Puzzle3dFocusSelectionWork::default())"#, r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!focus_selection_is_cursorized(&direct));
    for marker in ["Puzzle3dFocusSelectionStage::Selection", "Puzzle3dFocusSelectionStage::SumObjects", "Puzzle3dFocusSelectionStage::DistanceObjects", "Puzzle3dFocusSelectionStage::Publish"] {
        assert!(!focus_selection_is_cursorized(&source.replace(marker, "cursor-removed")), "missing focus cursor was falsely accepted: {marker}");
    }
}

fn patch_inspector_is_cursorized(source: &str) -> bool {
    source.contains(r#""patchInspector" => Box::new(Puzzle3dPatchInspectorWork::default())"#)
        && source.contains("Puzzle3dPatchInspectorStage::Selection")
        && source.contains("Puzzle3dPatchInspectorStage::Objects")
        && source.contains("Puzzle3dPatchInspectorStage::Vortices")
        && source.contains("Puzzle3dPatchInspectorStage::Attractions")
        && source.contains("Puzzle3dPatchInspectorStage::AttractionReconnect")
        && source.contains("Puzzle3dPatchInspectorStage::References")
        && source.contains("Puzzle3dPatchInspectorStage::Volumes")
        && !source.contains(r#""patchInspector" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn patch_inspector_hostile_static_law_rejects_old_reducer_and_hidden_collection_loops() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(patch_inspector_is_cursorized(source));
    let direct = source
        .replace(r#""patchInspector" => Box::new(Puzzle3dPatchInspectorWork::default())"#, r#""patchInspector" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!patch_inspector_is_cursorized(&direct));
    for marker in [
        "Puzzle3dPatchInspectorStage::Selection",
        "Puzzle3dPatchInspectorStage::Objects",
        "Puzzle3dPatchInspectorStage::Vortices",
        "Puzzle3dPatchInspectorStage::Attractions",
        "Puzzle3dPatchInspectorStage::AttractionReconnect",
        "Puzzle3dPatchInspectorStage::References",
        "Puzzle3dPatchInspectorStage::Volumes",
    ] {
        assert!(!patch_inspector_is_cursorized(&source.replace(marker, "cursor-removed")), "missing inspector cursor was falsely accepted: {marker}");
    }
}

fn create_attraction_is_cursorized(source: &str) -> bool {
    source.contains(r#""createAttraction" => Box::new(Puzzle3dCreateAttractionWork::default())"#)
        && source.contains("Puzzle3dCreateAttractionStage::Existing")
        && source.contains("Puzzle3dCreateAttractionStage::Attracting")
        && source.contains("Puzzle3dCreateAttractionStage::Attracted")
        && source.contains("Puzzle3dCreateAttractionStage::Compatibility")
        && source.contains("Puzzle3dCreateAttractionStage::Publish")
        && !source.contains(r#""createAttraction" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn create_attraction_hostile_static_law_rejects_nested_whole_scans() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(create_attraction_is_cursorized(source));
    let direct = source.replace(
        r#""createAttraction" => Box::new(Puzzle3dCreateAttractionWork::default())"#,
        r#""createAttraction" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!create_attraction_is_cursorized(&direct));
    for marker in ["Puzzle3dCreateAttractionStage::Existing", "Puzzle3dCreateAttractionStage::Attracting", "Puzzle3dCreateAttractionStage::Attracted", "Puzzle3dCreateAttractionStage::Compatibility", "Puzzle3dCreateAttractionStage::Publish"] {
        assert!(!create_attraction_is_cursorized(&source.replace(marker, "cursor-removed")), "missing attraction cursor was falsely accepted: {marker}");
    }
}

fn set_active_example_is_cursorized(source: &str) -> bool {
    source.contains(r#""setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default())"#)
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteAttractions")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteObjects")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteVolumes")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteReferences")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteCompatibility")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateObjects")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateAttractions")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateVolumes")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateReferences")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateCompatibility")
        && source.contains("Puzzle3dSetActiveExampleStage::Publish")
        && !source.contains(r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn set_active_example_hostile_static_law_rejects_whole_document_reset() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(set_active_example_is_cursorized(source));
    let direct = source.replace(
        r#""setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default())"#,
        r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!set_active_example_is_cursorized(&direct));
    for marker in [
        "Puzzle3dSetActiveExampleStage::DeleteAttractions",
        "Puzzle3dSetActiveExampleStage::DeleteObjects",
        "Puzzle3dSetActiveExampleStage::DeleteVolumes",
        "Puzzle3dSetActiveExampleStage::DeleteReferences",
        "Puzzle3dSetActiveExampleStage::DeleteCompatibility",
        "Puzzle3dSetActiveExampleStage::CreateObjects",
        "Puzzle3dSetActiveExampleStage::CreateAttractions",
        "Puzzle3dSetActiveExampleStage::CreateVolumes",
        "Puzzle3dSetActiveExampleStage::CreateReferences",
        "Puzzle3dSetActiveExampleStage::CreateCompatibility",
        "Puzzle3dSetActiveExampleStage::Publish",
    ] {
        assert!(!set_active_example_is_cursorized(&source.replace(marker, "cursor-removed")), "missing example cursor was falsely accepted: {marker}");
    }
}

/// 🧮️ The Relocate utility's retained work reads the drop, pages its proximity scan — one step per
/// `PUZZLE3D_RELOCATE_SCAN_PAGE` objects — and commits ONE selection record through the transform tool, so even
/// Nakagin's 180 objects admit it far inside the fixed work cap.
#[test]
fn world_relocate_extent_fits_within_cap_for_nakagin() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "nonexistent", "position": [0.0, 0.0, 0.0] })), None).expect("worldRelocate command decodes");
    let work = Puzzle3dTransformWork::new("worldRelocate", "seed".into());
    let pages = NAKAGIN_EXAMPLE_SNAPSHOT.objects.len().div_ceil(utilities::transform::PUZZLE3D_RELOCATE_SCAN_PAGE);
    assert_eq!(work.extent(&command, &snapshot, &interaction, None), Some(2 + pages), "a relocate reads its gesture, scans one page per step, then commits it");
    assert!(2 + pages <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
}

/// ⏹️ A Nakagin drop's proximity scan is paged and cancellable: every page is its own `Progress` step, and a work
/// closed in the middle of the scan retires to its terminal-empty shell without ever having published a mutation.
#[test]
fn world_relocate_scan_pages_progress_and_cancels_with_zero_trace() {
    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "25b0dba0-8f81-423a-94a1-b911a6031010", "position": [-8.84, -2.8499999999999996, 7.7] })), None).expect("worldRelocate command decodes");
    let mut work = Puzzle3dTransformWork::new("worldRelocate", "seed".into());
    let mut stages = Vec::new();
    for _ in 0..3 {
        match semio_s_plugin_puzzle_retained::testing::step(&mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { stage, en, de } => {
                assert!(!en.is_empty() && !de.is_empty() && en != de, "every progress step is localized in both languages");
                stages.push(stage);
            }
            other => panic!("a 180-object scan cannot finish in three steps: {}", matches!(other, PuzzleCommandWorkStep::Complete(_))),
        }
    }
    assert_eq!(stages, ["puzzle3d-transform-read", "puzzle3d-transform-scan", "puzzle3d-transform-scan"], "the scan reports one progress per page");
    let turns = semio_s_plugin_puzzle_retained::testing::close(&mut work);
    assert!(turns < 8, "closing a scanning relocate retires in a bounded number of turns");
}

/// 🔁️ A real Nakagin drop: object `25b0dba0-8f81-423a-94a1-b911a6031010` ("Capsule With Balcony
/// Backslash") moves one centimetre along x, and its one vortex (`…:link`, kind "door capsule right") still
/// lands within `proximity_radius` (0.75, `default_proximity_radius()`) of the "door tambour right" vortices
/// on neighbour `5f0266bc-…`. The work finishes inside its own extent and commits ONE transaction: the
/// `drag-selection` plus at least one `connect-vortices`, all stamped with the ref it minted.
#[test]
fn world_relocate_step_loop_stays_within_its_own_extent_for_nakagin() {
    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "25b0dba0-8f81-423a-94a1-b911a6031010", "position": [-8.84, -2.8499999999999996, 7.7] })), None).expect("worldRelocate command decodes");
    let mut work = Puzzle3dTransformWork::new("worldRelocate", "seed".into());
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("a relocate always fits the fixed bounded work envelope");
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= extent, "worldRelocate step() ran past its own declared extent {extent}");
        match semio_s_plugin_puzzle_retained::testing::step(&mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(matches!(emit.artifact_mutations.first(), Some(Puzzle3dMutation::DragSelection(_))), "the drop yields its drag first: {:?}", emit.artifact_mutations.first());
    assert!(emit.artifact_mutations.iter().skip(1).any(|mutation| matches!(mutation, Puzzle3dMutation::ConnectVortices(_))), "the relocated vortex sits within proximity_radius of a real neighbour, so the drop attracts it; observed {:?}", emit.artifact_mutations);
    assert!(emit.transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with("#worldRelocate")), "the drop is ONE tool transaction: {:?}", emit.transaction);
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dCreateAttractionWork::extent` doubled the
/// same flat per-object 64-vortex charge across two endpoint scans, computing `objects * 128 + ...`
/// = 23,055 on Nakagin's 180 objects — nearly 6x the cap. The rewritten bound counts the document's
/// actual vortices instead of assuming every object carries the worst-case vortex count.
#[test]
fn create_attraction_extent_fits_within_cap_for_nakagin() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("createAttraction", Some(json!({ "attracting": "nonexistent-a", "attracted": "nonexistent-b" })), None).expect("createAttraction command decodes");
    let work = Puzzle3dCreateAttractionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, "createAttraction extent {extent} must not exceed the fixed cap {}", semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): drives the real
/// `Attracting`/`Attracted` full-document vortex scans instead of only checking the bound fits
/// under the cap. `attracting`/`attracted` name a genuine compatible pair already present in the
/// Nakagin scene_snapshot — `25b0dba0-…:link` (kind "door capsule right", object index 27 in DSL
/// declaration order) and `5f0266bc-…:sl0_d0` (kind "door tambour right", object index 64) — which
/// `kind-compatibility` marks bidirectionally compatible, so `step()` runs every real stage
/// (`Existing → Attracting → Attracted → Compatibility → Publish`) to a genuine success instead of
/// one of the Work's several early-`Complete` short-circuits (duplicate/incompatible/empty-id).
#[test]
fn create_attraction_step_loop_stays_within_its_own_extent_for_nakagin() {
    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command =
        Puzzle3dCommand::from_action("createAttraction", Some(json!({ "attracting": "25b0dba0-8f81-423a-94a1-b911a6031010:link", "attracted": "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952:sl0_d0" })), None).expect("createAttraction command decodes");
    let mut work = Puzzle3dCreateAttractionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "createAttraction step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match semio_s_plugin_puzzle_retained::testing::step(&mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(iterations <= extent, "createAttraction step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(iterations > 50, "createAttraction must scan real objects and vortices to find both endpoints, not take a trivial early exit; observed only {iterations} iterations");
    assert_eq!(
        emit.artifact_mutations.len(),
        1,
        "a real, compatible, non-duplicate attracting/attracted pair must reach Publish and emit exactly one connect_vortices mutation, not one of the early-Complete short-circuits; observed {}",
        emit.artifact_mutations.len()
    );
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dAcceptSuggestionWork::extent`'s
/// `target_scans` term charged every scene object the same flat 64-vortex worst case
/// (`objects * 64`), computing 7,888+ on Nakagin's 180 objects. The rewritten bound counts the
/// document's actual vortices for the target scan while keeping the catalog-kind-cap terms
/// (`PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT`), which bound a real runtime-enforced invariant on
/// catalog kinds, not scene object instances.
#[test]
fn accept_suggestion_extent_fits_within_cap_for_nakagin() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", None, None).expect("acceptSuggestion command decodes");
    let work = Puzzle3dAcceptSuggestionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's catalogs and real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, "acceptSuggestion extent {extent} must not exceed the fixed cap {}", semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): drives the real `Target`
/// scan plus the full `Candidate → Vortices → ExistingAttractions → PublishObject → PublishAttraction →
/// PublishResult` chain to a genuine success, using a real Nakagin vortex (`25b0dba0-…:link`, object index 27
/// in DSL declaration order) as `fullId` and one free candidate the brush suggestions run published for it,
/// instead of the "no target requested" early-`Complete` short-circuit.
#[test]
fn accept_suggestion_step_loop_stays_within_its_own_extent_for_nakagin() {
    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), None).expect("acceptSuggestion command decodes");
    let work = Puzzle3dAcceptSuggestionWork::default();
    let kind = snapshot.typed().meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.objects.first()).expect("nakagin catalogues object kinds").id.clone();
    let owner = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(Box::new(Puzzle3dInstanceOperationOwner::default()));
    owner
        .with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| {
            let preview = semio_s_artifact_puzzle_3d_model::BrushPreviewState { target_vortex_full_id: "25b0dba0-8f81-423a-94a1-b911a6031010:link".into(), object_kind_id: kind, source_vortex_index: 0, mesh_url: "/nakagin/capsule.glb".into(), origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
            owner.brush_suggestions.found = Some(semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsFound { writer: (1, 0), target: preview.target_vortex_full_id.clone(), previews: vec![Some(preview)], verdicts: vec![semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionVerdict::Free], done: true });
            Ok(())
        })
        .expect("the instance owner");
    let mut work = work.bound(&Puzzle3dInstanceBinding { app_instance_id: 1, parent_document_id: "test".to_string(), owner });
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's catalogs and real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "acceptSuggestion step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match semio_s_plugin_puzzle_retained::testing::step(&mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(iterations <= extent, "acceptSuggestion step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(iterations > 20, "acceptSuggestion must scan real objects to find the requested target vortex, not take a trivial early exit; observed only {iterations} iterations");
    assert_eq!(
        emit.artifact_mutations.len(),
        2,
        "a real target vortex and a real catalog kind must reach PublishResult and emit exactly the created object plus its connecting attraction, not one of the early-Complete short-circuits; observed {}",
        emit.artifact_mutations.len()
    );
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dPatchInspectorWork::extent`'s `"vortex"`
/// arm charged `objects * PUZZLE_COMMAND_DECODED_ITEMS` (objects * 512) regardless of real vortex
/// count, computing ~92,160 on Nakagin's 180 objects — a 512x-over-budget design error, not a
/// precision edge case. The rewritten bound counts the document's actual vortex instances plus one
/// `step()` per object, matching the `Vortices` stage's real per-object "owner advance" call.
#[test]
fn patch_inspector_vortex_extent_fits_within_cap_for_nakagin() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("patchInspector", Some(json!({ "entity": "vortex" })), None).expect("patchInspector command decodes");
    let work = Puzzle3dPatchInspectorWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, "patchInspector vortex extent {extent} must not exceed the fixed cap {}", semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): the `Vortices` stage walks
/// every object and every real vortex UNCONDITIONALLY (selection only gates whether a visited
/// vortex is mutated, never whether it is visited), so this is a genuine full-document run by
/// construction, not a trivial early exit — selecting one real vortex
/// (`25b0dba0-…:link`) and setting `field: "hidden"` proves the walk both completes within its own
/// extent AND actually mutates the one entity that was selected out of the full scan.
///
/// Writing this test surfaced a real off-by-one in `extent()`'s own `Selection` accounting: the
/// `Selection` stage's own exhaustion call (`source_id` returning `None`) is a real `step()` call
/// that returns `Progress` — a `+1` distinct from each entity arm's own terminal call (which
/// returns `Complete` instead, contributing zero to the `Progress` count `work_cursor` tracks). The
/// old `let items = source.checked_add(scan)?;` omitted that `Selection`-stage `+1` entirely; fixed
/// to `let items = source.checked_add(scan)?.checked_add(1)?;`, which this test's `iterations <=
/// extent` assertion is what actually catches — the extent-only test above could not have (it
/// never drives the real loop, so no test previously computed the true `Progress`-call count).
#[test]
fn patch_inspector_vortex_step_loop_stays_within_its_own_extent_for_nakagin() {
    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("patchInspector", Some(json!({ "entity": "vortex", "field": "hidden", "value": true, "ids": ["25b0dba0-8f81-423a-94a1-b911a6031010:link"] })), None).expect("patchInspector command decodes");
    let mut work = Puzzle3dPatchInspectorWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "patchInspector step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match semio_s_plugin_puzzle_retained::testing::step(&mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(iterations <= extent, "patchInspector step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(
        iterations > 400,
        "patchInspector's Vortices stage unconditionally walks every object and every real vortex regardless of selection, so a real Nakagin run must take hundreds of steps, not a trivial early exit; observed only {iterations} iterations"
    );
    assert_eq!(emit.artifact_mutations.len(), 1, "exactly the one selected vortex must be patched out of the full unconditional scan; observed {} mutations", emit.artifact_mutations.len());
}

/// 🧵️ ticket 26/09/02/PUZZLE-3D-END-TO-END: direct functional proof that the previously-dead
/// `Puzzle3dSetActiveExampleWork` state machine actually walks its own stages across MULTIPLE
/// bounded `step()` calls for a real scene_snapshot (not a single-shot reducer), accumulating exactly
/// one mutation per deleted/created item and only publishing them all in the final `Complete`
/// emit — the same turn-by-turn contract `crate::retained_command::RetainedPuzzleCommandJob`
/// relies on when it drives an app-owned `PuzzleCommandWork` (`🎮️commands/🧵️retained/🦀️.rs`,
/// `PuzzleCommandPhase::Work`). The hostile-static-law test above only proves this arm's source
/// text exists; this proves it runs.
#[test]
fn set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin() {
    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample command decodes");
    let mut work = Puzzle3dSetActiveExampleWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin example fits the fixed bounded work envelope");
    assert!(extent > 4, "nakagin example must require more than the handful of fixed transition steps alone");
    let mut progress_steps = 0usize;
    let emit = loop {
        assert!(progress_steps <= extent + 8, "setActiveExample work did not reach Complete within its own declared extent");
        match semio_s_plugin_puzzle_retained::testing::step(&mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => progress_steps += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(progress_steps > 1, "setActiveExample must require multiple bounded step() calls for the nakagin example, not a single-shot reducer; observed {progress_steps}");
    assert!(emit.artifact_mutations.len() > 1, "the completed emit must carry every incrementally-collected mutation, one per deleted/created item; observed {}", emit.artifact_mutations.len());
    // 🏷️ Example loading publishes exactly ONE config row, and it moves exactly ONE field: the id of
    // the example just loaded, which is what `export_snapshot` names its download after. The user's own
    // shared preferences (fill count, contact tolerance, kind weights) ride through untouched.
    let Some(Puzzle3dConfigMutation::SetActiveExampleId(Puzzle3dConfigSetActiveExampleId { value: published })) = emit.config_mutations.first().cloned() else {
        panic!("example loading must stamp the active example id on the shared config: {:?}", emit.config_mutations);
    };
    assert_eq!(emit.config_mutations.len(), 1, "example loading publishes one config row, not many");
    assert_eq!(published, PUZZLE3D_EXAMPLE_NAKAGIN, "the stamped id must be the example that was loaded");
}

/// 🧲️ Both hosts paint a gumball drag locally and dispatch ONE pose delta on release, so there is no gesture
/// bracket left to declare: `transformBegin`/`transformEnd` are neither retained tools, nor manifest actions,
/// nor publication contracts.
#[test]
fn gumball_gestures_declare_no_bracket_verbs() {
    let manifest = create_puzzle3d_app();
    let contracts = <Puzzle3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    for action in ["transformBegin", "transformEnd"] {
        assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&action), "{action} must not be a retained tool id");
        assert!(action_declarations(&manifest, action).is_empty(), "{action} must not be declared");
        assert!(!contracts.iter().any(|contract| contract.tool_id == action), "{action} must carry no publication contract");
        assert!(Puzzle3dCommand::from_action(action, None, None).is_none(), "{action} must not decode as a command");
    }
}

/// 🌉️ ticket 26/09/02/PUZZLE-3D-END-TO-END: exercises the wiring this ticket adds — the
/// classification flip to `Migrated`, `PUZZLE3D_RETAINED_TOOL_IDS` registration, the
/// `ArtifactToolPublicationContract` for the Artifact+Config lanes, and
/// `Puzzle3dArtifactStorePreparationFactory` — by dispatching `setActiveExample` through the
/// real `InteractiveJob`/tool-job path (`Puzzle3dRetainedCommandJobFactory` ->
/// `RetainedPuzzleCommandJob` -> `ArtifactToolCompletion` -> the shared publication loop's
/// `self.store.begin_apply_batch(..., self.artifact_one_item_factory.as_ref(), None)`), driving the
/// resulting typed operation to completion via repeated `maintenance_step` turns exactly as a
/// real host does every actor tick, then asserting the document was actually swapped. Uses
/// the registry-backed, instance-bound `app()`: this plugin declares
/// `bounded_first_step_tool_proofs!`, so the bare registry-less `artifact_app_laws::new_app` faults closed
/// with `interactive-job.catalog-authority` before any dispatch is even attempted.
#[semio_framework_async_macros::async_test]
async fn set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    let before_first_id = first_object_id(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("dispatch setActiveExample through the migrated tool-job path");
    let mut ticks = 0usize;
    while ticks < 5_000 && (object_count(&app) == 0 || first_object_id(&app) == before_first_id) {
        semio_s_plugin_puzzle_jobs::testing::maintain_items(&mut *app, 1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
        ticks += 1;
    }
    assert!(object_count(&app) > 0, "nakagin document did not land after {ticks} maintenance turns");
    assert_ne!(first_object_id(&app), before_first_id, "setActiveExample did not actually swap the document's objects through the tool-job path");
}

/// 📏️ Sizes this artifact's three reserved refresh sections against the retained section carrier that
/// now publishes them (`semio_framework_plugin::section_component_tree`): the engagements, window
/// measure and tool measure maps must all admit into the bounded chunk tree and round-trip
/// byte-exactly, with the fill tool's count slider present in the tool payload.
#[semio_framework_async_macros::async_test]
async fn reserved_refresh_section_payloads_admit_into_the_retained_section_carrier() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    let view = semio_framework_plugin::ViewModel { window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let payloads = [
        (semio_framework_plugin::UiRefreshSection::Engagements, serde_json::to_string(&app.window_engagements(&view).await).expect("engagements serialize")),
        (semio_framework_plugin::UiRefreshSection::Measures, serde_json::to_string(&app.window_measures(&view).await).expect("measures serialize")),
        (semio_framework_plugin::UiRefreshSection::Tools, serde_json::to_string(&app.tool_measures(&view).await).expect("tool measures serialize")),
    ];
    assert!(payloads[2].1.contains("puzzle3d-fill-count"), "the fill tool must contribute its count slider to the tools section");
    assert!(payloads[1].1.contains(main::WINDOW_KIND_ID), "the main window instance must key its own entry in the measures section");
    for (section, payload) in payloads {
        let tree = semio_framework_plugin::section_component_tree(section, &payload).expect("reserved section payload admits into the bounded carrier");
        let projected: serde_json::Value = serde_json::from_str(&semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("carrier projects")).expect("carrier projection is json");
        assert_eq!(projected["key"], section.body_key());
        assert_eq!(semio_framework_plugin::artifact_app_laws::fixture_carrier_text(&projected), payload, "{} carrier must round-trip byte-exactly", section.key());
    }
}

fn add_brush_object_is_cursorized(source: &str) -> bool {
    source.contains(r#""addBrushObject" => Box::new(Puzzle3dAddBrushObjectWork::default())"#)
        && source.contains("Puzzle3dAddBrushObjectStage::Decode")
        && source.contains("Puzzle3dAddBrushObjectStage::Kind")
        && source.contains("Puzzle3dAddBrushObjectStage::Representation")
        && source.contains("Puzzle3dAddBrushObjectStage::Vortices")
        && source.contains("Puzzle3dAddBrushObjectStage::ExistingAttractions")
        && source.contains("Puzzle3dAddBrushObjectStage::PublishObject")
        && source.contains("Puzzle3dAddBrushObjectStage::PublishAttraction")
        && !source.contains(r#""addBrushObject" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_brush_object_hostile_static_law_rejects_engine_run_to_completion() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(add_brush_object_is_cursorized(source));
    let direct = source
        .replace(r#""addBrushObject" => Box::new(Puzzle3dAddBrushObjectWork::default())"#, r#""addBrushObject" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!add_brush_object_is_cursorized(&direct));
    for marker in [
        "Puzzle3dAddBrushObjectStage::Decode",
        "Puzzle3dAddBrushObjectStage::Kind",
        "Puzzle3dAddBrushObjectStage::Representation",
        "Puzzle3dAddBrushObjectStage::Vortices",
        "Puzzle3dAddBrushObjectStage::ExistingAttractions",
        "Puzzle3dAddBrushObjectStage::PublishObject",
        "Puzzle3dAddBrushObjectStage::PublishAttraction",
    ] {
        assert!(!add_brush_object_is_cursorized(&source.replace(marker, "cursor-removed")), "missing brush cursor was falsely accepted: {marker}");
    }
}

fn add_object_kind_is_cursorized(source: &str) -> bool {
    source.contains(r#""addObjectKind" => Box::new(Puzzle3dAddObjectKindWork::default())"#)
        && source.contains("Puzzle3dAddObjectKindStage::Decode")
        && source.contains("Puzzle3dAddObjectKindStage::Kind")
        && source.contains("Puzzle3dAddObjectKindStage::Representation")
        && source.contains("Puzzle3dAddObjectKindStage::Vortex")
        && source.contains("Puzzle3dAddObjectKindStage::Publish")
        && source.contains("PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT")
        && !source.contains(r#""addObjectKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_object_kind_hostile_static_law_rejects_whole_catalog_conversion() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(add_object_kind_is_cursorized(source));
    let direct =
        source.replace(r#""addObjectKind" => Box::new(Puzzle3dAddObjectKindWork::default())"#, r#""addObjectKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!add_object_kind_is_cursorized(&direct));
    for marker in ["Puzzle3dAddObjectKindStage::Kind", "Puzzle3dAddObjectKindStage::Representation", "Puzzle3dAddObjectKindStage::Vortex", "Puzzle3dAddObjectKindStage::Publish", "PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT"] {
        assert!(!add_object_kind_is_cursorized(&source.replace(marker, "cursor-removed")), "missing add-object-kind cursor was falsely accepted: {marker}");
    }
}

fn exact_window_routes_capture_instance_owners(source: &str) -> bool {
    source.contains("struct Puzzle3dWindowCommandWork")
        && source.contains("ArtifactCommandWorkStep::CompleteWithEphemeral")
        && source.contains("config_from_snapshot(window_config_snapshot)")
        && source.contains("transient_from_snapshot(window_transient_snapshot)")
        && source.contains("=> Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance))")
}

#[test]
fn exact_window_routes_reject_missing_owner_capture() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(exact_window_routes_capture_instance_owners(source));
    for marker in ["ArtifactCommandWorkStep::CompleteWithEphemeral", "config_from_snapshot(window_config_snapshot)", "transient_from_snapshot(window_transient_snapshot)"] {
        assert!(!exact_window_routes_capture_instance_owners(&source.replace(marker, "owner-capture-removed")));
    }
}

fn window_config_publish_does_not_silently_drop(source: &str) -> bool {
    source.contains("addressed_config_for(&wid, window_after)")
        && source.contains("addressed_transient_for(&wid, transient_after)")
        && !source.contains("addressed_config(view, window_after).ok()).into_iter()")
}

#[test]
fn window_config_publish_hostile_static_law_rejects_silent_ok_into_iter_drop() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(window_config_publish_does_not_silently_drop(source));
    let dropped = source.replace(
        "vec![window_ownership::addressed_config_for(&wid, window_after)]",
        "view_state.and_then(|view| window_ownership::addressed_config(view, window_after).ok()).into_iter().collect()",
    );
    assert!(!window_config_publish_does_not_silently_drop(&dropped), "a silent .ok().into_iter() drop must fail the law");
}

fn kind_weight_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle3dKindWeightWork::new(tool_id))"#)
        && source.contains("Puzzle3dKindWeightStage::Catalog")
        && source.contains("Puzzle3dKindWeightStage::Validate")
        && source.contains("Puzzle3dKindWeightStage::SumOthers")
        && source.contains("Puzzle3dKindWeightStage::Build")
        && source.contains("Puzzle3dConfigMutation::SetObjectKindWeights")
        && source.contains("Puzzle3dConfigMutation::SetVortexKindWeights")
        && !source.contains(r#""setObjectKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn kind_weight_hostile_static_law_rejects_whole_normalizer_and_missing_cursors() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(kind_weight_route_is_cursorized(source));
    let direct = source.replace(
        r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle3dKindWeightWork::new(tool_id))"#,
        r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!kind_weight_route_is_cursorized(&direct));
    assert!(!source.contains("puzzle3d_normalize_kind_weight_group(self.weights"));
}

#[test]
fn app_config_serialization_excludes_operation_and_window_state() {
    let config = Puzzle3dConfig::default();
    let spr = to_json_string(&config);
    let oracle: serde_json::Value = serde_json::from_str(&spr).expect("third-party JSON oracle accepts Puzzle 3D config");
    // 🧮️ Five shared preferences: the four fill/distribution ones, plus `activeExampleId` — document
    // identity rather than a preference, and the only thing `export_snapshot` can name its download after
    // (wave B26). Everything per-WINDOW or per-OPERATION still stays out, which is what this law is for.
    assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(5));
    assert!(spr.contains("activeExampleId"));
    for forbidden in ["fillCheckpoint", "fillApplyGeneration", "windowOptions", "camera", "suggestionMenu", "engagementInput"] {
        assert!(!spr.contains(forbidden));
    }
}

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn renders_world_scene() {
    let mut app = app().await;
    assert!(render_composite(&mut app).await.to_string().contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_the_concrete_forest_fixture() {
    let app = app().await;
    assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_SCENE_SNAPSHOT_SCHEMA));
    assert!(object_count(&app) > 0, "the concrete-forest default scene_snapshot ships with objects");
}

/// 📦️ `Puzzle3dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls), reusing the default concrete-forest scene_snapshot.
#[semio_framework_async_macros::async_test]
async fn puzzle3d_play_projection_pack_round_trips() {
    let app = app().await;
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
}

/// 🧬️ Wave B9 lane 2: the TYPED half of the snapshot is the authority
/// `ArtifactApp::interaction_topology` reads, and `protocol::validate_state` prunes every selected id
/// the resulting topology does not contain — so a snapshot whose typed half decoded to the DEFAULT
/// (empty) document answers every pick with an empty selection while its `value()` projection still
/// renders the whole world. That is exactly the boundary wave B6 proved in the browser. `PartialEq`
/// compares the typed half alone, so [`assert_dsl_pack_equivalence`] above reads two empty documents
/// as equal and cannot state this; the object count on BOTH halves can.
#[semio_framework_async_macros::async_test]
async fn play_snapshot_typed_authority_survives_every_store_round_trip() {
    use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};
    let app = app().await;
    let snapshot = app.snapshot().expect("projection");
    let objects = snapshot.value().get("objects").and_then(semio_framework_value::DslValue::as_array).map(<[_]>::len).unwrap_or(0);
    assert!(objects > 0, "the boot scene_snapshot ships objects");
    assert_eq!(snapshot.typed().objects.len(), objects, "the live snapshot's typed authority must carry the projection's objects");
    let via_pack = Puzzle3dPlaySnapshot::decode_pack(&snapshot.encode_pack()).expect("pack decode");
    assert_eq!(via_pack.typed().objects.len(), objects, "a store pack round trip must not empty the typed authority");
    let via_dsl = Puzzle3dPlaySnapshot::parse_dsl(&snapshot.print_dsl()).expect("dsl parse");
    assert_eq!(via_dsl.typed().objects.len(), objects, "a dsl round trip must not empty the typed authority");
    let via_value = Puzzle3dPlaySnapshot::new(semio_framework_value::FromValue::from_value(snapshot.value().clone()).expect("typed fixture admits"));
    assert_eq!(via_value.typed().objects.len(), objects, "rebuilding from the projection must not empty the typed authority");
}

/// 🕳️ Wave B9 lane 2: an ABSENT scene_snapshot-meta member must project as absent, never as `Null`. The
/// persisted twin types `meta.kindCompatibility` as a real array and refuses a `Null` for it, and
/// `Puzzle3dPlaySnapshot`'s decode is all-or-nothing — so one `null` member replaced the ENTIRE typed
/// authority with an empty document while the projection kept every object. `PartialEq` compares the
/// typed half alone, so no round-trip law above can see it; the object count on BOTH halves can.
#[test]
fn an_absent_fixture_meta_member_never_empties_the_typed_authority() {
    let mut seeded = empty_scene_snapshot();
    seeded.objects.clone_from(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT.objects);
    assert!(seeded.meta.kind_catalogs.is_none() && seeded.meta.kind_compatibility.is_none(), "this law is about the meta-less scene_snapshot shape");
    let projection: serde_json::Value = (&semio_framework_value::ToValue::to_value(&seeded)).into();
    let meta = projection.get("meta").expect("the projection carries a meta object");
    assert!(!meta.get("kindCatalogs").is_some_and(serde_json::Value::is_null), "an absent kind catalog must be absent, not null: {meta}");
    assert!(!meta.get("kindCompatibility").is_some_and(serde_json::Value::is_null), "an absent compatibility table must be absent, not null: {meta}");
    let snapshot = Puzzle3dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&(projection))).expect("typed fixture admits"));
    assert_eq!(snapshot.typed().objects.len(), seeded.objects.len(), "both halves of one snapshot must describe the same document");
}

/// 🔭️ Wave B9 lane 2: the pickable universe `protocol::validate_state` prunes against must name
/// every id the world lane paints. Measured against a document whose TYPED authority is empty while
/// its projection is whole — the exact divergence a refused meta decode produces, and the reason the
/// browser's first pick landed on the leftover and then vanished from the Inspection panel.
#[test]
fn interaction_topology_names_every_id_the_world_lane_paints() {
    let mut divergent = empty_scene_snapshot();
    divergent.objects.clone_from(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT.objects);
    divergent.meta.kind_compatibility = Some(semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::object([("target".to_string(), semio_framework_value::DslValue::String("b-l".to_string()))])]));
    let projection: serde_json::Value = (&semio_framework_value::ToValue::to_value(&divergent)).into();
    let refusal=<semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&projection));
    assert!(refusal.is_err(),"invalid catalogs must refuse rather than discard objects");
    divergent.meta.kind_compatibility=None;
    let snapshot=Puzzle3dPlaySnapshot::new(puzzle3d_snapshot_from_host_snapshot(&divergent).expect("valid host admits"));
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = Puzzle3dConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = Puzzle3dPlayApp::interaction_topology(&doc, &cfg).expect("valid retained interaction scene_snapshot");
    let domain = topology.domains.get(PUZZLE3D_INTERACTION_DOMAIN).expect("the vortex domain is declared");
    let ids: Vec<&str> = domain.ordered.iter().map(|node| node.id.as_str()).collect();
    for object in &divergent.objects {
        assert!(ids.contains(&object.id.as_str()), "a painted object must be pickable: {} missing from {ids:?}", object.id);
        for vortex in &object.vortices {
            let full = puzzle3d_vortex_full_id(&object.id, &vortex.id);
            assert!(ids.iter().any(|id| *id == full.as_str()), "a painted vortex marker must be pickable: {full} missing");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn open_add_object_dialog_emits_the_open_dialog_effect_with_no_document_change() {
    let mut app = app().await;
    let before = object_count(&app);
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("openAddObjectDialog");
    assert!(
        matches!(result.requested_effects.as_slice(), [Effect::OpenDialog { dialog_id, args, .. }] if dialog_id == "addObject" && args.is_none()),
        "expected a single OpenDialog effect for the addObject dialog, got {:?}",
        result.requested_effects,
    );
    assert_eq!(object_count(&app), before, "opening the dialog does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_the_document_and_undo_restores_it() {
    let mut app = app().await;
    let loaded = object_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0, "empty example clears the objects");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), loaded, "undo restores the concrete-forest objects");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_count(&app), 0);
}

/// 🛰️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B7: `undo` is a TWO-half framework-reserved gesture
/// and neither half may be silent. The ADMISSION half must change nothing and say so out loud — one
/// Isolated [`semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND`] spawn job and
/// `UiDirtyScope::None`, never a success that quietly committed nothing; the HOST half
/// (`complete_reserved_spawned_job` → `commit_framework_history_route`) is the only thing that may
/// move the document. Pins the regression wave B5 reported: a caller that stops at the admission
/// sees `dispatch("undo")` succeed while the store never moves, so every undo law degrades into a
/// no-op that still passes its dispatch.
#[semio_framework_async_macros::async_test]
async fn undo_admits_one_reserved_job_that_alone_restores_the_document() {
    let mut app = app().await;
    let loaded = object_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0, "empty example clears the objects");
    let admitted = dispatch_reserved_unsettled(&mut app, "undo", None, None).await.expect("undo admits");
    let spawned = admitted.requested_effects.iter().filter(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND)).count();
    assert_eq!(spawned, 1, "the admission half must ask the host for exactly one Isolated reserved job: {:?}", admitted.requested_effects);
    assert!(matches!(admitted.ui_scope, UiDirtyScope::None), "an undo that has not run yet must not claim a dirty scope: {:?}", admitted.ui_scope);
    assert_eq!(object_count(&app), 0, "the admission half must never move the document on its own");
    let committed = settle_reserved(&mut app, admitted).await.expect("the host half commits the history route");
    assert!(matches!(committed.ui_scope, UiDirtyScope::Full), "a committed undo republishes the whole document: {:?}", committed.ui_scope);
    assert_eq!(object_count(&app), loaded, "only the driven reserved job restores the concrete-forest objects");
}

#[semio_framework_async_macros::async_test]
async fn nakagin_example_loads_via_operations() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let projection = projection_of(&app);
    assert_eq!(projection.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_SCENE_SNAPSHOT_SCHEMA));
    assert!(projection.get("objects").and_then(|value| value.as_array()).is_some_and(|objects| !objects.is_empty()));
}

/// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: a mutating verb lands its OBJECT **and** its
/// history row inside the settle of its OWN dispatch — no second command. The browser measured the
/// opposite on wasm #47: an `addObjectKind` click settled in 23 continuations with
/// `frameKinds:["Invocation","Ephemeral"]` and `historyUpserts:0`, and the `create-object` row only
/// arrived inside the NEXT command's settle, which is why `catalogue-add-object-kind`,
/// `delete-selection`, `duplicate-selection` and `volume-brush-add-target-volume` all read
/// `before == after` at their own verdict (`📓️2026-09-12-wave-B19-mutation-lane-regression.md` §3).
///
/// The row rides [`Puzzle3dCompletion::history_patch`] — the `AppFrame::OperationCompleted` the
/// renderer applies through `subscribeOperationCompletions`. A typed operation never calls
/// `record_command`: its edit reaches the command log only through `backfill_command_log`, inside
/// `refresh_cache`, inside `history_patch` — so reading `history_dirty_sequences` BEFORE that
/// backfill answered "nothing dirty" on the very call whose publication had just landed.
#[semio_framework_async_macros::async_test]
async fn a_mutating_verb_lands_its_object_and_its_history_row_inside_its_own_settle() {
    for (action, args) in [("addObjectKind", json!({ "objectKind": "Object", "origin": [3.0, 0.0, 0.0] })), ("duplicateSelection", json!({}))] {
        let mut app = app().await;
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty example");
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("seed one object");
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the seed object");
        let before = object_count(&app);
        dispatch_unsettled(&mut app, action, Some(&args), None).await.unwrap_or_else(|fault| panic!("{action} must admit: {fault:?}"));
        let settled = settle(&mut app).await;
        assert_eq!(object_count(&app), before + 1, "{action} must land its object inside its own settle");
        let rows = settled.completions.iter().filter(|completion| completion.history_patch.as_ref().is_some_and(|patch| !patch.upserts.is_empty())).count();
        assert_eq!(rows, 1, "{action} must hand back exactly one completion carrying its history patch inside its own settle, got {:?}", settled.completions);
    }
}

#[semio_framework_async_macros::async_test]
async fn document_and_inspector_panels_render() {
    let mut app = app().await;
    for body in [artifact::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(!render_body(&mut app, body).await.to_string().is_empty());
    }
}
//#endregion 🔖️Operations

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`). Deliberately
/// dispatches through a standalone typed `Puzzle3dStore` — NOT through `Puzzle3dPlayApp`/
/// `Puzzle3dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app still uses)
/// — since `Puzzle3dMutation`'s canonical `Mutation<Puzzle3dSnapshot>` impl (not its
/// `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use semio_s_artifact_puzzle_3d_host::{close_puzzle3d_store, puzzle3d_store};
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dObject as TypedObject, semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactCommand};

    let mut store = puzzle3d_store(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("store");
    let object = TypedObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None)], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle3dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    close_puzzle3d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️Inspector
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: with nothing selected the inspector
/// shows the document summary; a live `object`-granularity selection switches it to that object's own
/// field group, carrying its real id/origin plus the `hidden`/`locked` `patchInspector` toggles.
#[semio_framework_async_macros::async_test]
async fn selected_object_inspector_renders_that_object_field_group() {
    let mut app = app().await;
    let empty = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(empty.contains("puzzle3d-play-inspector.empty"), "an empty selection shows the document summary: {empty}");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let json = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!json.contains("puzzle3d-play-inspector.empty"), "a live selection must replace the document summary: {json}");
    for expected in ["puzzle3d-play-inspector.object.id", "puzzle3d-play-inspector.object.origin", "puzzle3d-play-inspector.object.hidden", "puzzle3d-play-inspector.object.locked"] {
        assert!(json.contains(expected), "inspector must render {expected}: {json}");
    }
    assert!(json.contains(object_id.as_str()), "the rendered field group carries the selected object's own id: {json}");
}

/// 🕹️ Wave B9 lane 2, end to end: the FIRST pick of a fresh session must already be visible to the
/// guest's own render — the object's id and its `hidden`/`locked` `flag_row`s — and a lock taken on
/// that same selection must show as a pressed row. In the browser this hop folded: the pick reached
/// the leftover, `validate_state` pruned it against a topology narrower than the painted document,
/// and the panel kept re-rendering the empty-document summary for 73 s.
#[semio_framework_async_macros::async_test]
async fn first_pick_of_a_fresh_session_renders_the_object_and_its_lock_row() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("first interactionSelect of the session");
    let picked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!picked.contains("puzzle3d-play-inspector.empty"), "the FIRST pick must replace the document summary: {picked}");
    assert!(picked.contains(object_id.as_str()), "the first pick's own object id must reach the panel: {picked}");
    assert!(picked.contains("puzzle3d-play-inspector.object.locked"), "the first pick must assemble the lock flag_row: {picked}");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), None).await.expect("lock the picked object");
    let locked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(locked.contains("puzzle3d-play-inspector.object.locked"), "a locked selection keeps its lock flag_row: {locked}");
    assert!(
        projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id.as_str())).cloned()).and_then(|object| object.get("locked").and_then(Value::as_bool))
            == Some(true),
        "the lock taken on the first pick's selection must land on that object"
    );
}

/// 🕹️ Wave B13: hover after the first pick must keep Inspection on that object — leftover hover
/// used to republish `selectedIds: []` and the panel folded back to the empty summary.
#[semio_framework_async_macros::async_test]
async fn hover_after_first_pick_keeps_the_object_and_its_lock_row() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("first interactionSelect");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(object_id.as_str())).await.expect("hover after first pick");
    let hovered = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!hovered.contains("puzzle3d-play-inspector.empty"), "hover after first pick must keep the object fields: {hovered}");
    assert!(hovered.contains(object_id.as_str()), "hover after first pick must keep the object id: {hovered}");
    assert!(hovered.contains("puzzle3d-play-inspector.object.locked"), "hover after first pick must keep the lock flag_row: {hovered}");
}

/// 🕹️ Background click: empty-target `interactionSelect` with `replace` clears selection; hover may remain.
#[semio_framework_async_macros::async_test]
async fn empty_target_interaction_select_clears_selection_while_hover_remains() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(object_id.as_str())).await.expect("hover names the object");
    let admitted = dispatch_reserved_unsettled(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": "[]", "merge": "replace", "method": "pick" })),
        None,
    )
    .await
    .expect("empty-target interactionSelect admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("empty-target interactionSelect leftover");
    let view = settled.output.get("interactionView").expect("leftover InteractionView");
    let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
    assert!(ids.is_empty(), "empty-target interactionSelect must clear selectedIds, got {ids:?} hover={:?}", view.get("hoverTarget"));
    let hover = view.get("hoverTarget").expect("hoverTarget on leftover");
    assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some(object_id.as_str()));
    assert!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "empty-target interactionSelect must clear the interaction store"
    );
    let picked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(picked.contains("puzzle3d-play-inspector.empty"), "Inspection must fold to the empty summary after background deselect: {picked}");
}

/// 🪟️ WAVE B56 LAW: a window-addressed `interactionSelect` leftover encode carries the window
/// INSTANCE it was dispatched for, and a windowless dispatch carries none — never a synthetic window.
///
/// 🐛️ The leftover encode named no window at all, so the host had to infer the pane the overlay belonged
/// to; when it could not, the guest's own dirty fell back to the bare surface name `window` and the host
/// mounted a synthetic alias to give that name a context (wave W-G3 §8.29). That alias is the surface whose
/// reconcile reservation refusal starved every real one
/// (`reserve_refusal=1:window:registry-reservation-unavailable`, wave B54 §6.4). The instance rides the
/// encode now, so `publishLeftoverWorldSelectionV1` addresses a real pane and nothing has to be inferred
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
#[semio_framework_async_macros::async_test]
async fn interaction_select_leftover_window_instance_rides_the_encode() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), id: object_id.clone() }]);
    for window in [main::WINDOW_INSTANCE_PERSPECTIVE, main::WINDOW_INSTANCE_TOP] {
        let admitted = dispatch_reserved_unsettled(
            &mut app,
            "interactionSelect",
            Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets.clone(), "merge": "replace", "method": "pick" })),
            Some(window),
        )
        .await
        .expect("window-addressed interactionSelect admit");
        let settled = settle_reserved(&mut app, admitted).await.expect("window-addressed interactionSelect leftover");
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        assert_eq!(
            view.get("windowId").and_then(semio_framework_value::DslValue::as_str),
            Some(window),
            "the leftover encode must name the window instance the pick addressed, got {:?}",
            view.get("windowId")
        );
    }
}

/// 🎮 Wave B23: leftover.ids that name an object must reach the snapshot `inspection::render` reads
/// when persist `selection(vortex)` is empty.
#[test]
fn leftover_ids_name_object_empty_persist_vortex_from_state_wires_inspection_snapshot() {
    let mut state = protocol::InteractionState::default();
    state.selection.insert("vortex".into(), protocol::DomainSelection { granularity: String::new(), ids: Vec::new(), anchor_id: None });
    state.selection.insert("leftover".into(), protocol::DomainSelection { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), ids: vec!["seed-left-001".into()], anchor_id: None });
    let interaction = Puzzle3dInteractionSnapshot::from_state(&state, &semio_framework_plugin::app::InteractionHoverState::default());
    assert_eq!(interaction.selected, vec!["seed-left-001".to_string()], "leftover.ids must land on the snapshot Inspection reads");
    assert_eq!(interaction.granularity, PUZZLE3D_GRANULARITY_OBJECT);
}

/// 🕹️ A `vortex`-granularity selection switches the inspector to the vortex field group instead — the
/// per-granularity switch the panel's `selected_section` performs.
#[semio_framework_async_macros::async_test]
async fn selected_vortex_inspector_renders_the_vortex_field_group() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let json = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(json.contains("puzzle3d-play-inspector.vortex.full-id"), "inspector must render the vortex field group: {json}");
    assert!(json.contains("puzzle3d-play-inspector.vortex.radius"), "inspector must render the vortex radius: {json}");
    assert!(!json.contains("puzzle3d-play-inspector.object.id"), "a vortex selection must not render the object group: {json}");
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_sets_absolute_value_and_preserves_other_axes() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let before_y = projection_of(&app)
        .get("objects")
        .and_then(|value| value.as_array())
        .and_then(|objects| objects.first())
        .and_then(|object| object.get("origin"))
        .and_then(|value| value.as_array())
        .and_then(|origin| origin.get(1))
        .and_then(|value| value.as_f64())
        .expect("origin.y");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector");
    let projection = projection_of(&app);
    let objects = projection.get("objects").and_then(|value| value.as_array()).expect("objects");
    let object = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(object_id.as_str())).expect("patched object");
    let origin = object.get("origin").and_then(|value| value.as_array()).expect("origin");
    assert_eq!(origin[0].as_f64(), Some(42.5), "origin.x should be set to the absolute value");
    assert_eq!(origin[1].as_f64(), Some(before_y), "origin.y should be untouched by an origin.x edit");
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_delta_offsets_each_selected_object_from_its_own_current_value() {
    let mut app = app().await;
    let id_a = first_object_id(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [10.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let id_b = projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("added object id").to_string();
    assert_ne!(id_a, id_b, "the added object must be distinct from the first scene_snapshot object");
    let x_a_before = object_origin_x(&app, &id_a);
    let x_b_before = object_origin_x(&app, &id_b);
    assert_ne!(x_a_before, x_b_before, "the two objects must start at different x values for this test to prove per-object offset preservation");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [id_a.clone(), id_b.clone()], "field": "origin.x", "delta": 3.0 })), None).await.expect("patchInspector");
    assert_eq!(object_origin_x(&app, &id_a), x_a_before + 3.0, "a delta edit adds to each object's own current x");
    assert_eq!(object_origin_x(&app, &id_b), x_b_before + 3.0, "a delta edit preserves each object's own starting offset");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the inspector chrome no longer
/// renders per-entity stepper controls to inspect (see the sibling test's doc comment above), so
/// this now proves the same "resolve selection without embedding ids" contract one level down, at
/// `patchInspector`'s own `interaction.selection(vortex)` fallback (`commands::patch_inspector`) —
/// a bare `field`/`value` patch (no `ids` arg) must resolve against whatever the `vortex` domain's
/// `object` granularity currently holds.
#[semio_framework_async_macros::async_test]
async fn inspector_field_actions_resolve_selection_without_embedding_ids() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector without ids");
    assert_eq!(object_origin_x(&app, &object_id), 42.5, "patchInspector must resolve the patched object from the live selection, not an embedded id");
}
//#endregion 🔖️Inspector

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn app_definition_has_the_main_world_window() {
    let definition = create_puzzle3d_app();
    assert!(definition.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_the_add_object_dialog() {
    let definition = create_puzzle3d_app();
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    assert_eq!(dialog.submit_action.as_str(), "addObjectKind");
    assert_eq!(dialog.args.len(), 1);
}

/// 📌️ The four declared panel tabs are present (the framework injects its own tabs alongside, so
/// this asserts presence, never a total count).
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_its_four_panel_tabs() {
    let definition = create_puzzle3d_app();
    let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
    for expected in [artifact::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(body_keys.contains(&expected), "panel tab body {expected} must be declared, got {body_keys:?}");
    }
}

/// 🌉️ Every declared action must bridge through `command_from_action` and round-trip
/// `command_id` via the shared framework harness.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_bridges_to_a_command() {
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_tests).await;
    assert!(Puzzle3dPlayApp::command_from_action("noSuchAction", None, None).is_err());
}

/// 🌉️ Every declared app action (framework-injected verbs never reach `Puzzle3dCommand::from_action`
/// by design, so they simply fall through the `None` branch below) round-trips through the
/// macro-generated `Puzzle3dCommand::from_action`/`action_id` pair as well as the `ArtifactApp`
/// bridge asserted above.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_round_trips_through_the_command_enum() {
    let definition = create_puzzle3d_app();
    for action in dispatchable_actions(&definition) {
        let Some(command) = Puzzle3dCommand::from_action(&action.id, None, None) else {
            continue;
        };
        assert_eq!(command.action_id(), action.id.as_str(), "declared action {} must round-trip through Puzzle3dCommand", action.id);
    }
}

/// 🗣️ B1: manifest text is baked into `AppDefinition`/`App` as `LocalizedLabel` and resolved
/// directly via `.resolve(Terminology, Locale)` — no shell round-trip needed to assert on it.
#[semio_framework_async_macros::async_test]
async fn app_definition_labels_resolve_german_reuse_branded_for_aggregator() {
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    let definition = create_puzzle3d_app();
    let def = &definition;
    let (terminology, locale) = (Terminology::Reuse, Locale::De);
    let actions = || dispatchable_actions(def).into_iter();
    let action = |id: &str| actions().find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
    assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Bearbeiten");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Aggregator");
    let dialog = def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog");
    assert_eq!(dialog.title.resolve(terminology, locale), "Baukomponente hinzufügen");
    assert_eq!(dialog.submit_label.resolve(terminology, locale), "Hinzufügen");
    // 🗂️ The select's own label is the terminology-aware half; its OPTIONS are the declared examples'
    // catalog rows (`puzzle3d_object_kind_options`), carried as `LocalizedLabel::data` — document data,
    // never authored UI text, so a row reads identically on every axis. The literal `"Object"` option
    // this law used to assert on was the hardcoded placeholder that catalog derivation replaced.
    let arg = dialog.args.iter().find(|entry| entry.id == "objectKind").expect("objectKind arg");
    assert_eq!(arg.label.resolve(terminology, locale), "Art");
    match arg.control() {
        semio_framework_plugin::ActionArgControl::Select { options } => {
            assert!(!options.is_empty() && options.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "the objectKind select offers the declared examples' bounded catalog rows; observed {}", options.len());
            for option in options {
                assert!(!option.value.is_empty(), "every offered object kind names a catalog row");
                assert_eq!(option.label.resolve(terminology, locale), option.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "catalog row {} is document data and must not be re-authored per axis", option.value);
            }
        }
        _ => panic!("objectKind arg is not a select"),
    }
    assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Baukomponente hinzufügen");
    assert_eq!(action("openVortexSuggestions").label.resolve(terminology, locale), "Verbindungspunkt-Vorschläge öffnen");
    assert_eq!(action("createAttraction").label.resolve(terminology, locale), "Verbindung erstellen");
    assert_eq!(def.utilities.iter().find(|entry| entry.id == utilities::transform::UTILITY_ID).expect("transform utility").label.resolve(terminology, locale), "Transformieren");
    // 🎭️✏️ `create_puzzle3d_app()` no longer registers examples (see its own doc comment — `Editor::builder`
    // has no `.example(...)` and `AppDefinition` carries no `examples` field), so the concrete-forest
    // example's German label can no longer be asserted here; dropped, not silently left stale.
    let framework_interaction_actions = [
        INTERACTION_SELECT_ACTION_ID,
        semio_framework_plugin::INTERACTION_HOVER_ACTION_ID,
        semio_framework_plugin::CLEAR_SELECTION_ACTION_ID,
        semio_framework_plugin::SELECT_ALL_ACTION_ID,
        semio_framework_plugin::SET_SELECTION_MODE_ACTION_ID,
        semio_framework_plugin::SET_INTERACTION_GRANULARITY_ACTION_ID,
    ];
    for entry in actions() {
        if framework_interaction_actions.contains(&entry.id.as_str()) {
            continue;
        }
        let text = entry.label.resolve(terminology, locale);
        assert!(!text.contains("Hover") && !text.contains("Pick") && !text.contains("hovern"), "leftover English/mistranslation in {}: {text}", entry.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn app_definition_labels_stay_english_native_without_brand_locks() {
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    let definition = create_puzzle3d_app();
    let def = &definition;
    let (terminology, locale) = (Terminology::Native, Locale::En);
    let action = |id: &str| dispatchable_actions(def).into_iter().find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
    assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Edit");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Puzzle 3D");
    assert_eq!(def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog").title.resolve(terminology, locale), "Add Object");
    assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Add Object");
}

#[semio_framework_async_macros::async_test]
async fn document_and_kinds_trees_use_german_reuse_section_labels() {
    let mut app = app().await;
    // 🗣️ Panels resolve their label set from the host's own axes and fail closed on an unauthored one
    // (`puzzle3d_labels`), so the German reuse text this law is about only exists once the axes name it.
    app.set_label_axes(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Reuse);
    let document_json = render_body(&mut app, artifact::BODY_KEY).await.to_string();
    let kinds = render_body(&mut app, catalogue::BODY_KEY).await.to_string();
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_json = to_json_string(&app.window_measures(&view).await);
    assert!(document_json.contains("Baukomponenten"), "document tree objects section");
    assert!(document_json.contains("Verbindungen"), "document tree attractions section");
    assert!(document_json.contains("Referenzen"), "document tree references section");
    assert!(document_json.contains("Zielvolumina"), "document tree target volumes section");
    assert!(kinds.contains("Kabel"), "catalogue cables section");
    assert!(kinds.contains("Verbindungen"), "catalogue attractions section");
    assert!(!document_json.contains("\"Attractions\"") && !kinds.contains("\"Attractions\""), "English Attractions must not appear");
    assert!(!kinds.contains("\"Cables\""), "English Cables must not appear");
    assert!(measures_json.contains("Verbindungen"), "select measures attractions toggle");
    assert!(!measures_json.contains("\"Attractions\""), "select measures must not hardcode Attractions");
}

#[semio_framework_async_macros::async_test]
async fn main_window_utilities_lead_with_transform_without_select_tool_and_no_default_utility() {
    let definition = create_puzzle3d_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert!(!utility_ids.contains(&"select"), "puzzle 3d must not declare a select utility");
    assert!(!utility_ids.contains(&"scale"), "puzzle 3d must not declare a scale utility");
    assert!(!utility_ids.contains(&semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID), "fill is a mode-level tool, not a window utility");
    let window = definition.window_kinds.iter().find(|window| window.id == main::WINDOW_KIND_ID).expect("main window");
    let main_utilities: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(main_utilities.first().copied(), Some(utilities::transform::UTILITY_ID));
    assert!(!main_utilities.contains(&"select"));
    assert!(!main_utilities.contains(&semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID), "fill must not be bound to the main window as a utility");
    assert_eq!(PUZZLE3D_DEFAULT_UTILITY, "", "unset/cleared host utility must not impersonate transform");
}

/// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
#[semio_framework_async_macros::async_test]
async fn tool_registry_declares_fill_tool() {
    let definition = create_puzzle3d_app();
    let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
    assert_eq!(tool_ids, vec![semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID]);
    assert_eq!(definition.modes[0].tools, vec![ToolRef::new(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).await]);
    assert!(dispatchable_actions(&definition).iter().any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
}

/// 🛠️ Wave W-AB, restated by wave B31: an empty `setActiveTool` is a DISARM, so the only verb allowed to
/// emit one is the one whose whole meaning is "leave what is armed". Background and parameter traffic that
/// merely happens WHILE fill is armed must never emit it — a bounce there disarms the tool under the user's
/// hands mid-plan. `engagementAbort` is the exception and states it deliberately
/// (`🎮️commands/🛑️engagement-abort/🦀️.rs`, and
/// `escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool`): before B31 it was listed here as
/// a bounce too, which is exactly why Escape could never leave Fill in the browser.
#[semio_framework_async_macros::async_test]
async fn fill_flow_only_disarms_the_tool_when_the_user_aborts() {
    let empty_tool = |result: &semio_framework_plugin::InvocationResult| {
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty()))
    };
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("arm fill");
    let count = dispatch(&mut app, "setFillCount", Some(&json!({ "count": 3 })), None).await.expect("setFillCount");
    assert!(!empty_tool(&count), "adjusting the count mid-plan must not bounce-disarm: {:?}", count.requested_effects);
    let abort = dispatch(&mut app, "engagementAbort", None, None).await.expect("engagementAbort");
    assert!(empty_tool(&abort), "engagementAbort IS the disarm — it must emit the empty tool effect: {:?}", abort.requested_effects);
}
//#endregion 🔖️Manifest

//#region 🔖️Suggestions
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_vortex_and_prepends_suggest_objects() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("Suggest objects"), "menu should be {menu_json}");
    assert!(menu_json.contains("openVortexSuggestions"));
    assert!(menu_json.contains("sparkles"), "menu should include suggest icon: {menu_json}");
    assert!(menu_json.contains("Zoom to selection"), "menu should include zoom: {menu_json}");
    assert!(menu_json.contains("deleteSelection"), "menu should include delete: {menu_json}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden() {
    let mut app = app().await;
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("setTargetVolumeFlag"), "menu should be {menu_json}");
    assert!(menu_json.contains("menu.group.targets"), "hide/lock rows should be grouped under targets: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "destructive delete must be the last top-level row: {menu_json}");
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id.as_str(), "flag": "hidden", "value": true })), None).await.expect("setTargetVolumeFlag");
    let hidden = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("hidden")).and_then(Value::as_bool);
    assert_eq!(hidden, Some(true));
}

/// 🗂️ Grouped-disclosure contract for the object-selection branch: the top-level menu stays
/// scannable (leaves + groups + separator combined) and the destructive `deleteSelection` row is
/// the last top-level entry (`organize_context_menu` inserts the separator ahead of it).
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_object_groups_flags_and_keeps_delete_last() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    assert!(menu.len() <= 9, "top-level menu should stay scannable, got {} rows: {menu:?}", menu.len());
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("menu.group.hand"), "hide/lock rows should be grouped under hand: {menu_json}");
    assert!(menu_json.contains("duplicateSelection"), "menu should be {menu_json}");
    assert_eq!(menu.last().map(|item| item.id.as_str()), Some("delete"), "delete must be the last top-level row: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "delete must be marked destructive: {menu_json}");
}

/// 🎯️ Wave B11 (checklist §15, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 8): a right-click
/// on an object the document had NOT selected produced an empty plugin menu, so the shell's own fallback
/// ("Set Active Example", "Export", …) took over the viewport and none of `duplicate`/`select-same-kind`/
/// `zoom`/`hide-show`/`lock-unlock`/`delete` was ever reachable. `ContextMenuSurfaceTarget.hits` is the
/// pointer target the host already sends; the menu must read it, not only `selection`.
#[semio_framework_async_macros::async_test]
async fn right_clicking_an_unselected_object_opens_that_object_menu_not_an_empty_one() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let menu = context_menu_for_hit(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    let mut ids: Vec<String> = Vec::new();
    let mut walk: Vec<&semio_framework_plugin::ContextMenuItemSpec> = menu.iter().collect();
    while let Some(item) = walk.pop() {
        ids.push(item.id.clone());
        walk.extend(item.children.iter().flatten());
    }
    for row in ["duplicate", "select-same-kind", "zoom", "hide-show", "lock-unlock", "delete"] {
        assert!(ids.iter().any(|id| id == row), "the object vocabulary must be reachable from a bare hit, {row} missing from {ids:?}");
    }
}

/// 🎯️ …and a hit INSIDE the live selection never narrows the menu to the one row under the cursor: the
/// whole selection stays the subject, so a "Delete (3 objects)" row can never silently become a one-object
/// delete just because the press landed on a particular member.
#[semio_framework_async_macros::async_test]
async fn a_hit_inside_the_selection_keeps_the_whole_selection_as_the_menu_subject() {
    use semio_framework_plugin::{ContextMenuHit, ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [4.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).expect("objects").iter().filter_map(|object| object.get("id").and_then(Value::as_str)).map(str::to_string).collect();
    assert!(ids.len() >= 2, "this law needs a multi-object selection, got {ids:?}");
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget {
            surface_id: "world3d".into(),
            kind: "world3d".into(),
            hits: vec![ContextMenuHit { domain: PUZZLE3D_GRANULARITY_OBJECT.into(), id: ids[0].clone(), label: None }],
            selection: vec![ContextMenuSelectionGroup { domain: PUZZLE3D_GRANULARITY_OBJECT.into(), ids: ids.clone() }],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let view = app.window_view(main::WINDOW_KIND_ID);
    let menu = app.context_menu(&request, &view).await;
    let delete = menu.iter().find(|item| item.id == "delete").expect("the object menu carries a delete row");
    let label = delete.label.clone().unwrap_or_default();
    assert!(label.contains(&ids.len().to_string()), "the delete row must still name the whole selection, got {label:?} for {ids:?}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_opens_the_suggestion_popup() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let result = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 34.0 })), None).await.expect("openVortexSuggestions");
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "opening a one-shot suggestion must not switch the host-owned utility or tool: {:?}",
        result.requested_effects,
    );
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "context-menu suggestion stays in the current selection mode");
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("open").and_then(Value::as_bool), Some(true));
    assert_eq!(menu.get("x").and_then(Value::as_f64), Some(12.0));
    assert_eq!(menu.get("y").and_then(Value::as_f64), Some(34.0));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
    assert!(menu.get("windowId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "suggestion menu is scoped to the opening window: {menu}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_records_explicit_window_id() {
    // 🪟️ The popup is transient state of the pane the user acted in (the dispatch's own window
    // authority), and it RECORDS the explicit target pane the args named — so the assertion renders
    // the acting pane and reads the recorded `windowId` back out of it.
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 8.0, "y": 16.0, "windowId": main::WINDOW_INSTANCE_TOP })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("openVortexSuggestions");
    let interaction = interaction_of(&render_window(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await);
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("windowId").and_then(Value::as_str), Some(main::WINDOW_INSTANCE_TOP));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_with_full_id_places_even_if_selection_was_cleared() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    let before_count = object_count(&app);
    // 🧹️ Simulate the split-pane outside-dismiss race clearing vortex selection before accept.
    dispatch(&mut app, "clearSelection", None, None).await.expect("clearSelection");
    let result = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex.as_str() })), None).await.expect("acceptSuggestion");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accept must not switch utility/tool: {:?}", result.requested_effects);
    assert!(object_count(&app) > before_count, "accept with fullId must place even after selection clear");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_the_menu() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🖱️ Hovering a row in the suggestion popup moves the tracked candidate index an accept places, without
/// switching the host-owned active utility into brush mode. The popup lists the free candidates the brush
/// suggestions run found, each carrying its object kind's colour and icon.
#[semio_framework_async_macros::async_test]
async fn hover_suggestion_moves_the_candidate_index_over_the_free_candidates_the_run_found() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "suggestion hover must not enter brush mode");
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(0), "opening suggestions starts hover at the first candidate");
    assert_eq!(interaction.pointer("/suggestionMenu/pending").and_then(Value::as_bool), Some(false), "a settled search is no longer pending");
    let candidates = interaction.pointer("/suggestionMenu/candidates").and_then(Value::as_array).cloned().unwrap_or_default();
    assert!(!candidates.is_empty(), "suggestion candidates should be present");
    assert!(candidates[0].get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "candidates carry object-kind color: {candidates:?}");
    assert!(candidates[0].get("icon").and_then(Value::as_str).is_some_and(|icon| !icon.is_empty()), "candidates carry icon: {candidates:?}");
    assert!(interaction.get("brushPreviewJson").is_none(), "no host-painted ghost: the run's trace shows every candidate");
    dispatch(&mut app, "hoverSuggestion", Some(&json!({ "index": 1 })), None).await.expect("hoverSuggestion");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(1), "hovering a different row must move the tracked candidate index");
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_appends_an_object_and_closes_the_menu() {
    let mut app = app().await;
    let object_count_before = object_count(&app);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    let result = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before + 1);
    let placed = projection_of(&app).pointer("/objects").and_then(Value::as_array).and_then(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str)).find(|id| id.starts_with("puzzle3d.suggestion."))).map(str::to_string).expect("the placed object");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!((selection.granularity.as_str(), selection.ids), (PUZZLE3D_GRANULARITY_OBJECT, vec![placed]), "accept selects the object it placed");
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "accepting a one-shot suggestion must leave the host-owned utility/tool unchanged: {:?}",
        result.requested_effects,
    );
    let composite = render_composite(&mut app).await;
    let interaction = interaction_of(&composite);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
    assert!(interaction.get("hoveredVortexFullId").is_none_or(|value| value.is_null()), "accept must clear sticky vortex hover");
    let selected_vortices = vortices_of(&composite).iter().filter(|entry| entry.get("selected").and_then(Value::as_bool) == Some(true)).count();
    assert_eq!(selected_vortices, 0, "one-shot accept must leave no sticky vortex selection");
}

/// 🧹️ A failed place (unknown vortex) must still close the suggestion menu — otherwise
/// `suggestionMenu.open` stays true and every split pane's regular context menu is gated shut.
#[semio_framework_async_macros::async_test]
async fn accept_suggestion_closes_menu_even_when_placement_fails() {
    // 🕹️ `hover_id` dispatches through the real `interactionHover` verb, which resolves the
    // `vortex` domain against `self.registry` — the scene_snapshot `app()` is registry-backed for exactly
    // this reason (see its own doc comment).
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
    let before = interaction_of(&render_composite(&mut app).await);
    assert_eq!(before.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    let object_count_before = object_count(&app);
    dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": "missing-object::missing-vortex.as_str()" })), None).await.expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before, "unknown-vortex accept must not place");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()), "failed accept must still dismiss the suggestion menu");
}

/// 📏️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave K: a world scene names its built-in meshes by
/// REFERENCE and never carries their tessellation, so mesh geometry cannot spend the fixed
/// `UiFixedBytes` surface payload. Before this wave `world3d_mesh_kind_entry` inlined
/// `mesh_from_kind`'s buffers, and `vortex-marker` alone (an 80-triangle ico sphere printed as JSON
/// floats) took ~26 KiB of the 32 KiB payload — so a window that also carried a resolved suggestion
/// popup failed `scene-surface.encode` outright. Measured on the live popup-open scene AND on the
/// Nakagin catalog, whose 180 objects are the widest mesh set this editor can name.
#[semio_framework_async_macros::async_test]
async fn the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
    let node = render_composite(&mut app).await;
    assert_eq!(interaction_of(&node).pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "the law only measures the popup-open scene");
    let live_meshes = scene_meshes_of(&node);
    let (payload, capacity) = world_surface_payload_bytes(&mut app, main::BODY_KEY).await;
    assert!(payload <= capacity, "the popup-open world scene packs to {payload} bytes, over the fixed surface capacity {capacity}");
    for meshes in [live_meshes, parse(&main::world_meshes_json(&nakagin_scene_snapshot()), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("nakagin meshes").as_array().cloned().expect("mesh array")] {
        assert!(!meshes.is_empty(), "a world scene always declares its meshes");
        assert!(to_json_string(&meshes).len() <= PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET, "mesh references must stay a rounding error against the {capacity}-byte surface payload; observed {}", to_json_string(&meshes).len());
        for mesh in &meshes {
            assert!(mesh.get("id").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "every scene mesh is identified: {mesh}");
            assert!(mesh.get("data").is_none(), "a scene mesh must never carry inline geometry: {mesh}");
            assert!(mesh.get("kind").and_then(Value::as_str).is_some() || mesh.get("url").and_then(Value::as_str).is_some(), "every scene mesh resolves by kind or by url: {mesh}");
        }
        assert!(meshes.iter().any(|mesh| mesh.get("kind").and_then(Value::as_str) == Some("vortex-marker")), "the vortex marker is the kind that used to blow the payload and must still be declared");
    }
}

//#region 🚚️PagedSceneCarrier
/// 🚚️ The one language-neutral declaration of the world-3d lane carrier — the SAME file the scene
/// crate's Rust lane table and `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`'s `WORLD3D_SCENE_LANES` are
/// pinned against, read here so the real Nakagin publication is measured against the same bounds the
/// TypeScript assembler enforces.
static PUZZLE3D_WORLD3D_LANE_CONTRACT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json")).expect("committed test data reads"));

/// 🗼️ Loads the Nakagin Capsule Tower example through the REAL typed `setActiveExample` command and
/// drives it to quiescence, exactly as the navbar picker does.
async fn nakagin_app() -> Puzzle3dApp {
    let mut app = app().await;
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample is a declared puzzle3d command");
    let action_meta = app.meta("local");
    app.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("setActiveExample mints its retained whole-document operation");
    settle(&mut app).await;
    app
}

/// 🚚️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave P — LAW (a). The Nakagin Capsule Tower with its
/// vortex suggestion popup open is the document that made `scene-surface.encode` refuse the whole
/// refresh (`surface payload exceeds fixed capacity with 57281 bytes`, browser rebuild #25). It now
/// publishes as a small spine inside the fixed-capacity doc plus one individually paged carrier per
/// payload lane, and every one of those carriers must obey the SAME bounds the TypeScript assembler
/// enforces on the other side of the wire — leaf bytes, children per node, and a byte/hash manifest
/// that lets a partially arrived lane be told from a settled one.
#[semio_framework_async_macros::async_test]
async fn the_nakagin_world_scene_publishes_every_lane_under_the_page_cap_with_the_popup_open() {
    let contract: Value = parse(PUZZLE3D_WORLD3D_LANE_CONTRACT.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("world-3d lane contract is json");
    let leaf_bytes = contract.pointer("/carrier/leafBytes").and_then(Value::as_u64).expect("leafBytes") as usize;
    let children_max = contract.pointer("/carrier/childrenMax").and_then(Value::as_u64).expect("childrenMax") as usize;
    let doc_bytes_max = contract.pointer("/carrier/docBytesMax").and_then(Value::as_u64).expect("docBytesMax") as usize;

    let mut app = nakagin_app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");

    let census = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    assert_eq!(census.capacity, doc_bytes_max, "the doc ceiling this law measures against is the contract's own");
    assert!(census.doc_bytes <= census.capacity, "the Nakagin spine packs to {} bytes, over the fixed surface capacity {}", census.doc_bytes, census.capacity);
    assert!(!census.lanes.is_empty(), "a published world scene always carries its payload lanes");

    let declared: Vec<&str> = contract["lanes"].as_array().expect("lanes").iter().map(|lane| lane["bodyKey"].as_str().expect("bodyKey")).collect();
    for lane in &census.lanes {
        assert!(declared.contains(&lane.key.as_str()), "lane carrier {} is not a declared world-3d lane", lane.key);
        assert!(lane.widest_leaf <= leaf_bytes, "lane {} has a {}-byte text leaf, over the {leaf_bytes}-byte cap", lane.key, lane.widest_leaf);
        assert!(lane.widest_children <= children_max, "lane {} has a node with {} children, over the {children_max} cap", lane.key, lane.widest_children);
        assert_eq!(lane.bytes, lane.declared_bytes as usize, "lane {} carrier text disagrees with the byte count its spine declared", lane.key);
        assert_eq!(lane.declared_hash.len(), 16, "lane {} must declare a 16-hex-digit content hash", lane.key);
        assert!(lane.leaves >= 1, "lane {} publishes at least one leaf", lane.key);
        assert_eq!(lane.leaves > children_max, lane.leaf_depth > 1, "lane {} with {} leaves must page into a nested carrier exactly when it outgrows one level of {children_max} children (observed leaf depth {})", lane.key, lane.leaves, lane.leaf_depth);
    }

    let interaction = parse(census.assembled.interaction_json.as_deref().expect("the interaction lane reassembles"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("interaction lane is json");
    assert_eq!(interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "the law only measures the popup-open scene");
    let instances = census.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).expect("the instances lane always publishes");
    assert!(parse(&census.assembled.instances_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances lane is json").as_array().is_some_and(|array| array.len() > 100), "Nakagin publishes its whole catalog of objects through the instances lane");
    eprintln!(
        "nakagin popup-open world scene: spine={}B of {}B, {} lanes carrying {}B total ({}), widest lane={}B in {} leaves",
        census.doc_bytes,
        census.capacity,
        census.lanes.len(),
        census.payload_bytes(),
        census.report(),
        instances.bytes,
        instances.leaves
    );
    eprintln!("nakagin lane paging: {}", census.lanes.iter().map(|lane| format!("{}={}leaves@depth{}", lane.key.trim_start_matches(semio_framework_plugin::WORLD3D_SCENE_LANE_KEY_PREFIX), lane.leaves, lane.leaf_depth)).collect::<Vec<_>>().join(" "));
    assert!(census.payload_bytes() > census.capacity, "this law is only meaningful while the Nakagin payload is past what one fixed doc could ever hold");
}

/// 🚚️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave P — LAW (b). A partial refresh that changes only the
/// camera must leave every lane carrier byte-identical, so the reconciler emits no op for any of
/// them; `instances`, `interaction` and `lod` have very different change rates and the whole point of
/// splitting them is that an unchanged lane costs nothing.
#[semio_framework_async_macros::async_test]
async fn a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh() {
    let mut app = nakagin_app().await;
    let before = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [80.0, 80.0, 80.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), Some(main::WINDOW_KIND_ID)).await.expect("setCamera");
    let moved = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let republished: Vec<&str> = moved.lanes.iter().filter(|lane| before.lane(&lane.key).is_none_or(|previous| previous.declared_hash != lane.declared_hash)).map(|lane| lane.key.as_str()).collect();
    assert!(republished.is_empty(), "a camera move republished {republished:?}");
    assert_ne!(moved.assembled.camera_json, before.assembled.camera_json, "the camera itself must have moved for this law to mean anything");

    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("interactionSelect");
    let picked = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let changed: Vec<&str> = picked.lanes.iter().filter(|lane| before.lane(&lane.key).is_none_or(|previous| previous.declared_hash != lane.declared_hash)).map(|lane| lane.key.as_str()).collect();
    assert!(!changed.contains(&semio_framework_plugin::World3dSceneLane::Instances.body_key()), "a selection change must not republish the instances lane");
    assert_eq!(picked.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).map(|lane| lane.declared_hash.as_str()), before.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).map(|lane| lane.declared_hash.as_str()));
}
//#endregion 🚚️PagedSceneCarrier

/// 📏️ A whole scene's mesh declarations, as references, against the 32 KiB fixed surface payload:
/// generous enough for the widest catalog the editor can name, tight enough that a single inlined
/// primitive (26 KiB) fails it.
const PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET: usize = 4 * 1024;

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_sticky_hover() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🧰️ Context-menu / Alt+right-click suggestions are a one-shot placement: opening and accepting
/// must leave whatever host-owned utility was already active (e.g. transform) untouched.
#[semio_framework_async_macros::async_test]
async fn open_and_accept_vortex_suggestions_preserve_active_utility() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("activate transform");
    let vortex = first_vortex_full_id(&app);
    let open = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), Some(main::WINDOW_KIND_ID)).await.expect("openVortexSuggestions");
    assert!(open.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "opening suggestions must not emit utility/tool switches: {:?}", open.requested_effects);
    let open_node = render_window(&mut app, main::WINDOW_KIND_ID).await;
    let open_interaction = interaction_of(&open_node);
    assert_eq!(open_interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "transform remains non-brush scene mode during suggestions");
    assert_eq!(open_interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    let accept = dispatch(&mut app, "acceptSuggestion", None, Some(main::WINDOW_KIND_ID)).await.expect("acceptSuggestion");
    assert!(accept.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accepting suggestions must not emit utility/tool switches: {:?}", accept.requested_effects);
    let accept_interaction = interaction_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert!(accept_interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(accept_interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
}

/// 🎰️ Wave B6: `pending_reserved` is direct-mapped on `job % 64`, and a reserved verb that is NOT an
/// interaction (`noteShellCommand` — the shell records one for every user command) holds its residue
/// class until it settles. `retire_pending_reserved_latest_wins` may only retire interaction pendings,
/// so the colliding class used to fault the whole pick with `framework route 'interactionSelect' has no
/// exact pending spawn slot` (browser battery #44-pre) while 63 classes stood empty. 64 rounds cover
/// every residue class, so at least one of them mints into the occupied one.
#[semio_framework_async_macros::async_test]
async fn interaction_admits_in_every_residue_class_while_a_reserved_verb_stays_pending() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    dispatch_reserved_unsettled(&mut app, "noteShellCommand", Some(&json!({ "commandId": "framework.shell.b6-probe", "label": "Probe" })), None).await.expect("a non-interaction reserved verb admits and holds its residue class");
    for round in 0..64 {
        let admitted = select_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.unwrap_or_else(|error| panic!("round {round} must mint into a vacant residue class: {error:?}"));
        settle_reserved(&mut app, admitted).await.unwrap_or_else(|error| panic!("round {round} must commit: {error:?}"));
    }
}

/// 🖱️ Wave W-AB: a vortex pointermove storm admits 70 `interactionHover`s (then a click
/// `interactionSelect`) before Isolated reserved jobs finish — the #38 spawn-admit drop. Latest-wins keeps one
/// pending hover, so the armed brush's suggestions run searches the last target and a brush click places one of
/// its free candidates.
#[semio_framework_async_macros::async_test]
async fn vortex_hover_storm_admits_then_the_brush_run_searches_and_a_click_places() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let vortex = first_vortex_full_id(&app);
    let before = object_count(&app);
    let mut last = None;
    for _ in 0..70 {
        last = Some(hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("vortex hover storm must keep admitting"));
    }
    settle_reserved(&mut app, last.expect("storm admitted at least one hover")).await.expect("latest hover must commit");
    let select_admit = select_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("vortex click select must admit after the hover storm");
    settle_reserved(&mut app, select_admit).await.expect("select commit");
    target_brush_suggestions(&mut app, &vortex).await;
    assert!(tool_run_trace_records(&app).iter().any(|(_, verdict)| *verdict == semio_framework_tool_run::ToolRunVerdict::Success), "the storm's target has a free candidate on its trace");
    dispatch(&mut app, "acceptSuggestion", Some(&json!({ "fullId": vortex.as_str() })), None).await.expect("a brush click accepts");
    assert_eq!(object_count(&app), before + 1, "a brush click places exactly one free candidate");
}

/// 🪟️ Wave B9 lane 1: `ViewModel.active_utility_by_window_id` is keyed by window INSTANCE
/// (`puzzle3d-main-perspective`), and `plugin_refresh_ui` renders EVERY instance under the window
/// KIND's one body key — the instance identity reaches the guest only through
/// `ViewModel::for_window_instance`'s `window_id`. Resolving the roster's first pane instead
/// published that pane's armed utility into every other pane's world lane, which is why the browser
/// read `utility=select preview=null` in the pane the user had armed Brush in.
#[semio_framework_async_macros::async_test]
async fn each_window_instance_publishes_its_own_armed_utility_into_its_world_lane() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    let top = "puzzle3d-main-top";
    let arm = |utility: &'static str, window: &'static str| json!({ "utilityId": utility, "windowId": window });
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&arm(utilities::volume_brush::UTILITY_ID, main::WINDOW_KIND_ID)), Some(main::WINDOW_KIND_ID)).await.expect("arm the base pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&arm(utilities::brush::UTILITY_ID, perspective)), Some(perspective)).await.expect("arm brush in the perspective pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "", "windowId": top })), Some(top)).await.expect("the top pane stays unarmed");
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    let armed = render_window_refresh(&mut app, main::BODY_KEY, perspective).await;
    let unarmed = render_window_refresh(&mut app, main::BODY_KEY, top).await;
    let utility_of = |node: &Value| interaction_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(utility_of(&armed), "brush", "the pane the user armed must publish its OWN utility: {}", interaction_of(&armed));
    assert_eq!(utility_of(&unarmed), "select", "an unarmed pane must not inherit another pane's utility: {}", interaction_of(&unarmed));
}

/// 🪟️ Wave B39: the OTHER host route — `plugin_render_surface`'s `<body>:<windowInstanceId>` form,
/// the one a mounted surface is rendered through (`🪟️surfaces/🦀️.rs`'s per-surface binding). Two
/// mounted instances of one window kind must publish two DISTINCT world bodies, each carrying its
/// own armed utility, so the host's two `📃️UiDocumentStore`s never hold the same record: a body key
/// that resolved the roster's first pane made both surfaces publish one tree, and the host cannot
/// tell two identical trees apart.
#[semio_framework_async_macros::async_test]
async fn two_mounted_instances_publish_two_distinct_world_bodies_with_their_own_utilities() {
    let mut app = app().await;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    let top = main::WINDOW_INSTANCE_TOP;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID, "windowId": perspective })), Some(perspective)).await.expect("arm brush in the perspective pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "", "windowId": top })), Some(top)).await.expect("the top pane stays unarmed");
    let armed = render_window(&mut app, perspective).await;
    let unarmed = render_window(&mut app, top).await;
    let utility_of = |node: &Value| interaction_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(utility_of(&armed), utilities::brush::UTILITY_ID, "the armed pane's own surface body must carry its utility: {}", interaction_of(&armed));
    assert_eq!(utility_of(&unarmed), "select", "the sibling pane's surface body must stay unarmed: {}", interaction_of(&unarmed));
    assert_ne!(armed, unarmed, "two mounted instances must publish two distinct world bodies, never one shared tree");
}

/// 🖱️ Wave W-AB: after a committed vortex hover, Alt+right-click (`openVortexSuggestions` with the
/// hovered `fullId`) publishes `suggestionMenu.open` — the guest half of the host gesture.
#[semio_framework_async_macros::async_test]
async fn vortex_hover_then_open_vortex_suggestions_publishes_menu() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let mut last = None;
    for _ in 0..70 {
        last = Some(hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("suggestion hover storm must keep admitting"));
    }
    settle_reserved(&mut app, last.expect("storm")).await.expect("latest hover commits for suggestions");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 24.0 })), None).await.expect("openVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "vortex hover + openVortexSuggestions must publish the menu: {interaction}");
    assert_eq!(interaction.pointer("/suggestionMenu/vortexFullId").or_else(|| interaction.pointer("/suggestionMenu/vortex_full_id")).and_then(Value::as_str), Some(vortex.as_str()));
}

/// ⏯️ Lane W2-C: the armed brush pointing at a vortex starts a READ-ONLY tool run. Every candidate it tests is a
/// trace record with its final verdict, the committed document never moves and no undo entry appears; pointing at
/// another vortex retargets the same run; leaving every vortex aborts it without a trace in the document.
#[semio_framework_async_macros::async_test]
async fn the_armed_brush_runs_a_read_only_search_that_traces_every_candidate_and_leaving_aborts_it() {
    use semio_framework_tool_run::ToolRunVerdict;
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let committed = committed_document(&app);
    let vortices = vortex_full_ids(&app);
    let (first, last) = (vortices.first().cloned().expect("a vortex"), vortices.last().cloned().expect("a vortex"));
    assert!(app.tool_run_presence().is_none(), "arming the brush alone starts nothing");
    target_brush_suggestions(&mut app, &first).await;
    let presence = app.tool_run_presence().expect("a brush run");
    assert_eq!(presence.tool_id, utilities::brush::UTILITY_ID);
    let records = tool_run_trace_records(&app);
    assert_eq!(records.len() as u64, presence.total.expect("a counted search"), "every candidate of the target is on the trace");
    assert!(records.iter().all(|(_, verdict)| matches!(verdict, ToolRunVerdict::Success | ToolRunVerdict::Danger | ToolRunVerdict::Warning)), "a settled search leaves no candidate under test: {records:?}");
    assert_eq!(committed_document(&app), committed, "a read-only run never touches the document");
    let run = tool_run_trace_run(&app);
    target_brush_suggestions(&mut app, &last).await;
    assert_eq!(tool_run_trace_run(&app), run, "a new target retargets the same run");
    dispatch(&mut app, "targetBrushSuggestions", Some(&json!({})), Some(main::WINDOW_KIND_ID)).await.expect("leave every vortex");
    pump_brush_suggestions(&mut app, "leaving aborts the run", |presence| presence.is_some_and(|presence| presence.state == protocol::PresenceToolRunState::Aborted)).await;
    assert_eq!(committed_document(&app), committed, "the aborted search left the document byte-identical");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "and left no undo entry behind");
    target_brush_suggestions(&mut app, &first).await;
    assert_ne!(tool_run_trace_run(&app), run, "pointing at a vortex again starts a fresh run");
}

/// 🔒️ Lane W2-C: a suggestion popup opened outside brush mode starts the search for its vortex, and closing the
/// popup aborts it; accepting places exactly one free candidate as ONE undo entry.
#[semio_framework_async_macros::async_test]
async fn the_popup_search_is_aborted_on_close_and_an_accept_is_one_undoable_placement() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    pump_brush_suggestions(&mut app, "the popup's search settles", brush_suggestions_settled).await;
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    pump_brush_suggestions(&mut app, "closing aborts the run", |presence| presence.is_some_and(|presence| presence.state == protocol::PresenceToolRunState::Aborted)).await;
    let committed = committed_document(&app);
    let objects = object_count(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("reopen");
    pump_brush_suggestions(&mut app, "the reopened popup's search settles", brush_suggestions_settled).await;
    let (_, settled) = dispatch_reporting(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex.as_str() })), None).await;
    assert_eq!(object_count(&app), objects + 1, "an accept places exactly one candidate");
    assert_eq!(history_rows(&settled), 1, "an accept is one command row, got {:?}", history_row_labels(&settled));
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "one undo removes the accepted placement");
}

/// ⏯️ Lane W2-C: the brush declares a read-only `instance3d` tool run through the manifest, restarted by a document
/// or settings change, reading exactly the shared settings its job reads; its Utility Options keep no picker of
/// their own, and no suggestion tick survives.
#[test]
fn the_brush_utility_declares_a_read_only_tool_run_through_the_manifest() {
    use semio_s_artifact_puzzle_3d_model::BrushSuggestionsRunReason;
    use semio_framework_tool_run::{ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunTraceKind};
    let definition = create_puzzle3d_app();
    let brush = definition.utilities.iter().find(|utility| utility.id == utilities::brush::UTILITY_ID).expect("brush utility");
    let run = brush.run.as_ref().expect("the brush declares its ToolRunDefinition");
    run.validate().expect("the brush run definition validates");
    assert!(!run.mutating, "a candidate search never writes the document");
    assert_eq!((run.rebase, run.reconfigure, run.trace), (ToolRunRebasePolicy::Restart, ToolRunReconfigurePolicy::Restart, ToolRunTraceKind::Instance3d));
    assert_eq!((run.run_job.as_str(), run.revalidate_job.as_ref()), (utilities::brush::RUN_JOB_KIND, None));
    assert_eq!(run.settings.config, vec!["/contactTolerance", "/objectKindWeights", "/vortexKindWeights"]);
    for (reason, declared) in BrushSuggestionsRunReason::ALL.iter().zip(&run.reasons) {
        assert_eq!((declared.code, declared.id.as_str(), declared.verdict), (reason.code(), reason.id(), reason.verdict()));
    }
    let actions: Vec<&str> = dispatchable_actions(&definition).into_iter().map(|action| action.id.as_str()).collect();
    assert!(!actions.contains(&"suggestionsTick"), "the suggestions tick loop is gone");
    assert!(actions.contains(&"targetBrushSuggestions"));
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(find_measure_select(&main::window_measures(&scene, labels), "puzzle3d-brush-placement"), None, "the candidate search's readout is the framework ToolRun panel, not a picker");
}

/// leftover InteractionView after vortex-domain interactionHover must carry the vortex full id.
#[semio_framework_async_macros::async_test]
async fn interaction_hover_leftover_carries_vortex_full_id() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("vortex interactionHover admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("vortex interactionHover leftover");
    let view = settled.output.get("interactionView").expect("leftover InteractionView");
    let hover = view.get("hoverTarget").expect("hoverTarget on leftover");
    assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some(vortex.as_str()), "leftover hoverTarget must be the vortex full id, got {hover:?}");
    assert_eq!(hover.get("domain").and_then(semio_framework_value::DslValue::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "next scene render must project leftover hover onto hoveredVortexFullId");
}

//#endregion 🔖️Suggestions
