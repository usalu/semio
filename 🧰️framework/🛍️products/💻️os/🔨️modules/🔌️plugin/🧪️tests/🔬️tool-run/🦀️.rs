//! ⏯️ Runtime laws of the tool run ledger and driver (`📋️tool-run-contract.md` §2.2, §2.7, §2.8, §3.3):
//! overlay visible while committed is untouched, zero-trace abort, one-edit finalize, rebase with a
//! revalidation conflict, stale no-ops, single step, reconfigure resume and the O(k) append budget — plus
//! the integration seams (§3.2 trace lane delivery and cursor protocol, §3.4 presence, minimal dirty scope,
//! the provisional instance flag).
//! Language-agnostic expectations: `🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json`.

use super::*;
use crate::test_app_mutation_fixture::{ChangeTestConfigSelection, SetCount, SetLabel, TestConfig, TestConfigMutation, TestMutation, TestSnapshot};
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunState, ToolRunStepRing, ToolRunTick, ToolRunTickWriter,
    ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceKind, ToolRunTraceSubject, ToolRunVerdict,
};
use crate::{RequestId, ViewWindowInstance};
use semio_framework_ui_scene::{Board2dScene, World3dScene};
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
        settings: ToolRunSettingsReads { config: fixture()["settingsReads"]["config"].as_array().expect("declared config reads").iter().map(|pointer| text(pointer).to_string()).collect(), ..ToolRunSettingsReads::default() },
        windows: Vec::new(),
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
        self.writer.payload(self.done.to_le_bytes().to_vec());
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

/// 🎯️ The toy run job honours the in-place retarget hooks when it is built through
/// `build_retargetable_tool_run_job`: a rebind restamps its writer, a reconfigure adopts the new target (a lower
/// one retracts on its next step) and wakes a job that already completed.
impl ToolRunRetargetableJob<TestConfig> for ToyRunJob {
    fn rebind(&mut self, identity: ToolRunIdentity) {
        self.writer.rebind(identity);
    }

    fn reconfigure(&mut self, identity: ToolRunIdentity, config: std::sync::Arc<TestConfig>) -> bool {
        self.writer.rebind(identity);
        self.target = toy_target(&config);
        true
    }
}

/// 🗜️ A run that places `initialCount`, checkpoints, then compacts: its first compaction tick retracts everything
/// and re-appends the first count, every later tick re-appends the next, each followed by a wait on its port, and a
/// checkpoint ends the compaction — the shape of a layout run whose compaction spans several driver turns.
struct ToyCompactJob {
    port: ToolRunJobPort,
    writer: ToolRunTickWriter,
    stage: usize,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for ToyCompactJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if self.port.is_waiting() {
            return semio_framework_job::StepOutcome::Yield;
        }
        let expected = &fixture()["compact"];
        let counts: Vec<u64> = expected["compactCounts"].as_array().expect("compact counts").iter().map(number).collect();
        let checkpoint = |cx: &mut semio_framework_job::StepContext<'_>| match cx.payload_from_bytes(semio_framework_job::JobPayloadStream::CheckpointState, &[0]) {
            Ok(state) => semio_framework_job::StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state, applied_progress: 0 }),
            Err(rejected) => {
                drop(rejected.into_source());
                semio_framework_job::StepOutcome::Yield
            }
        };
        self.stage += 1;
        match self.stage {
            1 => {
                self.writer.append_op(encoded(SetCount { value: number(&expected["initialCount"]) as i32 }.into())).expect("toy op fits");
                self.writer.append_entity(1);
                let tick = self.writer.finish().expect("the initial tick");
                ToyRunJob::emit(cx, tick)
            }
            2 => checkpoint(cx),
            stage if stage < 3 + counts.len() => {
                let index = stage - 3;
                if index == 0 {
                    self.writer.retract_to(0);
                }
                self.writer.append_op(encoded(SetCount { value: counts[index] as i32 }.into())).expect("toy op fits");
                let tick = self.writer.finish().expect("a compaction tick");
                self.port.wait();
                ToyRunJob::emit(cx, tick)
            }
            stage if stage == 3 + counts.len() => checkpoint(cx),
            _ => ToyRunJob::complete(),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if self.closing { semio_framework_job::InteractiveJobCloseStep::Complete } else { semio_framework_job::InteractiveJobCloseStep::Blocked }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

thread_local! {
    /// 🪟️ `(tool id, window id, window config present)` of every job request the toy app received on this thread.
    static TOY_REQUEST_WINDOWS: std::cell::RefCell<Vec<(String, Option<String>, bool)>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// 📮️ A job whose every unit is an external round trip: it hands the host one `DispatchAction` through its
/// [`ToolRunJobPort`], waits, and counts the hop answered when the port is woken — the shape of a run whose
/// algorithm units live in another component.
struct ToyWaitJob {
    port: ToolRunJobPort,
    writer: ToolRunTickWriter,
    hops: u64,
    dispatched: u64,
    answered: u64,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for ToyWaitJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if self.dispatched > self.answered {
            self.answered = self.dispatched;
            self.writer.upsert(self.answered, ToolRunVerdict::Success, 1, ToolRunTraceSubject::Entity { entity: self.answered });
            let tick = self.writer.finish().expect("an answered hop tick");
            return ToyRunJob::emit(cx, tick);
        }
        if self.dispatched == self.hops {
            return ToyRunJob::complete();
        }
        self.dispatched += 1;
        cx.consume_fuel(1);
        self.port.wait();
        self.port.dispatch(Effect::DispatchAction { req: RequestId(self.dispatched), action: text(&fixture()["port"]["hopAction"]).into(), args: None, delay_ms: 0 });
        semio_framework_job::StepOutcome::Yield
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if self.closing { semio_framework_job::InteractiveJobCloseStep::Complete } else { semio_framework_job::InteractiveJobCloseStep::Blocked }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

/// 🧸️ The toy run or revalidate job a request describes: it continues from the request's checkpoint.
fn toy_run_job(request: ToolRunJobRequest<'_, ToyRunApp>) -> ToyRunJob {
    let done = request.checkpoint.and_then(|bytes| bytes.try_into().ok()).map_or(0, u32::from_le_bytes);
    let provisional_counts = request.provisional.iter().filter_map(|op| match op {
        TestMutation::SetCount(set) => Some(set.value),
        TestMutation::SetLabel(_) | TestMutation::SetSlotChildren(_) => None,
    });
    ToyRunJob {
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
    }
}

/// 🪟️ The toy world window's config: the same one-field document as the app config.
struct ToyWorldWindowConfig;

impl WindowConfigOwner for ToyWorldWindowConfig {
    const WINDOW_KIND_ID: &'static str = "world";
    const SCHEMA: &'static str = "semio.testkit-tool-run.world-window-config/v1";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = TestConfig;
    type Mutation = TestConfigMutation;

    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
        crate::app::bounded_window_config_store_owners::<Self>()
    }

    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        crate::app::bounded_window_config_preparation_factory::<Self>()
    }

    fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        crate::app::bounded_window_config_store_disposer::<Self>()
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

    fn register_window_config_owners(registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<ToyWorldWindowConfig>()
    }

    fn build_retargetable_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<Box<dyn ToolRunRetargetableJob<TestConfig>>>, Fault> {
        if request.tool_id != text(&fixture()["retarget"]["toolId"]) {
            return Ok(None);
        }
        Ok(Some(Box::new(toy_run_job(request))))
    }

    fn build_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> {
        TOY_REQUEST_WINDOWS.with(|windows| windows.borrow_mut().push((request.tool_id.to_string(), request.window_id.map(str::to_string), request.window_config.is_some())));
        if request.tool_id == text(&fixture()["compact"]["toolId"]) {
            return Ok(Some(Box::new(ToyCompactJob { port: request.port, writer: ToolRunTickWriter::with_provisional_base(request.identity, request.provisional.len() as u32), stage: 0, closing: false })));
        }
        if request.tool_id == text(&fixture()["port"]["toolId"]) || request.tool_id == text(&fixture()["concurrentReadOnly"]["readOnlyToolId"]) {
            request.instance_owner.with_mut::<EmptyArtifactInstanceOperationOwner, _>(|_| Ok(()))?;
            return Ok(Some(Box::new(ToyWaitJob { port: request.port, writer: ToolRunTickWriter::new(request.identity), hops: number(&fixture()["port"]["hops"]), dispatched: 0, answered: 0, closing: false })));
        }
        Ok(Some(Box::new(toy_run_job(request))))
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
        if body_key == text(&fixture()["boardLane"]["bodyKey"]) {
            let scene = Board2dScene::base(format!("{{\"count\":{}}}", doc.snapshot.count), "{}".into(), true);
            let surface = scene_surface(text(&fixture()["boardLane"]["surfaceId"]), SurfaceKind::Board2d, &scene)?;
            return column().try_id("board").map_err(|_| PluginAssemblyError::new("toy", "board id"))?.try_child(surface).map_err(|_| PluginAssemblyError::new("toy", "board child"))?.try_build().map(built_to_component_tree).map_err(|_| PluginAssemblyError::new("toy", "board build"));
        }
        if body_key != text(&fixture()["traceLane"]["bodyKey"]) {
            let run = doc.tool_run().map_or_else(String::new, |run| format!(" completed={} steps={} payload={}", run.progress.completed, run.progress.steps.len(), run.payload.as_deref().and_then(|payload| payload.try_into().ok()).map_or(0, u32::from_le_bytes)));
            return built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={}{run}", doc.snapshot.count)));
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
        .await
        .window_kind(text(&fixture["boardLane"]["windowId"]), LocalizedLabel::data("Board"), text(&fixture["boardLane"]["bodyKey"]), SurfaceKind::Board2d, IconName::AppWindow)
        .await;
    let mut tools = Vec::new();
    for (id, rebase) in [
        (text(&fixture["toolId"]), ToolRunRebasePolicy::Revalidate),
        (text(&fixture["freezeToolId"]), ToolRunRebasePolicy::Freeze),
        (text(&fixture["port"]["toolId"]), ToolRunRebasePolicy::Restart),
        (text(&fixture["retarget"]["toolId"]), ToolRunRebasePolicy::Revalidate),
        (text(&fixture["compact"]["toolId"]), ToolRunRebasePolicy::Revalidate),
    ] {
        builder = builder.tool(ToolDefinition { run: Some(toy_definition(rebase)), ..ToolDefinition::new(id, LocalizedLabel::native("Toy fill", "Spielfüllung"), IconName::PaintBucket).await }).await;
        tools.push(ToolRef::new(id).await);
    }
    let window_settings = &fixture["windowSettingsReads"];
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition {
                settings: ToolRunSettingsReads { window_config: [(text(&window_settings["windowKindId"]).to_string(), vec![text(&window_settings["pointer"]).to_string()])].into_iter().collect(), ..ToolRunSettingsReads::default() },
                ..toy_definition(ToolRunRebasePolicy::Revalidate)
            }),
            ..ToolDefinition::new(text(&window_settings["toolId"]), LocalizedLabel::native("Toy fill reading window settings", "Spielfüllung mit Fenstereinstellungen"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(text(&window_settings["toolId"])).await);
    let read_only_wait = text(&fixture["concurrentReadOnly"]["readOnlyToolId"]);
    builder = builder
        .tool(ToolDefinition { run: Some(ToolRunDefinition { mutating: false, ..toy_definition(ToolRunRebasePolicy::Revalidate) }), ..ToolDefinition::new(read_only_wait, LocalizedLabel::native("Toy read-only hops", "Schreibgeschützte Spielsprünge"), IconName::PaintBucket).await })
        .await;
    tools.push(ToolRef::new(read_only_wait).await);
    let reader = &fixture["readerWindows"];
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition { windows: reader["windows"].as_array().expect("reader windows").iter().map(|window| text(window).to_string()).collect(), ..toy_definition(ToolRunRebasePolicy::Revalidate) }),
            ..ToolDefinition::new(text(&reader["toolId"]), LocalizedLabel::native("Toy fill with a reader window", "Spielfüllung mit Lesefenster"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(text(&reader["toolId"])).await);
    let undeclared = text(&fixture["settingsReads"]["undeclaredToolId"]);
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition { settings: ToolRunSettingsReads::default(), ..toy_definition(ToolRunRebasePolicy::Revalidate) }),
            ..ToolDefinition::new(undeclared, LocalizedLabel::native("Toy fill without settings reads", "Spielfüllung ohne Einstellungen"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(undeclared).await);
    // 📖️ The READ-ONLY twin of the toy run: the same job, declared as publishing nothing.
    let read_only = text(&fixture["readOnlyRun"]["toolId"]);
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition { mutating: false, ..toy_definition(ToolRunRebasePolicy::Revalidate) }),
            ..ToolDefinition::new(read_only, LocalizedLabel::native("Toy read-only fill", "Schreibgeschützte Spielfüllung"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(read_only).await);
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

/// 🆔️ A fixture panel id template scoped to run `run` (`{run}` substituted).
fn panel_id(template: &Value, run: u64) -> String {
    text(template).replace("{run}", &run.to_string())
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

/// 📮️ Drains what the app sent: `(mutations batches, genesis announcements)`; a member lane's batch is
/// a mutations batch of that document, and acks are not document traffic.
async fn drain(probe: &mut MemoryBackbone) -> (usize, usize) {
    probe.receive().await.expect("probe receive").into_iter().fold((0, 0), |(mutations, genesis), message| match message {
        BackboneMessage::Mutations { .. } | BackboneMessage::Member { .. } => (mutations + 1, genesis),
        BackboneMessage::Genesis { .. } => (mutations, genesis + 1),
        BackboneMessage::Ack { .. } => (mutations, genesis),
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
    let run = app.tool_runs.slot().expect("slot").run;
    let panel: Value = serde_json::from_str(&render_text(&mut app, text(&panel_expected["bodyKey"])).await).expect("panel projection parses");
    assert_eq!(panel["key"], panel_expected["rootId"]);
    let status = find_node(&panel, &panel_id(&panel_expected["status"]["id"], run)).expect("status line");
    assert_eq!(status["accessibility"]["live"], panel_expected["status"]["live"]);
    let bar = find_node(&panel, &panel_id(&panel_expected["progress"]["id"], run)).expect("progress bar");
    assert_eq!(bar["component"]["type"], panel_expected["progress"]["type"]);
    assert_eq!(bar["component"]["completed"].as_f64(), Some(number(&panel_expected["progress"]["completed"]) as f64));
    assert_eq!(bar["component"]["total"].as_f64(), Some(number(&panel_expected["progress"]["total"]) as f64));
    let finalize = find_node(&panel, &panel_id(&panel_expected["finalize"]["id"], run)).expect("finalize button");
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

/// ⚖️ LAW: a run that has not completed finalizes its PARTIAL result — the run job stops at its tick boundary and the one
/// grouped edit carries exactly the provisional ops computed until the finalize, never an op the job would have computed
/// after it; the run ends `finalized` and the document holds exactly that many units.
#[semio_framework_async_macros::async_test]
async fn tool_run_finalize_while_running_publishes_exactly_the_partial_result_computed_so_far() {
    let fixture = fixture();
    let expected = &fixture["partialFinalize"];
    let mut app = toy_app(number(&expected["target"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    pump_until(&mut app, "the partial result", |app| app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeFinalize"]) * ops_per_unit).await;
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running), "the run is still computing");
    let computed = app.tool_runs.provisional().len();
    let edits = app.store.envelope().vcs.edits.len();
    let output = run_action(&mut app, "toolRunFinalize").await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    pump_until(&mut app, "partial finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work()).await;
    assert_eq!((app.store.envelope().vcs.edits.len() - edits) as u64, number(&expected["editsAdded"]));
    let edit = app.store.envelope().vcs.edits.last().expect("finalized edit");
    assert_eq!(edit.forwards.len(), computed, "the edit carries exactly the ops computed until the finalize");
    assert!(edit.mutation_meta.iter().all(|meta| meta.group_id.as_deref() == Some(text(&expected["groupId"]))));
    assert_eq!(app.snapshot().expect("committed").count as u64, computed as u64 / ops_per_unit, "the document holds exactly the computed units");
    assert!(app.tool_runs.provisional().is_empty());
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
    assert_eq!(drain(&mut probe).await, (number(&expected["mutationsMessages"]) as usize, number(&expected["genesisMessages"]) as usize), "one Mutations batch, no genesis re-announcement");
    let committed = app.snapshot().expect("committed after finalize");
    assert_eq!((committed.count, committed.label.as_str()), (number(&expected["countAfterFinalize"]) as i32, text(&expected["labelAfterFinalize"])));
    assert!(app.tool_runs.provisional().is_empty());
    app.store.dispatch(ArtifactCommand::Undo).await.expect("undo the finalized run");
    let undone = app.snapshot().expect("committed after undo");
    assert_eq!((undone.count, undone.label.as_str()), (number(&expected["countAfterUndo"]) as i32, text(&expected["labelAfterUndo"])), "one undo removes the whole run");
    drop(probe);
    close(&mut app);
}

/// ⚖️ LAW: a large finalize never piles up returned document roots — after every driver turn the document Store
/// holds at most one returned-but-unreclaimed snapshot read, so the publication's memory is bounded by the staged
/// root and the base its current op reads instead of one whole document per op.
#[semio_framework_async_macros::async_test]
async fn tool_run_large_finalize_reclaims_each_folded_root_before_the_next_op() {
    let fixture = fixture();
    let law = &fixture["largeFinalize"];
    let units = number(&law["units"]);
    let maximum = number(&law["maximumReturnedReads"]) as usize;
    let mut app = toy_app(units).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    let edits = app.store.envelope().vcs.edits.len();
    let output = run_action(&mut app, "toolRunFinalize").await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    let (mut turns, mut peak) = (0usize, 0usize);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while !(app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work()) {
        assert!(std::time::Instant::now() < deadline, "the large finalize never settled; state {:?}", app.tool_runs.state());
        app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("large finalize driver turn faulted: {fault:?}"));
        turns += 1;
        peak = peak.max(app.store.returned_snapshot_read_count());
        assert!(peak <= maximum, "turn {turns}: {peak} returned document roots are still held (at most {maximum})");
    }
    assert_eq!(app.store.envelope().vcs.edits.len() - edits, 1, "still exactly one grouped edit");
    assert_eq!(app.store.envelope().vcs.edits.last().expect("finalized edit").forwards.len() as u64, units * number(&fixture["opsPerUnit"]), "every provisional op landed");
    eprintln!("[DEBUG] large finalize: {units} units in {turns} driver turns, peak {peak} returned document roots");
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
async fn tool_run_stale_generation_and_run_actions_are_silent_no_ops_and_an_illegal_action_publishes_nothing() {
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
    let illegal = run_action(&mut app, "toolRunResume").await;
    assert_eq!(illegal.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["illegal"]["code"])));
    assert_eq!(app.store.generation(), generation, "an illegal action publishes nothing");
    let busy = tool_run_action(&mut app, "toolRunStart", vec![("toolId".into(), DslValue::String(text(&fixture["toolId"]).into()))]).await;
    assert_eq!(busy.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["busy"]["code"])));
    let panel: Value = serde_json::from_str(&render_text(&mut app, FRAMEWORK_TOOL_RUN_BODY_KEY).await).expect("panel parses");
    let finalize = find_node(&panel, &panel_id(&fixture["panel"]["finalize"]["id"], slot.run)).expect("finalize button");
    assert_eq!((finalize["disabled"].as_bool().unwrap_or(false), finalize["accessibility"].get("description")), (false, None), "a running run offers finalize for the partial result it holds");
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
            payload: None,
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
#[semio_framework_async_macros::async_test]
async fn tool_run_job_port_hands_effects_to_the_host_and_a_waiting_job_keeps_nothing_runnable_until_woken() {
    let fixture = fixture();
    let expected = &fixture["port"];
    let mut app = toy_app(1).await;
    start(&mut app, text(&expected["toolId"])).await;
    for hop in 1..=number(&expected["hops"]) {
        pump_until(&mut app, "the hop is handed to the host", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting) && !app.tool_runs.port().is_some_and(ToolRunJobPort::has_effects) && app.tool_runs.trace().is_some_and(|trace| trace.len() as u64 == hop - 1)).await;
        let mut effects = Vec::new();
        while let Some(effect) = app.take_typed_operation_effect() {
            effects.push(effect);
        }
        assert!(matches!(effects.as_slice(), [Effect::DispatchAction { req: RequestId(req), action, .. }] if *req == hop && action == text(&expected["hopAction"])), "hop {hop}: exactly the job's effect reaches the host outbox: {effects:?}");
        for _ in 0..2 {
            while app.take_typed_operation_ui_scope().is_some() {}
            app.flush_tool_run_ui_dirty();
        }
        assert!(!app.tool_runs.has_pending_work() && !app.tool_run_has_pending_work(), "hop {hop}: a waiting job keeps its instance idle");
        app.advance_typed_operation_publication().await.expect("an idle turn");
        assert!(app.take_typed_operation_effect().is_none(), "hop {hop}: an unwoken job is never stepped again");
        app.tool_runs.port().expect("the run's port").wake();
        assert!(app.tool_runs.has_pending_work(), "hop {hop}: the wake makes the run runnable again");
    }
    pump_until(&mut app, "every hop answered", |app| app.tool_runs.state().map(ToolRunState::as_str) == Some(text(&expected["state"]))).await;
    assert_eq!(app.tool_runs.trace().expect("trace").len() as u64, number(&expected["traceRecords"]), "one trace record per answered hop");
    abort_and_close(&mut app).await;
}
//#endregion 🔌️IntegrationSeams

//#region 🔢️ActionArgCarriers
/// 🔢️ The wire value a fixture row names, in the carrier it names — the shapes a §2.5 action argument
/// really arrives in: a guest's `DslValue::uint`, the shell's `UiValue::Number` float, and the text a
/// targeted action's `runId` travels as.
fn carrier_value(row: &Value) -> DslValue {
    match text(&row["carrier"]) {
        "uint" => DslValue::uint(row["value"].as_u64().expect("an unsigned fixture carrier")),
        "float" => DslValue::float(row["value"].as_f64().expect("a float fixture carrier")),
        "string" => DslValue::String(text(&row["value"]).to_string()),
        other => panic!("the fixture named an unknown arg carrier {other}"),
    }
}

/// ⚖️ LAW: every `actionArgCarriers` row — a §2.5 action argument that is an exact non-negative integer
/// reads as that integer whichever carrier minted it, and nothing else names an identity at all.
///
/// 🪪️ `generation` crosses as `Number(Float(1.0))` from the shell's own action arguments and from a
/// guest's `DslValue::uint` alike; a reader that took only an exact `u64` carrier or a numeric string
/// answered `None`, and `(_, None)` is `ToolRunRejection::Stale`. Every `toolRunFinalize` of every app
/// was refused, so runs reached `Complete` and stayed there — and because a start against a live run is
/// `ToolRunRejection::Busy`, no tool could run a second time in a session. Measured on 6018 as a 3d
/// preview that never re-evaluated after an inspector edit
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
#[test]
fn every_action_arg_carrier_of_an_exact_integer_reads_the_same_identity() {
    for row in fixture()["actionArgCarriers"]["rows"].as_array().expect("carrier rows") {
        let id = text(&row["id"]);
        let args = DslValue::Object(vec![("generation".to_string(), carrier_value(row)), ("runId".to_string(), carrier_value(row))]);
        let read = crate::app::tool_run::tool_run_arg_u64(Some(&args), "generation");
        let expected = row["reads"].as_u64();
        println!("[STATS] actionArgCarriers {id}: carrier={} read={read:?}", text(&row["carrier"]));
        assert_eq!(read, expected, "{id}: generation");
        assert_eq!(crate::app::tool_run::tool_run_arg_u64(Some(&args), "runId"), expected, "{id}: runId — one reader, both identity arguments");
    }
    assert_eq!(crate::app::tool_run::tool_run_arg_u64(Some(&DslValue::Object(Vec::new())), "generation"), None, "an absent argument names no identity");
    assert_eq!(crate::app::tool_run::tool_run_arg_u64(None, "generation"), None, "an action with no arguments at all names no identity");
}

/// ⚖️ LAW: a `toolRunFinalize` whose `generation` arrives in the fixture's declared wire carrier really
/// finalizes the complete run it names — the end-to-end reading of the law above, through the same
/// `handle_action` door the shell dispatches through.
#[semio_framework_async_macros::async_test]
async fn a_finalize_in_the_wire_carrier_finalizes_the_run_it_names() {
    let fixture = fixture();
    let carrier = text(&fixture["actionArgCarriers"]["finalizeCarrier"]);
    let mut app = toy_app(number(&fixture["finalize"]["units"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "the run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    let slot = app.tool_runs.slot().expect("a complete run has a slot");
    let row = serde_json::json!({ "carrier": carrier, "value": f64::from(slot.generation) });
    let arguments = vec![("runId".to_string(), DslValue::String(slot.run.to_string())), ("generation".to_string(), carrier_value(&row))];
    let output = tool_run_action(&mut app, "toolRunFinalize", arguments).await;
    println!("[STATS] finalize carrier={carrier} output={output:?}");
    assert_eq!(output.get("rejected").and_then(DslValue::as_str), None, "a finalize naming the run's own generation is never stale, whatever carrier the number arrived in");
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"), "it begins the finalize");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Finalizing), "and the run leaves complete");
    abort_and_close(&mut app).await;
}
//#endregion 🔢️ActionArgCarriers

/// ⚖️ LAW: a READ-ONLY run's completion is its finalization — the driver turn that completes its job
/// leaves `complete` on its own, and the tool can be started again without any finalize action ever
/// being dispatched.
///
/// 🪪️ `complete` means "the job is done and the run awaits the finalize that publishes its provisional
/// edits". A `mutating: false` run authors none, so there is nothing to publish and nothing to review;
/// parking it in `complete` only holds the tool's single run slot, and a `toolRunStart` against a
/// non-terminal run is `toolRun.busy`. Generation3d's read-only `previewEval` therefore ran exactly ONCE
/// per session and its 3d preview stopped re-evaluating after the first evaluation
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
#[semio_framework_async_macros::async_test]
async fn a_read_only_run_finalizes_itself_and_frees_its_slot_for_the_next_start() {
    let fixture = fixture();
    let expected = &fixture["readOnlyRun"];
    let tool_id = text(&expected["toolId"]);
    let mut app = toy_app(number(&expected["units"])).await;
    start(&mut app, tool_id).await;
    pump_until(&mut app, "the read-only run leaves complete on its own", |app| app.tool_runs.state().is_some_and(|state| state != ToolRunState::Running && state != ToolRunState::Starting && state != ToolRunState::Complete)).await;
    let state = app.tool_runs.state().expect("a state");
    println!("[STATS] readOnlyRun state={} after its job completed, with no finalize action dispatched", state.as_str());
    assert_ne!(state, ToolRunState::Complete, "a read-only run never parks in complete waiting for a finalize nobody owes it");
    pump_until(&mut app, "the read-only run settles", |app| app.tool_runs.state().is_some_and(ToolRunState::is_terminal) && !app.tool_runs.has_pending_work()).await;
    // ▶️ And the slot is free: the next start is admitted, with no finalize action anywhere in this test.
    let output = tool_run_action(&mut app, "toolRunStart", vec![("toolId".into(), DslValue::String(tool_id.into()))]).await;
    assert_eq!(output.get("rejected").and_then(DslValue::as_str), None, "the finished read-only run never refuses the next start as busy");
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "and the next start really spawns a job");
    assert_eq!(expected["startsAgainWithoutAnyFinalizeAction"].as_bool(), Some(true));
    abort_and_close(&mut app).await;
}

//#region 🎯️RetargetAndSettings
/// 📡️ A remote peer sets the count to `count`; `probe` forwards its mutation batch and `app` ingests it.
async fn ingest_remote_count(app: &mut ToyApp, probe: &mut MemoryBackbone, channel: &str, count: u64) {
    let mut remote = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest()).await;
    let mut remote_probe = attach_probe(&mut remote, channel).await;
    remote.store.set_local_actor_id(Some("remote".into())).expect("remote actor");
    remote.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: count as i32 }.into()], description: None }).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.store.generation();
    app.tick_backbone().await.expect("ingest remote edit");
    assert_eq!(app.store.generation(), generation + 1, "the remote edit is ingested");
    drop(remote_probe);
    close(&mut remote);
}

/// ⚖️ LAW: a base change during a run hands every later tick the new identity — a plain job is rebuilt under it
/// from its checkpoint, a retargetable job is rebound in place — so the run keeps appending after the rebase
/// instead of producing ticks the ledger drops as stale.
#[semio_framework_async_macros::async_test]
async fn tool_run_base_change_rebinds_the_running_job_so_ticks_never_carry_a_stale_identity() {
    let fixture = fixture();
    let expected = &fixture["rebind"];
    for (tool_id, rebuilt) in [(text(&fixture["toolId"]), true), (text(&fixture["retarget"]["toolId"]), false)] {
        let mut app = toy_app(number(&expected["target"])).await;
        let mut probe = attach_probe(&mut app, &format!("tool-run-rebind-{tool_id}")).await;
        start(&mut app, tool_id).await;
        pump_until(&mut app, "first checkpointed unit", |app| app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeRebase"]) * number(&fixture["opsPerUnit"]) && app.tool_runs.checkpoint().is_some() && app.tool_runs.state() == Some(ToolRunState::Running)).await;
        ingest_remote_count(&mut app, &mut probe, &format!("tool-run-rebind-remote-{tool_id}"), number(&expected["remoteCount"])).await;
        pump_until(&mut app, "rebase observed", |app| app.tool_runs.slot().is_some_and(|slot| u64::from(slot.generation) == number(&expected["generationAfterRebase"]))).await;
        let identity = app.tool_runs.identity().expect("identity");
        let after_rebase = app.tool_runs.provisional().len();
        pump_until(&mut app, "the run appends after the rebase", |app| app.tool_runs.provisional().len() > after_rebase + 2 * number(&fixture["opsPerUnit"]) as usize).await;
        assert_eq!(app.tool_runs.identity(), Some(identity), "{tool_id}: appending never moved the identity again");
        let resumed_from = app.tool_runs.progress().expect("progress").counters.first().map_or(0, |counter| counter.value);
        println!("[STATS] rebind {tool_id}: provisional {after_rebase} -> {} resumedFrom={resumed_from}", app.tool_runs.provisional().len());
        assert_eq!(resumed_from > 0, rebuilt, "{tool_id}: a plain job is rebuilt from its checkpoint, a retargetable job keeps running");
        drop(probe);
        abort_and_close(&mut app).await;
    }
}

/// ⚖️ LAW: a §2.5 action that names no identity (a keyboard chord) targets the instance's live run at its
/// generation of the dispatch; an identity argument that is stale still no-ops.
#[semio_framework_async_macros::async_test]
async fn tool_run_chord_actions_without_an_identity_resolve_the_live_run_and_stale_identities_still_no_op() {
    let fixture = fixture();
    let expected = &fixture["chord"];
    let mut app = toy_app(number(&expected["target"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "running", |app| app.tool_runs.state() == Some(ToolRunState::Running)).await;
    let output = app.handle_action("toolRunPause", None, &toy_meta()).await.expect("chord pause").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"), "a chord without arguments pauses the live run");
    let window_only = vec![("windowId".to_string(), DslValue::String(text(&expected["windowArgument"]).into()))];
    assert_eq!(tool_run_action(&mut app, "toolRunResume", window_only).await.get("toolRun").and_then(DslValue::as_str), Some("schedule"), "arguments that name no identity resolve the live run too");
    let slot = app.tool_runs.slot().expect("slot");
    let stale = tool_run_action(&mut app, "toolRunPause", vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(number(&fixture["stale"]["wrongGeneration"]).to_string()))]).await;
    assert_eq!(stale.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])), "a stale identity still no-ops");
    let generation_only = tool_run_action(&mut app, "toolRunPause", vec![("generation".into(), DslValue::String(slot.generation.to_string()))]).await;
    assert_eq!(generation_only.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])), "a partial identity is checked as named, never completed from the live run");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running));
    let output = app.handle_action("toolRunAbort", None, &toy_meta()).await.expect("chord abort").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("closeJob"), "a chord abort aborts the live run");
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    let output = app.handle_action("toolRunDismiss", None, &toy_meta()).await.expect("chord dismiss").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("clearTrace"));
    assert!(app.tool_runs.slot().is_none());
    close(&mut app);
}

/// ⚖️ LAW: `settingsChanged` fires only when a value behind a declared settings pointer changes. Republishing the
/// same target and a window-config publication (a camera move) never reconfigure — the run keeps its generation
/// and its job — and a run that declares no settings reads is never reconfigured at all.
#[semio_framework_async_macros::async_test]
async fn tool_run_settings_changed_fires_only_for_the_declared_settings_reads() {
    let fixture = fixture();
    let expected = &fixture["settingsReads"];
    for tool_id in [text(&fixture["toolId"]), text(&expected["undeclaredToolId"])] {
        let declared = tool_id == text(&fixture["toolId"]);
        let mut app = toy_app(number(&expected["target"])).await;
        start(&mut app, tool_id).await;
        pump_until(&mut app, "running with provisional units", |app| app.tool_runs.state() == Some(ToolRunState::Running) && !app.tool_runs.provisional().is_empty()).await;
        let config_generation = app.config_store.generation();
        set_target(&mut app, number(&expected["target"])).await;
        assert!(app.config_store.generation() > config_generation, "{tool_id}: the republished target is a real config publication");
        app.tool_runs.note_window_config_published();
        for _ in 0..8 {
            app.advance_typed_operation_publication().await.expect("turn");
        }
        assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterUnrelatedPublications"]), "{tool_id}: unrelated publications never reconfigure");
        assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(0), "{tool_id}: the job was never rebuilt");
        set_target(&mut app, number(&expected["changedTarget"])).await;
        for _ in 0..8 {
            app.advance_typed_operation_publication().await.expect("turn");
        }
        let generation = u64::from(app.tool_runs.slot().expect("slot").generation);
        println!("[STATS] settingsReads {tool_id}: declared={declared} generation after the changed target={generation}");
        assert_eq!(generation, if declared { number(&expected["generationAfterChangedTarget"]) } else { 0 }, "{tool_id}: only a declared read reconfigures");
        abort_and_close(&mut app).await;
    }
}

/// ⚖️ LAW: a retargetable job stays resident while its run is complete and `reconfigure: resume` retargets it in
/// place — a raise continues and a lower retracts without any rebuild — and a base change rebinds it in place.
#[semio_framework_async_macros::async_test]
async fn tool_run_reconfigure_resume_retargets_a_retargetable_job_in_place() {
    let fixture = fixture();
    let expected = &fixture["retarget"];
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    let mut app = toy_app(number(&expected["initialTarget"])).await;
    let mut probe = attach_probe(&mut app, "tool-run-retarget").await;
    start(&mut app, text(&expected["toolId"])).await;
    pump_until(&mut app, "initial target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    assert!(!app.tool_runs.has_pending_work(), "a resident job of a complete run is no work");
    let run = app.tool_runs.slot().expect("slot").run;
    set_target(&mut app, number(&expected["raisedTarget"])).await;
    pump_until(&mut app, "raised target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((slot.run, u64::from(slot.generation)), (run, number(&expected["generationAfterRaise"])));
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the raise retargeted the resident job, no rebuild replayed a checkpoint");
    set_target(&mut app, number(&expected["loweredTarget"])).await;
    pump_until(&mut app, "lowered target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["loweredTarget"]) * ops_per_unit && !app.tool_runs.is_refolding()).await;
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterLower"]));
    assert!(render_text(&mut app, "main").await.contains(&format!("count={}", number(&expected["loweredTarget"]))), "the retracted tail leaves the overlay");
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])));
    ingest_remote_count(&mut app, &mut probe, "tool-run-retarget-remote", number(&expected["remoteCount"])).await;
    set_target(&mut app, number(&expected["raisedTarget"])).await;
    pump_until(&mut app, "rebased and raised", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit && !app.tool_runs.is_refolding()).await;
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the rebased run was rebound and retargeted in place");
    assert_eq!(app.snapshot().expect("committed").count, number(&expected["remoteCount"]) as i32, "reconfigure never commits");
    drop(probe);
    abort_and_close(&mut app).await;
}

/// ⚖️ LAW: a retract followed by re-appends spread over several ticks keeps the previous overlay rendered until the
/// job reaches its checkpoint; only then does the refolded overlay replace it.
#[semio_framework_async_macros::async_test]
async fn tool_run_retract_keeps_the_previous_overlay_until_the_job_reaches_its_checkpoint() {
    let fixture = fixture();
    let expected = &fixture["compact"];
    let mut app = toy_app(1).await;
    start(&mut app, text(&expected["toolId"])).await;
    let counts: Vec<u64> = expected["compactCounts"].as_array().expect("counts").iter().map(number).collect();
    for (index, _) in counts.iter().enumerate() {
        pump_until(&mut app, "a compaction page landed and was folded", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting) && app.tool_runs.provisional().len() == index + 1 && !app.tool_runs.has_pending_work()).await;
        let body = render_text(&mut app, "main").await;
        println!("[STATS] compact page {index}: rendered {body}");
        assert!(app.tool_runs.is_refolding(), "page {index}: the retract refold waits for its boundary");
        assert!(body.contains(text(&expected["renderedWhileCompacting"])), "page {index}: the previous overlay stays rendered: {body}");
        app.tool_runs.port().expect("the run's port").wake();
    }
    pump_until(&mut app, "the compaction checkpoint swaps the overlay", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding()).await;
    let body = render_text(&mut app, "main").await;
    assert!(body.contains(text(&expected["renderedAfterCheckpoint"])), "the refolded overlay replaces the previous one at the boundary: {body}");
    abort_and_close(&mut app).await;
}

/// ⚖️ LAW: the run's trace key allocator hands out keys above every key its ticks upserted and every key allocated
/// before, the request carries the entity marks, and the request names the window the run was started from.
#[semio_framework_async_macros::async_test]
async fn tool_run_requests_carry_trace_key_allocation_entity_marks_and_the_starting_window() {
    let fixture = fixture();
    let expected = &fixture["traceKeys"];
    let window = &fixture["startWindow"];
    let mut app = toy_app(number(&expected["units"])).await;
    TOY_REQUEST_WINDOWS.with(|windows| windows.borrow_mut().clear());
    let mut meta = toy_meta();
    meta.view_state = Some(window_view(text(&window["windowId"]), None));
    let output = app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(text(&fixture["toolId"]).into()))])), &meta).await.expect("start").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"));
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    assert_eq!(app.tool_runs.window(), Some((text(&window["windowId"]), text(&window["windowKindId"]))));
    let requests = TOY_REQUEST_WINDOWS.with(|windows| windows.borrow().clone());
    assert!(requests.iter().any(|(tool, window_id, window_config)| tool == text(&fixture["toolId"]) && window_id.as_deref() == Some(text(&window["windowId"])) && !window_config), "the job request names the starting window: {requests:?}");
    let keys = app.tool_runs.trace_keys().expect("trace keys");
    assert_eq!(keys.next(), number(&expected["nextAfterRun"]));
    let allocated = keys.allocate(number(&expected["allocate"]));
    assert_eq!([allocated.start, allocated.end], [number(&expected["allocated"][0]), number(&expected["allocated"][1])]);
    assert_eq!(keys.next(), number(&expected["nextAfterAllocation"]));
    let marks = app.tool_runs.entity_marks();
    assert_eq!(marks.iter().map(|(_, entity)| *entity).collect::<Vec<_>>(), (1..=number(&expected["units"])).collect::<Vec<_>>());
    assert!(marks.iter().all(|(end, _)| *end as usize <= app.tool_runs.provisional().len()));
    abort_and_close(&mut app).await;
}

/// ⚖️ LAW: a board-2d scene surface carries the `toolRunTrace` lane exactly like a world-3d one — the spine names
/// the lane with its byte length and hash, and the carrier holds the whole trace delta of the run.
#[semio_framework_async_macros::async_test]
async fn tool_run_board_scene_render_carries_the_trace_lane() {
    let fixture = fixture();
    let expected = &fixture["boardLane"];
    let mut app = toy_app(number(&expected["units"])).await;
    start(&mut app, text(&fixture["toolId"])).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding()).await;
    let mut view = window_view(text(&expected["windowId"]), None);
    view.window_instances.push(ViewWindowInstance { id: text(&expected["windowId"]).to_string(), window_kind_id: text(&expected["windowId"]).to_string() });
    let tree = app.render(text(&expected["bodyKey"]), None, &view).await.unwrap_or_else(|fault| panic!("render board: {fault:?}"));
    let scene = artifact_app_laws::observe_and_retire_fixture_tree(tree, |root| {
        let surface = root.children.iter().find(|child| child.key.as_str() == text(&expected["surfaceId"])).expect("the board body carries its scene surface");
        artifact_app_laws::built_surface_scene::<Board2dScene>(surface).expect("the board scene surface assembles")
    });
    let lane = scene.tool_run_trace.as_deref().expect("a board rendered during a run carries the toolRunTrace lane");
    let reference = scene.lanes.iter().find(|reference| reference.lane == text(&expected["laneName"])).expect("the spine names the injected lane");
    assert_eq!((reference.bytes as usize, reference.hash.as_str()), (lane.len(), semio_framework_ui_scene::scene_lane_hash(lane).as_str()));
    let delta = ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(lane).expect("base64url")).expect("one ToolRunTraceDelta");
    assert!(delta.clear, "a renderer without a cursor gets clear plus the whole log");
    assert_eq!(delta.pages.iter().map(|page| page.ops.len()).sum::<usize>() as u64, number(&expected["units"]));
    abort_and_close(&mut app).await;
}

/// 🪟️ Publishes `selected` into the world window config of `window_id` through the production emit path.
async fn publish_window_selection(app: &mut ToyApp, window_id: &str, selected: u64) {
    let fixture = fixture();
    let mut view = window_view(window_id, None);
    view.window_instances.push(ViewWindowInstance { id: text(&fixture["windowSettingsReads"]["otherWindowId"]).to_string(), window_kind_id: text(&fixture["windowSettingsReads"]["windowKindId"]).to_string() });
    let mut meta = toy_meta();
    meta.view_state = Some(view);
    let mutation = WindowConfigMutation::of::<ToyWorldWindowConfig>(window_id, ChangeTestConfigSelection { selected: Some(selected.to_string()) }.into());
    app.dispatch_emit("setWorldSelection", Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { window_config_mutations: vec![mutation], ..Default::default() }, &meta).await.unwrap_or_else(|fault| panic!("window config publication: {fault:?}"));
}

/// ⚖️ LAW: a run started from a window reads the window-config fields it declares for that window's kind from the
/// starting window only — its job request carries that window's config snapshot, a publication on another window of the
/// same kind or of an unchanged value never reconfigures, and a changed value on the starting window does.
#[semio_framework_async_macros::async_test]
async fn tool_run_window_settings_reads_follow_the_starting_window_only() {
    let fixture = fixture();
    let expected = &fixture["windowSettingsReads"];
    let (start_window, other_window) = (text(&expected["startWindowId"]), text(&expected["otherWindowId"]));
    let mut app = toy_app(number(&expected["target"])).await;
    publish_window_selection(&mut app, start_window, 1).await;
    TOY_REQUEST_WINDOWS.with(|windows| windows.borrow_mut().clear());
    let mut meta = toy_meta();
    let mut view = window_view(start_window, None);
    view.window_instances.push(ViewWindowInstance { id: other_window.to_string(), window_kind_id: text(&expected["windowKindId"]).to_string() });
    meta.view_state = Some(view);
    let output = app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(text(&expected["toolId"]).into()))])), &meta).await.expect("start").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"));
    pump_until(&mut app, "running with provisional units", |app| app.tool_runs.state() == Some(ToolRunState::Running) && !app.tool_runs.provisional().is_empty()).await;
    let requests = TOY_REQUEST_WINDOWS.with(|windows| windows.borrow().clone());
    assert!(requests.iter().any(|(tool, window_id, window_config)| tool == text(&expected["toolId"]) && window_id.as_deref() == Some(start_window) && *window_config), "the job request carries the starting window's config snapshot: {requests:?}");
    publish_window_selection(&mut app, other_window, 7).await;
    publish_window_selection(&mut app, start_window, 1).await;
    for _ in 0..8 {
        app.advance_typed_operation_publication().await.expect("turn");
    }
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterOtherWindow"]), "another window and an unchanged value never reconfigure");
    publish_window_selection(&mut app, start_window, 2).await;
    for _ in 0..8 {
        app.advance_typed_operation_publication().await.expect("turn");
    }
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterStartWindow"]), "a changed value on the starting window reconfigures");
    abort_and_close(&mut app).await;
}

/// ⚖️ LAW: a tick dirties the bodies of the window kinds the run declares it renders in, next to the panel and the scene
/// windows, and those bodies read the run's progress, step ring and latest tick payload through `ArtifactView::tool_run()`.
#[semio_framework_async_macros::async_test]
async fn tool_run_reader_windows_refresh_every_tick_and_read_progress_steps_and_payload() {
    let fixture = fixture();
    let expected = &fixture["readerWindows"];
    let mut app = toy_app(number(&expected["target"])).await;
    start(&mut app, text(&expected["toolId"])).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running)).await;
    render_world(&mut app, None).await;
    app.flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let before = app.tool_runs.provisional().len();
    pump_until(&mut app, "a tick lands", |app| app.tool_runs.provisional().len() > before).await;
    let scope = app.take_typed_operation_ui_scope().expect("a tick owes a dirty scope");
    let strings = |value: &Value| value.as_array().expect("strings").iter().map(|item| text(item).to_string()).collect::<Vec<_>>();
    assert_eq!(scope, UiDirtyScope::Partial { window_bodies: strings(&expected["windowBodies"]), panel_bodies: strings(&expected["panelBodies"]), utilities: false, tools: false, engagements: false, measures: false, labels: false }, "the reader window refreshes with every tick, nothing unrelated does");
    run_action(&mut app, "toolRunPause").await;
    pump_until(&mut app, "paused and settled", |app| app.tool_runs.state() == Some(ToolRunState::Paused) && !app.tool_runs.has_pending_work()).await;
    let view = app.tool_runs.view().expect("run view");
    let payload = view.payload.as_deref().and_then(|payload| payload.try_into().ok()).map(u32::from_le_bytes).expect("the latest tick payload");
    assert_eq!(u64::from(payload), view.progress.completed, "the payload is the latest tick's: the toy writes its completed units");
    assert!(view.progress.total.is_some_and(|total| total == number(&expected["target"])));
    let body = render_text(&mut app, "main").await;
    println!("[STATS] reader body {body}");
    assert!(body.contains(&format!("completed={} steps={} payload={payload}", view.progress.completed, view.progress.steps.len())), "the reader body renders the run state it reads: {body}");
    abort_and_close(&mut app).await;
}

/// ▶️ Starts `tool_id` from window `window_id` and answers the dispatch output.
async fn start_in_window(app: &mut ToyApp, tool_id: &str, window_id: &str) -> DslValue {
    let mut meta = toy_meta();
    meta.view_state = Some(window_view(window_id, None));
    app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(tool_id.into()))])), &meta).await.unwrap_or_else(|fault| panic!("start {tool_id}: {fault:?}")).output
}

/// ⚖️ LAW: read-only runs run concurrently, one per (tool, window), each with its own run id, trace and panel group; a
/// second mutating start while a mutating run is non-terminal is `toolRun.busy`; actions address a run by its `runId`;
/// presence follows the mutating run.
#[semio_framework_async_macros::async_test]
async fn read_only_runs_in_two_windows_run_concurrently_and_a_second_mutating_start_is_busy() {
    let fixture = fixture();
    let expected = &fixture["concurrentReadOnly"];
    let read_only = text(&expected["readOnlyToolId"]);
    let windows: Vec<&str> = expected["windows"].as_array().expect("windows").iter().map(text).collect();
    let mut app = toy_app(number(&expected["target"])).await;
    for window in &windows {
        assert_eq!(start_in_window(&mut app, read_only, window).await.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "{window}: a read-only run starts beside the other window's");
    }
    assert_eq!(start_in_window(&mut app, read_only, windows[0]).await.get("rejected").and_then(DslValue::as_str), Some(text(&expected["busy"])), "one non-terminal read-only run per tool and window");
    pump_until(&mut app, "both read-only runs are running and waiting on their hop", |app| {
        let views = app.tool_runs.views();
        views.len() == 2 && views.iter().all(|view| view.state == ToolRunState::Running)
    })
    .await;
    let views = app.tool_runs.views();
    assert_ne!(views[0].identity.id.run, views[1].identity.id.run, "each run has its own id");
    for (window, view) in windows.iter().zip(&views) {
        assert_eq!(app.tool_runs.view_for(Some(window)).map(|found| found.identity.id.run), Some(view.identity.id.run), "{window} renders its own run");
    }
    assert_eq!(start_in_window(&mut app, text(&expected["mutatingToolId"]), windows[0]).await.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "a mutating run starts beside read-only runs");
    assert_eq!(start_in_window(&mut app, text(&expected["secondMutatingToolId"]), windows[1]).await.get("rejected").and_then(DslValue::as_str), Some(text(&expected["busy"])), "a second mutating start is busy");
    assert_eq!(app.tool_run_presence().map(|presence| presence.tool_id), Some(text(&expected["mutatingToolId"]).to_string()), "presence follows the mutating run");
    let panel: Value = serde_json::from_str(&render_text(&mut app, FRAMEWORK_TOOL_RUN_BODY_KEY).await).expect("panel parses");
    for view in app.tool_runs.views() {
        assert!(find_node(&panel, &panel_id(&fixture["panel"]["groupId"], view.identity.id.run)).is_some(), "run {} has its own panel group", view.identity.id.run);
        let delta = app.tool_runs.trace_delta(Some(ToolRunTraceCursor { run: view.identity.id.run, generation: view.identity.generation + 1, page: 0 }), usize::MAX).map(|lane| ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(&lane).expect("base64url")).expect("delta"));
        assert_eq!(delta.map(|delta| delta.identity.id.run), Some(view.identity.id.run), "run {} has its own trace", view.identity.id.run);
    }
    let first = views[0].clone();
    let aborted = tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(first.identity.id.run.to_string())), ("generation".into(), DslValue::uint(u64::from(first.identity.generation)))]).await;
    assert_eq!(aborted.get("toolRun").and_then(DslValue::as_str), Some("closeJob"), "an action addresses its run by runId");
    pump_until(&mut app, "the addressed run aborts alone", |app| app.tool_runs.views().iter().any(|view| view.identity.id.run == first.identity.id.run && view.state == ToolRunState::Aborted)).await;
    let states: Vec<(u64, ToolRunState)> = app.tool_runs.views().iter().map(|view| (view.identity.id.run, view.state)).collect();
    println!("[STATS] concurrent runs {states:?}");
    assert!(states.iter().filter(|(run, _)| *run != first.identity.id.run).all(|(_, state)| !state.is_terminal()), "the other runs keep running: {states:?}");
    for view in app.tool_runs.views().into_iter().filter(|view| !view.state.is_terminal()) {
        tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(view.identity.id.run.to_string())), ("generation".into(), DslValue::uint(u64::from(view.identity.generation)))]).await;
    }
    pump_until(&mut app, "every run settles", |app| app.tool_runs.views().iter().all(|view| view.state.is_terminal()) && !app.tool_runs.has_pending_work()).await;
    close(&mut app);
}

/// 🧾️ A `BuiltNode` as the host wire JSON every renderer's `BuiltNode` reads — every field spelled out.
fn built_node_wire(node: &BuiltNode) -> Value {
    serde_json::json!({
        "key": node.key.as_str(),
        "component": serde_json::to_value(&node.component).expect("component"),
        "layout": serde_json::to_value(&node.layout).expect("layout"),
        "style": serde_json::to_value(node.style).expect("style"),
        "activity": serde_json::to_value(node.activity).expect("activity"),
        "disabled": node.disabled,
        "accessibility": serde_json::to_value(&node.accessibility).expect("accessibility"),
        "bindings": serde_json::to_value(&node.bindings).expect("bindings"),
        "menu": serde_json::to_value(&node.menu).expect("menu"),
        "children": node.children.iter().map(built_node_wire).collect::<Vec<_>>(),
    })
}

/// 🪧️ The language-neutral panel every shell target mounts: the framework ToolRun panel `BuiltNode` of a running run
/// (`🔌️plugin/🧫️fixtures/⏯️tool-run/🪧️panel-running.json`). React (`🛠️ShellHelpers` panel law) and wgpu (`🐚️Shell`
/// panel law) render it and dispatch its buttons from this exact document.
const TOOL_RUN_PANEL_RUNNING: &str = include_str!("../../🧫️fixtures/⏯️tool-run/🪧️panel-running.json");

/// ⚖️ LAW: the ToolRun panel of a running run is exactly the committed shell fixture — one run group with a polite status,
/// a progressbar, real buttons addressing the run by id (pause and abort enabled, step disabled, finalize disabled with
/// its description) and an empty step log and trace list. `SEMIO_TOOL_RUN_PANEL_OUT` rewrites the fixture.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_of_a_running_run_is_the_shell_fixture() {
    let fixture = fixture();
    let mut app = toy_app(1).await;
    start(&mut app, text(&fixture["port"]["toolId"])).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.state() == Some(ToolRunState::Running) && app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting)).await;
    while app.take_typed_operation_effect().is_some() {}
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &ViewModel::default()).await.expect("render the panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    if let Some(path) = std::env::var_os("SEMIO_TOOL_RUN_PANEL_OUT") {
        std::fs::write(path, format!("{}\n", serde_json::to_string_pretty(&panel).expect("pretty panel"))).expect("write the panel fixture");
    }
    assert_eq!(panel, serde_json::from_str::<Value>(TOOL_RUN_PANEL_RUNNING).expect("the panel fixture parses"), "the running panel is the shell fixture");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app).await;
}

/// 🔁️ Every sibling key list under `node` names each key once — the admission law a retained UI surface enforces.
fn assert_unique_sibling_keys(node: &Value) {
    let children = node["children"].as_array().map(Vec::as_slice).unwrap_or_default();
    let mut keys: Vec<&str> = children.iter().filter_map(|child| child["key"].as_str()).collect();
    keys.sort_unstable();
    let before = keys.len();
    keys.dedup();
    assert_eq!(keys.len(), before, "duplicate sibling keys under {}", node["key"]);
    children.iter().for_each(assert_unique_sibling_keys);
}

/// ⚖️ LAW: a trace key the run re-upserts with a new verdict (testing → success/danger, the shape of every
/// collision-tested candidate) is one attempt row, moved to the newest position — never a duplicate sibling that
/// makes the panel unadmittable.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_lists_a_re_upserted_trace_key_once_at_its_newest_position() {
    let fixture = fixture();
    let expected = &fixture["verdictRevisions"];
    let mut app = toy_app(1).await;
    start(&mut app, text(&expected["toolId"])).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting)).await;
    while app.take_typed_operation_effect().is_some() {}
    let identity = app.tool_runs.identity().expect("identity");
    let mut writer = ToolRunTickWriter::new(identity);
    for upsert in expected["upserts"].as_array().expect("upserts") {
        let key = number(&upsert[0]);
        writer.upsert(key, ToolRunVerdict::parse(text(&upsert[1])).expect("verdict"), 1, ToolRunTraceSubject::Entity { entity: key });
        app.tool_runs.apply_tick(writer.finish().expect("an upsert tick")).expect("the upsert tick applies");
    }
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &ViewModel::default()).await.expect("render the panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert_unique_sibling_keys(&panel);
    let scope = semio_framework_tool_run::tool_run_panel_group_id(identity.id.run);
    let rows: Vec<&str> = find_node(&panel, &format!("{scope}.trace")).expect("the attempts tree")["children"].as_array().expect("rows").iter().filter_map(|row| row["key"].as_str()).collect();
    let expected_rows: Vec<String> = expected["traceRows"].as_array().expect("trace rows").iter().map(|key| format!("{scope}.trace.{}", number(key))).collect();
    assert_eq!(rows, expected_rows, "one attempt row per trace key, newest first");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app).await;
}

/// ⚖️ LAW: while the active tool declares a run and this instance holds none of it, the panel offers a ready group
/// whose enabled Start dispatches `toolRunStart` with the tool id; once the run exists its own group replaces it.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_offers_start_for_the_active_run_tool_until_its_run_exists() {
    let fixture = fixture();
    let expected = &fixture["readyGroup"];
    let mut app = toy_app(1).await;
    let view = ViewModel { active_tool_id: Some(text(&expected["toolId"]).to_string()), ..ViewModel::default() };
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &view).await.expect("render the idle panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert_unique_sibling_keys(&panel);
    let group = find_node(&panel, text(&expected["groupId"])).expect("the ready group");
    assert_eq!(group["children"][0]["component"]["value"], expected["status"], "the ready status");
    let start_button = find_node(group, text(&expected["startButtonId"])).expect("the ready Start button");
    assert_eq!(start_button["component"]["label"], expected["startLabel"]);
    assert_eq!(start_button["disabled"], false, "Start is enabled with no run");
    assert_eq!(start_button["bindings"][0]["action"]["name"], "toolRunStart");
    assert_eq!(start_button["bindings"][0]["args"]["toolId"], expected["toolId"], "Start names the active tool");
    start(&mut app, text(&expected["toolId"])).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting)).await;
    while app.take_typed_operation_effect().is_some() {}
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &view).await.expect("render the running panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert!(find_node(&panel, text(&expected["groupId"])).is_none(), "the run's group replaces the ready group");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app).await;
}

/// ⚖️ LAW: a TERMINAL run arms nothing. Once a run is finalized or aborted the ledger owes the driver no work
/// and hands the host no effect, however many turns the host takes.
///
/// 🪪️ Every wake a run hands the host is a REQUEST the host answers by dispatching an action back into the
/// guest, so a request a run that is OVER keeps re-making is an endless action storm on a shell nobody is
/// touching. Generation3d's finalized read-only `previewEval` re-armed its pace wake once per presentation
/// retry and filled the console with `toolRunPace` on a quiet, converged editor — seq 107…116 within seconds,
/// 2 800 dropped console lines (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-09-14 23:25). The latch is read
/// once per ANSWER, never once per REFRESH, and a terminal run answers nothing.
#[semio_framework_async_macros::async_test]
async fn a_terminal_run_arms_no_further_work_however_many_turns_the_host_takes() {
    let fixture = fixture();
    let expected = &fixture["terminalQuiet"];
    let turns = number(&expected["turns"]);
    for row in expected["rows"].as_array().expect("the terminalQuiet rows") {
        let id = text(&row["id"]);
        let mut app = toy_app(number(&row["units"])).await;
        start(&mut app, text(&row["toolId"])).await;
        if text(&row["terminalBy"]) == "abort" {
            pump_until(&mut app, "the run leaves starting", |app| app.tool_runs.state().is_some_and(|state| state != ToolRunState::Starting)).await;
            run_action(&mut app, "toolRunAbort").await;
        }
        pump_until(&mut app, "the run reaches a terminal state", |app| app.tool_runs.state().is_some_and(ToolRunState::is_terminal) && !app.tool_runs.has_pending_work()).await;
        while app.take_typed_operation_effect().is_some() {}
        let mut armed: Vec<String> = Vec::new();
        let mut pending_turns = 0u64;
        for _ in 0..turns {
            if app.tool_runs.has_pending_work() {
                pending_turns += 1;
            }
            app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{id}: a quiet turn faulted: {fault:?}"));
            while let Some(effect) = app.take_typed_operation_effect() {
                armed.push(format!("{effect:?}"));
            }
        }
        let state = app.tool_runs.state().expect("a terminal state");
        println!("[STATS] terminalQuiet {id}: state={} turns={turns} pendingWorkTurns={pending_turns} armed={}", state.as_str(), armed.len());
        assert!(state.is_terminal(), "{id}: the run stays terminal");
        assert_eq!(pending_turns == 0, expected_flag(&row["expected"]["hasPendingWork"]) == false, "{id}: a terminal run is no driver work over {turns} turns");
        assert_eq!(armed.len() as u64, number(&row["expected"]["effects"]), "{id}: a terminal run arms nothing, got {armed:?}");
        close(&mut app);
    }
}

fn expected_flag(value: &Value) -> bool {
    value.as_bool().expect("a boolean expectation")
}

//#endregion 🎯️RetargetAndSettings

//#region 🧹️RetainedConfigCloseCliff
/// 🧹️ A retained config larger than one `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` page must still reach
/// terminal-empty AFTER a body has rendered against it.
///
/// 🏁️ Measured 2026-09-16 (ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS): with a body rendered,
/// a config over ~one page livelocked `close_step(1, 4_096)` at `Pending { 0, 0 }` for 15 M turns,
/// while the same config closed with no render. The render backfills the command log, whose LABEL
/// carries the applied config op text; `close_retained_fields_step` priced that whole log entry
/// against one turn grant and pushed it back unreleased every turn. The cliff capped EVERY app's
/// retained config at one page regardless of its declared maximum (cad declares 65 536 B), so the
/// `sourcing.module` / `cad.computer` / `process.machines` host packs could never be retained.
#[semio_framework_async_macros::async_test]
async fn a_retained_config_over_one_envelope_page_closes_after_a_render() {
    for (bytes, rendered) in [(2_909usize, true), (3_706, true), (4_360, true), (16_384, true), (65_536, true), (3_820, false)] {
        let mut app = toy_app(1).await;
        app.config_store
            .dispatch(ArtifactCommand::Apply { mutations: vec![ChangeTestConfigSelection { selected: Some("c".repeat(bytes)) }.into()], description: None })
            .await
            .expect("a retained config past one envelope page applies");
        if rendered {
            let _ = render_text(&mut app, "main").await;
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut turns = 0u64;
        let mut productive = 0u64;
        let mut last = String::new();
        while std::time::Instant::now() < deadline {
            if app.close_terminal_is_empty() {
                break;
            }
            turns += 1;
            match app.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("close step") {
                PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, "a close page stays inside its exact grant");
                    if released_items != 0 || released_bytes != 0 {
                        productive = turns;
                    }
                    last = format!("Pending {{ {released_items}, {released_bytes} }}");
                }
                PluginCloseStep::Complete => break,
                step => last = format!("{step:?}"),
            }
        }
        println!("[STATS] retainedConfigClose bytes={bytes} rendered={rendered} turns={turns} lastProductiveTurn={productive} last={last}");
        assert!(app.close_terminal_is_empty(), "a {bytes} B retained config (rendered={rendered}) must reach terminal-empty; last step {last} after {turns} turns");
        assert_eq!(productive, turns, "every close turn released something; the pump never spends a turn on zero progress");
    }
}
//#endregion 🧹️RetainedConfigCloseCliff
