use super::*;
use crate::editor::dag::config::DagConfig;
use crate::editor::dag::unit_tests::context::{new_app_with_registry, DagApp};
use crate::editor::dag::DagPlayApp;
use crate::{DagHostDocumentEdge, DagNodeSpec};
use semio_framework_graph_layout_run::testing::{layout_run_close, layout_run_drive, layout_run_longest_path_layers};
use semio_framework_graph_layout_run::{LayoutRunJob, LayoutRunReason};
use semio_framework_job::INTERACTIVE_LANE_FUEL;
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::{ArtifactEditor, DslValue, PluginApp};
use semio_framework_tool_run::{ToolRunId, ToolRunVerdict, TOOL_RUN_ACTION_IDS};
use serde_json::Value;
use std::collections::BTreeMap;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🎞️layout-run.json");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("reorganize fixture parses")
}

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32])
}

fn verdict_id(verdict: ToolRunVerdict) -> &'static str {
    match verdict {
        ToolRunVerdict::Testing => "testing",
        ToolRunVerdict::Success => "success",
        ToolRunVerdict::Warning => "warning",
        ToolRunVerdict::Danger => "danger",
    }
}

fn number(value: &Value) -> f64 {
    value.as_f64().expect("fixture number")
}

fn demo() -> (DagSnapshot, DagLayoutGraph) {
    let document = crate::default_snapshot();
    let layout = layout_graph(&document, &layered_targets(&document, &DagConfig::default()).expect("layered targets"));
    (document, layout)
}

fn node_positions(document: &DagSnapshot) -> BTreeMap<String, (f64, f64)> {
    document.nodes().into_iter().map(|node| (node.id, (node.x, node.y))).collect()
}

/// ⏯️ The tool declares exactly the shared layout run with the anchor-only layered physics, stands in the edit mode,
/// injects the framework actions, and the synchronous `reorganize` verb is gone.
#[test]
fn reorganize_tool_declares_the_shared_layout_run_definition() {
    let fixture = fixture();
    let tool = &fixture["tool"];
    let definition = definition();
    assert_eq!((definition.id.as_str(), definition.icon_id.as_str()), (tool["id"].as_str().expect("id"), tool["iconId"].as_str().expect("icon")));
    let run = definition.run.as_ref().expect("reorganize declares run");
    assert_eq!(run, &layout_run_definition(JobKindId::new(LAYOUT_RUN_JOB)));
    let json = serde_json::to_value(run).expect("run serializes");
    for key in ["mutating", "rebase", "reconfigure", "trace", "runJob"] {
        assert_eq!(json[key], tool[key], "{key}");
    }
    let config = serde_json::to_value(layout_config()).expect("config serializes");
    for (key, value) in fixture["config"].as_object().expect("config") {
        assert_eq!(&config[key], value, "{key}");
    }
    layout_config().validate().expect("the layered config is valid");
    let app = crate::editor::dag::create_dag_app();
    assert!(app.tools.iter().any(|declared| declared.id == TOOL_ID && declared.run.is_some()));
    assert!(app.modes.iter().all(|mode| mode.tools.iter().any(|reference| reference.as_str() == TOOL_ID)));
    let injected = semio_framework::tool_run_action_definitions(&app);
    for action in TOOL_RUN_ACTION_IDS {
        assert!(injected.iter().any(|definition| definition.id == action), "{action} is injected");
    }
    assert!(<DagPlayApp as ArtifactEditor>::command_from_action(TOOL_ID, None).is_err(), "reorganize no longer resolves to a synchronous command");
}

/// 🕸️ First-seen node ids at their committed positions repel by half their diagonal (at least 8) and anchor to their
/// layered target when one exists; edges strip ports and drop self loops, dangling and duplicate springs.
#[test]
fn dag_document_maps_to_the_language_neutral_layout_graph() {
    let fixture = fixture();
    let mapping = &fixture["mapping"];
    let nodes = mapping["nodes"].as_array().expect("nodes").iter().map(|node| DagNodeSpec { id: node["id"].as_str().expect("id").into(), x: number(&node["x"]), y: number(&node["y"]), width: number(&node["width"]), height: number(&node["height"]), ..DagNodeSpec::default() }).collect();
    let edges = mapping["edges"].as_array().expect("edges").iter().map(|edge| DagHostDocumentEdge { id: edge["id"].as_str().expect("id").into(), source: edge["source"].as_str().expect("source").into(), target: edge["target"].as_str().expect("target").into(), ..DagHostDocumentEdge::default() }).collect();
    let document = DagSnapshot { schema: crate::default_snapshot().schema, content: crate::dag_content_child_with_owner(nodes, edges) };
    let targets = mapping["targets"].as_object().expect("targets").iter().map(|(id, point)| (id.clone(), LayoutRunPoint::new(number(&point["x"]), number(&point["y"])))).collect();
    let layout = layout_graph(&document, &targets);
    let expected = &mapping["expected"];
    let expected_nodes = expected["nodes"].as_array().expect("expected nodes");
    assert_eq!(layout.node_ids, expected_nodes.iter().map(|node| node["id"].as_str().expect("id").to_string()).collect::<Vec<_>>());
    for (node, row) in layout.graph.nodes.iter().zip(expected_nodes) {
        assert_eq!(node.entity, layout_run_entity(row["id"].as_str().expect("id")));
        assert_eq!((serde_json::to_value(node.origin).expect("origin"), serde_json::to_value(node.anchor).expect("anchor")), (row["origin"].clone(), row["anchor"].clone()));
        assert_eq!((node.radius, node.pinned), (number(&row["radius"]), false));
    }
    assert_eq!(serde_json::to_value(&layout.graph.edges).expect("edges"), expected["edges"]);
    layout.graph.validate().expect("the mapped graph is valid");
}

/// 🎞️ The demo run: every iteration is one fuel unit and one tick, the run lands on the layered layout, and the
/// provisional list ends with exactly one `move-node` per moved node carrying its entity.
#[test]
fn reorganize_run_over_the_demo_example_lands_on_the_layered_layout() {
    let fixture = fixture();
    let law = &fixture["run"];
    let (_, layout) = demo();
    assert_eq!((layout.graph.nodes.len() as u64, layout.graph.edges.len() as u64), (law["nodes"].as_u64().expect("nodes"), law["edges"].as_u64().expect("edges")));
    let mut job = LayoutRunJob::new(identity(), &layout.graph, layout_config(), move_encoder(layout.node_ids.clone())).expect("job");
    let recording = layout_run_drive(&mut job, 1).expect("the run completes");
    let stop = serde_json::to_value(job.stop().expect("stopped")).expect("stop");
    let positions = job.positions();
    layout_run_close(&mut job);
    let prefix: Vec<String> = recording.upserts.iter().take(24).map(|(key, verdict, reason)| format!("{key}:{}/{}", verdict_id(*verdict), LayoutRunReason::from_code(*reason).expect("reason").id())).collect();
    let deviation = layout.graph.nodes.iter().zip(&positions).filter_map(|(node, position)| node.anchor.map(|anchor| (anchor.x - position.x).hypot(anchor.y - position.y))).fold(0.0, f64::max);
    assert_eq!(stop, law["stop"]);
    assert_eq!(recording.progress.as_ref().map(|progress| progress.completed), law["iterations"].as_u64());
    assert_eq!(u64::from(recording.checkpoints), law["checkpoints"].as_u64().expect("checkpoints"));
    assert_eq!(serde_json::to_value(&prefix).expect("prefix"), law["verdictPrefix"]);
    assert!(layout.graph.nodes.iter().all(|node| node.anchor.is_some()), "every node has a layered target");
    assert!(deviation <= number(&law["layeredToleranceWorld"]), "the run lands on the layered layout (deviation {deviation})");
    assert_eq!(recording.ops.len() as u64, law["movedNodes"].as_u64().expect("moved"), "finalize publishes exactly one op per moved node");
    let mut expected_entities = std::collections::BTreeSet::new();
    for bytes in &recording.ops {
        let DagMutation::MoveNode(payload) = protocol::OpBinary::decode_op(bytes).expect("move op decodes") else { panic!("only move-node ops are provisional") };
        let index = layout.node_ids.iter().position(|id| *id == payload.id).expect("moved node is laid out");
        assert_eq!((payload.x, payload.y), (positions[index].x, positions[index].y), "the op carries the final position");
        expected_entities.insert(layout_run_entity(&payload.id));
    }
    assert_eq!(recording.entity_set(), expected_entities, "the provisional entity set is the moved nodes");
    let mut verdicts: BTreeMap<&str, u64> = BTreeMap::new();
    for (_, record) in recording.trace.as_ref().expect("trace").records() {
        *verdicts.entry(verdict_id(record.verdict)).or_default() += 1;
    }
    assert_eq!(serde_json::to_value(&verdicts).expect("verdicts"), law["finalVerdicts"]);
}

/// 🔮️ Third-party layering oracle: starting from the demo mirrored left to right, along the left-right axis the landed
/// layout never places a node left of a
/// `petgraph` longest-path predecessor layer, every layer 0 source shares the leftmost column and the deepest layer
/// holds the rightmost column. `DagHost`'s tidy-tree layering may still fold a longer path into a shallower column
/// (`📓️wave-W3-F1.md` open items), so columns are compared as non-decreasing, not strictly per layer.
#[test]
fn reorganize_lands_on_columns_ordered_by_the_petgraph_longest_path_layers() {
    let tolerance = number(&fixture()["run"]["layeredToleranceWorld"]);
    let demo = crate::default_snapshot();
    let mirrored = demo.nodes().into_iter().map(|node| DagNodeSpec { x: -node.x, ..node }).collect();
    let document = DagSnapshot { schema: demo.schema.clone(), content: crate::dag_content_child_with_owner(mirrored, demo.edges()) };
    let layout = layout_graph(&document, &layered_targets(&document, &DagConfig::default()).expect("layered targets"));
    let index: BTreeMap<&str, u32> = layout.node_ids.iter().enumerate().map(|(at, id)| (id.as_str(), at as u32)).collect();
    let directed: Vec<(u32, u32)> = document.edges().iter().filter_map(|edge| Some((*index.get(crate::schema::split_endpoint(&edge.source).0.as_str())?, *index.get(crate::schema::split_endpoint(&edge.target).0.as_str())?))).filter(|(source, target)| source != target).collect();
    let layers = layout_run_longest_path_layers(layout.graph.nodes.len(), &directed).expect("the demo is acyclic");
    let committed: Vec<f64> = layout.graph.nodes.iter().map(|node| node.origin.expect("placed").x).collect();
    assert!(directed.iter().any(|(source, target)| committed[*target as usize] < committed[*source as usize] - tolerance), "the mirrored committed layout points edges against the layer axis");
    let mut job = LayoutRunJob::new(identity(), &layout.graph, layout_config(), move_encoder(layout.node_ids.clone())).expect("job");
    layout_run_drive(&mut job, INTERACTIVE_LANE_FUEL).expect("the run completes");
    let positions = job.positions();
    layout_run_close(&mut job);
    let x = |node: u32| positions[node as usize].x;
    assert!(directed.iter().all(|(source, target)| x(*target) >= x(*source) - tolerance), "no edge points against the layer axis: {positions:?}");
    for (a, layer_a) in layers.iter().enumerate() {
        for (b, layer_b) in layers.iter().enumerate() {
            if layer_a < layer_b {
                assert!(x(b as u32) >= x(a as u32) - tolerance, "a deeper petgraph layer never sits left of a shallower one: {layers:?} {positions:?}");
            }
        }
    }
    let (leftmost, rightmost) = positions.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), point| (low.min(point.x), high.max(point.x)));
    let deepest = layers.iter().copied().max().expect("layers");
    assert!(layers.iter().zip(&positions).filter(|(layer, _)| **layer == 0).all(|(_, point)| point.x - leftmost <= tolerance), "sources share the leftmost column");
    assert!(layers.iter().zip(&positions).filter(|(layer, _)| **layer == deepest).all(|(_, point)| rightmost - point.x <= tolerance), "the deepest layer holds the rightmost column");
}

/// ⏱️ Every whole step of the run over the plugin's largest example stays below the interactive budget (best of the
/// fixture's attempts per step; a frozen clock keeps the slicing identical across attempts).
#[test]
fn reorganize_step_stays_below_the_interactive_ceiling_on_the_demo_example() {
    let fixture = fixture();
    let law = &fixture["interactive"];
    let (_, layout) = demo();
    let mut best: Vec<u64> = Vec::new();
    for _ in 0..law["attempts"].as_u64().expect("attempts") {
        let mut job = LayoutRunJob::new(identity(), &layout.graph, layout_config(), move_encoder(layout.node_ids.clone())).expect("job");
        let recording = layout_run_drive(&mut job, INTERACTIVE_LANE_FUEL).expect("the run completes");
        layout_run_close(&mut job);
        if best.is_empty() {
            best = recording.step_micros;
        } else {
            assert_eq!(best.len(), recording.step_micros.len(), "attempts slice identically");
            best.iter_mut().zip(recording.step_micros).for_each(|(slot, micros)| *slot = (*slot).min(micros));
        }
    }
    let worst = best.iter().copied().max().expect("steps");
    assert!(worst < law["budgetUs"].as_u64().expect("budget"), "worst drive_step {worst} µs");
}

fn tool_run_action(app: &mut DagApp, action: &str, entries: &[(&str, DslValue)]) -> DslValue {
    let args = DslValue::Object(entries.iter().map(|(key, value)| ((*key).to_string(), value.clone())).collect());
    semio_framework::io::resolve_ready(app.handle_action(action, Some(&args), &meta("local"))).unwrap_or_else(|fault| panic!("{action}: {fault:?}")).output
}

fn run_state(app: &DagApp) -> Option<&'static str> {
    app.tool_run_presence().map(|presence| presence.state.wire_name())
}

fn pump_until(app: &mut DagApp, what: &str, done: impl Fn(&DagApp) -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return;
        }
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).unwrap_or_else(|fault| panic!("{what}: maintenance faulted: {fault:?}"));
        semio_framework::io::resolve_ready(app.advance_typed_operation_publication()).unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
        if let Some(page) = app.take_typed_operation_result_page(1) {
            assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "{what}: typed operation faulted: {}", String::from_utf8_lossy(page.bytes()));
            app.acknowledge_typed_operation_result(page.token).expect("acknowledge result page");
        }
        let _ = app.take_typed_operation_effect();
        let _ = app.take_typed_operation_event();
        let _ = semio_framework::io::resolve_ready(app.take_typed_operation_completion()).expect("completion");
        let _ = app.take_typed_operation_ui_scope();
    }
    panic!("{what} never settled; state {:?}", run_state(app));
}

fn history_len(app: &mut DagApp) -> usize {
    semio_framework::io::resolve_ready(app.history_snapshot()).expect("history").upserts.len()
}

fn demo_app() -> DagApp {
    let mut app = semio_framework::io::resolve_ready(new_app_with_registry());
    semio_framework::io::resolve_ready(app.bind_instance_id(meta("local").instance_id));
    app
}

fn start(app: &mut DagApp) {
    assert_eq!(tool_run_action(app, semio_framework_plugin::TOOL_RUN_START_ACTION_ID, &[("toolId", DslValue::String(TOOL_ID.into()))]).get("toolRun").and_then(DslValue::as_str), Some("spawnJob"));
}

fn run_arguments() -> [(&'static str, DslValue); 2] {
    [("runId", DslValue::String("1".into())), ("generation", DslValue::String("0".into()))]
}

/// 🏁️ start → complete → finalize: nothing is committed while the run holds provisional moves, finalize publishes
/// them as one history entry, and one undo restores every committed position.
#[test]
fn reorganize_start_complete_finalize_is_one_undo_entry() {
    let law = &fixture()["lifecycle"];
    let mut app = demo_app();
    let before = node_positions(&app.snapshot().expect("snapshot"));
    let history = history_len(&mut app);
    start(&mut app);
    pump_until(&mut app, "reorganize completes", |app| run_state(app) == Some("complete"));
    assert_eq!(node_positions(&app.snapshot().expect("snapshot")), before, "a complete run has committed nothing");
    assert_eq!(history_len(&mut app), history);
    assert!(app.tool_run_trace_delta(None).is_some_and(|delta| !delta.is_empty()), "the run published trace pages");
    assert_eq!(tool_run_action(&mut app, semio_framework_plugin::TOOL_RUN_FINALIZE_ACTION_ID, &run_arguments()).get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    pump_until(&mut app, "reorganize finalizes", |app| run_state(app) == Some("finalized"));
    let after = node_positions(&app.snapshot().expect("snapshot"));
    assert_eq!(after.keys().collect::<Vec<_>>(), before.keys().collect::<Vec<_>>());
    assert_ne!(after, before, "finalize published the layered positions");
    assert_eq!((history_len(&mut app) - history) as u64, law["historyEntriesAdded"].as_u64().expect("entries"));
    semio_framework::io::resolve_ready(semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app, "undo", meta("local").instance_id));
    assert_eq!(node_positions(&app.snapshot().expect("snapshot")), before, "one undo restores every committed position");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// 🛑️ Abort after provisional moves exist leaves the document pack byte-identical and the history untouched.
#[test]
fn aborting_a_reorganize_run_leaves_the_document_byte_identical() {
    let law = &fixture()["lifecycle"];
    let mut app = demo_app();
    let pack = semio_framework::io::resolve_ready(app.document_pack()).expect("pack");
    let history = history_len(&mut app);
    start(&mut app);
    let iterations = law["abortAfterIterations"].as_u64().expect("iterations");
    pump_until(&mut app, "provisional moves exist", |app| app.tool_run_presence().is_some_and(|presence| presence.completed >= iterations));
    assert_eq!(tool_run_action(&mut app, semio_framework_plugin::TOOL_RUN_ABORT_ACTION_ID, &run_arguments()).get("toolRun").and_then(DslValue::as_str), Some("closeJob"));
    pump_until(&mut app, "abort settles", |app| run_state(app) == Some("aborted"));
    let after = semio_framework::io::resolve_ready(app.document_pack()).expect("pack");
    assert_eq!((after.pack, after.spr), (pack.pack, pack.spr), "abort leaves the document byte-identical");
    assert_eq!(history_len(&mut app), history);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
