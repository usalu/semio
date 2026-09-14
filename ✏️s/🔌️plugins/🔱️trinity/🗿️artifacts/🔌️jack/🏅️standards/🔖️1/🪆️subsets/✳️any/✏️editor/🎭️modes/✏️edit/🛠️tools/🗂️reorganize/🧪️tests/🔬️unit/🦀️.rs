use super::*;
use crate::editor::jack::{create_trinity_jack_app, TrinityJackPlayApp};
use crate::{Edge, Node};
use semio_framework_graph_layout_run::testing::{layout_run_close, layout_run_drive, layout_run_fdg_layout, layout_run_hop_distances, layout_run_normalized_stress, LayoutRunOracleParameters};
use semio_framework_graph_layout_run::{LayoutRunJob, LayoutRunReason};
use semio_framework_job::INTERACTIVE_LANE_FUEL;
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::{App, ArtifactEditor, DslValue, EditorApp, PluginApp, VcsArtifactApp};
use semio_framework_tool_run::{ToolRunId, ToolRunVerdict, TOOL_RUN_ACTION_IDS};
use serde_json::Value;
use std::collections::BTreeMap;

type JackApp = VcsArtifactApp<EditorApp<TrinityJackPlayApp>>;

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

fn nakagin() -> JackSnapshot {
    crate::editor::jack::default_fixture()
}

fn node_positions(document: &JackSnapshot) -> BTreeMap<String, (f64, f64)> {
    crate::jack_working_scene(document).nodes.into_iter().map(|node| (node.id, (node.x, node.y))).collect()
}

/// ⏯️ The tool declares exactly the shared layout run over the graph's 120-iteration budget, stands in the edit mode,
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
    let app = create_trinity_jack_app();
    assert!(app.tools.iter().any(|declared| declared.id == TOOL_ID && declared.run.is_some()));
    assert!(app.modes.iter().all(|mode| mode.tools.iter().any(|reference| reference.as_str() == TOOL_ID)));
    let injected = semio_framework::tool_run_action_definitions(&app);
    for action in TOOL_RUN_ACTION_IDS {
        assert!(injected.iter().any(|definition| definition.id == action), "{action} is injected");
    }
    assert!(<TrinityJackPlayApp as ArtifactEditor>::command_from_action(TOOL_ID, None).is_err(), "reorganize no longer resolves to a synchronous command");
}

/// 🕸️ First-seen node ids at their committed positions repel by a quarter of their clamped extent sum; connections
/// strip ports and drop self loops, dangling and duplicate springs.
#[test]
fn jack_graph_maps_to_the_language_neutral_layout_graph() {
    let fixture = fixture();
    let mapping = &fixture["mapping"];
    let nodes = mapping["nodes"].as_array().expect("nodes").iter().map(|node| Node { id: node["id"].as_str().expect("id").into(), kind: "Piece".into(), name: String::new(), x: number(&node["x"]), y: number(&node["y"]), width: number(&node["width"]), height: number(&node["height"]), properties: Default::default(), ports: Vec::new() }).collect();
    let edges = mapping["edges"].as_array().expect("edges").iter().map(|edge| Edge { id: edge["id"].as_str().expect("id").into(), kind: "Connection".into(), source: edge["source"].as_str().expect("source").into(), target: edge["target"].as_str().expect("target").into(), properties: Default::default() }).collect();
    let document = JackSnapshot { content: crate::jack_content_child_with_owner(nodes, edges), ..crate::empty_trinity_graph_fixture() };
    let layout = layout_graph(&document);
    let expected = &mapping["expected"];
    let expected_nodes = expected["nodes"].as_array().expect("expected nodes");
    assert_eq!(layout.node_ids, expected_nodes.iter().map(|node| node["id"].as_str().expect("id").to_string()).collect::<Vec<_>>());
    for (node, row) in layout.graph.nodes.iter().zip(expected_nodes) {
        assert_eq!(node.entity, layout_run_entity(row["id"].as_str().expect("id")));
        assert_eq!(serde_json::to_value(node.origin).expect("origin"), row["origin"]);
        assert_eq!((node.radius, node.pinned, node.anchor), (number(&row["radius"]), false, None));
    }
    assert_eq!(serde_json::to_value(&layout.graph.edges).expect("edges"), expected["edges"]);
    layout.graph.validate().expect("the mapped graph is valid");
}

/// 🎞️ The Nakagin run: every iteration is one fuel unit and one tick, the provisional list ends with exactly one
/// `move-node` per moved node carrying its entity, and every node ends with its final verdict.
#[test]
fn reorganize_run_over_the_nakagin_example_matches_the_fixture() {
    let fixture = fixture();
    let law = &fixture["run"];
    let layout = layout_graph(&nakagin());
    assert_eq!((layout.graph.nodes.len() as u64, layout.graph.edges.len() as u64), (law["nodes"].as_u64().expect("nodes"), law["edges"].as_u64().expect("edges")));
    let mut job = LayoutRunJob::new(identity(), &layout.graph, layout_config(), move_encoder(layout.node_ids.clone())).expect("job");
    let recording = layout_run_drive(&mut job, 1).expect("the run completes");
    let stop = serde_json::to_value(job.stop().expect("stopped")).expect("stop");
    let digest = job.positions_digest();
    let positions = job.positions();
    layout_run_close(&mut job);
    let prefix: Vec<String> = recording.upserts.iter().take(24).map(|(key, verdict, reason)| format!("{key}:{}/{}", verdict_id(*verdict), LayoutRunReason::from_code(*reason).expect("reason").id())).collect();
    let mut verdicts: BTreeMap<&str, u64> = BTreeMap::new();
    for (_, record) in recording.trace.as_ref().expect("trace").records() {
        *verdicts.entry(verdict_id(record.verdict)).or_default() += 1;
    }
    assert_eq!(stop, law["stop"]);
    assert_eq!(format!("{digest:016x}"), law["positionsDigest"].as_str().expect("digest"), "the Nakagin layout is bit-deterministic");
    assert_eq!(recording.progress.as_ref().map(|progress| progress.completed), law["iterations"].as_u64());
    assert_eq!(u64::from(recording.checkpoints), law["checkpoints"].as_u64().expect("checkpoints"));
    assert_eq!(serde_json::to_value(&prefix).expect("prefix"), law["verdictPrefix"]);
    assert_eq!(recording.ops.len() as u64, law["movedNodes"].as_u64().expect("moved"), "finalize publishes exactly one op per moved node");
    let mut expected_entities = std::collections::BTreeSet::new();
    for bytes in &recording.ops {
        let TrinityGraphMutation::MoveNode(payload) = protocol::OpBinary::decode_op(bytes).expect("move op decodes") else { panic!("only move-node ops are provisional") };
        let index = layout.node_ids.iter().position(|id| *id == payload.id).expect("moved node is laid out");
        assert_eq!((payload.x, payload.y), (positions[index].x, positions[index].y), "the op carries the final position");
        expected_entities.insert(layout_run_entity(&payload.id));
    }
    assert_eq!(recording.entity_set(), expected_entities, "the provisional entity set is the moved nodes");
    assert_eq!(serde_json::to_value(&verdicts).expect("verdicts"), law["finalVerdicts"]);
}

/// 🔮️ Layout quality on the real document: the run's normalized stress stays within the fixture ratio of the
/// `fdg-sim` Fruchterman-Reingold layout from the same start and improves the committed layout.
#[test]
fn reorganize_layout_quality_matches_the_fdg_sim_oracle() {
    let fixture = fixture();
    let oracle = &fixture["oracle"];
    let parameters: LayoutRunOracleParameters = serde_json::from_value(oracle.clone()).expect("oracle parameters");
    let layout = layout_graph(&nakagin());
    let hops = layout_run_hop_distances(&layout.graph);
    let mut job = LayoutRunJob::new(identity(), &layout.graph, layout_config(), move_encoder(layout.node_ids.clone())).expect("job");
    let initial = job.positions();
    layout_run_drive(&mut job, INTERACTIVE_LANE_FUEL).expect("the run completes");
    let ours: Vec<[f64; 2]> = job.positions().iter().map(|point| [point.x, point.y]).collect();
    layout_run_close(&mut job);
    let theirs = layout_run_fdg_layout(&layout.graph, &initial, parameters);
    let start: Vec<[f64; 2]> = initial.iter().map(|point| [point.x, point.y]).collect();
    let (start, mine, reference) = (layout_run_normalized_stress(&start, &hops), layout_run_normalized_stress(&ours, &hops), layout_run_normalized_stress(&theirs, &hops));
    assert!(mine <= reference * number(&oracle["stressRatioMax"]), "stress {mine:.4} vs fdg-sim {reference:.4}");
    assert!(mine <= start * (1.0 - number(&oracle["improvementMin"])), "stress {mine:.4} vs initial {start:.4}");
}

/// ⏱️ Every whole step of the run over the plugin's largest example stays below the interactive budget (best of the
/// fixture's attempts per step; a frozen clock keeps the slicing identical across attempts).
#[test]
fn reorganize_step_stays_below_the_interactive_ceiling_on_the_nakagin_example() {
    let fixture = fixture();
    let law = &fixture["interactive"];
    let layout = layout_graph(&nakagin());
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

fn tool_run_action(app: &mut JackApp, action: &str, entries: &[(&str, DslValue)]) -> DslValue {
    let args = DslValue::Object(entries.iter().map(|(key, value)| ((*key).to_string(), value.clone())).collect());
    semio_framework::io::resolve_ready(app.handle_action(action, Some(&args), &meta("local"))).unwrap_or_else(|fault| panic!("{action}: {fault:?}")).output
}

fn run_state(app: &JackApp) -> Option<&'static str> {
    app.tool_run_presence().map(|presence| presence.state.wire_name())
}

fn pump_until(app: &mut JackApp, what: &str, done: impl Fn(&JackApp) -> bool) {
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

fn history_len(app: &mut JackApp) -> usize {
    semio_framework::io::resolve_ready(app.history_snapshot()).expect("history").upserts.len()
}

fn nakagin_app() -> JackApp {
    let mut app = semio_framework::io::resolve_ready(semio_framework_plugin::artifact_app_laws::new_app_with_registry::<EditorApp<TrinityJackPlayApp>>(|| App { definition: create_trinity_jack_app(), examples: Vec::new() }));
    semio_framework::io::resolve_ready(app.bind_instance_id(meta("local").instance_id));
    app
}

fn start(app: &mut JackApp) {
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
    let mut app = nakagin_app();
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
    assert_ne!(after, before, "finalize published the laid out positions");
    assert_eq!((history_len(&mut app) - history) as u64, law["historyEntriesAdded"].as_u64().expect("entries"));
    semio_framework::io::resolve_ready(semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app, "undo", meta("local").instance_id));
    assert_eq!(node_positions(&app.snapshot().expect("snapshot")), before, "one undo restores every committed position");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// 🛑️ Abort after provisional moves exist leaves the document pack byte-identical and the history untouched.
#[test]
fn aborting_a_reorganize_run_leaves_the_document_byte_identical() {
    let law = &fixture()["lifecycle"];
    let mut app = nakagin_app();
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
