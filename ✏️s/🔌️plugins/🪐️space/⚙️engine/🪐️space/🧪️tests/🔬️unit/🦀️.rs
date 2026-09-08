
use super::*;

//#region 🧪️RetainedConfigOracle
#[test]
fn retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one() {
    let base = SpaceConfig::default();
    let base_value = pack::json_from_dsl_value(&dsl::ToValue::to_value(&base));
    let mut expected: serde_json::Value = serde_json::from_str(&pack::json_to_string(&base_value)).expect("third-party JSON decode");
    expected["workflowEngagementInput"] = serde_json::json!("draft");
    let (post, inverse, _) = prepare_space_config(&base, SpaceConfigMutation::SetWorkflowEngagementInput { value: "draft".into() }).expect("bounded config candidate");
    let post_value = pack::json_from_dsl_value(&dsl::ToValue::to_value(&post));
    let post_oracle: serde_json::Value = serde_json::from_str(&pack::json_to_string(&post_value)).expect("third-party JSON decode");
    assert_eq!(post_oracle, expected);
    assert!(matches!(inverse, SpaceConfigMutation::SetWorkflowEngagementInput { value } if value == base.workflow_engagement_input));
    assert!(space_config_mutation_bytes(&SpaceConfigMutation::SetWorkflowEngagementInput { value: "x".repeat(SPACE_CONFIG_TEXT_BYTES) }).is_ok());
    assert!(space_config_mutation_bytes(&SpaceConfigMutation::SetWorkflowEngagementInput { value: "x".repeat(SPACE_CONFIG_TEXT_BYTES + 1) }).is_err());
    // 🚪️ `openSpace`/`openInstance`/`spawnApp` publish exactly these four session mutations, so
    // the retained config lane must admit them — it rejected every one of them before.
    for admitted in [
        SpaceConfigMutation::SetClipboard { node_ids: Vec::new() },
        SpaceConfigMutation::SetSpaceId { space_id: Some("demo".into()) },
        SpaceConfigMutation::SetActiveNode { node_id: Some("node-1".into()) },
        SpaceConfigMutation::SetFocusedNode { node_id: Some("node-1".into()) },
    ] {
        assert!(space_config_mutation_bytes(&admitted).is_ok(), "the retained config lane must admit {admitted:?}");
        assert!(prepare_space_config(&base, admitted).is_ok());
    }
    assert!(space_config_mutation_bytes(&SpaceConfigMutation::SetLocale { value: "de-DE".into() }).is_err());
    assert_eq!(SPACE_CONFIG_MAXIMUM_BYTES * 4 + 1_024, 263_168);
}
//#endregion 🧪️RetainedConfigOracle
use crate::demo_space_projection;
use crate::engine::space::testkit::{empty_history, studio_emit};
use semio_framework_plugin::testkit as plugin_testkit;
use semio_framework_plugin::{PluginApp, VcsArtifactApp};

//#region 🧪️RetainedCatalogOracle
#[derive(Debug, PartialEq, Eq)]
struct SpaceRetainedCatalogSummary {
    routes: usize,
    bounded: usize,
    batch: usize,
    migrated: usize,
    unique: bool,
    bounded_ids: std::collections::BTreeSet<String>,
    migrated_ids: std::collections::BTreeSet<String>,
    host_only_ids: std::collections::BTreeSet<String>,
}

trait SpaceRetainedCatalogOracle {
    fn summarize(&self, fixture: &str) -> SpaceRetainedCatalogSummary;
}

struct SerdeJsonSpaceRetainedCatalogOracle;

impl SpaceRetainedCatalogOracle for SerdeJsonSpaceRetainedCatalogOracle {
    fn summarize(&self, fixture: &str) -> SpaceRetainedCatalogSummary {
        let document: pack::JsonValue = pack::parse_json(fixture).expect("language-neutral retained catalog fixture");
        let routes = document.get("routes").and_then(pack::JsonValue::as_array).expect("routes array");
        let bounded_ids = routes
            .iter()
            .filter(|route| route.get("execution").and_then(pack::JsonValue::as_str) == Some("bounded"))
            .filter_map(|route| route.get("id").and_then(pack::JsonValue::as_str).map(str::to_string))
            .collect::<std::collections::BTreeSet<_>>();
        let batch = routes.iter().filter(|route| route.get("execution").and_then(pack::JsonValue::as_str) == Some("batch")).count();
        let migrated_ids =
            routes.iter().filter(|route| route.get("status").and_then(pack::JsonValue::as_str) == Some("migrated")).filter_map(|route| route.get("id").and_then(pack::JsonValue::as_str).map(str::to_string)).collect::<std::collections::BTreeSet<_>>();
        let host_only_ids = document
            .get("publicationContracts")
            .and_then(pack::JsonValue::as_array)
            .expect("publication contracts array")
            .iter()
            .filter(|contract| contract.get("lanes").and_then(pack::JsonValue::as_array).is_some_and(|lanes| lanes.as_slice() == [pack::JsonValue::String("hostOnly".into())]))
            .filter_map(|contract| contract.get("toolId").and_then(pack::JsonValue::as_str).map(str::to_string))
            .collect::<std::collections::BTreeSet<_>>();
        let ids = routes.iter().filter_map(|route| route.get("id").and_then(pack::JsonValue::as_str)).collect::<std::collections::BTreeSet<_>>();
        SpaceRetainedCatalogSummary { routes: routes.len(), bounded: bounded_ids.len(), batch, migrated: migrated_ids.len(), unique: ids.len() == routes.len(), bounded_ids, migrated_ids, host_only_ids }
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_command_catalog_matches_the_serde_json_oracle() {
    let oracle = SerdeJsonSpaceRetainedCatalogOracle.summarize(include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json"));
    let bounded_ids = SPACE_BOUNDED_TOOL_IDS.iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    let host_only_ids = <SpaceCommandJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .filter(|contract| contract.lanes == [semio_framework_plugin::ArtifactToolPublicationLane::HostOnly])
        .map(|contract| contract.tool_id.to_string())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(oracle, SpaceRetainedCatalogSummary { routes: 40, bounded: 15, batch: 25, migrated: 15, unique: true, bounded_ids: bounded_ids.clone(), migrated_ids: bounded_ids.clone(), host_only_ids: host_only_ids.clone() });
    assert_eq!(bounded_ids.len(), SPACE_BOUNDED_TOOL_IDS.len());
    assert_eq!(host_only_ids.len(), 6);
    assert_eq!(SPACE_BATCH_ONLY_TOOL_IDS.len(), 25);
}

#[semio_framework_async_macros::async_test]
async fn retained_publication_oracle_rejects_hostile_tool_and_lane_fixtures() {
    let fixture = include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json");
    let expected = ["setActiveExample", "importSpacePack", "goHome", "navigateVirtualFileSystemNode", "importSpacePackPayload", "setAppRegistrations"].iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    let wrong_lane = fixture.replacen("\"hostOnly\"", "\"artifact\"", 1);
    let wrong_tool = fixture.replacen("\"setActiveExample\"", "\"forgedTool\"", 1);
    assert_ne!(SerdeJsonSpaceRetainedCatalogOracle.summarize(&wrong_lane).host_only_ids, expected);
    assert_ne!(SerdeJsonSpaceRetainedCatalogOracle.summarize(&wrong_tool).host_only_ids, expected);
}
//#endregion 🧪️RetainedCatalogOracle

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_empty_not_demo() {
    let _app = SpaceApp::default();
    assert!(SpaceApp::initial_snapshot().await.graph.nodes.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn demo_document_has_instances_and_edges() {
    let projection = demo_space_projection().await;
    assert!(projection.graph.nodes.len() >= 5);
    assert!(!projection.graph.edges.is_empty());
    assert!(semio_framework_os::validate_workflow(&projection.graph).ok);
}

#[semio_framework_async_macros::async_test]
async fn space_window_kind_actions_scope_editing_to_workflow() {
    let definition = create_space_app().await.definition;
    let resolve = |window_id: &str| -> Vec<String> {
        let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
        semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
    };
    let graph = resolve(crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW);
    let vfs = resolve(crate::engine::space::modes::main::windows::media_vfs::S_PLAY_WINDOW_MEDIA_VFS);
    let dag = resolve(crate::engine::space::modes::main::windows::compiled_dag::S_PLAY_WINDOW_COMPILED_DAG);
    for graph_operation in ["spawnApp", "connectMediaPorts", "removeAppInstance", "exportMedia", "addParameter"] {
        assert!(graph.contains(&graph_operation.to_string()), "Workflow must expose {graph_operation}");
        assert!(!vfs.contains(&graph_operation.to_string()), "Media VFS must NOT expose {graph_operation}");
        assert!(!dag.contains(&graph_operation.to_string()), "Compiled DAG must NOT expose {graph_operation}");
    }
    assert!(vfs.contains(&"navigateVirtualFileSystemNode".to_string()));
    assert!(!graph.contains(&"navigateVirtualFileSystemNode".to_string()));
    assert!(dag.contains(&"compiledDagEngagementSubmit".to_string()));
    assert!(!graph.contains(&"compiledDagEngagementSubmit".to_string()));
    // 🌐️ Global navigation/utility actions stay orphans on every window.
    for shared in ["setActiveExample", "goHome"] {
        assert!(graph.contains(&shared.to_string()) && vfs.contains(&shared.to_string()) && dag.contains(&shared.to_string()), "{shared} stays global");
    }
}

#[semio_framework_async_macros::async_test]
async fn space_manifest_uses_studio_app_id() {
    let app = create_space_app().await;
    assert_eq!(app.definition.id, S_PLAY_APP_ID);
    assert_eq!(app.definition.controller_id, "s.space.studio@1/*#editor");
}

/// 🪪️ `ArtifactStore::dispatch_inner`'s `CommitCheckpoint` arm (🏪️store `🦀️.rs`) rejects an
/// empty checkpoint (`VcsError::ValidationFailed("cannot create an empty checkpoint")`) — a
/// freshly-constructed `VcsArtifactApp` has no uncommitted edits at all (no `genesis()` on
/// `SpaceApp`, empty `initial_snapshot`), so this must spawn a real edit before committing, exactly
/// like the sibling `checkout_checkpoint_restores_projection` below.
#[semio_framework_async_macros::async_test]
async fn commit_checkpoint_round_trips_projection() {
    use crate::engine::space::commands::spawn_app;
    testkit::seed_draw_plugin().await;
    let mut app = VcsArtifactApp::<SpaceApp>::new(SpaceApp::default()).await;
    app.dispatch_typed(SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: testkit::test_surface_id("draw").await, x: 80.0, y: 80.0 }), &plugin_testkit::meta("local")).await.expect("spawn");
    let before = app.snapshot().expect("projection").graph.nodes.len();
    let commit_args = pack::json_to_dsl_value(&pack::json!({ "message": "snapshot" }));
    app.handle_action("commitCheckpoint", Some(&commit_args), &plugin_testkit::meta("local")).await.expect("commit");
    assert_eq!(app.snapshot().expect("projection").graph.nodes.len(), before);
}

#[semio_framework_async_macros::async_test]
async fn checkout_checkpoint_restores_projection() {
    use crate::engine::space::commands::spawn_app;
    testkit::seed_draw_plugin().await;
    let mut app = VcsArtifactApp::<SpaceApp>::new(SpaceApp::default()).await;
    let before = app.snapshot().expect("projection").graph.nodes.len();
    app.dispatch_typed(SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: testkit::test_surface_id("draw").await, x: 80.0, y: 80.0 }), &plugin_testkit::meta("local")).await.expect("spawn");
    let commit_args = pack::json_to_dsl_value(&pack::json!({ "message": "after-first-spawn" }));
    app.handle_action("commitCheckpoint", Some(&commit_args), &plugin_testkit::meta("local")).await.expect("commit");
    let after_first = app.snapshot().expect("projection").graph.nodes.len();
    assert!(after_first > before);
    let files = app.document_pack().await.expect("document pack");
    let parsed: store::ParsedDocumentText<WorkflowSnapshot, WorkflowMutation> = store::parse_document_pack(&files.pack, &files.spr).await.expect("parse document pack");
    let checkpoint_id = parsed.envelope.vcs.checkpoints[0].id.clone();
    app.dispatch_typed(SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: testkit::test_surface_id("draw").await, x: 80.0, y: 80.0 }), &plugin_testkit::meta("local")).await.expect("spawn2");
    assert!(app.snapshot().expect("projection").graph.nodes.len() > after_first);
    let checkout_args = pack::json_to_dsl_value(&pack::json!({ "checkpointId": checkpoint_id }));
    app.handle_action("checkoutCheckpoint", Some(&checkout_args), &plugin_testkit::meta("local")).await.expect("checkout");
    assert_eq!(app.snapshot().expect("projection").graph.nodes.len(), after_first);
}

/// 🧪️ The definitional proof: two independent instances start from `SpaceApp::initial_snapshot()`
/// (genuinely EMPTY — `paired_apps`/`new_app::<A>()` never seed the bundled demo projection; this
/// test previously assumed otherwise and dispatched a rename against a node id that could never
/// exist in either instance, which the missing-target guard correctly rejected once the crate could
/// finally link and run this test for the first time), apply DISJOINT edits (A spawns a "draw"
/// instance, B spawns a "shooting" instance from a different plugin), and exchanging operations
/// over a backbone converges both sides onto the same projection — impossible under whole-document
/// `setDocument` snapshots, where one side's write would clobber the other's.
///
/// 🚧️ BLOCKED (2026-08-17, lane 2-G): fails with `Fault { code: "module.vcs", message:
/// "validation failed: change ... has an invalid edit reference ..." }` inside `pump b`
/// (`assert_two_instances_converge`'s own `instance_b.handle_action("commitCheckpoint", ...)`,
/// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5852`, itself calling into
/// `🏪️store/🦀️.rs`'s `validate_durable_history`/`CommitCheckpoint` handling). Confirmed via
/// two independent isolated (`--test-threads=1`) reproductions that this is NOT specific to
/// "shooting" or to using two different plugins: substituting a second `draw` spawn (different
/// position) at B reproduces the identical fault. This is therefore a genuine, pre-existing
/// framework bug in the backbone-relay + checkpoint path for CREATE-type mutations (`AddNode`),
/// never exercised before this lane: the ORIGINAL test always failed earlier, at `instance_b`'s own
/// `dispatch_typed` (missing-target, since it renamed a node id that never existed), so this
/// checkpoint code path was never reached by any test until the canonical-surface-id fix let this
/// suite compile and run for the first time. `🏪️store/**` and `🔌️plugin/**` are both under
/// `🧰️framework/**` — forbidden to this lane. Left failing per the brief's explicit instruction
/// ("never delete/#\[ignore\] to force green; leave failing + sharedFileRequest"); see this lane's
/// `📓️w2-g-report.md` for the sharedFileRequest.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_on_disjoint_edits_via_backbone() {
    use crate::engine::space::commands::spawn_app;
    testkit::seed_draw_plugin().await;
    testkit::seed_multi_port_plugins().await;
    let draw_surface_id = testkit::test_surface_id("draw").await;
    let shooting_surface_id = testkit::test_surface_id("shooting").await;
    plugin_testkit::assert_two_instances_converge::<SpaceApp, (usize, usize)>(
        "mem://s-studio-convergence",
        SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: draw_surface_id, x: 80.0, y: 80.0 }),
        SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "shooting".into(), app_id: shooting_surface_id, x: 300.0, y: 100.0 }),
        move |app| {
            let projection = app.snapshot().expect("projection");
            let draw_count = projection.graph.nodes.iter().filter(|node| node.plugin_id == "draw").count();
            let shooting_count = projection.graph.nodes.iter().filter(|node| node.plugin_id == "shooting").count();
            (draw_count, shooting_count)
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn space_declares_expected_actions_and_examples() {
    let studio = create_space_app().await;
    let workflow = studio.definition.window_kinds.iter().find(|window| window.id == crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW).expect("workflow window");
    assert!(workflow.actions.iter().any(|action| action.id == "spawnApp"));
    assert!(workflow.actions.iter().any(|action| action.id == "reorganizeWorkflow"));
    let registrations = studio.definition.commands.iter().find(|command| command.id == "setAppRegistrations").expect("host registration command");
    assert!(!registrations.in_palette);
    assert_eq!(registrations.args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), vec!["json"]);
    assert_eq!(studio.examples.len(), S_STUDIO_EXAMPLES.len());
}

#[semio_framework_async_macros::async_test]
async fn space_labels_resolve_native_english_by_default() {
    let projection = demo_space_projection().await;
    let history = empty_history();
    let doc = ArtifactView::new(&projection, &history);
    let config = SpaceConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let _app = SpaceApp::default();
    let catalogue_tree = SpaceApp::render(S_PLAY_CATALOGUE_BODY_KEY, &doc, &cfg).await.expect("catalogue tree");
    let catalogue_json = plugin_testkit::project_and_retire_fixture_tree(catalogue_tree).expect("catalogue projection");
    assert!(catalogue_json.contains("\"Apps\""));

    let parameters_tree = SpaceApp::render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &cfg).await.expect("parameters tree");
    let parameters_json = plugin_testkit::project_and_retire_fixture_tree(parameters_tree).expect("parameters projection");
    assert!(parameters_json.contains("Add Parameter"));
    assert!(parameters_json.contains("\"Name\""));
    assert!(parameters_json.contains("\"Remove\""));
    assert!(!parameters_json.contains("Parameter hinzufügen"));
}

#[semio_framework_async_macros::async_test]
async fn space_labels_resolve_native_german_locale() {
    let projection = demo_space_projection().await;
    let history = empty_history();
    let doc = ArtifactView::new(&projection, &history);
    let config = SpaceConfig { locale: "de".into(), ..SpaceConfig::default() };
    let cfg = ConfigView { snapshot: &config };
    let _app = SpaceApp::default();
    let parameters_tree = SpaceApp::render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &cfg).await.expect("parameters tree");
    let parameters_json = plugin_testkit::project_and_retire_fixture_tree(parameters_tree).expect("parameters projection");
    assert!(parameters_json.contains("Parameter hinzufügen"));
    assert!(parameters_json.contains("\"Entfernen\""));
    assert!(!parameters_json.contains("Add Parameter"));

    let inspector_tree = SpaceApp::render(S_PLAY_INSPECTOR_BODY_KEY, &doc, &cfg).await.expect("inspector tree");
    let inspector_json = plugin_testkit::project_and_retire_fixture_tree(inspector_tree).expect("inspector projection");
    assert!(inspector_json.contains("Wähle Workflow-Knoten im Arbeitsbereich aus."));
}

/// 🗂️ Grouped-disclosure context menu: at most 9 top-level rows (leaves+groups combined) and the
/// destructive `removeAppInstance` row is always the final top-level entry.
#[semio_framework_async_macros::async_test]
async fn space_workflow_context_menu_stays_within_budget_with_destructive_tail() {
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_space_app().await.definition);
    let labels = semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&SpaceConfig::default().locale);
    let selected_node_ids = vec!["node-1".to_string()];
    let items = space_workflow_context_menu_items(&registry, labels, false, None, &selected_node_ids).await;
    assert!(items.len() <= 9, "top-level context menu rows must stay within budget: {} rows", items.len());
    let last = items.last().expect("non-empty menu");
    assert_eq!(last.id, "remove-instance");
    assert_eq!(last.destructive, Some(true), "removeAppInstance must be the last, destructive top-level row");
}

// 🌉️ Keeps `studio_emit`/`empty_history` imports exercised at this module's own level too (every
// command-group file also imports them directly from `testkit`).
#[semio_framework_async_macros::async_test]
async fn testkit_studio_emit_smoke_test() {
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let _ = empty_history();
    let _ = studio_emit(&projection, &config, &SpaceCommand::GoHome(go_home::GoHome {})).await.expect("handle");
}
