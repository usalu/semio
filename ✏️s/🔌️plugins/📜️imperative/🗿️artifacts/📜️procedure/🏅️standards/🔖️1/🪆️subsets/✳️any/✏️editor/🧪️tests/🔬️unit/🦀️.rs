pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::app::App;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type ImperativeApp = VcsArtifactApp<EditorApp<ImperativePlayApp>>;
    
    /// ✏️ `ImperativePlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<ImperativePlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<ImperativePlayApp>` builds it.
    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn imperative_app() -> ImperativeApp {
        new_app::<EditorApp<ImperativePlayApp>>().await
    }
    
    /// 🧪️ Adapts `create_imperative_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
    /// still expect — framework test context gap (w2-cad-report "SDK gaps found" #3), not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub fn imperative_app_manifest_for_tests() -> App {
        App { definition: create_imperative_app(), examples: Vec::new() }
    }
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline and materializes
    /// declared action-arg defaults (e.g. `addStep`'s `kind`).
    pub async fn imperative_app_with_registry() -> ImperativeApp {
        new_app_with_registry::<EditorApp<ImperativePlayApp>>(imperative_app_manifest_for_tests).await
    }
    
    pub async fn dispatch(app: &mut ImperativeApp, command: ImperativeCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }
    
    pub async fn render(app: &mut ImperativeApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
    }
}

use super::*;
use crate::editor::procedure::unit_tests::context::{dispatch, imperative_app, imperative_app_with_registry, render};
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::{EditorApp, PluginApp};
use std::collections::BTreeMap;
use store::{Backbone, BackboneMessage, MemoryBackbone};

const RETAINED_ROUTES: &str = include_str!("../../🧫️fixtures/🛣️retained-command-routes.json");

#[test]
fn retained_route_fixture_matches_the_exact_factory_and_fail_closed_census() {
    let fixture: serde_json::Value = serde_json::from_str(RETAINED_ROUTES).expect("Imperative route fixture decodes through serde_json");
    let routes = fixture.get("routes").and_then(serde_json::Value::as_array).expect("routes");
    let commands = every_command();
    let ids: Vec<_> = commands.iter().map(ImperativeCommand::command_id).collect();
    let recorded: Vec<_> = routes.iter().map(|route| route.get("id").and_then(serde_json::Value::as_str).expect("route id")).collect();
    assert_eq!(recorded, ids);
    assert!(<ImperativePlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().is_empty());
    assert!(routes.iter().all(|route| route.get("disposition").and_then(serde_json::Value::as_str) == Some("BatchOnlyPendingRewrite") && route.get("lanes").and_then(serde_json::Value::as_array).is_some_and(Vec::is_empty)));
}

#[semio_framework_async_macros::async_test]
async fn app_definition_builds_without_panicking() {
    let app = create_imperative_app();
    assert_eq!(app.id, semio_framework::surface_app_id(&crate::PROCEDURE_DIALECT.into(), semio_framework::AppRole::Editor));
    assert!(app.keybindings.iter().any(|binding| binding.action.action == "undo"));
}

#[semio_framework_async_macros::async_test]
async fn imperative_io_is_declared_on_the_manifest() {
    let app = create_imperative_app();
    assert_eq!(app.io.artifact.id, "computation.procedure");
    assert_eq!(app.io.ports.len(), 1);
    assert_eq!(app.io.ports[0].id, "result:out");
}

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 10, "every ImperativeCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
/// silently breaks (the record prints with no keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected = match id {
            "setContributions" => "contributions".to_string(),
            _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
        };
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// ⚖️ Rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact bytes
/// captured from the pre-merge `semio-s-app-imperative-protocol` crate (ticket
/// `🧪️wire-baseline-before.txt`). A regression here is a real format break, not a test-fixture
/// mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let cases: [(ImperativeCommand, &str, &str); 2] = [
        (ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: Some(1) }), "add-step add-step kind=log.print index=1", "010001096c6f672e7072696e7402000600010401"),
        (ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), "add-step add-step kind=log.print", "010001096c6f672e7072696e7401000600"),
    ];
    for (command, text, _hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<ImperativeCommand> {
    let mut params = BTreeMap::new();
    params.insert("message".to_string(), crate::document_dsl::value_to_value_dsl(&neural_engine::Value::Atom(neural_engine::Atom::String("updated".into()))));
    vec![
        ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: Some(1) }),
        ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some("step-if".into()), slot: Some("then".into()) }),
        ImperativeCommand::RemoveStep(remove_step::RemoveStep { id: "step-1".into() }),
        ImperativeCommand::RemoveStepAt(remove_step_at::RemoveStepAt { id: "step-1".into(), owner: Some("step-if".into()), slot: Some("then".into()) }),
        ImperativeCommand::MoveStep(move_step::MoveStep { id: "step-1".into(), index: 2 }),
        ImperativeCommand::MoveStepAt(move_step_at::MoveStepAt { id: "step-1".into(), index: 2, owner: None, slot: None }),
        ImperativeCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params: params.clone() }),
        ImperativeCommand::SetStepParamsAt(set_step_params_at::SetStepParamsAt { id: "step-1".into(), owner: Some("step-if".into()), slot: Some("then".into()), params }),
        ImperativeCommand::Run(run::Run {}),
        ImperativeCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_imperative_app()).expect("app definition json");
    for id in [IMPERATIVE_PLAY_WINDOW_MAIN, script::IMPERATIVE_PLAY_WINDOW_SCRIPT] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::IMPERATIVE_PLAY_MODE_EDIT), "mode missing from the manifest");
    for body in [IMPERATIVE_PLAY_BODY_DOCUMENT, IMPERATIVE_PLAY_BODY_CATALOGUE, IMPERATIVE_PLAY_BODY_INSPECTOR] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("computation.procedure"), "artifact kind missing from the manifest");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️Interaction
/// 🕹️ The `steps` domain is declared `HierarchyProvider::Topology`, transitive on both hover and
/// selection, and scoped to the main window kind — the manifest side of ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
#[semio_framework_async_macros::async_test]
async fn steps_interaction_domain_is_declared_topology_and_transitive_on_the_main_window() {
    let definition = create_imperative_app();
    let steps = definition.interactions.iter().find(|interaction| interaction.id == IMPERATIVE_INTERACTION_STEPS).expect("steps interaction domain declared");
    assert!(matches!(steps.hierarchy, HierarchyProvider::Topology));
    assert!(steps.hover.transitive, "steps hover must be transitive so a control step's hover covers its nested body steps");
    assert!(steps.selection.transitive, "steps selection must be transitive so a control step's selection covers its nested body steps");
    let main_window = definition.window_kinds.iter().find(|window| window.id == IMPERATIVE_PLAY_WINDOW_MAIN).expect("main window kind declared");
    assert!(main_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == IMPERATIVE_INTERACTION_STEPS), "main window must reference the steps interaction domain");
}

/// 🌳️ `interaction_topology` walks a `control.if` step's `bodies["then"]` nesting into
/// `TopologyNode.parent` links — the owner step has no parent, the nested step's parent is the
/// owner's own row id.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_walks_nested_control_bodies_into_parent_links() {
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "control.if".into(), index: None })).await;
    let owner_id = crate::procedure_working_scene(&app.snapshot().expect("projection")).path.steps.last().expect("owner").id.clone();
    dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some(owner_id.clone()), slot: Some("then".into()) })).await;
    let document = app.snapshot().expect("projection");
    let config = ImperativeConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = ImperativePlayApp::interaction_topology(&doc, &cfg);
    let steps = topology.domains.get(IMPERATIVE_INTERACTION_STEPS).expect("steps domain present in topology");
    let owner_row_id = document_panel::step_row_id(&owner_id);
    let owner_node = steps.ordered.iter().find(|node| node.id == owner_row_id).expect("owner node present");
    assert!(owner_node.parent.is_none(), "top-level owner step has no parent");
    let nested = steps.ordered.iter().find(|node| node.parent.as_deref() == Some(owner_row_id.as_str())).expect("nested step present under owner");
    assert_eq!(nested.granularity, "step");
}

/// 🌱️ A document with no steps has an empty `steps` topology — every stale `steps` selection id
/// gets pruned.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_is_empty_for_a_document_with_no_steps() {
    let document = ProcedureSnapshot::default();
    let config = ImperativeConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = ImperativePlayApp::interaction_topology(&doc, &cfg);
    assert!(topology.domains.get(IMPERATIVE_INTERACTION_STEPS).expect("steps domain present in topology").ordered.is_empty());
}
//#endregion 🔖️Interaction

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn add_step_materializes_kind_default_and_run_emits_no_artifact_mutations() {
    let mut app = imperative_app_with_registry().await;
    // AddStep fired with no explicit kind: the declared `kind` default ("log.print") must be
    // materialized by the registry's action-arg default resolution.
    app.dispatch_typed(ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), &meta("local")).await.expect("add step");
    let document = app.snapshot().expect("materialize projection");
    let path = crate::procedure_working_scene(&document).path;
    assert_eq!(path.steps.last().unwrap().kind, "log.print");
    // `run` is a View-kind command: under registry enforcement it must not emit document operations.
    let result = app.dispatch_typed(ImperativeCommand::Run(run::Run {}), &meta("local")).await.expect("run");
    assert!(result.mutations.is_empty(), "run evaluates into config, never the document");
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_steps() {
    let app = imperative_app().await;
    let path = crate::procedure_working_scene(&app.snapshot().expect("projection")).path;
    assert_eq!(path.steps.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn add_step_command_appends_step() {
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None })).await;
    let path = crate::procedure_working_scene(&app.snapshot().expect("projection")).path;
    assert!(path.steps.len() > 2);
}

#[semio_framework_async_macros::async_test]
async fn add_step_at_owner_slot_nests_into_control_body() {
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "control.if".into(), index: None })).await;
    let owner_id = crate::procedure_working_scene(&app.snapshot().expect("projection")).path.steps.last().expect("owner").id.clone();
    let root_len = crate::procedure_working_scene(&app.snapshot().expect("projection")).path.steps.len();
    dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some(owner_id.clone()), slot: Some("then".into()) })).await;
    let document = app.snapshot().expect("projection");
    let path = crate::procedure_working_scene(&document).path;
    let owner_step = path.steps.iter().find(|step| step.id == owner_id).expect("owner step");
    assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
    assert_eq!(path.steps.len(), root_len, "nested step lives in the slot, not the root path");
}

#[semio_framework_async_macros::async_test]
async fn add_step_at_falls_back_to_root_for_unknown_owner() {
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some("missing-step".into()), slot: Some("then".into()) })).await;
    let document = app.snapshot().expect("projection");
    let path = crate::procedure_working_scene(&document).path;
    let added_id = path.steps.last().expect("added").id.clone();
    assert!(path.steps.iter().any(|step| step.id == added_id));
}

#[semio_framework_async_macros::async_test]
async fn undo_after_add_step_restores_original_document_exactly() {
    let mut app = imperative_app().await;
    let base = default_snapshot();
    let mut path = crate::procedure_working_scene(&base).path;
    path.steps.push(Step { id: "step-3".into(), kind: "log.print".into(), params: crate::Dictionary::new(), bodies: BTreeMap::new() });
    let expected_after = crate::procedure_snapshot_with_content(&base.schema, &path, &crate::procedure_working_scene(&base).seed);
    app.dispatch_typed(ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), &meta("local")).await.expect("apply command");
    assert_eq!(app.snapshot().expect("projection"), expected_after);
    app.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().expect("projection"), default_snapshot());
    app.handle_action("redo", None, &meta("local")).await.expect("redo");
    assert_eq!(app.snapshot().expect("projection"), expected_after);
}

#[semio_framework_async_macros::async_test]
async fn remove_step_command_is_exact_inverse_of_add() {
    let mut app = imperative_app().await;
    let original = app.snapshot().expect("projection");
    dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None })).await;
    let added_id = crate::procedure_working_scene(&app.snapshot().expect("projection")).path.steps.last().expect("added").id.clone();
    dispatch(&mut app, ImperativeCommand::RemoveStep(remove_step::RemoveStep { id: added_id })).await;
    assert_eq!(app.snapshot().expect("projection"), original);
}

/// 🧪️ The definitional regression proof: two independent instances start from the same document,
/// apply DISJOINT edits (A appends a root step, B patches an existing step's params), and exchanging
/// operations over a `MemoryBackbone` converges both sides onto an identical projection — impossible
/// under whole-document `setDocument` snapshots, which would clobber one side's write.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    let mut params = BTreeMap::new();
    params.insert("key".to_string(), crate::document_dsl::value_to_value_dsl(&neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into()))));
    let (mut instance_a, mut instance_b) = semio_framework_plugin::artifact_app_laws::paired_apps::<EditorApp<ImperativePlayApp>>("mem://imperative-convergence").await;
    instance_a.dispatch_typed(ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }), &meta("actor-a")).await.expect("a applies its edit");
    instance_b.dispatch_typed(ImperativeCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params }), &meta("actor-b")).await.expect("b applies its edit");
    instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");
    assert_eq!(instance_a.snapshot().expect("a projection"), instance_b.snapshot().expect("b projection"));
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_imperative() {
    let mut sender = imperative_app().await;
    let (near, mut far) = MemoryBackbone::pair("mem://imperative-idempotent", "mem://imperative-idempotent").await;
    sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach sender");
    sender.dispatch_typed(ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }), &meta("local")).await.expect("apply command");
    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    let operations = protocol::encode_envelopes(&envelopes);
    let mut receiver = imperative_app().await;
    receiver.ingest_operations(&operations).await.expect("ingest once");
    let once = receiver.snapshot().expect("projection");
    receiver.ingest_operations(&operations).await.expect("ingest twice");
    assert_eq!(receiver.snapshot().expect("projection"), once);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = imperative_app().await;
    assert!(render(&mut app, "imperative.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️CrossCutting
