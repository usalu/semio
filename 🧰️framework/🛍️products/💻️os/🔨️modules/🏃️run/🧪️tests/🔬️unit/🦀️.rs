
use super::*;
use semio_framework::{MediaPortDirection, MediaPortSpec};
use workflow::{WORKFLOW_SCHEMA, WorkflowMediaPort, placeholder_media_contract};

//#region 🔖️TestMediaCache
/// 🧪️ Test-local map-backed `MediaCache` for exercising cached runner outputs.
#[derive(Default)]
struct TestMediaCache {
    entries: HashMap<String, Media>,
}

impl MediaCache for TestMediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media> {
        self.entries.get(&fingerprint.0).cloned()
    }

    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media) {
        self.entries.insert(fingerprint.0.clone(), media.clone());
    }
}
//#endregion 🔖️TestMediaCache

/// 🧪️ A fake `AppChannelHost` for tests: no wasm at all, just a per-instance document/config, a
/// fixed structured output per port, and an in-process `InMemoryBlobStore` — enough to interpret
/// the exact `AppCommand`/`AppFrame` frame script `SpaceRunner::compute_node` sends, so
/// `SpaceRunner`'s dirty/clean bookkeeping can be exercised without a real program.
/// 🧪️ Outputs are keyed by app id, not by handle — a real app's export is a function of its
/// document/logic, not of the ephemeral instance handle a host happens to mint this call, and a
/// node genuinely does get re-opened (a fresh handle) on every dirty re-run.
#[derive(Default)]
struct FakeHost {
    documents: HashMap<u32, (Vec<u8>, Vec<u8>)>,
    configs: HashMap<u32, (Vec<u8>, Vec<u8>)>,
    handle_app: HashMap<u32, String>,
    outputs: HashMap<(String, String), Media>,
    blob_store: InMemoryBlobStore,
    next: u32,
    imported: Vec<(u32, String, Media)>,
}

impl FakeHost {
    fn set_output(&mut self, app_id: &str, port: &str, json: &str) {
        self.outputs.insert((app_id.to_string(), port.to_string()), Media { media_type: fake_media_type(), payload: MediaPayload::Structured { schema: "test".into(), json: json.into() } });
    }
}

fn fake_media_type() -> MediaType {
    MediaType { class: MediaClass::Data, form: MediaForm::Value }
}

impl AppChannelHost for FakeHost {
    async fn open(&mut self, _plugin_id: &str, app_id: &str, _artifact_ref: &str) -> Result<u32, RunError> {
        self.next += 1;
        self.handle_app.insert(self.next, app_id.to_string());
        Ok(self.next)
    }

    async fn exchange(&mut self, _ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
        let app_id = self.handle_app.get(&node).cloned().unwrap_or_default();
        let mut frames = Vec::new();
        for command in commands {
            match command {
                // 🧬️ Channel v12 retires `Hello`/`Welcome` — `open` (above) is what now
                // establishes the instance, matching the reactor ABI's `Event::InstanceOpen`.
                AppCommand::LoadConfig { seq, pack, spr } => {
                    self.configs.insert(node, (pack, spr));
                    frames.push(AppFrame::Done { in_reply_to: seq });
                }
                AppCommand::LoadDocument { seq, pack, spr } => {
                    self.documents.insert(node, (pack, spr));
                    frames.push(AppFrame::Done { in_reply_to: seq });
                }
                AppCommand::MediaIn { seq, port, descriptor, data } => match media_from_artifact(&descriptor, data, &self.blob_store).await {
                    Ok(media) => {
                        self.imported.push((node, port, media));
                        frames.push(AppFrame::Done { in_reply_to: seq });
                    }
                    Err(error) => frames.push(AppFrame::Error { in_reply_to: Some(seq), fault: run_fault_bytes("handler", error.to_string()), report: Vec::new() }),
                },
                AppCommand::MediaOut { seq, port, .. } => match self.outputs.get(&(app_id.clone(), port.clone())) {
                    Some(media) => match media_to_artifact(media, &self.blob_store).await {
                        Ok((descriptor, data)) => frames.push(AppFrame::Media { in_reply_to: seq, port, descriptor, data }),
                        Err(error) => frames.push(AppFrame::Error { in_reply_to: Some(seq), fault: run_fault_bytes("handler", error.to_string()), report: Vec::new() }),
                    },
                    None => frames.push(AppFrame::Error { in_reply_to: Some(seq), fault: run_fault_bytes("handler", "no output"), report: Vec::new() }),
                },
                AppCommand::MediaFingerprint { seq, port } => match self.outputs.get(&(app_id.clone(), port)) {
                    Some(media) => {
                        let fingerprint = MediaFingerprint::of(media);
                        // 🩹️ Pre-existing bug fixed in passing: `store::pack_rt::encode_wire_value` takes
                        // `&dsl::DslValue`, not `&serde_json::Value` — `decode_fingerprint_wire` (this
                        // file) already expects the `DslValue`-transparent-string encoding `to_dsl_value`
                        // produces for a `MediaFingerprint(String)` newtype.
                        let value = to_dsl_value(&fingerprint).unwrap_or(dsl::DslValue::Null);
                        frames.push(AppFrame::MediaFingerprint { in_reply_to: seq, port: String::new(), fingerprint: store::pack_rt::encode_wire_value(&value) });
                    }
                    None => frames.push(AppFrame::Error { in_reply_to: Some(seq), fault: run_fault_bytes("handler", "no output"), report: Vec::new() }),
                },
                AppCommand::ReadDocument { seq } => {
                    let (pack, spr) = self.documents.get(&node).cloned().unwrap_or_default();
                    frames.push(AppFrame::Document { in_reply_to: seq, pack, spr, ops: String::new() });
                }
                AppCommand::ReadConfig { seq } => {
                    let (pack, spr) = self.configs.get(&node).cloned().unwrap_or_default();
                    frames.push(AppFrame::Config { in_reply_to: seq, pack, spr, ops: String::new() });
                }
                AppCommand::SetMergePolicy { seq, .. } => {
                    frames.push(AppFrame::Done { in_reply_to: seq });
                }
                _ => {}
            }
        }
        Ok(frames)
    }
}

fn media_port(node_id: &str, spec_id: &str, direction: MediaPortDirection, kind_id: &str, multiplicity: PortMultiplicity, required: bool) -> WorkflowMediaPort {
    let direction_word = match direction {
        MediaPortDirection::In => "in",
        MediaPortDirection::Out => "out",
    };
    WorkflowMediaPort { id: format!("{node_id}:{spec_id}:{direction_word}"), spec: MediaPortSpec { id: spec_id.into(), label: spec_id.into(), direction, media_type: fake_media_type(), kind_id: Some(kind_id.into()), required, multiplicity } }
}

fn workflow_node(id: &str, outputs: Vec<WorkflowMediaPort>, inputs: Vec<WorkflowMediaPort>) -> WorkflowNode {
    WorkflowNode {
        id: id.into(),
        plugin_id: "program".into(),
        app_id: format!("app-{id}"),
        label: id.into(),
        yields: String::new(),
        artifact_ref: format!("artifacts/{id}"),
        config_ref: format!("config/{id}"),
        x: 0.0,
        y: 0.0,
        width: 220.0,
        height: 100.0,
        inputs,
        outputs,
    }
}

async fn two_node_graph() -> Workflow {
    let source = workflow_node("node-a", vec![media_port("node-a", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
    let target = workflow_node("node-b", Vec::new(), vec![media_port("node-b", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, true)]);
    let edge =
        WorkflowEdge { id: "edge-1".into(), source_node_id: "node-a".into(), source_port_id: "node-a:out:out".into(), target_node_id: "node-b".into(), target_port_id: "node-b:in:in".into(), contract: placeholder_media_contract("data.value").await };
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![source, target], edges: vec![edge] }
}

fn empty_documents(graph: &Workflow) -> BTreeMap<String, (Vec<u8>, Vec<u8>)> {
    graph.nodes.iter().map(|node| (node.artifact_ref.clone(), (Vec::new(), Vec::new()))).collect()
}

fn empty_configs(graph: &Workflow) -> BTreeMap<String, (Vec<u8>, Vec<u8>)> {
    graph.nodes.iter().map(|node| (node.config_ref.clone(), (Vec::new(), Vec::new()))).collect()
}

/// 🧪️ A fresh, unsealed `RunSink` with `Start` already recorded — every test below emits
/// `NodeStarted`/`NodeFinished` through `SpaceRunner::run` on top of this, then (where memoization
/// across two runs matters) seals it and extracts `prior_node_records_from` for the second `run()`.
async fn fresh_sink() -> RunSink {
    let mut sink = RunSink::new(semio_framework_artifact_workflow_run::empty_run_document().await);
    sink.record(RunMutation::StartRun(StartRun {
        workflow_ref: "test.workflow".into(),
        workflow_checkpoint_id: String::new(),
        input_collection_ref: String::new(),
        input_snapshot_id: String::new(),
        parameter_values: Vec::new(),
        output_collection_ref: String::new(),
        trigger: semio_framework_artifact_workflow_run::RunTrigger::Manual { actor: "test".into() },
    }))
    .await
    .expect("Start on a fresh sink always applies");
    sink
}

#[semio_framework_async_macros::async_test]
async fn run_sink_preserves_typed_admission_rejection_without_recording_it() {
    let mut sink = fresh_sink().await;
    let document = sink.document.clone();
    let mutations = sink.mutations.clone();
    let duplicate = RunMutation::StartRun(StartRun {
        workflow_ref: "test.workflow".into(),
        workflow_checkpoint_id: String::new(),
        input_collection_ref: String::new(),
        input_snapshot_id: String::new(),
        parameter_values: Vec::new(),
        output_collection_ref: String::new(),
        trigger: semio_framework_artifact_workflow_run::RunTrigger::Manual { actor: "test".into() },
    });
    let error = sink.record(duplicate).await.expect_err("a second Start must be rejected");
    match error {
        RunError::MutationApply(error) => {
            assert_eq!(error.code, "mutation.apply.conflicting-target");
            assert_eq!(error.target, vec!["status"]);
        }
        other => panic!("expected typed mutation rejection, got {other:?}"),
    }
    assert_eq!(sink.document, document);
    assert_eq!(sink.mutations, mutations);
}

/// 🧪️ `semio_framework_artifact_workflow_run::RunArtifact.node_records`, keyed by node id — the shape `SpaceRunner::run`'s
/// `prior_node_records` parameter takes, built from a (test-)sealed prior run's document.
fn prior_node_records_from(document: &semio_framework_artifact_workflow_run::RunArtifact) -> BTreeMap<String, RunNodeRecord> {
    document.node_records.iter().map(|record| (record.node_id.clone(), record.clone())).collect()
}

#[semio_framework_async_macros::async_test]
async fn topological_order_respects_edges() {
    let graph = two_node_graph().await;
    let order = topological_order(&graph).expect("acyclic");
    assert_eq!(order, vec!["node-a".to_string(), "node-b".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn detects_cycles() {
    let mut graph = two_node_graph().await;
    graph.edges.push(WorkflowEdge { id: "edge-2".into(), source_node_id: "node-b".into(), source_port_id: "b-out".into(), target_node_id: "node-a".into(), target_port_id: "a-in".into(), contract: placeholder_media_contract("data.value").await });
    assert!(matches!(topological_order(&graph), Err(RunError::Cycle(_))));
}

#[semio_framework_async_macros::async_test]
async fn first_run_recomputes_every_node_second_run_is_a_no_operation() {
    let graph = two_node_graph().await;
    let mut host = FakeHost::default();
    host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
    let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
    let mut cache = TestMediaCache::default();
    let documents = empty_documents(&graph);
    let configs = empty_configs(&graph);

    let mut sink_1 = fresh_sink().await;
    let report_1 = runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink_1).await.expect("first run");
    assert_eq!(report_1.recomputed, vec!["node-a".to_string(), "node-b".to_string()]);
    assert!(report_1.clean.is_empty());
    sink_1.record(RunMutation::SealRun(SealRun { status: semio_framework_artifact_workflow_run::RunStatus::Succeeded })).await.expect("seal first run");

    // 🔒️ Non-destructive: the SOURCE `documents`/`configs` maps `run()` read are byte-identical to
    // what was passed in — nothing was written back into them (the load-bearing proof for this wave).
    assert_eq!(documents, empty_documents(&graph), "run() must never mutate its source documents map");
    assert_eq!(configs, empty_configs(&graph), "run() must never mutate its source configs map");

    let prior = prior_node_records_from(&sink_1.document);
    let mut sink_2 = fresh_sink().await;
    let report_2 = runner.run(&graph, &documents, &configs, &[], &[], &prior, &mut cache, &mut sink_2).await.expect("second run");
    assert!(report_2.recomputed.is_empty(), "unchanged documents must not re-trigger recompute: {:?}", report_2.recomputed);
    assert_eq!(report_2.clean, vec!["node-a".to_string(), "node-b".to_string()]);
    assert!(sink_2.document.node_records.iter().all(|record| record.status == RunNodeStatus::CacheHit), "every node on the second run must be a CacheHit: {:?}", sink_2.document.node_records);
}

#[semio_framework_async_macros::async_test]
async fn editing_upstream_document_dirties_downstream_only_through_the_wire() {
    let graph = two_node_graph().await;
    let mut host = FakeHost::default();
    host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
    let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
    let mut cache = TestMediaCache::default();
    let documents = empty_documents(&graph);
    let configs = empty_configs(&graph);
    let mut sink_1 = fresh_sink().await;
    runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink_1).await.expect("first run");
    sink_1.record(RunMutation::SealRun(SealRun { status: semio_framework_artifact_workflow_run::RunStatus::Succeeded })).await.expect("seal first run");

    let mut documents_2 = documents.clone();
    // 🧮️ Fingerprints are still `.spr`-bytes hashes (see `node_fingerprints`) — editing the op log
    // (`spr`), not the pack, is what must dirty the node. `documents` (the first run's SOURCE map)
    // is untouched by `run()` itself — this test edits its OWN local copy to simulate a live UI edit
    // landing on the source between runs.
    documents_2.insert("artifacts/node-a".to_string(), (Vec::new(), b"edited".to_vec()));
    let prior = prior_node_records_from(&sink_1.document);
    let mut sink_2 = fresh_sink().await;
    let report_2 = runner.run(&graph, &documents_2, &configs, &[], &[], &prior, &mut cache, &mut sink_2).await.expect("second run");
    assert_eq!(report_2.recomputed, vec!["node-a".to_string()], "node-a's own document changed, so node-a must recompute");
    assert_eq!(report_2.clean, vec!["node-b".to_string()], "node-a's FakeHost output is fixed, so its output fingerprint is unchanged — node-b must stay clean (the early-cutoff this whole design exists for)");
}

/// 🧪️ Changing a node's own effective config — document and resolved inputs held constant — must
/// dirty exactly that node on the very next `plan()`/`run()`, mirroring
/// `editing_upstream_document_dirties_downstream_only_through_the_wire`'s shape but on the config
/// dimension instead of the document one.
#[semio_framework_async_macros::async_test]
async fn changing_a_nodes_config_alone_dirties_it_without_touching_document_or_inputs() {
    let graph = two_node_graph().await;
    let mut host = FakeHost::default();
    host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
    let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
    let mut cache = TestMediaCache::default();
    let documents = empty_documents(&graph);
    let configs_1 = empty_configs(&graph);
    let mut sink_1 = fresh_sink().await;
    runner.run(&graph, &documents, &configs_1, &[], &[], &BTreeMap::new(), &mut cache, &mut sink_1).await.expect("first run");
    sink_1.record(RunMutation::SealRun(SealRun { status: semio_framework_artifact_workflow_run::RunStatus::Succeeded })).await.expect("seal first run");
    let prior = prior_node_records_from(&sink_1.document);

    let plan_unchanged = plan(&graph, &documents, &configs_1, &[], &[], &prior).await.expect("plan with unchanged config");
    assert!(plan_unchanged.recomputed.is_empty(), "nothing changed, plan must report every node clean: {:?}", plan_unchanged.recomputed);

    let mut configs_2 = configs_1.clone();
    configs_2.insert("config/node-a".to_string(), (Vec::new(), b"threshold=2".to_vec()));
    let plan_changed = plan(&graph, &documents, &configs_2, &[], &[], &prior).await.expect("plan with changed config");
    assert_eq!(plan_changed.recomputed, vec!["node-a".to_string()], "only node-a's own config changed, so only node-a should be recomputed by the plan");

    let mut sink_2 = fresh_sink().await;
    let report_2 = runner.run(&graph, &documents, &configs_2, &[], &[], &prior, &mut cache, &mut sink_2).await.expect("second run with changed config");
    assert_eq!(report_2.recomputed, vec!["node-a".to_string()], "node-a's config changed, so node-a must recompute even though its document and inputs did not");
    assert_eq!(report_2.clean, vec!["node-b".to_string()], "node-a's FakeHost output is fixed regardless of config, so node-b must stay clean");
}

/// 🧪️ A `--param` override bound onto a node's config field must dirty that node purely through the
/// fingerprint overlay (`node_parameter_overlay_bytes`) even though the raw config `.spr` bytes this
/// crate sends the app are byte-identical — see this crate's module doc on why the override isn't
/// patched into the opaque config bytes directly.
#[semio_framework_async_macros::async_test]
async fn parameter_overlay_alone_dirties_its_bound_node_without_changing_raw_config_bytes() {
    let graph = two_node_graph().await;
    let mut host = FakeHost::default();
    host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
    let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
    let mut cache = TestMediaCache::default();
    let documents = empty_documents(&graph);
    let configs = empty_configs(&graph);
    let bindings = vec![WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "node-a".into(), field_path: "/threshold".into() }];

    let mut sink_1 = fresh_sink().await;
    runner.run(&graph, &documents, &configs, &[], &bindings, &BTreeMap::new(), &mut cache, &mut sink_1).await.expect("first run");
    sink_1.record(RunMutation::SealRun(SealRun { status: semio_framework_artifact_workflow_run::RunStatus::Succeeded })).await.expect("seal first run");
    let prior = prior_node_records_from(&sink_1.document);

    let plan_unchanged = plan(&graph, &documents, &configs, &[], &bindings, &prior).await.expect("plan with no parameter values yet");
    assert!(plan_unchanged.recomputed.is_empty(), "no bound parameter value yet — nothing should be dirty: {:?}", plan_unchanged.recomputed);

    let parameter_values = vec![RunParameterValue { parameter_id: "p1".into(), value: "42".into() }];
    let plan_changed = plan(&graph, &documents, &configs, &parameter_values, &bindings, &prior).await.expect("plan with a parameter value bound");
    assert_eq!(plan_changed.recomputed, vec!["node-a".to_string()], "the bound node's fingerprint must change purely from the parameter overlay: {:?}", plan_changed);

    let mut sink_2 = fresh_sink().await;
    let report_2 = runner.run(&graph, &documents, &configs, &parameter_values, &bindings, &prior, &mut cache, &mut sink_2).await.expect("second run with a param override");
    assert_eq!(report_2.recomputed, vec!["node-a".to_string()]);
    assert_eq!(sink_2.node_configs.get("node-a"), Some(&configs["config/node-a"]), "raw config bytes sent to the app are untouched by the overlay — only the fingerprint changes");
}

//#region 🔖️ExchangeOrderingTests
/// 🧪️ `AppChannelHost::exchange`'s own doc promises no second `exchange` for the same `node`
/// handle overlaps the first. `SpaceRunner` enforces that structurally today (it owns `H` and
/// calls through `&mut self`, so two `exchange` futures against the SAME owned host could never
/// even be POLLED concurrently — the borrow checker forbids it). `RecorderHost` proves the
/// DETECTOR below actually catches overlap when nothing prevents it, so
/// `space_runner_never_overlaps_exchange_for_the_same_node_across_a_real_run`'s "never overlaps"
/// isn't a vacuous pass: it shares its bookkeeping through an `Rc<RefCell<_>>`, so cloning it
/// (unlike sharing one owned `H`) genuinely CAN be driven concurrently, and `exchange` yields
/// once mid-call (`semio_framework_async::yield_once`) so an interleaving executor has a real
/// chance to expose overlap.
#[derive(Default)]
struct RecorderState {
    next_handle: u32,
    in_flight: std::collections::HashSet<u32>,
    overlap_detected: bool,
    completed_in_order: Vec<u32>,
}

#[derive(Clone, Default)]
struct RecorderHost(std::rc::Rc<std::cell::RefCell<RecorderState>>);

/// 🧪️ Every `AppCommand` variant `RecorderHost`'s own test graphs ever send (no media ports —
/// see `two_independent_solo_nodes`) mapped to the SPECIFIC `AppFrame` reply `compute_node`
/// expects for it — `ReadDocument`/`ReadConfig` each demand their own typed frame
/// (`AppFrame::Document`/`AppFrame::Config`), not a generic `Done` (see `compute_node`'s own
/// `reply_to`/pattern-match).
fn reply_for(command: &AppCommand) -> AppFrame {
    match command {
        AppCommand::SetMergePolicy { seq, .. } => AppFrame::Done { in_reply_to: *seq },
        AppCommand::LoadConfig { seq, .. } => AppFrame::Done { in_reply_to: *seq },
        AppCommand::LoadDocument { seq, .. } => AppFrame::Done { in_reply_to: *seq },
        AppCommand::ReadDocument { seq } => AppFrame::Document { in_reply_to: *seq, pack: Vec::new(), spr: Vec::new(), ops: String::new() },
        AppCommand::ReadConfig { seq } => AppFrame::Config { in_reply_to: *seq, pack: Vec::new(), spr: Vec::new(), ops: String::new() },
        other => panic!("RecorderHost's test graphs never send {other:?}"),
    }
}

impl AppChannelHost for RecorderHost {
    async fn open(&mut self, _plugin_id: &str, _app_id: &str, _artifact_ref: &str) -> Result<u32, RunError> {
        let mut state = self.0.borrow_mut();
        state.next_handle += 1;
        Ok(state.next_handle)
    }

    async fn exchange(&mut self, _ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
        {
            let mut state = self.0.borrow_mut();
            if !state.in_flight.insert(node) {
                state.overlap_detected = true;
            }
        }
        semio_framework_async::yield_once().await;
        let frames = commands.iter().map(reply_for).collect();
        {
            let mut state = self.0.borrow_mut();
            state.in_flight.remove(&node);
            state.completed_in_order.push(node);
        }
        Ok(frames)
    }
}

fn two_independent_solo_nodes() -> Workflow {
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![workflow_node("node-a", Vec::new(), Vec::new()), workflow_node("node-b", Vec::new(), Vec::new())], edges: Vec::new() }
}

fn root_ctx() -> OperationContext {
    OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root_now(), capability: None }
}

/// 🧪️ Sanity check for `RecorderHost` itself: two callers racing the SAME node id, with nothing
/// serializing them (unlike `SpaceRunner`, which owns its host exclusively), DO overlap once
/// interleaved — proves the detector the next test relies on would actually catch a real
/// regression rather than passing no matter what.
#[test]
fn recorder_host_detects_genuine_overlap_when_nothing_prevents_it() {
    let recorder = RecorderHost::default();
    let mut a = recorder.clone();
    let mut b = recorder.clone();
    let ctx = root_ctx();
    let fut_a = a.exchange(&ctx, 7, vec![AppCommand::ReadDocument { seq: 1 }]);
    let fut_b = b.exchange(&ctx, 7, vec![AppCommand::ReadDocument { seq: 2 }]);
    let (_a, _b) = semio_framework_async::block_on(semio_framework_async::join2(fut_a, fut_b));
    assert!(recorder.0.borrow().overlap_detected, "two callers racing the same node id with no ownership guard must overlap");
}

/// 🧪️ The actual ordering property this ticket asks for: `SpaceRunner::run`'s own call pattern
/// (strictly sequential, one node at a time, in topological order) never overlaps two `exchange`
/// calls for the same node — even against `RecorderHost`'s yield point, which would expose an
/// accidental `join`/`spawn` introduced by a later refactor as `overlap_detected`.
#[semio_framework_async_macros::async_test]
async fn space_runner_never_overlaps_exchange_for_the_same_node_across_a_real_run() {
    let graph = two_independent_solo_nodes();
    let host = RecorderHost::default();
    let recorder = host.clone();
    let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
    let mut cache = TestMediaCache::default();
    let documents = empty_documents(&graph);
    let configs = empty_configs(&graph);
    let mut sink = fresh_sink().await;

    runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink).await.expect("both solo nodes compute cleanly against RecorderHost");

    let state = recorder.0.borrow();
    assert!(!state.overlap_detected, "SpaceRunner must never issue two exchange calls for the same node concurrently");
    assert_eq!(state.completed_in_order, vec![1, 2], "node-a (handle 1) completes strictly before node-b (handle 2) — sequential topological order preserved");
}

/// 🧪️ `compute_node` checks `self.cancel` BEFORE `open`/`exchange` (see `RunError::Cancelled`'s
/// doc) — cancelling `SpaceRunner`'s own token stops the run before its NEXT node. Node-a's own
/// `exchange` still completes (cancellation is checked between nodes, not preemptible mid-call —
/// the same honest limitation `semio-framework-os-services::ComputePool` documents), so this
/// asserts node-b specifically never runs.
#[semio_framework_async_macros::async_test]
async fn cancelling_the_run_token_stops_the_run_before_the_next_node() {
    let graph = two_independent_solo_nodes();
    let host = RecorderHost::default();
    let recorder = host.clone();
    let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
    let cancel = runner.cancel_token();
    let mut cache = TestMediaCache::default();
    let documents = empty_documents(&graph);
    let configs = empty_configs(&graph);
    let mut sink = fresh_sink().await;

    // 🛑️ Cancel BEFORE the run even starts — deterministic, no real sleep/race needed: the very
    // first `compute_node` call (node-a) must already observe `Cancelled`.
    cancel.cancel();
    let result = runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink).await;
    assert!(matches!(result, Err(RunError::Cancelled)), "a run cancelled before its first node must fail with RunError::Cancelled, got {result:?}");
    assert!(recorder.0.borrow().completed_in_order.is_empty(), "no node's exchange should run once the token is cancelled before the run starts");
}
//#endregion 🔖️ExchangeOrderingTests

#[semio_framework_async_macros::async_test]
async fn rejects_incompatible_edge_media_types() {
    let mut graph = two_node_graph().await;
    graph.nodes[1].inputs[0].spec.media_type = MediaType { class: MediaClass::Text, form: MediaForm::Document };
    let host = FakeHost::default();
    let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
    let mut cache = TestMediaCache::default();
    let documents = empty_documents(&graph);
    let configs = empty_configs(&graph);
    let mut sink = fresh_sink().await;
    let result = runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink).await;
    assert!(matches!(result, Err(RunError::Incompatible { .. })));
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_missing_required_input() {
    let node = workflow_node("solo", Vec::new(), vec![media_port("solo", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, true)]);
    let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node], edges: Vec::new() };
    assert!(matches!(validate_edge_kinds(&graph).await, Err(RunError::MissingRequiredInput { .. })));
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_multiplicity_one_input_with_two_incoming_edges() {
    let source_a = workflow_node("src-a", vec![media_port("src-a", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
    let source_b = workflow_node("src-b", vec![media_port("src-b", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
    let target = workflow_node("target", Vec::new(), vec![media_port("target", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, false)]);
    let graph = Workflow {
        schema: WORKFLOW_SCHEMA.into(),
        nodes: vec![source_a, source_b, target],
        edges: vec![
            WorkflowEdge { id: "e1".into(), source_node_id: "src-a".into(), source_port_id: "src-a:out:out".into(), target_node_id: "target".into(), target_port_id: "target:in:in".into(), contract: placeholder_media_contract("data.value").await },
            WorkflowEdge { id: "e2".into(), source_node_id: "src-b".into(), source_port_id: "src-b:out:out".into(), target_node_id: "target".into(), target_port_id: "target:in:in".into(), contract: placeholder_media_contract("data.value").await },
        ],
    };
    assert!(matches!(validate_edge_kinds(&graph).await, Err(RunError::MultiplicityViolation { .. })));
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_unregistered_conversion() {
    let source = workflow_node("src", vec![media_port("src", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
    let target = workflow_node("dst", Vec::new(), vec![media_port("dst", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, false)]);
    let mut contract = placeholder_media_contract("data.value").await;
    // 🧪️ A conversion form pair this test file never registers a converter for — distinct from
    // any pair the `media_converter_registry_*` tests below register, so the global
    // `MEDIA_CONVERTERS` table (shared across all tests in this process) can't race with this one.
    contract.conversion = Some((MediaForm::Trinity, MediaForm::Dag));
    let graph = Workflow {
        schema: WORKFLOW_SCHEMA.into(),
        nodes: vec![source, target],
        edges: vec![WorkflowEdge { id: "e1".into(), source_node_id: "src".into(), source_port_id: "src:out:out".into(), target_node_id: "dst".into(), target_port_id: "dst:in:in".into(), contract }],
    };
    assert!(matches!(validate_edge_kinds(&graph).await, Err(RunError::UnregisteredConversion { .. })));
}

#[semio_framework_async_macros::async_test]
async fn convert_media_is_identity_when_contract_has_no_conversion() {
    let contract = placeholder_media_contract("data.value").await;
    let media = Media { media_type: fake_media_type(), payload: MediaPayload::Structured { schema: "test".into(), json: "\"hi\"".into() } };
    let converted = convert_media(&contract, media.clone()).expect("identity conversion never fails");
    assert_eq!(converted, media);
}

#[semio_framework_async_macros::async_test]
async fn media_converter_registry_applies_registered_converter() {
    // 🧪️ A `(class, from, to)` triple unique to this test (see `validate_rejects_unregistered_
    // conversion`'s note on the shared global table).
    register_media_converter(MediaClass::Kit, MediaForm::Design, MediaForm::Sequence, |media| Ok(Media { media_type: media.media_type, payload: MediaPayload::Structured { schema: "converted".into(), json: "\"converted\"".into() } }));
    let mut contract = placeholder_media_contract("kit.design").await;
    contract.media_type = MediaType { class: MediaClass::Kit, form: MediaForm::Sequence };
    contract.conversion = Some((MediaForm::Design, MediaForm::Sequence));
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design }, payload: MediaPayload::Structured { schema: "design".into(), json: "\"raw\"".into() } };
    let converted = convert_media(&contract, media).expect("registered converter applies");
    assert_eq!(converted.payload, MediaPayload::Structured { schema: "converted".into(), json: "\"converted\"".into() });
}

#[test]
fn vector_to_raster_rasterizes_svg_to_a_2d_image_media() {
    let svg = Media {
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 4 4" width="4" height="4"><rect width="4" height="4" fill="#ff0000"/></svg>"##.into() },
    };
    let raster = vector_to_raster(&svg).expect("rasterizes real svg text");
    assert_eq!(raster.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Raster });
    let MediaPayload::Structured { schema, json } = raster.payload else { panic!("expected structured payload") };
    assert_eq!(schema, "2d.image");
    assert!(!json.is_empty(), "png_base64 must be non-empty for real svg content");
}

#[test]
fn vector_to_raster_rejects_non_structured_payload() {
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Binary { format_kind: "png".into(), blob_hash: "hash".into() } };
    assert!(vector_to_raster(&media).is_err());
}

#[semio_framework_async_macros::async_test]
async fn register_builtin_converters_wires_vector_to_raster_through_convert_media() {
    register_builtin_converters();
    let mut contract = placeholder_media_contract("2d.drawing").await;
    contract.media_type = MediaType { class: MediaClass::TwoD, form: MediaForm::Raster };
    contract.conversion = Some((MediaForm::Vector, MediaForm::Raster));
    let media = Media {
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2 2" width="2" height="2"><rect width="2" height="2" fill="#00ff00"/></svg>"##.into() },
    };
    let converted = convert_media(&contract, media).expect("builtin vector->raster converter is registered");
    assert_eq!(converted.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Raster });
}

//#region 🔖️NativeManifestSmoke
/// 🧭️ Walks up from `CARGO_MANIFEST_DIR` looking for `nx.json` — the SAME strategy
/// `🏗️bootstrap/🦀️.rs`'s own `find_repo_root` uses, duplicated here (not `include!`d — the bin crate's
/// own doc explains a `[[bin]]` target does not share the lib's module tree).
fn test_repo_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("nx.json").is_file() {
            return dir;
        }
        assert!(dir.pop(), "walked past the filesystem root looking for nx.json");
    }
}

/// 🧪️ Loads the committed note descriptor using the canonical runtime profile order.
/// Absence skips this optional integration probe and is never publication identity evidence.
#[semio_framework_async_macros::async_test]
async fn note_plugin_manifest_loads_from_its_committed_descriptor() {
    let repo_root = test_repo_root();
    let descriptor_path = repo_root.join("✏️s/🔌️plugins/🗒️note/🛂️.descriptor.semio");
    assert!(descriptor_path.is_file(), "committed note descriptor missing at {}", descriptor_path.display());

    let candidate_wasm_paths = [repo_root.join("target/wasm32-wasip2/wasm-dev/semio_s_plugin_note.wasm"), repo_root.join("target/wasm32-wasip2/wasm-release/semio_s_plugin_note.wasm")];
    let Some(wasm_path) = candidate_wasm_paths.into_iter().find(|path| path.is_file()) else {
        eprintln!("[DEBUG] note_plugin_manifest_loads_from_its_committed_descriptor: SKIPPED — no compiled note wasm in any candidate location");
        return;
    };

    let mut plugin_paths = HashMap::new();
    plugin_paths.insert("note".to_string(), wasm_path);
    let mut descriptor_paths = HashMap::new();
    descriptor_paths.insert("note".to_string(), descriptor_path);
    // 🚫️async: this `#[test] fn` body is a sanctioned executor entry point (R4 clause 5) — the
    // one thread root driving `WasmtimeNodeHost::new`/`manifest_for`, both `async fn` now that
    // they build/drive the real `NativeKernelRuntime` (packet `run-kernel-wiring`). Same
    // `semio_framework_async::block_on` convention every other test in this module already uses
    // (see this crate's own `Cargo.toml` doc comment on why: a plain single-poll executor, not
    // tokio).
    let mut host = WasmtimeNodeHost::new(plugin_paths, descriptor_paths, Arc::new(InMemoryBlobStore::default())).await;

    let manifest = host.manifest_for("note").await.expect("note must load natively from its committed descriptor, zero live describe() calls");
    assert_eq!(manifest.plugin_id, "note");
    assert!(!manifest.apps.is_empty(), "note's real manifest declares at least one app");
    assert!(manifest.dependencies.is_empty(), "note's committed descriptor declares zero PluginManifest.dependencies");

    let (routed_plugins, _routes) = host.io_router_stats().await;
    assert_eq!(routed_plugins, 1, "note must be the one plugin registered with the io router after this load");
    assert!(host.plugin_graph().is_registered("note").await.unwrap_or(false), "note must be registered in the plugin graph");
    assert!(host.app_router().owned_surface_gaps().await.is_empty(), "note's own panels leave no viewer/editor surface gap");
}
//#endregion 🔖️NativeManifestSmoke
