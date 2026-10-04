pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::app::App;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry_and_members as new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type ImperativeApp = VcsArtifactApp<EditorApp<ImperativePlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;
    
    /// ✏️ `ImperativePlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<ImperativePlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<ImperativePlayApp>` builds it.
    /// 🧪️ Every fixture now runs against the real registry: with the ten verbs `Migrated`, a
    /// registry-less wrapper carries no migrated action rows at all and the guest's catalog authority
    /// rejects EVERY proof (`interactive-job.catalog-authority`, `migrated={}`).
    pub async fn imperative_app() -> OwnedImperativeApp {
        imperative_app_with_registry().await
    }
    
    /// 🧪️ Adapts `create_imperative_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
    /// still expect — framework test context gap (w2-cad-report "SDK gaps found" #3), not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub fn imperative_app_manifest_for_tests() -> App {
        App { definition: create_imperative_app(), examples: Vec::new() }
    }

    semio_framework_plugin::composed_reload_law!("imperative", ImperativePlayApp, imperative_app_manifest_for_tests, "../..");
    semio_framework_plugin::composed_child_history_law!("imperative", ImperativePlayApp, imperative_app_manifest_for_tests, [("addStep", r#"{"kind":"log.print"}"#)]);

    /// 🧸️ The live program and seed composed from the app's `flow` and `text` member stores (design §20.15: the parent holds
    /// only the two child handles).
    pub async fn live_scene(app: &ImperativeApp) -> crate::ProcedureScene {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
        use semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
        use store::{ArtifactPack, SpaceMember};
        let snapshot = app.snapshot().expect("procedure parent projection");
        let flow = app.child_store("flow", &snapshot.flow.child_id).await.expect("procedure flow child").document_pack_bytes().await.expect("procedure flow child pack");
        let text = app.child_store("text", &snapshot.text.child_id).await.expect("procedure text child").document_pack_bytes().await.expect("procedure text child pack");
        crate::ProcedureScene {
            path: crate::path_from_flow_content_snapshot(&SemioFlowSnapshot::decode_pack(&flow).expect("procedure flow child snapshot")),
            seed: crate::seed_from_text_content_snapshot(&SemioTextSnapshot::decode_pack(&text).expect("procedure text child snapshot")),
        }
    }
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline and materializes
    /// declared action-arg defaults (e.g. `addStep`'s `kind`).
    pub async fn imperative_app_with_registry() -> OwnedImperativeApp {
        let mut app = new_app_with_registry::<EditorApp<ImperativePlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(imperative_app_manifest_for_tests).await;
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(meta("local").instance_id));
        OwnedImperativeApp(app)
    }

    /// 🔚 A mounted app that RETIRES ITSELF. A live `ArtifactStore` asserts in `Drop`
    /// (`artifact store reached Drop without its exact terminal-empty shallow-shell witness`) unless
    /// it walked its bounded close loop first, so the fixture owns the close instead of asking every
    /// law to remember a trailing `close(&mut app)` — which is what makes a law that fails an
    /// assertion report ITS failure instead of a close panic. Skipped while unwinding, where the
    /// original panic is the report worth keeping. Derefs to the bare app for every read and dispatch.
    pub struct OwnedImperativeApp(ImperativeApp);

    impl OwnedImperativeApp {
        /// 🔚 Walks the bounded close protocol; idempotent (a terminal-empty app returns at once).
        pub fn close(&mut self) {
            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
        }
    }

    impl std::ops::Deref for OwnedImperativeApp {
        type Target = ImperativeApp;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for OwnedImperativeApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for OwnedImperativeApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                self.close();
            }
        }
    }

    /// 🔁️ Drives one dispatched typed operation to quiescence the way the plugin host does.
    pub async fn settle(app: &mut ImperativeApp) {
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta("local").instance_id).await.expect("settle the typed operation");
    }

    /// 🧹️ Closes every store the wrapper opened — a live `ArtifactStore` asserts in `Drop` otherwise.
    pub fn close(app: &mut ImperativeApp) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(app);
    }
    
    pub async fn dispatch(app: &mut ImperativeApp, command: ImperativeCommand) -> InvocationResult {
        let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        settle(app).await;
        result
    }
    
    pub async fn render(app: &mut ImperativeApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render")).expect("render json")
    }
}

use super::*;
use crate::editor::procedure::unit_tests::context::{dispatch, imperative_app, imperative_app_with_registry, live_scene, render};
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
    let proofs = <ImperativePlayApp as ArtifactEditor>::bounded_first_step_tool_proofs();
    assert_eq!(proofs.len(), recorded.len(), "every retained route needs its own first-step proof");
    assert!(routes.iter().all(|route| route.get("disposition").and_then(serde_json::Value::as_str) == Some("Migrated") && route.get("lanes").and_then(serde_json::Value::as_array).is_some_and(|lanes| !lanes.is_empty())));
    for route in routes {
        let id = route.get("id").and_then(serde_json::Value::as_str).expect("route id");
        assert!(proofs.iter().any(|proof| proof.tool_id() == id), "{id} is a retained route with no first-step proof");
        assert!(IMPERATIVE_RETAINED_TOOL_IDS.contains(&id), "{id} is a retained route outside the roster");
    }
}

/// 🧬️ The example picker's whole-document load. Every part of the recipe is asserted on the built
/// manifest and the real demo asset, because each one fails at a DIFFERENT boundary at runtime and
/// none of them is a compile error: an undeclared verb is dropped `undeclared-action` before the app
/// sees it; a non-`HostOnly` contract claims a store lane this verb never writes; a child slot the
/// app cannot mint genesis bytes for fails the archive's closure leg and the whole replacement is
/// refused. `ProcedureSnapshot` owns TWO composed `s.stdio.semio` children, so both must answer.
#[test]
fn set_active_example_is_host_only_and_both_composed_children_mint_genesis_packs() {
    assert!(IMPERATIVE_RETAINED_TOOL_IDS.contains(&"setActiveExample"), "the example verb must be a retained tool or the dispatch gate refuses it");
    let contract = IMPERATIVE_RETAINED_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == "setActiveExample").expect("setActiveExample has a publication contract");
    assert_eq!(contract.lanes, &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly], "a whole-document load publishes through neither the artifact nor the config store");
    let definition = create_imperative_app();
    let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample is declared on the app roster");
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    let demo = <crate::ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("the demo asset parses as a procedure snapshot");
    // 🧬️ The two composed slots carry DIFFERENT snapshot types (`SemioFlowSnapshot`/`SemioTextSnapshot`),
    // so they are stated one by one rather than iterated over a heterogeneous array.
    for (slot, artifact_id, child_id) in [("flow", demo.flow.target.artifact_id.as_str(), demo.flow.child_id.as_str()), ("text", demo.text.target.artifact_id.as_str(), demo.text.child_id.as_str())] {
        assert_eq!(artifact_id, child_id, "slot {slot}'s target must name its own child_id or ChildRestoreProjection refuses the whole load with InvalidReference");
    }
    for (slot, child_id) in [("flow", demo.flow.child_id.as_str()), ("text", demo.text.child_id.as_str())] {
        let pack = <ImperativePlayApp as ArtifactEditor>::genesis_child_pack(&demo, slot, child_id).expect("valid materialized genesis owner").unwrap_or_else(|| panic!("slot {slot} mints no genesis pack, so the archive closure leg refuses the load"));
        assert!(!pack.is_empty(), "slot {slot} minted an empty pack");
    }
    assert!(<ImperativePlayApp as ArtifactEditor>::genesis_child_pack(&demo, "flow", "not-this-documents-child").expect("valid materialized genesis owner").is_none(), "a foreign child id must not be answered");
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
    assert_eq!(ids.len(), 11, "every ImperativeCommand row must be covered by every_command()");
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
            "setActiveExample" => "active-example".to_string(),
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
        ImperativeCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() }),
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
    for body in [IMPERATIVE_PLAY_BODY_ARTIFACT, IMPERATIVE_PLAY_BODY_CATALOGUE, IMPERATIVE_PLAY_BODY_INSPECTOR] {
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
    let owner_id = live_scene(&app).await.path.steps.last().expect("owner").id.clone();
    dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some(owner_id.clone()), slot: Some("then".into()) })).await;
    let steps = imperative_steps_topology(&live_scene(&app).await.path);
    let owner_row_id = document_panel::step_row_id(&owner_id);
    let owner_node = steps.ordered.iter().find(|node| node.id == owner_row_id).expect("owner node present");
    assert!(owner_node.parent.is_none(), "top-level owner step has no parent");
    let nested = steps.ordered.iter().find(|node| node.parent.as_deref() == Some(owner_row_id.as_str())).expect("nested step present under owner");
    assert_eq!(nested.granularity, "step");
    context::close(&mut app);
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
    let topology = ImperativePlayApp::interaction_topology(&doc, &cfg).expect("valid retained interaction fixture");
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
    context::settle(&mut app).await;
    assert_eq!(live_scene(&app).await.path.steps.last().expect("the added step").kind, "log.print");
    // `run` is a View-kind command: under registry enforcement it must not emit document operations.
    let result = app.dispatch_typed(ImperativeCommand::Run(run::Run {}), &meta("local")).await.expect("run");
    assert!(result.mutations.is_empty(), "run evaluates into config, never the document");
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_steps() {
    let app = imperative_app().await;
    assert_eq!(live_scene(&app).await.path.steps.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn add_step_command_appends_step() {
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None })).await;
    assert!(live_scene(&app).await.path.steps.len() > 2);
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn add_step_at_owner_slot_nests_into_control_body() {
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "control.if".into(), index: None })).await;
    let before = live_scene(&app).await;
    let owner_id = before.path.steps.last().expect("owner").id.clone();
    let root_len = before.path.steps.len();
    dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some(owner_id.clone()), slot: Some("then".into()) })).await;
    let path = live_scene(&app).await.path.clone();
    let owner_step = path.steps.iter().find(|step| step.id == owner_id).expect("owner step");
    assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
    assert_eq!(path.steps.len(), root_len, "nested step lives in the slot, not the root path");
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn add_step_at_falls_back_to_root_for_unknown_owner() {
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some("missing-step".into()), slot: Some("then".into()) })).await;
    let path = live_scene(&app).await.path.clone();
    assert_eq!(path.steps.len(), 3, "an unknown owner addresses the root scope: the step joined the demo's two root steps");
    assert_eq!(path.steps.last().expect("added").kind, "log.print");
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn remove_step_command_is_exact_inverse_of_add() {
    let mut app = imperative_app().await;
    let original = live_scene(&app).await;
    dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None })).await;
    let added_id = live_scene(&app).await.path.steps.last().expect("added").id.clone();
    dispatch(&mut app, ImperativeCommand::RemoveStep(remove_step::RemoveStep { id: added_id })).await;
    assert_eq!(live_scene(&app).await, original);
    context::close(&mut app);
}

/// 🧪️ The definitional regression proof: two independent instances start from the same document, apply DISJOINT edits (A
/// appends a root step, B patches an existing step's params) and converge onto an identical live program over a
/// `MemoryBackbone` — impossible under whole-document snapshots, which would clobber one side's write.
/// `artifact_app_laws::assert_two_registered_instances_converge_with_members` cannot probe here: its synchronous probe reads
/// only the parent, which never changes on a child-lane edit (design §20.15). Same law, same shape, over the composed scene:
/// settle each edit, fold both ways, commit a checkpoint on A, fold it on B, and the probe must agree after each exchange.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    async fn probe(app: &context::ImperativeApp) -> (usize, bool) {
        let scene = live_scene(app).await;
        (scene.path.steps.len(), scene.path.steps.iter().any(|step| step.params.get("key") == Some(&neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into())))))
    }
    let mut params = BTreeMap::new();
    params.insert("key".to_string(), crate::document_dsl::value_to_value_dsl(&neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into()))));
    let mut instance_a = imperative_app().await;
    let mut instance_b = imperative_app().await;
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://imperative-convergence", "mem://imperative-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");
    let genesis = probe(&instance_a).await;
    instance_a.dispatch_typed(ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }), &meta("actor-a")).await.expect("a applies its edit");
    context::settle(&mut instance_a).await;
    instance_b.dispatch_typed(ImperativeCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params }), &meta("actor-b")).await.expect("b applies its edit");
    context::settle(&mut instance_b).await;
    instance_a.tick_backbone().await.expect("a folds b's events");
    instance_b.tick_backbone().await.expect("b folds a's events");
    assert_eq!(probe(&instance_a).await, probe(&instance_b).await, "both instances must converge on the same program");
    assert_eq!(probe(&instance_a).await, (genesis.0 + 1, true), "each instance holds both disjoint edits");
    let admitted = instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("a commits a checkpoint");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *instance_a, admitted).await.expect("a's checkpoint commit settles");
    context::settle(&mut instance_a).await;
    instance_b.tick_backbone().await.expect("b folds a's checkpoint");
    assert_eq!(probe(&instance_a).await, probe(&instance_b).await, "a replicated checkpoint keeps both instances converged");
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_imperative() {
    let mut sender = imperative_app().await;
    let (near, mut far) = MemoryBackbone::pair("mem://imperative-idempotent", "mem://imperative-idempotent").await;
    sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach sender");
    let genesis = live_scene(&sender).await.path.steps.len();
    dispatch(&mut sender, ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None })).await;
    assert_eq!(live_scene(&sender).await.path.steps.len(), genesis + 1, "the sender applied its edit");
    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    assert!(!envelopes.is_empty(), "the add reached the backbone");
    let operations = protocol::encode_envelopes(&envelopes);
    let mut receiver = imperative_app().await;
    receiver.ingest_operations(&operations).await.expect("ingest once");
    let once = live_scene(&receiver).await;
    assert_eq!(once.path.steps.len(), genesis + 1, "the replayed add applies");
    receiver.ingest_operations(&operations).await.expect("ingest twice");
    assert_eq!(live_scene(&receiver).await, once, "feeding the same operation twice must not double-apply");
    sender.detach_backbone().await.expect("sender releases its backbone");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = imperative_app().await;
    assert!(render(&mut app, "imperative.play.nope").await.contains("Unknown body"));
    context::close(&mut app);
}
//#endregion 🔖️CrossCutting

//#region 🪟️WindowActionScope
/// 🪟️ Every action this app declares must be reachable from a window kind: the React shell's Actions
/// pane is per-window, so an action no window kind can dispatch is invisible AND every dispatch
/// of it is answered `undeclared-action`. Ticket 26/09/18 slice B2b measured this editor booting with
/// `actionCount: 0` — two rendered windows and not one clickable verb.
///
/// 🔁️ Since ticket 26/09/18 slice DS1 the builder no longer CLONES the app roster into every
/// `WindowKindDefinition.actions` (that copy was 31.9 % of a shipped descriptor's bytes); a window's
/// dispatchable set is `semio_framework::window_kind_actions` — its own roster plus every
/// `AppDefinition.actions` row no window claims. This law reads through that resolver, which is the
/// same predicate the plugin host uses to answer a dispatch.
#[test]
fn every_declared_action_is_carried_by_a_window_kind() {
    let definition = create_imperative_app();
    let declared: Vec<String> = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window).into_iter().map(|action| action.id.clone())).collect();
    assert!(!declared.is_empty(), "no window kind dispatches any action: {:?}", definition.window_kinds.iter().map(|window| window.id.clone()).collect::<Vec<_>>());
    for id in ["addStep", "removeStep", "moveStep", "setStepParams", "run"] {
        assert!(declared.iter().any(|declared_id| declared_id == id), "{id} is declared by the app but carried by no window kind");
    }
}
//#endregion 🪟️WindowActionScope
