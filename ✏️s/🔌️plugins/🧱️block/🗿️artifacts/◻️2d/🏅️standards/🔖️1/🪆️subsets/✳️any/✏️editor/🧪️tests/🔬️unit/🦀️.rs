
use super::*;
use crate::editor::block2d::testkit::{Block2dApp, new_app};
use semio_framework_plugin::PluginApp;

//#region 🔖️CommandSurface
/// ⚖️ LAW: block2d's retained route table, its publication contracts, its bounded-first-step proofs
/// and the manifest's `Migrated` classifications are the SAME nine ids — the exact join the
/// framework's `validate_tool_job_rows` demands (`interactive-job.catalog-authority` /
/// `interactive-job.catalog-incomplete`). Mirrors generation2d's
/// `retained_route_dispositions_are_exact_and_exhaustive`.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    assert_eq!(BLOCK2D_RETAINED_TOOL_IDS.len(), 9);
    assert_eq!(<Block2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 9);
    assert_eq!(Block2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 9);
    assert_eq!(block2d_retained_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(block2d_retained_contract().cancellation, ToolCancellationPolicy::PerOperation);
    for tool_id in BLOCK2D_RETAINED_TOOL_IDS {
        let contract = Block2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == *tool_id).unwrap_or_else(|| panic!("publication contract for {tool_id}"));
        assert_eq!(contract.lanes, [ArtifactToolPublicationLane::Artifact].as_slice(), "every block2d handler emits document mutations only");
    }
    let definition = create_block2d_app();
    let migrated: Vec<&str> = every_command().iter().map(Block2dCommand::command_id).collect();
    assert_eq!(migrated.iter().copied().collect::<std::collections::BTreeSet<_>>(), BLOCK2D_RETAINED_TOOL_IDS.iter().copied().collect::<std::collections::BTreeSet<_>>());
    for tool_id in BLOCK2D_RETAINED_TOOL_IDS {
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} declared"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be UI-dispatchable");
    }
}

fn every_command() -> Vec<Block2dCommand> {
    vec![
        Block2dCommand::PatchNodeKind(patch_node_kind::PatchNodeKind { field: "name".into(), value: "x".into() }),
        Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {}),
        Block2dCommand::RemoveHandleKind(remove_handle_kind::RemoveHandleKind { id: "h0".into() }),
        Block2dCommand::AddHandle(add_handle::AddHandle {}),
        Block2dCommand::RemoveHandle(remove_handle::RemoveHandle { id: "h0".into() }),
        Block2dCommand::AddCompatibilityRule(add_compatibility_rule::AddCompatibilityRule { source: "a".into(), target: "b".into() }),
        Block2dCommand::RemoveCompatibilityRule(remove_compatibility_rule::RemoveCompatibilityRule { id: "c0".into() }),
        Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: "left".into() }),
        Block2dCommand::Edit(edit::Edit { text: "{}".into() }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(Block2dCommand::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 9, "every Block2dCommand row must be covered by every_command()");
}

#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let printed = protocol::OpText::print_op(&command);
        assert!(printed.starts_with(command.command_id()), "row {} printed {printed:?}", command.command_id());
    }
}

/// 🧷️ Pins the exact pre-migration bytes for the rows the `app_commands!` decomposition could have
/// silently rewritten — copied verbatim from the ticket's `🧪️wire-baseline-2d-before.txt`.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &Block2dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
    assert_eq!(hex(&Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {})), "01010000");
    assert_eq!(hex(&Block2dCommand::AddHandle(add_handle::AddHandle {})), "01030000");
}

/// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
/// `command_id`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Block2dPlayApp>>(testkit::block2d_app_manifest_for_testkit).await;
    assert!(Block2dPlayApp::command_from_action("noSuchAction", None).is_err());
}
//#endregion 🔖️CommandSurface

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_block2d_app();
    assert_eq!(definition.modes.len(), 1);
    assert_eq!(definition.window_kinds.len(), 1);
    for body_key in [document_panel::BLOCK2D_BODY_DOCUMENT, inspection_panel::BLOCK2D_BODY_INSPECTOR] {
        assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
    }
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `handle` domain is declared
/// once, with both granularities, a `Topology` hierarchy, and scoped to the board window kind.
#[semio_framework_async_macros::async_test]
async fn declares_the_handle_interaction_domain_scoped_to_the_board_window() {
    let definition = create_block2d_app();
    let interaction = definition.interactions.iter().find(|def| def.id == BLOCK2D_INTERACTION_HANDLE).expect("handle domain declared");
    assert_eq!(interaction.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec![BLOCK2D_GRANULARITY_HANDLE, BLOCK2D_GRANULARITY_HANDLE_KIND]);
    assert!(matches!(interaction.hierarchy, HierarchyProvider::Topology));
    let board_window = definition.window_kinds.iter().find(|window| window.id == board::BLOCK2D_WINDOW_BOARD).expect("board window declared");
    assert!(board_window.interactions.contains(&InteractionRef::new(BLOCK2D_INTERACTION_HANDLE)));
}

/// 🕹️ `interaction_topology` nests every handle under its own handle-kind, enabling both pruning
/// (`removeHandleKind`/`removeHandle`) and transitive hover from a kind to its handles.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_nests_handles_under_their_handle_kind() {
    let mut app: Block2dApp = new_app().await;
    testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {})).await;
    testkit::dispatch(&mut app, Block2dCommand::AddHandle(add_handle::AddHandle {})).await;
    let snapshot = app.snapshot().expect("snapshot");
    let kind_id = snapshot.handles[0].handle_kind.clone();
    let handle_id = snapshot.handles[0].id.clone();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = Block2dConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let topology = Block2dPlayApp::interaction_topology(&doc, &cfg);
    let domain = topology.domains.get(BLOCK2D_INTERACTION_HANDLE).expect("handle domain topology present");
    let handle_node = domain.ordered.iter().find(|node| node.id == format!("handle:{handle_id}")).expect("handle node present");
    assert_eq!(handle_node.parent.as_deref(), Some(format!("handleKind:{kind_id}").as_str()));
}

#[semio_framework_async_macros::async_test]
async fn block2d_io_is_wired_into_the_manifest() {
    let definition = create_block2d_app();
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
}

#[semio_framework_async_macros::async_test]
async fn block2d_io_declares_the_catalog_out_port() {
    let io = block2d_io();
    assert_eq!(io.document_schema, BLOCK_2D_SCHEMA);
    let ports = io.all_ports().await;
    let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
    assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(catalog.direction, MediaPortDirection::Out);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = new_app().await;
    assert!(testkit::render(&mut app, "block2d.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️Manifest

//#region 🔖️Behavior
#[semio_framework_async_macros::async_test]
async fn add_handle_kind_then_add_handle_then_remove_round_trips() {
    let mut app: Block2dApp = new_app().await;
    let booted = app.snapshot().expect("snapshot");
    let (kinds, handles) = (booted.handle_kinds.len(), booted.handles.len());
    let booted_ids: Vec<String> = booted.handles.iter().map(|handle| handle.id.clone()).collect();
    testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {})).await;
    assert_eq!(app.snapshot().expect("snapshot").handle_kinds.len(), kinds + 1);
    testkit::dispatch(&mut app, Block2dCommand::AddHandle(add_handle::AddHandle {})).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.handles.len(), handles + 1);
    let handle_id = projection.handles.iter().map(|handle| handle.id.clone()).find(|id| !booted_ids.contains(id)).expect("the added handle");
    testkit::dispatch(&mut app, Block2dCommand::RemoveHandle(remove_handle::RemoveHandle { id: handle_id })).await;
    assert_eq!(app.snapshot().expect("snapshot").handles.len(), handles);
}

#[semio_framework_async_macros::async_test]
async fn patch_node_kind_updates_name() {
    let mut app = new_app().await;
    testkit::dispatch(&mut app, Block2dCommand::PatchNodeKind(patch_node_kind::PatchNodeKind { field: "name".into(), value: "Renamed".into() })).await;
    assert_eq!(app.snapshot().expect("snapshot").node_kind.name, "Renamed");
}

/// 📄️ The app boots on a real document, so every window renders content before the first action.
#[semio_framework_async_macros::async_test]
async fn boots_on_the_forest_left_example_document() {
    let mut app: Block2dApp = new_app().await;
    let booted = app.snapshot().expect("snapshot");
    assert_eq!(booted.node_kind.id, "Hexagonal Cut Concrete Forest Left");
    assert_eq!(booted.handles.len(), 11);
    assert!(!booted.handle_kinds.is_empty());
    assert_ne!(booted, crate::standards::v1::subsets::any::schema::empty_block2d_snapshot());
    assert!(testkit::render(&mut app, board::BLOCK2D_BODY_BOARD).await.contains("Hexagonal Cut Concrete Forest Left"));
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_left_fixture() {
    let mut app = new_app().await;
    testkit::dispatch(&mut app, Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK2D_EXAMPLE_LEFT.into() })).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.node_kind.id, "Hexagonal Cut Concrete Forest Left");
    assert_eq!(projection.handles.len(), 11);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app = new_app().await;
    let kinds = app.snapshot().expect("snapshot").handle_kinds.len();
    testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {})).await;
    assert_eq!(app.snapshot().expect("snapshot").handle_kinds.len(), kinds + 1);
    app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().expect("snapshot").handle_kinds.len(), kinds);
    app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("redo");
    assert_eq!(app.snapshot().expect("snapshot").handle_kinds.len(), kinds + 1);
}

/// 🌉️ `puzzle2d_manifest_fragment`'s new caller round-trips through the `"catalog:out"` media port.
#[semio_framework_async_macros::async_test]
async fn export_media_catalog_out_wraps_the_puzzle2d_fragment() {
    let mut app = new_app().await;
    testkit::dispatch(&mut app, Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK2D_EXAMPLE_LEFT.into() })).await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("catalog:out")).expect("export catalog");
    assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "kit.catalog");
            let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
            assert_eq!(value["nodeKinds"][0]["id"], "Hexagonal Cut Concrete Forest Left");
        }
        other => panic!("expected Structured payload, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn command_from_action_bridges_set_active_example() {
    let _app = Block2dPlayApp;
    assert!(matches!(Block2dPlayApp::command_from_action("setActiveExample", Some(&dsl::json::to_dsl_value(&dsl::json!({ "exampleId": "left" })))), Ok(Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id })) if id == "left"));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the old `setSelection` view
/// action this test exercised is gone — 2d now declares zero app-level view actions (selection
/// moved to the framework-owned `handle` domain, an `ActionKind::Interaction` verb entirely
/// outside `Block2dCommand`/kind-discipline dispatch; that domain declaration itself is asserted
/// by `declares_the_handle_interaction_domain_scoped_to_the_board_window` above).
/// `app_with_registry` still earns its keep here: a genuine `Mutation`-kind command must still
/// emit document operations under the real, kind-discipline-enforcing registry.
#[semio_framework_async_macros::async_test]
async fn mutation_commands_still_emit_artifact_mutations_under_the_real_registry() {
    let mut app = testkit::app_with_registry().await;
    let result = testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {})).await;
    assert!(!result.mutations.is_empty(), "addHandleKind is a mutation and must reach document operations under kind discipline");
}
//#endregion 🔖️Behavior
