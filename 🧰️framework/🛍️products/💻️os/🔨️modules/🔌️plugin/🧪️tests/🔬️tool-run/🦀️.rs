//! ⏯️ Runtime laws of the tool run ledger and driver (`📋️tool-run-contract.md` §2.2, §2.7, §2.8, §3.3):
//! overlay visible while committed is untouched, zero-trace abort, one-edit finalize, rebase with a
//! revalidation conflict, stale no-ops, single step, reconfigure resume and the O(k) append budget — plus
//! the integration seams (§3.2 trace lane delivery and cursor protocol, §3.4 presence, minimal dirty scope,
//! the provisional instance flag).
//! Language-agnostic expectations: `🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json`.

use super::*;
use crate::test_app_mutation_fixture::{ChangeTestConfigSelection, SetCount, SetLabel, TestConfig, TestConfigMutation, TestMutation, TestSnapshot};
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunDefinition, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunStageDefinition, ToolRunState, ToolRunStepRing, ToolRunTick, ToolRunTickWriter, ToolRunTraceCursor,
    ToolRunTraceDelta, ToolRunTraceKind, ToolRunTraceSubject, ToolRunVerdict,
};
use crate::ViewWindowInstance;
use semio_framework_ui_scene::World3dScene;
use store::{Backbone, BackboneMessage, MemoryBackbone};

const TOOL_RUN_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/⏯️tool-run/🔣️.json");

fn fixture() -> Value {
    let fixture: Value = serde_json::from_str(TOOL_RUN_FIXTURE_JSON).expect("tool run fixture parses");
    assert_eq!(fixture["schema"], "framework.plugin.tool-run.v1");
    fixture
}

fn number(value: &Value) -> u64 {
    value.as_u64().unwrap_or_else(|| panic!("fixture number expected, found {value}"))
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, found {value}"))
}

//#region 🧸️ToyRunApp
fn toy_definition(rebase: ToolRunRebasePolicy) -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: true,
        rebase,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: LocalizedLabel::native("units", "Einheiten"),
        stages: vec![ToolRunStageDefinition { id: "fill".into(), label: LocalizedLabel::native("Filling", "Füllen") }],
        counters: vec![semio_framework_tool_run::ToolRunCounterDefinition { id: "resumedFrom".into(), label: LocalizedLabel::native("Resumed from", "Fortgesetzt ab") }],
        reasons: vec![
            ToolRunReasonDefinition { code: 1, id: "fits".into(), verdict: ToolRunVerdict::Success, template: LocalizedLabel::native("Fits", "Passt") },
            ToolRunReasonDefinition { code: 2, id: "conflict".into(), verdict: ToolRunVerdict::Danger, template: LocalizedLabel::native("Conflicts with the document", "Konflikt mit dem Dokument") },
        ],
        trace: ToolRunTraceKind::Entity,
        run_job: JobKindId::new("toyFill.run"),
        revalidate_job: Some(JobKindId::new("toyFill.revalidate")),
    }
}

fn toy_target(config: &TestConfig) -> u32 {
    config.selected.as_deref().and_then(|value| value.parse().ok()).unwrap_or(number(&fixture()["defaultTarget"]) as u32)
}

fn encoded(mutation: TestMutation) -> Vec<u8> {
    ::protocol::OpBinary::encode_op(&mutation).expect("toy op encodes")
}

/// 🧸️ One unit appends `SetCount(base + unit)` and `SetLabel("unit-<unit>")`, one entity and one `success`
/// trace record per unit; a revalidation retracts from the first unit the head no longer continues.
/// ✍️ Both jobs write through `ToolRunTickWriter::with_provisional_base`: a resumed or revalidate job starts at
/// the provisional length it continues from, so its `retract_to` below that length emits `retractTo`.
struct ToyRunJob {
    purpose: ToolRunJobPurpose,
    writer: ToolRunTickWriter,
    base_count: i32,
    target: u32,
    done: u32,
    resumed_from: u32,
    provisional_counts: Vec<i32>,
    checkpoint_due: bool,
    finished: bool,
    closing: bool,
}

impl ToyRunJob {
    fn emit(cx: &mut semio_framework_job::StepContext<'_>, tick: ToolRunTick) -> semio_framework_job::StepOutcome {
        let bytes = tick.encode().expect("toy tick encodes");
        match cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Preview, &bytes) {
            Ok(payload) => semio_framework_job::StepOutcome::PreviewReady(payload),
            Err(rejected) => {
                drop(rejected.into_source());
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) })
            }
        }
    }

    fn complete() -> semio_framework_job::StepOutcome {
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn run_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if self.done > self.target {
            self.writer.retract_to(self.target * 2);
            self.done = self.target;
        }
        let mut units = 0;
        while self.done < self.target && units < 64 {
            self.done += 1;
            units += 1;
            let unit = u64::from(self.done);
            self.writer.append_op(encoded(SetCount { value: self.base_count + self.done as i32 }.into())).expect("toy op fits the provisional cap");
            self.writer.append_op(encoded(SetLabel { value: format!("unit-{}", self.done) }.into())).expect("toy op fits the provisional cap");
            self.writer.append_entity(unit);
            self.writer.upsert(unit, ToolRunVerdict::Success, 1, ToolRunTraceSubject::Entity { entity: unit });
            cx.consume_fuel(1);
            if cx.should_yield() {
                break;
            }
        }
        if self.writer.is_empty() {
            return Self::complete();
        }
        self.writer.progress(ToolRunProgress {
            identity: self.writer.identity(),
            sequence: 0,
            state: ToolRunState::Running,
            stage: 0,
            completed: u64::from(self.done),
            total: Some(u64::from(self.target)),
            counters: vec![ToolRunCounter { counter: 0, value: u64::from(self.resumed_from) }],
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        });
        self.checkpoint_due = true;
        let tick = self.writer.finish().expect("a pending toy tick");
        Self::emit(cx, tick)
    }

    fn revalidate_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if self.finished {
            return Self::complete();
        }
        self.finished = true;
        let head = self.base_count;
        let Some(first) = self.provisional_counts.iter().enumerate().position(|(index, value)| *value != head + index as i32 + 1) else { return Self::complete() };
        self.writer.retract_to(first as u32 * 2);
        for unit in first + 1..=self.provisional_counts.len() {
            self.writer.upsert(unit as u64, ToolRunVerdict::Danger, 2, ToolRunTraceSubject::Entity { entity: unit as u64 });
        }
        let tick = self.writer.finish().expect("a pending revalidation tick");
        Self::emit(cx, tick)
    }
}

impl semio_framework_job::InteractiveJob for ToyRunJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if self.checkpoint_due {
            self.checkpoint_due = false;
            return match cx.payload_from_bytes(semio_framework_job::JobPayloadStream::CheckpointState, &self.done.to_le_bytes()) {
                Ok(state) => semio_framework_job::StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state, applied_progress: u64::from(self.done) }),
                Err(rejected) => {
                    drop(rejected.into_source());
                    semio_framework_job::StepOutcome::Yield
                }
            };
        }
        match self.purpose {
            ToolRunJobPurpose::Run => self.run_step(cx),
            ToolRunJobPurpose::Revalidate => self.revalidate_step(cx),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if !self.provisional_counts.is_empty() || self.provisional_counts.capacity() != 0 {
            self.provisional_counts = Vec::new();
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.provisional_counts.capacity() == 0
    }
}

#[derive(Default)]
struct ToyRunApp;

impl ArtifactApp for ToyRunApp {
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.tool-run", standard: StandardId("1"), subset: SubsetId::ANY };
    const APP_ID: &'static str = "s.test.tool-run@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-tool-run/v1";
    type Snapshot = TestSnapshot;
    type Mutation = TestMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TestMutation;

    fn build_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> {
        let done = request.checkpoint.and_then(|bytes| bytes.try_into().ok()).map_or(0, u32::from_le_bytes);
        let provisional_counts = request.provisional.iter().filter_map(|op| match op {
            TestMutation::SetCount(set) => Some(set.value),
            TestMutation::SetLabel(_) => None,
        });
        let job = ToyRunJob {
            purpose: request.purpose,
            writer: ToolRunTickWriter::with_provisional_base(request.identity, request.provisional.len() as u32),
            base_count: request.snapshot.count,
            target: toy_target(&request.config),
            done,
            resumed_from: done,
            provisional_counts: match request.purpose {
                ToolRunJobPurpose::Run => Vec::new(),
                ToolRunJobPurpose::Revalidate => provisional_counts.collect(),
            },
            checkpoint_due: false,
            finished: false,
            closing: false,
        };
        Ok(Some(Box::new(job)))
    }

    async fn initial_snapshot() -> TestSnapshot {
        TestSnapshot::default()
    }

    async fn handle(
        command: &TestMutation,
        _doc: &ArtifactView<'_, TestSnapshot>,
        _cfg: &ConfigView<'_, TestConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<TestMutation, TestConfigMutation, NoDraftMutation> {
        Ok(Emit { artifact_mutations: vec![command.clone()], ..Default::default() })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        if body_key != text(&fixture()["traceLane"]["bodyKey"]) {
            return built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={}", doc.snapshot.count)));
        }
        let provisional = |unit: i32| doc.tool_run().is_some_and(|run| run.provisional_entities.contains(&(unit as u64)));
        let instances: Vec<Value> = (1..=doc.snapshot.count).map(|unit| serde_json::json!({ "id": format!("unit-{unit}"), "meshId": "unit", "provisional": provisional(unit) })).collect();
        let scene = World3dScene::base("{}".into(), "[]".into(), Value::Array(instances).to_string(), semio_framework_ui_scene::world3d_default_selection_json());
        let surface = scene_surface(text(&fixture()["traceLane"]["surfaceId"]), SurfaceKind::World3d, &scene)?;
        column().try_id("world").map_err(|_| PluginAssemblyError::new("toy", "world id"))?.try_child(surface).map_err(|_| PluginAssemblyError::new("toy", "world child"))?.try_build().map(built_to_component_tree).map_err(|_| PluginAssemblyError::new("toy", "world build"))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(bounded_document_store_owners::<Self::Draft, Self::DraftMutation>())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("tool-run-doc", 4_096))
    }

    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(mutation_fixture::no_state::presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(mutation_fixture::no_state::transient_store_disposer())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(mutation_fixture::no_state::presence_peer_retirement_factory())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(mutation_fixture::no_state::presence_local_root_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(mutation_fixture::no_state::transient_local_root_retirement_factory())
    }
}
//#endregion 🧸️ToyRunApp

//#region 🧰️Harness
type ToyApp = VcsArtifactApp<ToyRunApp>;

/// 🛂️ The toy manifest declares both tools with their `ToolRunDefinition` — the run's source of record.
async fn toy_manifest() -> App {
    let fixture = fixture();
    let mut builder = App::builder(ToyRunApp::APP_ID, LocalizedLabel::data("Tool Run Fixture"))
        .await
        .document(["state"])
        .mode("edit", LocalizedLabel::data("Edit"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::data("Main"), "tool-run.main", SurfaceKind::Canvas2d, IconName::AppWindow)
        .await
        .window_kind(text(&fixture["traceLane"]["windowId"]), LocalizedLabel::data("World"), text(&fixture["traceLane"]["bodyKey"]), SurfaceKind::World3d, IconName::AppWindow)
        .await;
    let mut tools = Vec::new();
    for (key, rebase) in [("toolId", ToolRunRebasePolicy::Revalidate), ("freezeToolId", ToolRunRebasePolicy::Freeze)] {
        let id = text(&fixture[key]);
        builder = builder.tool(ToolDefinition { run: Some(toy_definition(rebase)), ..ToolDefinition::new(id, LocalizedLabel::native("Toy fill", "Spielfüllung"), IconName::PaintBucket).await }).await;
        tools.push(ToolRef::new(id).await);
    }
    App::from_builder(builder.mode_tools("edit", tools).await).await
}

async fn toy_app(target: u64) -> ToyApp {
    let mut app = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest()).await;
    set_target(&mut app, target).await;
    app
}

async fn set_target(app: &mut ToyApp, target: u64) {
    app.config_store.dispatch(ArtifactCommand::Apply { mutations: vec![ChangeTestConfigSelection { selected: Some(target.to_string()) }.into()], description: None }).await.expect("toy target config applies");
}

fn toy_meta() -> ActionMeta {
    artifact_app_laws::meta(text(&fixture()["actor"]))
}

async fn tool_run_action(app: &mut ToyApp, action: &str, arguments: Vec<(String, DslValue)>) -> DslValue {
    app.handle_action(action, Some(&DslValue::Object(arguments)), &toy_meta()).await.unwrap_or_else(|fault| panic!("{action} dispatch: {fault:?}")).output
}

async fn run_action(app: &mut ToyApp, action: &str) -> DslValue {
    let arguments = run_arguments(app);
    tool_run_action(app, action, arguments).await
}

fn run_arguments(app: &ToyApp) -> Vec<(String, DslValue)> {
    let slot = app.tool_runs.slot().expect("a run slot exists");
    vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(slot.generation.to_string()))]
}

async fn start(app: &mut ToyApp, tool_id: &str) {
    let output = tool_run_action(app, "toolRunStart", vec![("toolId".into(), DslValue::String(tool_id.into()))]).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "start spawns the run job");
}

async fn pump_until(app: &mut ToyApp, what: &str, done: impl Fn(&ToyApp) -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return;
        }
        app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
    }
    panic!("{what} never settled; state {:?}", app.tool_runs.state());
}

async fn render_text(app: &mut ToyApp, body_key: &str) -> String {
    let tree = app.render(body_key, None, &ViewModel::default()).await.unwrap_or_else(|fault| panic!("render {body_key}: {fault:?}"));
    artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error| panic!("project {body_key}: {error}"))
}

fn find_node<'a>(node: &'a Value, key: &str) -> Option<&'a Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| find_node(child, key))
}

async fn attach_probe(app: &mut ToyApp, channel: &str) -> MemoryBackbone {
    let (backbone, probe) = MemoryBackbone::pair(channel, channel).await;
    app.attach_backbone(store::Backbones::Memory(backbone)).await.expect("attach probe backbone");
    probe
}

/// 📮️ Drains what the app sent: `(mutations batches, full snapshots)`; acks are not document traffic.
async fn drain(probe: &mut MemoryBackbone) -> (usize, usize) {
    probe.receive().await.expect("probe receive").into_iter().fold((0, 0), |(mutations, snapshots), message| match message {
        BackboneMessage::Mutations { .. } => (mutations + 1, snapshots),
        BackboneMessage::Snapshot { .. } => (mutations, snapshots + 1),
        BackboneMessage::Ack { .. } => (mutations, snapshots),
    })
}

fn close(app: &mut ToyApp) {
    artifact_app_laws::close_registered_fixture_app(app);
    assert!(app.tool_runs.terminal_is_empty(), "the tool run ledger retires every owner on close");
}
//#endregion 🧰️Harness

#[semio_framework_async_macros::async_test]
async fn tool_run_ticks_render_the_overlay_while_the_committed_document_stays_untouched() {
    let fixture = fixture();
    let expected = &fixture["overlay"];
    let mut app = toy_app(number(&expected["units"])).await;
    let generation = app.store.generation();
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "overlay run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding()).await;
    assert_eq!(app.store.generation(), generation, "ticks never touch the store generation");
    assert_eq!(app.snapshot().expect("committed snapshot").count, number(&expected["committedCount"]) as i32);
    assert_eq!(app.tool_runs.provisional().len() as u64, number(&expected["provisionalOps"]));
    let entities: Vec<u64> = expected["entities"].as_array().expect("entities").iter().map(number).collect();
    assert_eq!(app.tool_runs.provisional_entities().expect("entities").iter().copied().collect::<Vec<_>>(), entities);
    assert_eq!(app.tool_runs.trace().expect("trace").len() as u64, number(&expected["traceRecords"]));
    let body = render_text(&mut app, "main").await;
    assert!(body.contains(text(&expected["mainBodyText"])), "the renderer reads committed ⊕ provisional: {body}");
    let panel_expected = &fixture["panel"];
    let panel: Value = serde_json::from_str(&render_text(&mut app, text(&panel_expected["bodyKey"])).await).expect("panel projection parses");
    assert_eq!(panel["key"], panel_expected["rootId"]);
    let status = find_node(&panel, text(&panel_expected["status"]["id"])).expect("status line");
    assert_eq!(status["accessibility"]["live"], panel_expected["status"]["live"]);
    let bar = find_node(&panel, text(&panel_expected["progress"]["id"])).expect("progress bar");
    assert_eq!(bar["component"]["type"], panel_expected["progress"]["type"]);
    assert_eq!(bar["component"]["completed"].as_f64(), Some(number(&panel_expected["progress"]["completed"]) as f64));
    assert_eq!(bar["component"]["total"].as_f64(), Some(number(&panel_expected["progress"]["total"]) as f64));
    let finalize = find_node(&panel, text(&panel_expected["finalize"]["id"])).expect("finalize button");
    assert_eq!(finalize["component"]["type"], "button");
    assert_eq!(finalize["accessibility"]["shortcut"], panel_expected["finalize"]["shortcut"]);
    assert!(finalize["bindings"].as_array().is_some_and(|bindings| !bindings.is_empty()), "finalize is a real bound button");
    let presence = app.tool_run_presence().expect("presence summary");
    assert_eq!((presence.completed, presence.total), (number(&expected["units"]), Some(number(&expected["units"]))));
    assert!(app.tool_run_trace_delta(None).is_some_and(|delta| !delta.is_empty() && !delta.contains(['+', '/', '='])), "trace delta is base64url");
    run_action(&mut app, "toolRunAbort").await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted)).await;
    assert!(render_text(&mut app, "main").await.contains(text(&expected["committedBodyText"])), "an aborted run renders the committed document again");
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_abort_leaves_store_generation_edits_command_log_and_outbox_untouched() {
    let fixture = fixture();
    let expected = &fixture["abort"];
    let mut app = toy_app(number(&expected["target"])).await;
    let mut probe = attach_probe(&mut app, "tool-run-abort").await;
    drain(&mut probe).await;
    app.refresh_cache().await.expect("backfill the command log before the invariant capture");
    let (generation, edits, commands) = (app.store.generation(), app.store.envelope().vcs.edits.len(), app.command_log.len());
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "first provisional unit", |app| app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeAbort"]) * number(&fixture["opsPerUnit"])).await;
    let output = run_action(&mut app, "toolRunAbort").await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("closeJob"));
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    assert_eq!(app.tool_runs.state().map(ToolRunState::as_str), Some(text(&expected["state"])));
    assert_eq!(app.store.generation(), generation, "abort: store generation");
    assert_eq!(app.store.envelope().vcs.edits.len(), edits, "abort: envelope.vcs.edits length");
    assert_eq!(app.command_log.len(), commands, "abort: command log");
    assert_eq!(drain(&mut probe).await, (0, 0), "abort: memory-backbone outbox");
    assert!(app.tool_runs.provisional().is_empty(), "abort retires every provisional op");
    assert_eq!(app.snapshot().expect("committed").count, 0);
    drop(probe);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_finalize_publishes_one_grouped_edit_one_mutations_batch_and_undo_removes_all() {
    let fixture = fixture();
    let expected = &fixture["finalize"];
    let mut app = toy_app(number(&expected["units"])).await;
    let mut probe = attach_probe(&mut app, "tool-run-finalize").await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    drain(&mut probe).await;
    let (generation, edits, commands) = (app.store.generation(), app.store.envelope().vcs.edits.len(), app.command_log.len());
    let output = run_action(&mut app, "toolRunFinalize").await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    pump_until(&mut app, "finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work()).await;
    assert_eq!(app.store.generation() - generation, number(&expected["storeGenerationAdded"]));
    assert_eq!((app.store.envelope().vcs.edits.len() - edits) as u64, number(&expected["editsAdded"]));
    assert_eq!((app.command_log.len() - commands) as u64, number(&expected["commandRowsAdded"]));
    let edit = app.store.envelope().vcs.edits.last().expect("finalized edit");
    assert_eq!(edit.forwards.len(), number(&fixture["overlay"]["provisionalOps"]) as usize, "every provisional op lands in the one edit");
    assert!(edit.mutation_meta.iter().all(|meta| meta.group_id.as_deref() == Some(text(&expected["groupId"]))), "the edit is stamped with the run's group id");
    assert_eq!(drain(&mut probe).await, (number(&expected["mutationsMessages"]) as usize, number(&expected["snapshotMessages"]) as usize), "one Mutations batch, no snapshot broadcast");
    let committed = app.snapshot().expect("committed after finalize");
    assert_eq!((committed.count, committed.label.as_str()), (number(&expected["countAfterFinalize"]) as i32, text(&expected["labelAfterFinalize"])));
    assert!(app.tool_runs.provisional().is_empty());
    app.store.dispatch(ArtifactCommand::Undo).await.expect("undo the finalized run");
    let undone = app.snapshot().expect("committed after undo");
    assert_eq!((undone.count, undone.label.as_str()), (number(&expected["countAfterUndo"]) as i32, text(&expected["labelAfterUndo"])), "one undo removes the whole run");
    drop(probe);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_remote_ingest_rebases_and_a_revalidation_conflict_returns_to_complete_with_the_next_generation() {
    let fixture = fixture();
    let expected = &fixture["conflict"];
    let mut app = toy_app(number(&fixture["overlay"]["units"])).await;
    let mut probe = attach_probe(&mut app, "tool-run-conflict").await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    let mut remote = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest()).await;
    let mut remote_probe = attach_probe(&mut remote, "tool-run-conflict-remote").await;
    remote.store.set_local_actor_id(Some("remote".into())).expect("remote actor");
    remote.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: number(&expected["remoteCount"]) as i32 }.into()], description: None }).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.store.generation();
    app.tick_backbone().await.expect("ingest remote edit");
    assert_eq!(app.store.generation(), generation + 1, "the remote edit is ingested");
    pump_until(&mut app, "rebase refold settles", |app| app.tool_runs.slot().is_some_and(|slot| u64::from(slot.generation) == number(&expected["generationAfterRebase"])) && !app.tool_runs.is_refolding()).await;
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Complete));
    let store_generation = app.store.generation();
    run_action(&mut app, "toolRunFinalize").await;
    pump_until(&mut app, "revalidation returns", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work()).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!(slot.state.as_str(), text(&expected["state"]));
    assert_eq!(u64::from(slot.generation), number(&expected["generationAfterConflict"]));
    assert_eq!(u64::from(app.tool_runs.conflicts()), number(&expected["retracted"]));
    assert_eq!(app.store.generation(), store_generation, "a conflict return never publishes");
    assert!(app.tool_runs.provisional().is_empty(), "conflicting ops are retracted");
    assert!(app.tool_runs.steps().expect("steps").iter().any(|step| step.reason == semio_framework_tool_run::TOOL_RUN_REASON_CONFLICT), "a danger conflict step is shown");
    drop((probe, remote_probe));
    close(&mut app);
    close(&mut remote);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_stale_generation_and_run_actions_are_silent_no_ops_and_finalize_is_illegal_outside_complete() {
    let fixture = fixture();
    let mut app = toy_app(number(&fixture["abort"]["target"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "run is running", |app| app.tool_runs.state() == Some(ToolRunState::Running)).await;
    let slot = app.tool_runs.slot().expect("slot");
    let stale = tool_run_action(&mut app, "toolRunPause", vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(number(&fixture["stale"]["wrongGeneration"]).to_string()))]).await;
    assert_eq!(stale.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])));
    let wrong_run = tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(number(&fixture["stale"]["wrongRun"]).to_string())), ("generation".into(), DslValue::String(slot.generation.to_string()))]).await;
    assert_eq!(wrong_run.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])));
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running), "stale actions change nothing");
    let generation = app.store.generation();
    let illegal = run_action(&mut app, "toolRunFinalize").await;
    assert_eq!(illegal.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["illegal"]["code"])));
    assert_eq!(app.store.generation(), generation, "an illegal finalize publishes nothing");
    let busy = tool_run_action(&mut app, "toolRunStart", vec![("toolId".into(), DslValue::String(text(&fixture["toolId"]).into()))]).await;
    assert_eq!(busy.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["busy"]["code"])));
    let panel: Value = serde_json::from_str(&render_text(&mut app, FRAMEWORK_TOOL_RUN_BODY_KEY).await).expect("panel parses");
    let finalize = find_node(&panel, text(&fixture["panel"]["finalize"]["id"])).expect("finalize button");
    assert_eq!(finalize["accessibility"]["description"], fixture["panel"]["runningFinalizeDescription"], "finalize stays present and describes why it is disabled while running");
    run_action(&mut app, "toolRunAbort").await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_pause_then_step_drives_exactly_one_unit_per_step() {
    let fixture = fixture();
    let expected = &fixture["pauseStep"];
    let mut app = toy_app(number(&expected["target"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running)).await;
    assert_eq!(run_action(&mut app, "toolRunPause").await.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"));
    for _ in 0..8 {
        app.advance_typed_operation_publication().await.expect("paused turn");
    }
    let mut provisional = app.tool_runs.provisional().len() as u64;
    for _ in 0..number(&expected["steps"]) {
        assert_eq!(run_action(&mut app, "toolRunStep").await.get("toolRun").and_then(DslValue::as_str), Some("driveOneUnit"));
        pump_until(&mut app, "single step settles", |app| !app.tool_runs.has_pending_work()).await;
        let after = app.tool_runs.provisional().len() as u64;
        assert_eq!(after - provisional, number(&expected["opsPerStep"]), "one step is exactly one algorithm unit");
        provisional = after;
        for _ in 0..8 {
            app.advance_typed_operation_publication().await.expect("paused turn");
        }
        assert_eq!(app.tool_runs.provisional().len() as u64, provisional, "a paused run schedules nothing");
    }
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Paused));
    assert_eq!(run_action(&mut app, "toolRunPause").await.get("toolRun").and_then(DslValue::as_str), Some("schedule"), "the shared mod+alt+enter chord binds toolRunPause, which resumes a paused run");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running));
    run_action(&mut app, "toolRunAbort").await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_reconfigure_resumes_from_its_checkpoint_and_a_lowered_target_retracts() {
    let fixture = fixture();
    let expected = &fixture["reconfigure"];
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    let mut app = toy_app(number(&expected["initialTarget"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "initial target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    let run = app.tool_runs.slot().expect("slot").run;
    set_target(&mut app, number(&expected["raisedTarget"])).await;
    pump_until(&mut app, "raised target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((slot.run, u64::from(slot.generation)), (run, number(&expected["generationAfterRaise"])), "reconfigure keeps the run id and bumps the generation");
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the job resumed from its checkpoint");
    assert!(render_text(&mut app, "main").await.contains(&format!("count={}", number(&expected["raisedTarget"]))));
    set_target(&mut app, number(&expected["loweredTarget"])).await;
    pump_until(&mut app, "lowered target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["loweredTarget"]) * ops_per_unit && !app.tool_runs.is_refolding()).await;
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterLower"]));
    assert!(render_text(&mut app, "main").await.contains(&format!("count={}", number(&expected["loweredTarget"]))), "the retracted tail leaves the overlay");
    assert_eq!(app.snapshot().expect("committed").count, 0, "reconfigure never commits");
    run_action(&mut app, "toolRunAbort").await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_freeze_policy_rejects_local_artifact_emits_with_busy() {
    let fixture = fixture();
    let mut app = toy_app(number(&fixture["abort"]["target"])).await;
    start(&mut app, text(&fixture["freezeToolId"])).await;
    let result = app.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { artifact_mutations: vec![SetCount { value: 1 }.into()], ..Default::default() }, &toy_meta()).await;
    let fault = result.err().expect("a freezing run rejects local artifact emits");
    assert_eq!(fault.code, FaultCode::new(text(&fixture["busy"]["code"])));
    run_action(&mut app, "toolRunAbort").await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    close(&mut app);
}

/// ⏱️ Bench-style law: a Nakagin-sized fill (771 ticks, two ops each) appends to the overlay in O(k).
/// Machine-load note: tests share the host with concurrent cargo builds, so up to
/// `allowedPreemptedTicks` single ticks may exceed the budget through preemption alone; a real O(n) fold
/// would exceed it on every late tick.
#[semio_framework_async_macros::async_test]
async fn tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks() {
    let fixture = fixture();
    let expected = &fixture["bench"];
    let mut app = toy_app(number(&fixture["defaultTarget"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    let identity = app.tool_runs.identity().expect("identity");
    let label = "x".repeat(number(&expected["labelBytes"]) as usize);
    let ticks = number(&expected["ticks"]);
    let budget = std::time::Duration::from_micros(number(&expected["budgetUs"]));
    let mut over_budget = 0;
    let mut worst = std::time::Duration::ZERO;
    for index in 0..ticks {
        let tick = ToolRunTick {
            identity,
            sequence: index,
            progress: None,
            steps: Vec::new(),
            trace: Vec::new(),
            append_ops: vec![encoded(SetCount { value: index as i32 + 1 }.into()), encoded(SetLabel { value: label.clone() }.into())],
            append_entities: vec![index + 1],
            retract_to: None,
        };
        let started = std::time::Instant::now();
        let receipt = app.tool_runs.apply_tick(tick).expect("tick applies");
        let elapsed = started.elapsed();
        assert_eq!(receipt.appended, 2);
        worst = worst.max(elapsed);
        if elapsed >= budget {
            over_budget += 1;
        }
    }
    assert!(over_budget <= number(&expected["allowedPreemptedTicks"]), "{over_budget} of {ticks} overlay appends exceeded {budget:?} (worst {worst:?})");
    assert_eq!(app.tool_runs.provisional().len() as u64, ticks * 2);
    assert!(render_text(&mut app, "main").await.contains(&format!("count={ticks}")));
    run_action(&mut app, "toolRunAbort").await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    close(&mut app);
}

//#region 🔌️IntegrationSeams
/// 🪟️ The per-window view state a renderer sends: both toy windows live, `window_id` targeted with its echoed cursor.
fn window_view(window_id: &str, cursor: Option<ToolRunTraceCursor>) -> ViewModel {
    let lane = &fixture()["traceLane"];
    let (world, other) = (text(&lane["windowId"]).to_string(), text(&lane["otherWindowId"]).to_string());
    ViewModel {
        window_id: Some(window_id.to_string()),
        window_instances: vec![ViewWindowInstance { id: world.clone(), window_kind_id: world }, ViewWindowInstance { id: other.clone(), window_kind_id: other }],
        tool_run_trace_cursor_by_window_id: cursor.map(|cursor| (window_id.to_string(), cursor)).into_iter().collect(),
        ..ViewModel::default()
    }
}

/// 🎬️ Renders the `world` body for `cursor` and returns its assembled scene plus the decoded trace delta.
async fn render_world(app: &mut ToyApp, cursor: Option<ToolRunTraceCursor>) -> (World3dScene, Option<ToolRunTraceDelta>) {
    let lane = &fixture()["traceLane"];
    let tree = app.render(text(&lane["bodyKey"]), None, &window_view(text(&lane["windowId"]), cursor)).await.unwrap_or_else(|fault| panic!("render world: {fault:?}"));
    let scene = artifact_app_laws::observe_and_retire_fixture_tree(tree, |root| {
        let surface = root.children.iter().find(|child| child.key.as_str() == text(&lane["surfaceId"])).expect("the world body carries its scene surface");
        artifact_app_laws::built_surface_scene::<World3dScene>(surface).expect("the scene surface assembles")
    });
    let delta = scene.tool_run_trace.as_deref().map(|lane| ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(lane).expect("the lane is base64url")).expect("the lane is one ToolRunTraceDelta"));
    (scene, delta)
}

fn cursor_after(delta: &ToolRunTraceDelta) -> ToolRunTraceCursor {
    ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next }
}

async fn abort_and_close(app: &mut ToyApp) {
    run_action(app, "toolRunAbort").await;
    pump_until(app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    close(app);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_scene_render_carries_the_trace_lane_and_honours_the_echoed_cursor() {
    let fixture = fixture();
    let expected = &fixture["traceLane"];
    let mut app = toy_app(number(&expected["units"])).await;
    let (idle, idle_delta) = render_world(&mut app, None).await;
    assert!(idle_delta.is_none() && idle.lanes.iter().all(|lane| lane.lane != text(&expected["laneName"])), "no run, no lane");
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "trace run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding()).await;
    let store_pages = app.tool_runs.trace().expect("trace").next_page();
    assert!(store_pages > number(&expected["resumePage"]) as u32, "the run logged several trace pages ({store_pages})");
    let (scene, delta) = render_world(&mut app, None).await;
    let delta = delta.expect("a scene rendered during a run carries the toolRunTrace lane");
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((delta.identity.id.run, delta.identity.generation, delta.clear, delta.next), (slot.run, slot.generation, true, store_pages), "a renderer without a cursor gets clear plus the whole log");
    assert_eq!(delta.pages.iter().map(|page| page.ops.len()).sum::<usize>() as u64, number(&expected["units"]), "every tested unit reaches the renderer");
    let reference = scene.lanes.iter().find(|lane| lane.lane == text(&expected["laneName"])).expect("the spine names the injected lane");
    let lane_text = scene.tool_run_trace.as_deref().expect("lane text");
    assert_eq!((reference.bytes as usize, reference.hash.as_str()), (lane_text.len(), semio_framework_ui_scene::scene_lane_hash(lane_text).as_str()), "the lane ref matches its carrier exactly as split_lanes would emit it");
    let (_, caught_up) = render_world(&mut app, Some(cursor_after(&delta))).await;
    assert!(caught_up.is_none(), "a caught-up cursor costs nothing");
    let resume = number(&expected["resumePage"]) as u32;
    let (_, resumed) = render_world(&mut app, Some(ToolRunTraceCursor { page: resume, ..cursor_after(&delta) })).await;
    let resumed = resumed.expect("a lagging cursor gets the pages after it");
    assert!(!resumed.clear && resumed.pages.first().map(|page| page.page) == Some(resume) && resumed.next == store_pages, "resume continues from the echoed page");
    let (_, rebound) = render_world(&mut app, Some(ToolRunTraceCursor { generation: slot.generation + 1, page: resume, ..cursor_after(&delta) })).await;
    let rebound = rebound.expect("a generation mismatch resends");
    assert!(rebound.clear && rebound.pages.first().map(|page| page.page) == Some(0), "a run/generation mismatch sends clear and resends from page 0");
    run_action(&mut app, "toolRunAbort").await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    assert_eq!(run_action(&mut app, "toolRunDismiss").await.get("toolRun").and_then(DslValue::as_str), Some("clearTrace"));
    assert!(app.tool_runs.slot().is_none(), "a dismissed run leaves no slot");
    let (_, cleared) = render_world(&mut app, Some(cursor_after(&delta))).await;
    let cleared = cleared.expect("a dismissed run clears a renderer still holding pages");
    assert!(cleared.clear && cleared.pages.is_empty() && cleared.next == 0);
    let (_, empty) = render_world(&mut app, Some(cursor_after(&cleared))).await;
    assert!(empty.is_none(), "a cleared renderer at page 0 gets nothing");
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_trace_backlog_keeps_the_scene_dirty_until_the_echoed_cursor_stalls() {
    let fixture = fixture();
    let expected = &fixture["traceLane"];
    let mut app = toy_app(number(&expected["units"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "trace run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding()).await;
    while app.take_typed_operation_ui_scope().is_some() {}
    app.flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let (window, body) = (text(&expected["windowId"]), text(&expected["bodyKey"]));
    let budget = number(&expected["pageByteBudget"]) as usize;
    let pages = app.tool_runs.trace().expect("trace").next_page();
    let mut cursor = None;
    for delivered in 1..=pages {
        let lane = app.tool_runs.trace_lane(window, body, cursor, budget).expect("one page per refresh under a one-byte budget");
        let delta = ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(&lane).expect("base64url")).expect("delta");
        assert_eq!((delta.pages.len(), delta.next), (1, delivered), "the byte budget still delivers one page per refresh");
        assert_eq!(app.tool_runs.is_ui_dirty(), delivered < pages, "a backlog keeps the window dirty; the last page does not");
        app.flush_tool_run_ui_dirty();
        while app.take_typed_operation_ui_scope().is_some() {}
        cursor = Some(cursor_after(&delta));
    }
    let lagging = Some(ToolRunTraceCursor { page: 0, ..cursor.expect("cursor") });
    let mut dirty_refreshes = 0;
    for _ in 0..number(&expected["stallRefreshes"]) * 2 {
        app.tool_runs.trace_lane(window, body, lagging, budget).expect("the backlog is resent");
        dirty_refreshes += u64::from(app.tool_runs.is_ui_dirty());
        app.flush_tool_run_ui_dirty();
        while app.take_typed_operation_ui_scope().is_some() {}
    }
    assert_eq!(dirty_refreshes, number(&expected["stallRefreshes"]), "a renderer that never echoes costs a bounded number of refreshes");
    abort_and_close(&mut app).await;
}

#[semio_framework_async_macros::async_test]
async fn tool_run_tick_dirty_scope_is_the_panel_plus_the_scene_windows_and_excludes_unrelated_windows() {
    let fixture = fixture();
    let expected = &fixture["dirtyScope"];
    let mut app = toy_app(number(&fixture["abort"]["target"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running)).await;
    render_world(&mut app, None).await;
    let other = text(&fixture["traceLane"]["otherWindowId"]);
    let tree = app.render(text(&expected["excludedWindowBody"]), None, &window_view(other, None)).await.expect("render the unrelated window");
    artifact_app_laws::project_and_retire_fixture_tree(tree).expect("project the unrelated window");
    app.flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let before = app.tool_runs.provisional().len();
    pump_until(&mut app, "a tick lands", |app| app.tool_runs.provisional().len() > before).await;
    let scope = app.take_typed_operation_ui_scope().expect("a tick owes a dirty scope");
    let strings = |value: &Value| value.as_array().expect("strings").iter().map(|item| text(item).to_string()).collect::<Vec<_>>();
    assert_eq!(scope, UiDirtyScope::Partial { window_bodies: strings(&expected["windowBodies"]), panel_bodies: strings(&expected["panelBodies"]), utilities: false, tools: false, engagements: false, measures: false, labels: false }, "never the whole UI per tick");
    let UiDirtyScope::Partial { window_bodies, .. } = &scope else { unreachable!() };
    assert!(!window_bodies.iter().any(|body| body == text(&expected["excludedWindowBody"])), "a window without a scene surface is not dirtied by a tick");
    let pause = run_action(&mut app, "toolRunPause").await;
    assert_eq!(pause.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"));
    abort_and_close(&mut app).await;
}

#[semio_framework_async_macros::async_test]
async fn tool_run_presence_and_the_ephemeral_snapshot_follow_the_run_with_completed_never_above_total() {
    let fixture = fixture();
    let expected = &fixture["presence"];
    let reconfigure = &fixture["reconfigure"];
    let mut app = toy_app(number(&fixture["abort"]["target"])).await;
    assert!(app.tool_run_presence().is_none() && app.ephemeral_snapshot().await.tool_run.is_none(), "no run, no presence summary");
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "running", |app| app.tool_runs.state() == Some(ToolRunState::Running) && app.tool_runs.progress().is_some_and(|progress| progress.completed > 0)).await;
    let running = app.tool_run_presence().expect("running presence");
    assert_eq!((running.tool_id.as_str(), running.state.wire_name()), (text(&fixture["toolId"]), text(&expected["running"])));
    assert!(running.total.is_none_or(|total| running.completed <= total));
    assert_eq!(app.ephemeral_snapshot().await.tool_run, Some(running), "the ephemeral snapshot carries the summary the heartbeat publishes");
    run_action(&mut app, "toolRunAbort").await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    assert_eq!(app.tool_run_presence().map(|presence| presence.state.wire_name()), Some(text(&expected["aborted"])));
    close(&mut app);
    let mut app = toy_app(number(&reconfigure["raisedTarget"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "raised run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    set_target(&mut app, number(&reconfigure["loweredTarget"])).await;
    let mut clamped = true;
    pump_until(&mut app, "lowered run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&reconfigure["loweredTarget"]) * number(&fixture["opsPerUnit"])).await;
    for _ in 0..4 {
        let presence = app.tool_run_presence().expect("presence");
        clamped &= presence.total.is_none_or(|total| presence.completed <= total);
        app.advance_typed_operation_publication().await.expect("turn");
    }
    let presence = app.tool_run_presence().expect("presence");
    assert!(clamped, "completed never exceeds total");
    assert_eq!((presence.state.wire_name(), presence.total), (text(&expected["complete"]), Some(number(&expected["loweredTotal"]))));
    abort_and_close(&mut app).await;
}

#[semio_framework_async_macros::async_test]
async fn tool_run_provisional_entities_ride_the_instance_records_the_producer_stamps() {
    let fixture = fixture();
    let expected = &fixture["provisional"];
    let mut app = toy_app(number(&expected["units"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding()).await;
    let (scene, _) = render_world(&mut app, None).await;
    let instances: Vec<Value> = serde_json::from_str(&scene.instances_json).expect("instances json");
    let flagged: Vec<&str> = instances.iter().filter(|instance| instance["provisional"] == Value::Bool(true)).map(|instance| text(&instance["id"])).collect();
    assert_eq!(flagged, expected["flaggedInstances"].as_array().expect("flagged").iter().map(text).collect::<Vec<_>>(), "ArtifactView::tool_run().provisional_entities drives the provisional instance flag");
    abort_and_close(&mut app).await;
}
//#endregion 🔌️IntegrationSeams
