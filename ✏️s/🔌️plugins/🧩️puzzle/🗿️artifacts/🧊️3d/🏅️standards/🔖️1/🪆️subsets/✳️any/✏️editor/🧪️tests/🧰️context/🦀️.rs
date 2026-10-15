//! 🧰️ Shared fixture of the puzzle3d editor integration targets — the raw app, its runtime budgets, render and dispatch drivers, and the heap witness.

use super::*;
use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, MAINTENANCE_STAGES, PluginApp, PluginLifecycleStep, VcsArtifactApp, ViewModel, ViewWindowInstance, artifact_app_laws};

pub type Puzzle3dRawApp = VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>;

/// 📏️ The byte grant the OS runtime's cooperative-maintenance clock hands one live-cleanup unit
/// (`RuntimeLiveCleanupJob::step`'s `maintenance_step(1, RUNTIME_CLOSE_BYTES_PER_STEP)`), restated
/// here so a step-budget law measures the exact unit the host drives rather than a wider one.
pub const RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP: usize = 4_096;

/// 📏️ What the cooperative-maintenance units of one scene_snapshot released, per fixed maintenance stage, against the grant
/// each unit was handed. A unit is bounded by its grant (`maximum_items`, `maximum_bytes`), not by the wall clock: the
/// count is the same on an idle and a saturated machine, where the same unit that costs 19us alone was seen costing
/// 14571us. Fixed capacity: the round robin has exactly [`MAINTENANCE_STAGES`] stages.
#[derive(Default)]
pub struct MaintenanceStageGrants {
    units: [u32; MAINTENANCE_STAGES as usize],
    worst_items: [usize; MAINTENANCE_STAGES as usize],
    worst_bytes: [usize; MAINTENANCE_STAGES as usize],
    first_overrun: Option<(u8, usize, usize, usize, usize)>,
}

impl MaintenanceStageGrants {
    /// 🚨️ The first unit that released more than its grant: `(stage, items, bytes, granted items, granted bytes)`.
    pub fn first_overrun(&self) -> Option<(u8, usize, usize, usize, usize)> {
        self.first_overrun
    }

    /// 🔢️ Units every stage ran.
    pub fn units(&self) -> u32 {
        self.units.iter().sum()
    }

    /// 📊️ Per-stage `stage=worstItems/worstBytes/units` breakdown of every unit audited so far.
    pub fn report(&self) -> String {
        let mut report = String::new();
        for stage in 0..MAINTENANCE_STAGES as usize {
            if self.units[stage] > 0 {
                report.push_str(&format!("{stage}={}i/{}B/{}u ", self.worst_items[stage], self.worst_bytes[stage], self.units[stage]));
            }
        }
        report
    }

    fn record(&mut self, stage: u8, grant: semio_framework_value::retained_clone::RetainedCloneGrant, step: &Result<PluginLifecycleStep, Fault>) {
        let index = stage as usize;
        self.units[index] += 1;
        let Ok(PluginLifecycleStep::Progress(progress)) = step else { return };
        let bytes = progress.copied_bytes.max(progress.released_bytes);
        self.worst_items[index] = self.worst_items[index].max(progress.copied_items);
        self.worst_bytes[index] = self.worst_bytes[index].max(bytes);
        if self.first_overrun.is_none() && !progress.fits(grant) {
            self.first_overrun = Some((stage, progress.copied_items, bytes, grant.maximum_items, grant.maximum_copy_bytes));
        }
    }
}

/// 🧪️ The one puzzle3d scene_snapshot app. Wraps the raw wrapper so every scene_snapshot drains its stores on the
/// way out: a registry-backed `VcsArtifactApp` installs framework-owned `ArtifactStoreCursorDisposer`
/// members whose own `Drop` asserts terminal-empty ownership (`🏪️store/🦀️.rs`), so a bare drop panics
/// inside a destructor. `Drop` here therefore NEVER panics and never asserts: a second panic while a
/// failing assertion is already unwinding is a non-unwinding abort that kills the whole test binary
/// and hides the assertion that actually failed. A drain that cannot reach the witness leaks the raw
/// app instead ([`std::mem::forget`]) — leaking a scene_snapshot inside a test process costs nothing, and the
/// close contract itself is stated exactly once, explicitly, by [`close_witness`].
/// 📦️ The raw app is boxed on purpose: `Puzzle3dRawApp` is 42 KiB by value, and a law's `#[async_test]` future — which
/// `#[async_test]` pins ON THE STACK — moves the scene_snapshot through every frame of the harness. The default test thread
/// has 2 MiB, and the app-scene_snapshot laws in this crate already spend most of it inside the framework's own construction
/// chain.
pub struct Puzzle3dApp {
    raw: Option<Box<Puzzle3dRawApp>>,
    /// 🪪️ The instance identity this scene_snapshot bound ([`next_fixture_instance_id`]) and therefore the receiver
    /// `take_typed_operation_result_page` answers for and the key of its session registry slot.
    pub instance_id: u32,
    /// 🏛️ The HOST's own session state, owned here because the host owns it in production: the live
    /// window roster, the mode-wide `active_tool_id`, and the per-window `active_utility_by_window_id`
    /// (`📓️2026-09-09-peer-config-runtime-split.md` §1(d)). `setActiveTool`/`setActiveUtility` are
    /// dispatched by the framework as an empty `Emit` (`🔌️plugin/🦀️.rs` `dispatch_action`), so the app
    /// writes nothing and the ONLY thing that can carry an activation to the next call is this record.
    /// Minting a fresh `ViewModel` per call — what this scene_snapshot used to do — made every activation
    /// unobservable one call later, which is a state no real host can be in.
    view: ViewModel,
    /// 📏️ Every cooperative-maintenance unit this scene_snapshot drove, audited against its grant per fixed stage.
    pub maintenance: Box<MaintenanceStageGrants>,
}

/// 🧰️ `resolveUtilityActivation` (`🛠️ShellHelpers/🟦️.tsx`), verbatim: an empty request — or
/// re-requesting what is already active — deactivates; anything else activates.
fn resolve_activation(current: Option<&str>, requested: &str) -> Option<String> {
    (!requested.is_empty() && current != Some(requested)).then(|| requested.to_string())
}

impl std::ops::Deref for Puzzle3dApp {
    type Target = Puzzle3dRawApp;
    fn deref(&self) -> &Self::Target {
        self.raw.as_deref().expect("scene_snapshot app was already consumed by close_witness")
    }
}

impl std::ops::DerefMut for Puzzle3dApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.raw.as_deref_mut().expect("scene_snapshot app was already consumed by close_witness")
    }
}

impl Puzzle3dApp {
    /// 🪟️ Registers one window instance in the scene_snapshot's live roster, exactly as the shell's own
    /// `sessionWindowInstances` grows the moment a pane is opened or split. Idempotent.
    fn ensure_window(&mut self, window_id: &str) {
        if !self.view.window_instances.iter().any(|instance| instance.id == window_id) {
            self.view.window_instances.push(ViewWindowInstance { id: window_id.into(), window_kind_id: main::WINDOW_KIND_ID.into() });
        }
    }

    /// 🎯️ The exact per-call projection a host sends: the whole live session — roster, mode-wide tool,
    /// per-window utility map — addressed at ONE concrete window instance through the framework's own
    /// `ViewModel::for_window_instance`, which is what stamps `active_utility_id` for that pane.
    /// 🗣️ Names the host's live label axes for every later render/measures call on this scene_snapshot. The
    /// app resolves its label set from `ViewModel.locale`/`.terminology` and fails closed on an axis it
    /// never authored (`puzzle3d_labels`), so a test that asserts German or reuse text has to say so —
    /// there is no default language to fall back to.
    pub fn set_label_axes(&mut self, locale: semio_framework_ui_locale::Locale, terminology: semio_framework_ui_locale::Terminology) {
        self.view.locale = locale;
        self.view.terminology = terminology;
    }

    /// 📼️ What a renderer echoes after it applied a tool run trace delta for `window_id`.
    pub fn echo_tool_run_trace_cursor(&mut self, window_id: &str, cursor: semio_framework_tool_run::ToolRunTraceCursor) {
        self.view.tool_run_trace_cursor_by_window_id.insert(window_id.to_string(), cursor);
    }

    pub fn window_view(&self, window_id: &str) -> ViewModel {
        let mut view = self.view.clone();
        if !view.window_instances.iter().any(|instance| instance.id == window_id) {
            view.window_instances.push(ViewWindowInstance { id: window_id.into(), window_kind_id: main::WINDOW_KIND_ID.into() });
        }
        view.for_window_instance(window_id).expect("puzzle3d test window roster")
    }

    /// 🛠️🧰️ The shell's own activation branches (`🏛️ShellHost/🟦️.tsx`'s `SET_ACTIVE_TOOL_ACTION_ID`
    /// and `SET_ACTIVE_UTILITY_ACTION_ID`), applied to this scene_snapshot's session BEFORE the verb is
    /// forwarded — the same order the shell uses, so the plugin call already sees the new activation.
    /// A tool and a window utility are mutually exclusive interaction owners: activating a tool clears
    /// every window's utility, activating a utility clears the tool.
    fn activate(&mut self, action: &str, args: Option<&Value>, window_id: &str) {
        let requested = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).unwrap_or_default();
        if action == SET_ACTIVE_TOOL_ACTION_ID {
            let next = resolve_activation(self.view.active_tool_id.as_deref(), requested("toolId"));
            self.view.active_tool_id = next.clone();
            if next.is_some() {
                self.view.active_utility_by_window_id = Default::default();
            }
            return;
        }
        let window = args.and_then(|value| value.get("windowId")).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or(window_id).to_string();
        match resolve_activation(self.view.active_utility_by_window_id.get(&window).map(String::as_str), requested("utilityId")) {
            Some(utility) => {
                self.view.active_utility_by_window_id.insert(window, utility);
                self.view.active_tool_id = None;
            }
            None => {
                self.view.active_utility_by_window_id.remove(&window);
            }
        }
    }

    /// 📏️ One cooperative-maintenance unit shaped exactly like the OS runtime's live-cleanup clock drives it, audited
    /// against its grant and attributed to the fixed stage that ran it. The stage is read BEFORE the call:
    /// `maintenance_step`'s idle early return leaves the round-robin cursor untouched, so reading it afterwards names a
    /// stale stage.
    pub fn measure_maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginLifecycleStep, Fault> {
        let stage = self.next_maintenance_stage();
        let raw = self.raw.as_deref_mut().expect("scene_snapshot app was already consumed by close_witness");
        let demand = PluginApp::maintenance_retirement_demands(raw, maximum_bytes).map_err(|error| Fault::from(error.into_message()))?;
        let grant = semio_s_plugin_puzzle_jobs::testing::funded_turn(demand, maximum_items);
        let step = PluginApp::maintenance_step(raw, grant);
        self.maintenance.record(stage, grant, &step);
        step
    }

    /// 🪪️ A local-actor `ActionMeta` addressed at THIS scene_snapshot's bound instance. The framework refuses a typed
    /// command whose meta names any other instance (`interactive-job.live-instance`), and every scene_snapshot binds its
    /// own process-unique id ([`next_fixture_instance_id`]).
    pub fn meta(&self, actor: &str) -> ActionMeta {
        ActionMeta { instance_id: self.instance_id, ..artifact_app_laws::meta(actor) }
    }
}

/// 🧹️ Drives `app` through the real `PluginApp` close state machine, one item and one envelope page
/// per turn exactly as a host actor tick does, and returns the terminal-empty witness or the fault
/// that stopped it. Bounded only as a runaway guard: this app carries five stores plus its retained
/// tool operations, which outgrows `context::close_registered_fixture_app`'s own 64-turn cap.
fn drain_close(app: &mut Puzzle3dRawApp) -> Result<bool, Fault> {
    let mut blocked_on = None;
    for _ in 0..1_048_576 {
        if app.close_terminal_is_empty() {
            return Ok(true);
        }
        let demand = PluginApp::close_retirement_demands(app, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| Fault::from(error.into_message()))?;
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        match PluginApp::close_step(app, grant)? {
            semio_framework_plugin::PluginLifecycleStep::Complete(_) => break,
            semio_framework_plugin::PluginLifecycleStep::Blocked { reason } | semio_framework_plugin::PluginLifecycleStep::AwaitingInput { reason } => blocked_on = Some(reason),
            semio_framework_plugin::PluginLifecycleStep::Progress(_) => {}
        }
    }
    if app.close_terminal_is_empty() {
        return Ok(true);
    }
    // 🧷️ A close that ends non-terminal because an owner is BLOCKED is not "the machine reported
    // complete too early" — it is a live lease the caller still holds (a captured window transient
    // snapshot is the usual one: `capture` hands out a read of the partition's own store). Naming that
    // reason is the difference between a diagnosable failure and a bare `false` after a million spins.
    match blocked_on {
        Some(reason) => Err(Fault::from(format!("close drained non-terminal while blocked on {reason}"))),
        None => Ok(false),
    }
}

/// 🧹️ Consumes one scene_snapshot app through its real close state machine and reports the outcome, so the
/// close contract is asserted by a test rather than by a destructor. `Err` carries the framework's own
/// fault; `Ok(false)` means the machine reported `Complete` without reaching terminal-empty ownership.
pub fn close_witness(mut app: Puzzle3dApp) -> Result<bool, Fault> {
    let mut raw = app.raw.take().expect("scene_snapshot app was already consumed by close_witness");
    match drain_close(&mut raw) {
        Ok(true) => Ok(true),
        other => {
            std::mem::forget(raw);
            other
        }
    }
}

impl Drop for Puzzle3dApp {
    fn drop(&mut self) {
        puzzle3d_session_release(self.instance_id);
        let Some(mut raw) = self.raw.take() else {
            return;
        };
        if !matches!(drain_close(&mut raw), Ok(true)) {
            std::mem::forget(raw);
        }
    }
}

/// 🌉️ `new_app_with_registry`'s `manifest: fn() -> App` shape predates the `AppDefinition`-returning
/// `create_puzzle3d_app()` convention (contract §2.4 / SDK gap 3) — this tiny local wrapper bridges
/// the two, mirroring `📓️w2-cad-report.md`'s recipe step 7.
pub fn puzzle3d_manifest_for_tests() -> App {
    App { definition: create_puzzle3d_app(), examples: Vec::new() }
}

semio_framework_plugin::history_edit_acceptance_law!("puzzle", Puzzle3dPlayApp, puzzle3d_manifest_for_tests, "../..");

/// 🧰️ The registry-backed, instance-bound scene_snapshot app — the ONLY constructor this plugin can use.
/// puzzle3d declares `bounded_first_step_tool_proofs!`, so `VcsArtifactApp::with_registry_on_bus`'s
/// `registry.tool_job_registration::<A>(…)` join needs the manifest's migrated generated
/// declarations; the registry-less `artifact_app_laws::new_app` carries an `AppActionRegistry::default()` and
/// fails closed with `interactive-job.catalog-authority` before any dispatch is attempted. The bound
/// instance id is what `advance_typed_operation_publication`/`maintenance_step` and the local
/// interaction query authority key their live-runtime bookkeeping on, exactly as the host binds it.
/// 🧱 Builds the action registry on its own frame and drops the definition before `with_registry`.
#[inline(never)]
fn puzzle3d_action_registry() -> semio_framework_plugin::AppActionRegistry {
    let definition = create_puzzle3d_app();
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&definition);
    drop(definition);
    registry
}

pub async fn app() -> Puzzle3dApp {
    let registry = puzzle3d_action_registry();
    let mut app = semio_framework_plugin::with_authoring_identity!(|identity| VcsArtifactApp::with_registry(EditorApp::<Puzzle3dPlayApp>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut identity).await);
    let instance_id = next_fixture_instance_id();
    app.bind_instance_id(instance_id).await;
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: main::WINDOW_KIND_ID.into(), window_kind_id: main::WINDOW_KIND_ID.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    Puzzle3dApp { raw: Some(Box::new(app)), instance_id, view, maintenance: Box::new(MaintenanceStageGrants::default()) }
}

/// 🖌️ Host-session utility activation without minting a typed operation or running `settle`.
#[inline(never)]
pub fn activate_window_utility(app: &mut Puzzle3dApp, utility_id: &str) {
    app.ensure_window(main::WINDOW_KIND_ID);
    app.activate(semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utility_id })), main::WINDOW_KIND_ID);
}

/// 🪪️ A process-unique instance identity per scene_snapshot app. The puzzle 3d session registry is process-global
/// and keyed by instance id, so fixtures sharing one id would adopt, evict and re-key each other's caches
/// (and the standing mesh re-upload set inside them) across concurrently running laws.
pub fn next_fixture_instance_id() -> u32 {
    static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// 🔁️ Runs the host's continuation loop to quiescence and hands back the effects/events/UI scope the
/// host would have forwarded to the shell. A registry-backed `VcsArtifactApp` does NOT apply a retained
/// tool job inline — `dispatch_typed` mints a `ToolOperationSpec` whose worker, store publication,
/// result page and outboxes are advanced only by later continuation turns, which is why the
/// registry-less scene_snapshot used to see mutations land synchronously and this one does not. One turn is
/// exactly the host's own: `maintenance_step` (worker + retirement stages), one
/// `advance_typed_operation_publication` unit, the result page for the bound receiver plus its
/// mandatory ACK, then one effect, one event, one COMPLETION witness and one UI scope. The ACK is what
/// actually retires the operation — without it the app stays pending forever. Bounded only as a
/// runaway guard.
///
/// 📬️ The completion witness is drained in exactly the host's own position — `advance_typed_operation_output`
/// (`🧰️framework/…/🔌️plugin/🦀️.rs`) takes it between the event and the UI scope and forwards it as
/// `AppFrame::OperationCompleted`. It is not optional bookkeeping: every terminal typed operation
/// pushes one into a 64-slot outbox that ONLY this take drains, and `has_pending_typed_operations`
/// counts it, so a scene_snapshot that skips it can never observe quiescence again after its first
/// completed operation.
pub async fn settle(app: &mut Puzzle3dApp) -> Puzzle3dSettled {
    settle_with_items(app, 1).await
}

/// ⏱️ [`settle`] with the host's OWN per-turn item grant instead of the one-item-per-turn paging every
/// other law reads. A turn is a host↔guest round trip, and the host grants 256 items / 64 KiB per
/// turn (`🔌️PluginRuntime/🟦️.tsx`'s `uiGrant`), so a law about per-command LATENCY must count turns
/// at that granularity — at one item per turn the count is the item count, which says nothing about
/// how many round trips a browser actually pays. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44.
pub const SETTLE_HOST_TURN_ITEMS: usize = 256;

pub async fn settle_with_items(app: &mut Puzzle3dApp, maximum_items: usize) -> Puzzle3dSettled {
    let mut settled = Puzzle3dSettled::default();
    let census_before = semio_framework_plugin::app::typed_operation_unit_census();
    for _ in 0..1_048_576 {
        if !app.has_pending_typed_operations() {
            settled.census = semio_framework_plugin::app::typed_operation_unit_census() - census_before;
            return settled;
        }
        host_turn(app, maximum_items, &mut settled).await;
    }
    panic!("puzzle3d app never quiesced: pending typed operations outlived the settle budget");
}

/// 🔁️ ONE host continuation turn — the unit [`settle_with_items`] repeats — folding what it forwards
/// into `settled`.
pub async fn host_turn(app: &mut Puzzle3dApp, maximum_items: usize, settled: &mut Puzzle3dSettled) {
    settled.turns += 1;
    app.measure_maintenance_step(maximum_items, SETTLE_TURN_BYTES).expect("maintenance step drives the mounted operation's worker and retirement stages");
    app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.expect("advance one typed operation publication unit");
    let instance_id = app.instance_id;
    if let Some(page) = app.take_typed_operation_result_page(instance_id) {
        assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "retained operation faulted: {}", String::from_utf8_lossy(page.bytes()));
        // ⬇️ The Download lane's page is the ONLY place the segmented handle is named, and the ACK
        // below is what admits the chunks into the app's `segmented_downloads` authority — so the
        // handle is captured here, exactly where the host's own `consumeTypedOperationEffects`
        // captures it, or a law can never drain what it just published.
        if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Download {
            settled.downloads.push(Puzzle3dDownload::from_page(page.token.operation, page.bytes()));
        }
        assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge one presented result page"), "the app's own presented result page must accept its exact token");
    }
    settled.effects.extend(app.take_typed_operation_effect());
    settled.events.extend(app.take_typed_operation_event());
    if let Some(completion) = app.take_typed_operation_completion().await.expect("take one typed operation completion witness") {
        settled.completions.push(Puzzle3dCompletion { operation: completion.operation, ui_scope: completion.ui_scope, history_patch: completion.history_patch });
    }
    settled.scope = app.take_typed_operation_ui_scope().or(settled.scope.take());
    acknowledge_local_interaction_pages(app);
}

/// 📬️ Everything the HOST forwards to the shell once a dispatch has run to quiescence — the same
/// four channels `plugin_runtime` carries, so a law reads exactly what a client would.
///
/// 🧲️ [`Self::completions`] is the channel the browser reads as `AppFrame::OperationCompleted` and the
/// shell applies through `subscribeOperationCompletions` → `applyHostEffects`. It used to be drained
/// and DROPPED here, which is why a command that answers before it runs — every latest-wins verb
/// enters that channel and its accepted invocation returns
/// `{operationId, generation}` immediately — had no observable outcome in-process at all, and a
/// refusal on that lane could not be asserted (`📓️2026-09-09-wave-L-…md` §5.4).
#[derive(Debug, Default)]
pub struct Puzzle3dSettled {
    /// ⏱️ How many host continuation turns this settle spent. Ticket 26/09/02/PUZZLE-3D-END-TO-END
    /// wave B44: the observable a latency law reads, because per-command latency in the browser is
    /// turns × per-turn cost and a turn count that grows with the document is the shape that burns a
    /// 30-second interaction budget.
    pub turns: usize,
    /// 📊️ What those turns spent their publication units on, split by ladder. Ticket
    /// 26/09/02/PUZZLE-3D-END-TO-END wave B54: a turn now drives a bounded RUN of units, so `turns`
    /// alone can no longer say where a mutation's cost went and `turns` vs `census.units` is the
    /// wave's whole before/after ratio.
    pub census: semio_framework_plugin::app::TypedOperationUnitCensus,
    pub effects: Vec<Effect>,
    pub events: Vec<semio_framework_plugin::AppEvent>,
    pub scope: Option<UiDirtyScope>,
    pub completions: Vec<Puzzle3dCompletion>,
    pub downloads: Vec<Puzzle3dDownload>,
}

/// ⬇️ One segmented download handle a typed operation published — the metadata the host turns into
/// `download-media-export` with a `semio-segmented-handle-v1:` marker, plus the operation id the chunks
/// are drained by ([`drain_segmented_download`]).
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dDownload {
    pub operation: u64,
    pub filename: String,
    pub mime_type: String,
    pub encoding: Option<String>,
    pub bytes: usize,
}

impl Puzzle3dDownload {
    /// 📦️ The Download lane's wire shape is a flat 4-element JSON array
    /// (`DownloadResultPayload`'s hand-written `ToValue`), read here exactly as the renderer reads it.
    fn from_page(operation: u64, page: &[u8]) -> Self {
        let text = String::from_utf8_lossy(page).into_owned();
        let value: Value = parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| panic!("download result page must be JSON: {text}"));
        let row = value.as_array().unwrap_or_else(|| panic!("download result page must be a 4-element array: {text}"));
        Self {
            operation,
            filename: row.first().and_then(Value::as_str).unwrap_or_default().to_string(),
            mime_type: row.get(1).and_then(Value::as_str).unwrap_or_default().to_string(),
            encoding: row.get(2).and_then(Value::as_str).map(str::to_string),
            bytes: row.get(3).and_then(Value::as_u64).unwrap_or_default() as usize,
        }
    }
}

/// ⬇️ Drains one published segmented download the way the host's `drainSegmentedMediaExport` does: one
/// bounded chunk per await until the producer answers `None`, concatenated in order. The whole point of
/// the lane is that the payload never crosses as one contiguous block, so a law that asserts the
/// REASSEMBLED bytes has to drain it exactly like this.
pub async fn drain_segmented_download(app: &mut Puzzle3dApp, download: &Puzzle3dDownload) -> Vec<u8> {
    let mut assembled = Vec::with_capacity(download.bytes);
    let mut chunks = 0usize;
    while let Some(chunk) = app.take_segmented_download_chunk(download.operation).await.expect("take one segmented download chunk") {
        assert!(!chunk.is_empty(), "a segmented download chunk is never empty");
        assert!(chunk.len() <= semio_framework_plugin::app::ArtifactOutputChunks::CHUNK_BYTES, "chunk {} exceeds the wire's own page cap", chunks);
        assembled.extend_from_slice(&chunk);
        chunks += 1;
        assert!(chunks <= download.bytes / semio_framework_plugin::app::ArtifactOutputChunks::CHUNK_BYTES + 2, "segmented drain never terminated");
    }
    assembled
}

/// 📬️ One `AppFrame::OperationCompleted` witness, projected to what a law can assert on.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dCompletion {
    pub operation: u64,
    pub ui_scope: UiDirtyScope,
    pub history_patch: Option<semio_framework::kernel::HistoryPatch>,
}

/// 📏️ One continuation turn's byte grant. Deliberately one fixed envelope page, not a huge number: the
/// point of the loop is that every unit is bounded exactly as a host actor tick bounds it.
const SETTLE_TURN_BYTES: usize = 16_384;

/// 🕹️ Plays the CLIENT half of the local interaction query protocol: drains the replies the app
/// published this turn and acknowledges every page, which is what returns the document snapshot-read
/// leases the query captured — the same Started → Page → acknowledge → Closed round trip
/// `local_interaction_query_return_does_not_fault_the_next_maintenance_step` drives by hand.
fn acknowledge_local_interaction_pages(app: &mut Puzzle3dApp) {
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            assert!(app.acknowledge_local_interaction_query(&token), "the app's own local-interaction page must accept its exact token");
        }
    }
}

async fn settle_into_reporting_with_items(app: &mut Puzzle3dApp, result: Result<InvocationResult, Fault>, maximum_items: usize) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
    let settled = settle_with_items(app, maximum_items).await;
    let folded = result.map(|mut result| {
        result.requested_effects.extend(settled.effects.iter().cloned());
        result.events.extend(settled.events.iter().cloned());
        // 🧲️ A latest-wins command answers BEFORE it runs, so its own terminal `UiDirtyScope` only
        // exists on the completion lane; folding the last completion's scope in is what makes
        // `InvocationResult::ui_scope` mean the same thing for a coalesced verb as for a plain one.
        if let Some(scope) = settled.scope.clone().or_else(|| settled.completions.last().map(|completion| completion.ui_scope.clone())) {
            result.ui_scope = scope;
        }
        result
    });
    (folded, settled)
}

/// 🧾️ How many command-log rows one settle published on the completion lane — the `historyUpserts` a
/// browser client counts on its `AppFrame::OperationCompleted` frames.
pub fn history_rows(settled: &Puzzle3dSettled) -> usize {
    settled.completions.iter().filter_map(|completion| completion.history_patch.as_ref()).map(|patch| patch.upserts.len()).sum()
}

/// 🧾️ `(seq, action_id, kind, applied)` of every history row one settle recorded — what a row-count
/// assertion must print, or "2 vs 1" names neither the extra row nor the verb that wrote it.
pub fn history_row_labels(settled: &Puzzle3dSettled) -> Vec<String> {
    settled
        .completions
        .iter()
        .filter_map(|completion| completion.history_patch.as_ref())
        .flat_map(|patch| patch.upserts.iter())
        .map(|entry| format!("#{} {} kind={} applied={} ops={}", entry.seq, entry.action_id, entry.kind, entry.applied, entry.op_lines.len()))
        .collect()
}

/// 📂️ How many whole-document loads (`Effect::LoadDocument`) one settle published — the only shape a natural-file
/// import takes, since a load is a genesis of the document, never a history row.
pub fn loads_in(settled: &Puzzle3dSettled) -> usize {
    settled.effects.iter().filter(|effect| matches!(effect, Effect::LoadDocument { .. })).count()
}

/// 📂️ The projection of the one document a settle's `Effect::LoadDocument` carries, decoded from its pack bytes.
pub fn imported_projection(settled: &Puzzle3dSettled) -> Value {
    let pack = settled.effects.iter().find_map(|effect| match effect {
        Effect::LoadDocument { pack, .. } => Some(pack.clone()),
        _ => None,
    });
    let document = <Puzzle3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack.expect("an import publishes one whole-document load")).expect("the load carries a puzzle 3D document");
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&document))
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle3dCommand` from the same
/// `(action, args, window_id)` triple every pre-B1 test already passed.
///
/// 🛰️ Wave B7: a framework-reserved verb is a TWO-half gesture since `dispatch_framework_reserved_action`
/// began answering every non-clipboard route with `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }`.
/// `handle_action` only ADMITS it; the document changes when the host drives that Isolated job to a
/// terminal step and hands the bytes back through `complete_reserved_spawned_job`. A scene_snapshot that stopped
/// at the admission observed `undo`/`interactionSelect`/`interactionHover` succeeding while the store and
/// the interaction snapshot never moved — the exact silent no-op shape wave B5 reported. [`settle_reserved`]
/// plays that host half, and is a no-op for the clipboard routes that still commit inline.
pub async fn dispatch(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    dispatch_reporting(app, action, args, window_id).await.0
}

/// 🧾️ [`dispatch`] plus the settle census, for laws that must read the completion lane — above all
/// [`history_rows`], the command-log delta a MUTATING verb publishes on
/// `AppFrame::OperationCompleted` rather than on its accepted invocation. See
/// [`settle_into_reporting`] for why the two channels stay separate.
pub async fn dispatch_reporting(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
    dispatch_reporting_with_items(app, action, args, window_id, 1).await
}

/// 📥️ [`dispatch_reporting`] through the SDK's own action dispatch (`handle_action`) instead of the typed channel —
/// the lane a shell's picked-file chunks take, where the framework's import staging admits them first.
pub async fn dispatch_action_reporting(app: &mut Puzzle3dApp, action: &str, args: &semio_framework_value::DslValue) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
    let window_id = main::WINDOW_KIND_ID;
    app.ensure_window(window_id);
    let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..app.meta("local") };
    let answered = app.handle_action(action, Some(args), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    settle_into_reporting_with_items(app, answered, 1).await
}

/// ⏱️ [`dispatch_reporting`] settling at the HOST's own per-turn item grant, so `Puzzle3dSettled::turns`
/// counts host↔guest round trips rather than paging items. See [`SETTLE_HOST_TURN_ITEMS`].
pub async fn dispatch_reporting_with_items(
    app: &mut Puzzle3dApp,
    action: &str,
    args: Option<&Value>,
    window_id: Option<&str>,
    maximum_items: usize,
) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
    let window_id = window_id.unwrap_or(main::WINDOW_KIND_ID);
    app.ensure_window(window_id);
    // 🏛️ The host resolves its OWN session state first and only then forwards the verb — so the very
    // call that activates a tool/utility already carries it, exactly as `🏛️ShellHost/🟦️.tsx` does.
    if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID) {
        app.activate(action, args, window_id);
    }
    let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..app.meta("local") };
    // 🕰️ Framework-reserved verbs stay on `handle_action`: this is exactly the `skip` set
    // `PluginBuilder`'s own declared-action bridge check uses (`🧰️framework/…/🔌️plugin/🦀️.rs`), i.e. every
    // verb the framework injects and handles itself. A reserved verb sent down the typed channel faults
    // `interactive-job.missing-factory` — the app owns no tool proof for a verb it never declared.
    if matches!(
        action,
        "undo"
            | "redo"
            | "commitCheckpoint"
            | "createAlternative"
            | "switchAlternative"
            | "checkoutCheckpoint"
            | "revertToCommand"
            | "setHistoryCommandFilter"
            | "noteShellCommand"
            | "recordTutorial"
            | "startIntroduction"
            | "startTutorial"
            | "copy"
            | "cut"
            | "paste"
            | "setActiveUtility"
            | "setActiveTool"
            | "interactionSelect"
            | "interactionHover"
            | "clearSelection"
            | "selectAll"
            | "setSelectionMode"
            | "setInteractionGranularity"
    ) {
        let dsl_args = args.map(semio_framework_pack_json::to_dsl_value);
        let admitted = app.handle_action(action, dsl_args.as_ref(), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        let reserved = match admitted {
            Ok(admitted) => settle_reserved(app, admitted).await,
            Err(fault) => Err(fault),
        };
        return settle_into_reporting_with_items(app, reserved, maximum_items).await;
    }
    let typed = app.dispatch_typed(Puzzle3dCommand::from_action(action, args.cloned(), Some(window_id.to_string())).unwrap_or_else(|| panic!("unknown puzzle3d action id in test: {action}")), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    settle_into_reporting_with_items(app, typed, maximum_items).await
}

/// 📤 `dispatch` up to the point the host has minted the typed operation and NOT one continuation
/// turn further: the operation is mounted, its worker session holds one of the process-wide
/// [`semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS`] admissions, and nothing has retired it. Dropping
/// the scene_snapshot here is exactly what a closed tab, a cancelled command or a torn-down app does, which
/// is the ONLY way a law can witness the retirement array a live app is expected to give back.
pub async fn dispatch_unsettled(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let window_id = window_id.unwrap_or(main::WINDOW_KIND_ID);
    app.ensure_window(window_id);
    let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..app.meta("local") };
    let command = Puzzle3dCommand::from_action(action, args.cloned(), Some(window_id.to_string())).unwrap_or_else(|| panic!("unknown puzzle3d action id in test: {action}"));
    app.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
/// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
/// deleted `worldPick`/`worldSelect`/`worldVortexSelect`/`setSelection` actions.
pub async fn select_id(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None).await
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionHover`
/// for one `(granularity, id)` pair on the `pointer` channel in the `vortex` domain — the
/// test-side replacement for the deleted `worldHover`/`setHover`/`worldVortexHover`/`setKindHover`
/// actions. `id: None` clears the hover (mirrors the old "hover nothing" call shape).
pub async fn hover_id(app: &mut Puzzle3dApp, granularity: &str, id: Option<&str>) -> Result<InvocationResult, Fault> {
    let targets: Vec<InteractionTarget> = id.map(|id| InteractionTarget { granularity: granularity.into(), id: id.into() }).into_iter().collect();
    let targets_json = to_json_string(&targets);
    dispatch(app, "interactionHover", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "channel": "pointer", "targets": targets_json })), None).await
}

/// 🖱️ Browser pointermove over vortices admits many `interactionHover`s before Isolated reserved
/// jobs finish. Laws that reproduce that storm must not `settle` between admits.
pub async fn dispatch_reserved_unsettled(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let window_id = window_id.unwrap_or(main::WINDOW_KIND_ID);
    app.ensure_window(window_id);
    let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..app.meta("local") };
    let dsl_args = args.map(semio_framework_pack_json::to_dsl_value);
    app.handle_action(action, dsl_args.as_ref(), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await
}

pub async fn hover_id_unsettled(app: &mut Puzzle3dApp, granularity: &str, id: Option<&str>) -> Result<InvocationResult, Fault> {
    let targets: Vec<InteractionTarget> = id.map(|id| InteractionTarget { granularity: granularity.into(), id: id.into() }).into_iter().collect();
    let targets_json = to_json_string(&targets);
    dispatch_reserved_unsettled(app, "interactionHover", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "channel": "pointer", "targets": targets_json })), None).await
}

pub async fn select_id_unsettled(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    dispatch_reserved_unsettled(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None).await
}

pub async fn settle_reserved(app: &mut Puzzle3dApp, admitted: InvocationResult) -> Result<InvocationResult, Fault> {
    semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await
}

/// 🖼️ The rendered body, as JSON — every panel/window assertion navigates this value.
///
/// 🪟️ The ViewModel is addressed at the very window the body key names (`<body>:<windowInstanceId>`,
/// else the main instance), exactly like `dispatch`: a host never asks "render this pane" without
/// saying which pane, and the window-owned config/transient partitions are captured through
/// `ViewModel::window_id` — a bare `ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)` renders every window-owned field at its
/// type default (no camera, no suggestion popup, no engagement scratch), which is a state no real
/// render can be in.
pub async fn render_body(app: &mut Puzzle3dApp, body_key: &str) -> Value {
    let window_id = body_key.split_once(':').map_or(main::WINDOW_KIND_ID, |(_, window)| window);
    app.ensure_window(window_id);
    let view = app.window_view(window_id);
    let tree = app.render(body_key, None, &view).await.expect("render");
    rendered_body_value(tree)
}

/// 📌️ One APP-LEVEL PANEL body rendered exactly the way `plugin_refresh_ui` renders it:
/// `ViewModel::for_panel()` over the live session, i.e. with NO `window_id` at all — the projection a
/// panel actually receives, and the one a `render_body`/`render_window_refresh` call can never
/// reproduce because both address a concrete instance. `focused_window_id` is the shell's last-focused
/// pane, the only per-call carrier of "which window is the user looking at" a panel is given.
pub async fn render_panel_body(app: &mut Puzzle3dApp, body_key: &str, focused_window_id: Option<&str>) -> Value {
    if let Some(window_id) = focused_window_id {
        app.ensure_window(window_id);
    }
    let mut view = app.view.clone();
    view.focused_window_id = focused_window_id.map(str::to_string);
    let tree = app.render(body_key, None, &view.for_panel()).await.expect("render");
    rendered_body_value(tree)
}

/// 🪟️ One window INSTANCE rendered exactly the way `plugin_refresh_ui` renders it: the window KIND's
/// own body key — the host sends `windowKinds[].n` for every instance, never a `<body>:<instance>`
/// spelling — with the view narrowed at that one instance through `ViewModel::for_window_instance`.
/// The `<body>:<instance>` form [`render_window`] uses is the OTHER host route
/// (`plugin_render_surface`'s bound surface context), so a law about what a split pane publishes has
/// to state THIS shape: it is the only one where the instance id reaches the guest through the view
/// alone.
pub async fn render_window_refresh(app: &mut Puzzle3dApp, body_key: &str, window_id: &str) -> Value {
    app.ensure_window(window_id);
    let view = app.window_view(window_id);
    let tree = app.render(body_key, None, &view).await.expect("render");
    rendered_body_value(tree)
}

/// 🖼️ The world-3d scene a rendered tree publishes, else the projected node JSON — the one projection
/// every render probe in this test context reads.
fn rendered_body_value(tree: semio_framework_plugin::ComponentTree) -> Value {
    let mut stack = vec![&tree.root];
    let mut rendered_scene = None;
    while let Some(node) = stack.pop() {
        if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
            if surface.doc_schema.as_str() == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                let scene: semio_framework_ui_scene::World3dScene = artifact_app_laws::built_surface_scene(node).expect("assemble world scene");
                let world3d = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&scene));
                rendered_scene = Some(object([("schema".to_string(), Value::from(surface.doc_schema.as_str())), ("world3d".to_string(), world3d)]));
                if scene.interaction_json.is_some() {
                    break;
                }
            }
        }
        stack.extend(node.children.iter());
    }
    let projected = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render projection");
    rendered_scene.unwrap_or_else(|| parse(&projected.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rendered node JSON"))
}

/// 📏️ The packed byte length of the world-3d surface payload this body actually admits, plus the
/// fixed capacity it is admitted against — the two numbers `scene-surface.encode` compares. The
/// payload is read back off the built `Surface` node rather than re-encoded, so the law measures the
/// same bytes the host shipped.
pub async fn world_surface_payload_bytes(app: &mut Puzzle3dApp, body_key: &str) -> (usize, usize) {
    let window_id = body_key.split_once(':').map_or(main::WINDOW_KIND_ID, |(_, window)| window);
    app.ensure_window(window_id);
    let view = app.window_view(window_id);
    let tree = app.render(body_key, None, &view).await.expect("render");
    let mut stack = vec![&tree.root];
    let mut widest = 0;
    while let Some(node) = stack.pop() {
        if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
            if surface.doc_schema.as_str() == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                widest = widest.max(surface.doc.bytes.as_slice().len());
            }
        }
        stack.extend(node.children.iter());
    }
    assert!(widest > 0, "{body_key} rendered no world-3d surface");
    (widest, semio_framework_ui_contract::UI_FIXED_BYTES)
}

fn count_built_nodes(node: &semio_framework_ui_contract::BuiltNode) -> usize {
    1 + node.children.iter().map(count_built_nodes).sum::<usize>()
}

//#region 🚚️WorldSceneCarrierCensus
/// 🚚️ One out-of-doc payload lane as it was actually published: the carrier's measured shape next to
/// what the spine declared about it.
pub struct WorldSceneLaneCensus {
    pub key: String,
    pub bytes: usize,
    pub leaves: usize,
    pub leaf_depth: usize,
    pub widest_children: usize,
    pub widest_leaf: usize,
    pub declared_bytes: u32,
    pub declared_hash: String,
}

/// 🚚️ Everything the paged-carrier laws measure about one rendered world body: the spine that rode
/// inside the fixed-capacity surface doc, every lane carrier beside it, and the scene a render host
/// reassembles from the two.
pub struct WorldSceneCarrierCensus {
    pub doc_bytes: usize,
    pub capacity: usize,
    pub nodes: usize,
    pub lanes: Vec<WorldSceneLaneCensus>,
    pub assembled: semio_framework_ui_scene::World3dScene,
}

impl WorldSceneCarrierCensus {
    pub fn payload_bytes(&self) -> usize {
        self.lanes.iter().map(|lane| lane.bytes).sum()
    }

    pub fn lane(&self, key: &str) -> Option<&WorldSceneLaneCensus> {
        self.lanes.iter().find(|lane| lane.key == key)
    }

    pub fn report(&self) -> String {
        self.lanes.iter().map(|lane| format!("{}={}B/{}leaves", lane.key.trim_start_matches(semio_framework_ui_scene::WORLD3D_SCENE_LANE_KEY_PREFIX), lane.bytes, lane.leaves)).collect::<Vec<_>>().join(" ")
    }
}

/// 🚚️ Renders `body_key` and censuses the world-3d surface it publishes. Reads the built tree
/// directly — never a re-encode — so the law measures exactly the bytes the host shipped.
pub async fn world_surface_carrier_census(app: &mut Puzzle3dApp, body_key: &str) -> WorldSceneCarrierCensus {
    let window_id = body_key.split_once(':').map_or(main::WINDOW_KIND_ID, |(_, window)| window);
    app.ensure_window(window_id);
    let view = app.window_view(window_id);
    let tree = app.render(body_key, None, &view).await.expect("render");
    let mut stack = vec![&tree.root];
    let mut census = None;
    while let Some(node) = stack.pop() {
        if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
            if surface.doc_schema.as_str() == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                let spine: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(surface).expect("decode world spine");
                let mut lanes = Vec::new();
                for child in node.children.iter() {
                    let payload = artifact_app_laws::built_carrier_text(child);
                    let (mut leaves, mut leaf_depth, mut widest_children, mut widest_leaf) = (0usize, 0usize, 0usize, 0usize);
                    let mut frontier = vec![(child, 0usize)];
                    while let Some((current, level)) = frontier.pop() {
                        widest_children = widest_children.max(current.children.len());
                        if let semio_framework_ui_contract::Component::Text(text) = &current.component {
                            leaves += 1;
                            leaf_depth = leaf_depth.max(level);
                            widest_leaf = widest_leaf.max(text.value.0.as_str().len());
                        }
                        frontier.extend(current.children.iter().map(|grandchild| (grandchild, level + 1)));
                    }
                    let reference = spine.lanes.iter().find(|reference| semio_framework_ui_scene::World3dSceneLane::from_name(&reference.lane).is_some_and(|lane| lane.body_key() == child.key.as_str()));
                    lanes.push(WorldSceneLaneCensus {
                        key: child.key.as_str().to_string(),
                        bytes: payload.len(),
                        leaves,
                        leaf_depth,
                        widest_children,
                        widest_leaf,
                        declared_bytes: reference.map_or(0, |reference| reference.bytes),
                        declared_hash: reference.map_or_else(String::new, |reference| reference.hash.clone()),
                    });
                }
                census = Some(WorldSceneCarrierCensus { doc_bytes: surface.doc.bytes.as_slice().len(), capacity: semio_framework_ui_contract::UI_FIXED_BYTES, nodes: count_built_nodes(node), lanes, assembled: artifact_app_laws::built_surface_scene(node).expect("assemble world scene") });
                break;
            }
        }
        stack.extend(node.children.iter());
    }
    let census = census.unwrap_or_else(|| panic!("{body_key} rendered no world-3d surface"));
    artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render projection");
    census
}
//#endregion 🚚️WorldSceneCarrierCensus

/// 🪟️ The world composite body for one window INSTANCE — the `<body>:<windowInstanceId>` form is
/// how a split pane asks for its own materialized options (see `ArtifactApp::render`).
pub async fn render_window(app: &mut Puzzle3dApp, window_id: &str) -> Value {
    render_body(app, &format!("{}:{window_id}", main::BODY_KEY)).await
}

pub async fn render_composite(app: &mut Puzzle3dApp) -> Value {
    render_body(app, main::BODY_KEY).await
}

pub fn projection_of(app: &Puzzle3dApp) -> Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(app.snapshot().expect("projection").typed()))
}

pub fn object_count(app: &Puzzle3dApp) -> usize {
    projection_of(app).get("objects").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
}

pub fn first_object_id(app: &Puzzle3dApp) -> String {
    projection_of(app).get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("first object id").to_string()
}

pub fn vortex_full_ids(app: &Puzzle3dApp) -> Vec<String> {
    let projection = projection_of(app);
    let mut ids = Vec::new();
    for object in projection.get("objects").and_then(Value::as_array).into_iter().flatten() {
        let object_id = object.get("id").and_then(Value::as_str).unwrap_or_default();
        for vortex in object.get("vortices").and_then(Value::as_array).into_iter().flatten() {
            if let Some(vortex_id) = vortex.get("id").and_then(Value::as_str) {
                ids.push(puzzle3d_vortex_full_id(object_id, vortex_id));
            }
        }
    }
    ids
}

pub fn first_vortex_full_id(app: &Puzzle3dApp) -> String {
    vortex_full_ids(app).into_iter().next().expect("seed vortex")
}

//#region 🔖️SceneProbes
fn scene_field(node: &Value, field: &str) -> Value {
    node.pointer(&format!("/world3d/{field}")).and_then(Value::as_str).and_then(|raw| parse(raw, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).unwrap_or(Value::Null)
}

pub fn instances_of(node: &Value) -> Vec<Value> {
    scene_field(node, "instancesJson").as_array().cloned().unwrap_or_default()
}

pub fn instance_count(node: &Value) -> usize {
    instances_of(node).len()
}

pub fn scene_meshes_of(node: &Value) -> Vec<Value> {
    scene_field(node, "meshesJson").as_array().cloned().unwrap_or_default()
}

pub fn vortices_of(node: &Value) -> Vec<Value> {
    scene_field(node, "vorticesJson").as_array().cloned().unwrap_or_default()
}

pub fn interaction_of(node: &Value) -> Value {
    scene_field(node, "interactionJson")
}

pub fn selection_of(node: &Value) -> Value {
    scene_field(node, "selectionJson")
}

pub fn lod_of(node: &Value) -> Value {
    scene_field(node, "lodJson")
}

pub fn camera_of(node: &Value) -> Value {
    scene_field(node, "cameraJson")
}

//#endregion 🔖️SceneProbes

//#region 🔖️MeasureProbes
/// 🔍️ Depth-first search for a `WindowMeasure::Slider`'s value by id, descending into groups (the
/// fill-count slider nests inside the fill tool's measure group rather than sitting on the engagement).
pub fn find_measure_slider(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Slider { id, value, .. } if id == slider_id => Some(*value),
        WindowMeasure::Group { children, .. } => find_measure_slider(children, slider_id),
        _ => None,
    })
}

pub fn find_measure_number(measures: &[WindowMeasure], number_id: &str) -> Option<f64> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Number { id, value, .. } if id == number_id => Some(*value),
        WindowMeasure::Group { children, .. } => find_measure_number(children, number_id),
        _ => None,
    })
}

pub fn find_measure_number_max(measures: &[WindowMeasure], number_id: &str) -> Option<Option<f64>> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Number { id, max, .. } if id == number_id => Some(*max),
        WindowMeasure::Group { children, .. } => find_measure_number_max(children, number_id),
        _ => None,
    })
}

pub fn find_measure_select(measures: &[WindowMeasure], select_id: &str) -> Option<String> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Select { id, value, .. } if id == select_id => Some(value.clone()),
        WindowMeasure::Group { children, .. } => find_measure_select(children, select_id),
        _ => None,
    })
}

pub fn find_measure_toggle(measures: &[WindowMeasure], toggle_id: &str) -> Option<bool> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Toggle { id, pressed, .. } if id == toggle_id => Some(*pressed),
        WindowMeasure::Group { children, .. } => find_measure_toggle(children, toggle_id),
        _ => None,
    })
}

/// 🎯️ Top-level utility tag of a `WindowMeasure::Group` by id, or `None` when the group is absent.
pub fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Group { id, active_utility_id, .. } if id == group_id => Some(active_utility_id.clone()),
        _ => None,
    })
}

//#endregion 🔖️MeasureProbes

//#region ⏯️ToolRunProbes
/// ⏱️ Host turns one fill run law may spend before it calls the run stuck.
pub const FILL_RUN_TURNS: usize = 262_144;

/// ⏯️ Dispatches one framework tool run action exactly as the host routes it: straight to
/// `handle_action`, applied to the ledger inside the same call and never queued as a typed operation.
/// Answers the action's output (`{"toolRun": effect}` or `{"rejected": code}`).
pub async fn tool_run_action(app: &mut Puzzle3dApp, action: &str, args: Value) -> Value {
    app.ensure_window(main::WINDOW_KIND_ID);
    let action_meta = ActionMeta { view_state: Some(app.window_view(main::WINDOW_KIND_ID)), ..app.meta("local") };
    let result = app.handle_action(action, Some(&semio_framework_pack_json::to_dsl_value(&args)), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{action} faulted: {fault:?}"));
    semio_framework_pack_json::from_dsl_value(&result.output)
}

/// 🎬️ Registers the lifecycle scene_snapshot's cube meshes, then starts a fill run through the panel action.
pub async fn start_fill_run(app: &mut Puzzle3dApp) -> Value {
    let scene_snapshot = puzzle3d_scene_snapshot_from_document(app.snapshot().expect("live fill scene_snapshot").typed());
    let (positions, indices) = puzzle3d_fallback_mesh_buffers();
    for url in collect_mesh_urls(&scene_snapshot) {
        assert!(semio_s_artifact_puzzle_3d_precompute::derive_brush_mesh(&url, &positions, &indices).is_some(), "scene_snapshot mesh {url}");
    }
    tool_run_action(app, semio_framework_tool_run::TOOL_RUN_START_ACTION_ID, object([(semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID.to_string(), Value::from(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID))])).await
}

/// 🧭️ The live fill run's `{runId, generation}` exactly as the main window's Escape binding carries it
/// (`on_abort` of its engagement input), or `None` while no fill run is live.
pub async fn fill_run_identity(app: &mut Puzzle3dApp) -> Option<Value> {
    let view = app.window_view(main::WINDOW_KIND_ID);
    let engagements = app.window_engagements(&view).await;
    let abort = engagements.get(main::WINDOW_KIND_ID)?.input.as_ref()?.on_abort.clone()?;
    (abort.action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID).then(|| semio_framework_pack_json::from_dsl_value(abort.args.as_ref().expect("a tool run abort carries its identity")))
}

/// ⏯️ The run's ephemeral-shared presence summary: state, stage, placements and requested count.
pub fn fill_run_presence(app: &Puzzle3dApp) -> protocol::PresenceToolRun {
    app.tool_run_presence().expect("a fill run slot exists")
}

/// 🔁️ Host turns until `until` holds for the run's presence summary; answers the turns it took.
pub async fn pump_fill_run(app: &mut Puzzle3dApp, what: &str, until: impl Fn(&protocol::PresenceToolRun) -> bool) -> usize {
    let mut settled = Puzzle3dSettled::default();
    for turn in 0..FILL_RUN_TURNS {
        if app.tool_run_presence().is_some_and(|presence| until(&presence)) {
            return turn;
        }
        host_turn(app, 1, &mut settled).await;
    }
    panic!("{what} never happened within {FILL_RUN_TURNS} host turns; presence {:?}", app.tool_run_presence());
}

/// 🚦️ Plays the refresh half of the host for the brush suggestions run: dispatches what `pending_effects`
/// asks for exactly as the host routes a tool run action, then one host turn, until `until` holds for the
/// brush run's presence summary; answers the turns it took.
pub async fn pump_brush_suggestions(app: &mut Puzzle3dApp, what: &str, until: impl Fn(Option<&protocol::PresenceToolRun>) -> bool) -> usize {
    let mut settled = Puzzle3dSettled::default();
    for turn in 0..FILL_RUN_TURNS {
        let view = app.window_view(main::WINDOW_KIND_ID);
        for effect in app.pending_effects(Some(&view)).await {
            if let Effect::DispatchAction { action, args, .. } = effect {
                tool_run_action(app, &action, args.as_ref().map(semio_framework_pack_json::from_dsl_value).unwrap_or(Value::Null)).await;
            }
        }
        if until(app.tool_run_presence().as_ref()) {
            return turn;
        }
        host_turn(app, 1, &mut settled).await;
    }
    panic!("{what} never happened within {FILL_RUN_TURNS} host turns; presence {:?}", app.tool_run_presence());
}

/// 🔎️ The brush suggestions search settled: a live run has decided every candidate of its target.
pub fn brush_suggestions_settled(presence: Option<&protocol::PresenceToolRun>) -> bool {
    presence.is_some_and(|presence| presence.state == protocol::PresenceToolRunState::Running && presence.total.is_some_and(|total| total > 0 && presence.completed == total))
}

/// 🎣️ Points the armed brush of the main window at `vortex` and pumps until its search settles.
pub async fn target_brush_suggestions(app: &mut Puzzle3dApp, vortex: &str) {
    dispatch(app, "targetBrushSuggestions", Some(&json!({ "fullId": vortex })), Some(main::WINDOW_KIND_ID)).await.expect("targetBrushSuggestions");
    pump_brush_suggestions(app, "the brush suggestions search settles", brush_suggestions_settled).await;
}

/// 🪪️ The run id the live trace belongs to.
pub fn tool_run_trace_run(app: &Puzzle3dApp) -> Option<u64> {
    let encoded = app.tool_run_trace_delta(None)?;
    Some(semio_framework_tool_run::ToolRunTraceDelta::decode(&semio_framework_io_base64::base64_url_decode(encoded).expect("base64url")).expect("trace delta").identity.id.run)
}

/// 🪞️ Every trace record the ledger holds for the live run, decoded from its base64url delta.
pub fn tool_run_trace_records(app: &Puzzle3dApp) -> Vec<(u64, semio_framework_tool_run::ToolRunVerdict)> {
    let Some(encoded) = app.tool_run_trace_delta(None) else { return Vec::new() };
    let delta = semio_framework_tool_run::ToolRunTraceDelta::decode(&semio_framework_io_base64::base64_url_decode(encoded).expect("base64url")).expect("trace delta");
    let mut store = semio_framework_tool_run::ToolRunTraceStore::new(delta.identity);
    for page in &delta.pages {
        store.apply_page(page).expect("trace page");
    }
    let mut records: Vec<(u64, semio_framework_tool_run::ToolRunVerdict)> = store.records().map(|(key, record)| (key, record.verdict)).collect();
    records.sort_unstable_by_key(|(key, _)| *key);
    records
}

/// 📄️ The committed document, as the exact text the store holds.
pub fn committed_document(app: &Puzzle3dApp) -> String {
    semio_framework_pack_json::to_json_string(app.snapshot().expect("committed snapshot").typed())
}

/// 🪞️ How many objects the main window renders — committed ⊕ provisional while a run holds placements.
pub async fn rendered_object_count(app: &mut Puzzle3dApp) -> usize {
    instance_count(&render_composite(app).await)
}
//#endregion ⏯️ToolRunProbes

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `context_menu` reads the
/// CLIENT-supplied `request.surface.selection` now (selection is framework-owned, no live config
/// field to derive it from) — the test-side replacement for the deleted `contextMenuAt` command's
/// "select then open the menu" round trip.
pub async fn context_menu_for_selection(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget { surface_id: "world3d".into(), kind: "world3d".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: granularity.into(), ids: vec![id.to_string()] }], text: None }),
        window_instance_id: None,
        point: None,
    };
    let view = app.window_view(main::WINDOW_KIND_ID);
    app.context_menu(&request, &view).await
}

/// 🎯️ The menu a right-click on an UNSELECTED entity opens: the surface carries a `hits` entry and an
/// empty `selection`, exactly what `World3dHost` sends when `resolveWorldContextMenuTarget` resolves a
/// pointer target the document has not selected.
pub async fn context_menu_for_hit(app: &mut Puzzle3dApp, domain: &str, id: &str) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{ContextMenuHit, ContextMenuRequest, ContextMenuSurfaceTarget, UiMenuRef};
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget {
            surface_id: "world3d".into(),
            kind: "world3d".into(),
            hits: vec![ContextMenuHit { domain: domain.into(), id: id.to_string(), label: None }],
            selection: Vec::new(),
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let view = app.window_view(main::WINDOW_KIND_ID);
    app.context_menu(&request, &view).await
}

//#region 🧮️HeapWitness
/// 🧮️ The test binary's own allocator, counting retained bytes across the WHOLE PROCESS. The guest
/// heap this artifact ships into is a fixed `GUEST_LINEAR_MEMORY_MAXIMUM_BYTES` wasm linear memory
/// with ONE allocator and no threads at all, so an owner the tick loop never frees is a hard trap in
/// production and nothing at all in a native suite — the only way a law can state the growth bound
/// is to weigh the heap itself.
///
/// 🧵️ Process-wide, deliberately NOT per-thread: natively a mounted worker session allocates its
/// owners on a pool thread and the tick loop frees them on the caller's, so a per-thread counter
/// reads a growing leak as a NEGATIVE number on the law's own thread and reports the exact opposite
/// of the truth. A growth law therefore runs its ticks with the fill registry guard held
/// ([`crate::editor::puzzle3d::precompute::fill_envelope_test_guard`]) and differences this counter.
///
/// 🧬️ The instrument itself is `semio_framework_trace::HeapWitness`, shared with the procedural
/// artifact's guest-memory laws instead of written twice
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[global_allocator]
static PUZZLE3D_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

//#endregion 🧮️HeapWitness

pub fn edit_rows(settled: &Puzzle3dSettled) -> Vec<semio_framework::kernel::HistoryEntry> {
    settled.completions.iter().filter_map(|completion| completion.history_patch.as_ref()).flat_map(|patch| patch.upserts.iter()).filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()
}

pub fn object_origin(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

pub fn object_origin_x(app: &Puzzle3dApp, object_id: &str) -> f64 {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).and_then(|origin| origin.first()).and_then(Value::as_f64))
        .expect("origin.x")
}

pub fn dispatchable_actions(definition: &semio_framework_plugin::AppDefinition) -> Vec<&ActionDefinition> {
    let mut seen = std::collections::BTreeSet::new();
    definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(definition, window))
        .filter(|action| seen.insert(action.id.clone()))
        .collect()
}
