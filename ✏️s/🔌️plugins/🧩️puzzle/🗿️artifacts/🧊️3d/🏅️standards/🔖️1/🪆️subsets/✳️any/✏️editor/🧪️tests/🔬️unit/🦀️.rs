pub(crate) mod context {
    
    use super::super::*;
    use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, MAINTENANCE_STAGES, PluginApp, PluginCloseStep, VcsArtifactApp, ViewModel, ViewWindowInstance, artifact_app_laws};
    
    pub type Puzzle3dRawApp = VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>;
    
    /// 📏️ The byte grant the OS runtime's cooperative-maintenance clock hands one live-cleanup unit
    /// (`RuntimeLiveCleanupJob::step`'s `maintenance_step(1, RUNTIME_CLOSE_BYTES_PER_STEP)`), restated
    /// here so a step-budget law measures the exact unit the host drives rather than a wider one.
    pub const RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP: usize = 4_096;
    
    /// 📐️ Units retained per stage for the typical-cost statistic. Fixed, and taken from the FIRST
    /// units a stage runs — the busy ones, where a stage that has real work does it.
    const MAINTENANCE_UNIT_SAMPLES: usize = 64;
    
    /// ⏱️ Measured wall cost of the cooperative-maintenance units of one run, per fixed maintenance
    /// stage. Fixed capacity by construction: the round robin has exactly [`MAINTENANCE_STAGES`]
    /// stages, each keeping [`MAINTENANCE_UNIT_SAMPLES`] readings, and nothing grows.
    ///
    /// Two statistics, because one cannot carry both halves of the framework's law. [`worst`] is the
    /// per-stage MEDIAN — a stage that overruns because of its OWN work overruns unit after unit, so a
    /// median catches a systemic regression while ignoring the machine (the runtime's ceiling verdict
    /// times `maintenance_step` on the wall clock of a contended thread: the same unit that costs 19us
    /// alone was seen costing 14571us inside a fully parallel suite run). [`worst_unit`] is the plain
    /// maximum, which is what the ceiling itself actually bounds, and is therefore budgeted against the
    /// ceiling rather than against the far tighter typical-unit budget. Median precedent:
    /// `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️fixed-operation-registry/🦀️.rs`.
    pub struct MaintenanceStageBudget {
        samples_us: [[u64; MAINTENANCE_UNIT_SAMPLES]; MAINTENANCE_STAGES as usize],
        sampled: [usize; MAINTENANCE_STAGES as usize],
        worst_us: [u64; MAINTENANCE_STAGES as usize],
        units: [u32; MAINTENANCE_STAGES as usize],
    }
    
    impl Default for MaintenanceStageBudget {
        fn default() -> Self {
            Self { samples_us: [[0; MAINTENANCE_UNIT_SAMPLES]; MAINTENANCE_STAGES as usize], sampled: [0; MAINTENANCE_STAGES as usize], worst_us: [0; MAINTENANCE_STAGES as usize], units: [0; MAINTENANCE_STAGES as usize] }
        }
    }
    
    impl MaintenanceStageBudget {
        /// ⏱️ Typical cost of one unit of `stage`: the median of its retained readings.
        pub fn median(&self, stage: usize) -> u64 {
            let sampled = self.sampled[stage];
            if sampled == 0 {
                return 0;
            }
            let mut ordered = self.samples_us[stage];
            ordered[..sampled].sort_unstable();
            ordered[sampled / 2]
        }
    
        /// ⏱️ The stage whose typical unit is the most expensive, with that median in microseconds.
        pub fn worst(&self) -> (u8, u64) {
            let mut worst = (0_u8, 0_u64);
            for stage in 0..MAINTENANCE_STAGES as usize {
                let median = self.median(stage);
                if median > worst.1 {
                    worst = (stage as u8, median);
                }
            }
            worst
        }
    
        /// ⏱️ The stage that ran the single most expensive unit, with that maximum in microseconds.
        pub fn worst_unit(&self) -> (u8, u64) {
            let mut worst = (0_u8, 0_u64);
            for stage in 0..MAINTENANCE_STAGES as usize {
                if self.worst_us[stage] > worst.1 {
                    worst = (stage as u8, self.worst_us[stage]);
                }
            }
            worst
        }
    
        /// 🎲️ Folds one more independent round of the same scenario in by keeping, per stage, the
        /// SMALLEST median and the SMALLEST maximum any round observed. Real work costs the same in
        /// every round; a run that happened to share the machine with a heavier neighbour is dropped.
        pub fn keep_best_round(&mut self, round: &Self) {
            for stage in 0..MAINTENANCE_STAGES as usize {
                if round.units[stage] == 0 {
                    continue;
                }
                if self.sampled[stage] == 0 || round.median(stage) < self.median(stage) {
                    self.samples_us[stage] = round.samples_us[stage];
                    self.sampled[stage] = round.sampled[stage];
                }
                if self.units[stage] == 0 || round.worst_us[stage] < self.worst_us[stage] {
                    self.worst_us[stage] = round.worst_us[stage];
                }
                self.units[stage] = self.units[stage].max(round.units[stage]);
            }
        }
    
        /// 📊️ Per-stage `stage=median/worst/units` breakdown of every unit measured so far.
        pub fn report(&self) -> String {
            let mut report = String::new();
            for stage in 0..MAINTENANCE_STAGES as usize {
                if self.units[stage] > 0 {
                    report.push_str(&format!("{stage}={}/{}us/{}u ", self.median(stage), self.worst_us[stage], self.units[stage]));
                }
            }
            report
        }
    
        fn record(&mut self, stage: u8, elapsed_us: u64) {
            let stage = stage as usize;
            if self.sampled[stage] < MAINTENANCE_UNIT_SAMPLES {
                self.samples_us[stage][self.sampled[stage]] = elapsed_us;
                self.sampled[stage] += 1;
            }
            self.units[stage] += 1;
            self.worst_us[stage] = self.worst_us[stage].max(elapsed_us);
        }
    }
    
    /// 🧪️ The one puzzle3d fixture app. Wraps the raw wrapper so every fixture drains its stores on the
    /// way out: a registry-backed `VcsArtifactApp` installs framework-owned `ArtifactStoreCursorDisposer`
    /// members whose own `Drop` asserts terminal-empty ownership (`🏪️store/🦀️.rs`), so a bare drop panics
    /// inside a destructor. `Drop` here therefore NEVER panics and never asserts: a second panic while a
    /// failing assertion is already unwinding is a non-unwinding abort that kills the whole test binary
    /// and hides the assertion that actually failed. A drain that cannot reach the witness leaks the raw
    /// app instead ([`std::mem::forget`]) — leaking a fixture inside a test process costs nothing, and the
    /// close contract itself is stated exactly once, explicitly, by [`close_witness`].
    /// 📦️ Both owners are boxed on purpose. `Puzzle3dRawApp` is 42 KiB by value and
    /// [`MaintenanceStageBudget`] carries a fixed `[[u64; MAINTENANCE_UNIT_SAMPLES]; MAINTENANCE_STAGES]`
    /// sample matrix; inline, one fixture is 55 KiB, and a law's `#[async_test]` future — which
    /// `#[async_test]` pins ON THE STACK — moves it through every frame of the harness. The default test
    /// thread has 2 MiB, and the app-fixture laws in this crate already spend most of it inside the
    /// framework's own construction chain.
    pub struct Puzzle3dApp {
        raw: Option<Box<Puzzle3dRawApp>>,
        /// 🏛️ The HOST's own session state, owned here because the host owns it in production: the live
        /// window roster, the mode-wide `active_tool_id`, and the per-window `active_utility_by_window_id`
        /// (`📓️2026-09-09-peer-config-runtime-split.md` §1(d)). `setActiveTool`/`setActiveUtility` are
        /// dispatched by the framework as an empty `Emit` (`🔌️plugin/🦀️.rs` `dispatch_action`), so the app
        /// writes nothing and the ONLY thing that can carry an activation to the next call is this record.
        /// Minting a fresh `ViewModel` per call — what this fixture used to do — made every activation
        /// unobservable one call later, which is a state no real host can be in.
        view: ViewModel,
        /// ⏱️ Every cooperative-maintenance unit this fixture drove, attributed to its own fixed stage.
        pub maintenance: Box<MaintenanceStageBudget>,
    }
    
    /// 🧰️ `resolveUtilityActivation` (`🛠️ShellHelpers/🟦️.tsx`), verbatim: an empty request — or
    /// re-requesting what is already active — deactivates; anything else activates.
    fn resolve_activation(current: Option<&str>, requested: &str) -> Option<String> {
        (!requested.is_empty() && current != Some(requested)).then(|| requested.to_string())
    }
    
    impl std::ops::Deref for Puzzle3dApp {
        type Target = Puzzle3dRawApp;
        fn deref(&self) -> &Self::Target {
            self.raw.as_deref().expect("fixture app was already consumed by close_witness")
        }
    }
    
    impl std::ops::DerefMut for Puzzle3dApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.raw.as_deref_mut().expect("fixture app was already consumed by close_witness")
        }
    }
    
    impl Puzzle3dApp {
        /// 🪟️ Registers one window instance in the fixture's live roster, exactly as the shell's own
        /// `sessionWindowInstances` grows the moment a pane is opened or split. Idempotent.
        fn ensure_window(&mut self, window_id: &str) {
            if !self.view.window_instances.iter().any(|instance| instance.id == window_id) {
                self.view.window_instances.push(ViewWindowInstance { id: window_id.into(), window_kind_id: main::WINDOW_KIND_ID.into() });
            }
        }
    
        /// 🎯️ The exact per-call projection a host sends: the whole live session — roster, mode-wide tool,
        /// per-window utility map — addressed at ONE concrete window instance through the framework's own
        /// `ViewModel::for_window_instance`, which is what stamps `active_utility_id` for that pane.
        /// 🗣️ Names the host's live label axes for every later render/measures call on this fixture. The
        /// app resolves its label set from `ViewModel.locale`/`.terminology` and fails closed on an axis it
        /// never authored (`puzzle3d_labels`), so a test that asserts German or reuse text has to say so —
        /// there is no default language to fall back to.
        pub fn set_label_axes(&mut self, locale: semio_framework_plugin::Locale, terminology: semio_framework_plugin::Terminology) {
            self.view.locale = locale;
            self.view.terminology = terminology;
        }
    
        pub fn window_view(&self, window_id: &str) -> ViewModel {
            let mut view = self.view.clone();
            if !view.window_instances.iter().any(|instance| instance.id == window_id) {
                view.window_instances.push(ViewWindowInstance { id: window_id.into(), window_kind_id: main::WINDOW_KIND_ID.into() });
            }
            view.for_window_instance(window_id).expect("puzzle3d test window roster")
        }
    
        /// 🛠️🧰️ The shell's own activation branches (`🏛️ShellHost/🟦️.tsx`'s `SET_ACTIVE_TOOL_ACTION_ID`
        /// and `SET_ACTIVE_UTILITY_ACTION_ID`), applied to this fixture's session BEFORE the verb is
        /// forwarded — the same order the shell uses, so the plugin call already sees the new activation.
        /// A tool and a window utility are mutually exclusive interaction owners: activating a tool clears
        /// every window's utility, activating a utility clears the tool.
        fn activate(&mut self, action: &str, args: Option<&Value>, window_id: &str) {
            let requested = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).unwrap_or_default();
            if action == SET_ACTIVE_TOOL_ACTION_ID {
                let next = resolve_activation(self.view.active_tool_id.as_deref(), requested("toolId"));
                self.view.active_tool_id = next.clone();
                if next.is_some() {
                    self.view.active_utility_by_window_id.clear();
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
    
        /// ⏱️ One cooperative-maintenance unit shaped exactly like the OS runtime's live-cleanup clock
        /// drives it, timed by the framework's own clock and attributed to the fixed stage that ran it.
        /// The stage is read BEFORE the call: `maintenance_step`'s idle early return leaves the
        /// round-robin cursor untouched, so reading it afterwards names a stale stage.
        pub fn measure_maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            let stage = self.next_maintenance_stage();
            let started_us = semio_framework_job::default_now_us();
            let step = PluginApp::maintenance_step(self.raw.as_deref_mut().expect("fixture app was already consumed by close_witness"), maximum_items, maximum_bytes);
            if let (Some(started_us), Some(finished_us)) = (started_us, semio_framework_job::default_now_us()) {
                self.maintenance.record(stage, finished_us.saturating_sub(started_us));
            }
            step
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
            match PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)? {
                PluginCloseStep::Complete => break,
                PluginCloseStep::Blocked { reason } | PluginCloseStep::AwaitingInput { reason } => blocked_on = Some(reason),
                PluginCloseStep::Pending { .. } => {}
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
    
    /// 🧹️ Consumes one fixture app through its real close state machine and reports the outcome, so the
    /// close contract is asserted by a test rather than by a destructor. `Err` carries the framework's own
    /// fault; `Ok(false)` means the machine reported `Complete` without reaching terminal-empty ownership.
    pub fn close_witness(mut app: Puzzle3dApp) -> Result<bool, Fault> {
        let mut raw = app.raw.take().expect("fixture app was already consumed by close_witness");
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
            let Some(mut raw) = self.raw.take() else {
                return;
            };
            if !matches!(drain_close(&mut raw), Ok(true)) {
                std::mem::forget(raw);
            }
        }
    }
    
    pub fn meta(actor: &str) -> ActionMeta {
        artifact_app_laws::meta(actor)
    }
    
    /// 🌉️ `new_app_with_registry`'s `manifest: fn() -> App` shape predates the `AppDefinition`-returning
    /// `create_puzzle3d_app()` convention (contract §2.4 / SDK gap 3) — this tiny local wrapper bridges
    /// the two, mirroring `📓️w2-cad-report.md`'s recipe step 7.
    pub fn puzzle3d_manifest_for_tests() -> App {
        App { definition: create_puzzle3d_app(), examples: Vec::new() }
    }
    
    /// 🧰️ The registry-backed, instance-bound fixture app — the ONLY constructor this plugin can use.
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
        crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
        let registry = puzzle3d_action_registry();
        let mut app = VcsArtifactApp::with_registry(EditorApp::<Puzzle3dPlayApp>::default(), registry).await;
        app.bind_instance_id(1).await;
        let view = ViewModel { window_instances: vec![ViewWindowInstance { id: main::WINDOW_KIND_ID.into(), window_kind_id: main::WINDOW_KIND_ID.into() }], ..Default::default() };
        Puzzle3dApp { raw: Some(Box::new(app)), view, maintenance: Box::new(MaintenanceStageBudget::default()) }
    }
    
    /// 🖌️ Host-session utility activation without minting a typed operation or running `settle`.
    #[inline(never)]
    pub fn activate_window_utility(app: &mut Puzzle3dApp, utility_id: &str) {
        app.ensure_window(main::WINDOW_KIND_ID);
        app.activate(semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utility_id })), main::WINDOW_KIND_ID);
    }
    
    /// 🪪️ The instance identity every fixture binds, and therefore the receiver `take_typed_operation_result_page`
    /// answers for.
    pub const FIXTURE_INSTANCE_ID: u32 = 1;
    
    /// 🔁️ Runs the host's continuation loop to quiescence and hands back the effects/events/UI scope the
    /// host would have forwarded to the shell. A registry-backed `VcsArtifactApp` does NOT apply a retained
    /// tool job inline — `dispatch_typed` mints a `ToolOperationSpec` whose worker, store publication,
    /// result page and outboxes are advanced only by later continuation turns, which is why the
    /// registry-less fixture used to see mutations land synchronously and this one does not. One turn is
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
    /// counts it, so a fixture that skips it can never observe quiescence again after its first
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
            settled.turns += 1;
            app.measure_maintenance_step(maximum_items, SETTLE_TURN_BYTES).expect("maintenance step drives the mounted operation's worker and retirement stages");
            app.advance_typed_operation_publication().await.expect("advance one typed operation publication unit");
            if let Some(page) = app.take_typed_operation_result_page(FIXTURE_INSTANCE_ID) {
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
            settled.scope = app.take_typed_operation_ui_scope().or(settled.scope);
            acknowledge_local_interaction_pages(app);
        }
        panic!("puzzle3d app never quiesced: pending typed operations outlived the settle budget");
    }
    
    /// 📬️ Everything the HOST forwards to the shell once a dispatch has run to quiescence — the same
    /// four channels `plugin_runtime` carries, so a law reads exactly what a client would.
    ///
    /// 🧲️ [`Self::completions`] is the channel the browser reads as `AppFrame::OperationCompleted` and the
    /// shell applies through `subscribeOperationCompletions` → `applyHostEffects`. It used to be drained
    /// and DROPPED here, which is why a command that answers before it runs — every verb carrying a
    /// `coalesce_key` enters the latest-wins channel and its accepted invocation returns
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
            let value: Value = parse(&text).unwrap_or_else(|_| panic!("download result page must be JSON: {text}"));
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
    
    /// 🔁️ Folds one settled turn's forwarded output into the invocation the shell observes, so a test reads
    /// the same `(requested_effects, events, ui_scope)` triple a client would after the continuation turns.
    async fn settle_into(app: &mut Puzzle3dApp, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
        settle_into_reporting(app, result).await.0
    }
    
    /// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: [`settle_into`] plus the settle census it folded
    /// from, for laws that must read a channel the `InvocationResult` deliberately does NOT carry — above
    /// all the command-log delta. A typed operation never calls `record_command`: its edit reaches the log
    /// through `backfill_command_log` and the patch that carries it is minted by
    /// `take_typed_operation_completion`, so a MUTATING verb's row rides `AppFrame::OperationCompleted`
    /// while `InvocationResult::history_patch` keeps its own narrower meaning — the delta the ADMISSION
    /// itself recorded. Six import laws asserted the former on the latter and were green only because the
    /// row they read was the PREVIOUS command's un-delivered one (the one-command lag).
    async fn settle_into_reporting(app: &mut Puzzle3dApp, result: Result<InvocationResult, Fault>) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
        settle_into_reporting_with_items(app, result, 1).await
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
    
    /// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
    /// `Self::Command` channel). Reconstructs the `Puzzle3dCommand` from the same
    /// `(action, args, window_id)` triple every pre-B1 test already passed.
    ///
    /// 🛰️ Wave B7: a framework-reserved verb is a TWO-half gesture since `dispatch_framework_reserved_action`
    /// began answering every non-clipboard route with `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }`.
    /// `handle_action` only ADMITS it; the document changes when the host drives that Isolated job to a
    /// terminal step and hands the bytes back through `complete_reserved_spawned_job`. A fixture that stopped
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
        let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..meta("local") };
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
            let dsl_args = args.map(json::to_dsl_value);
            let admitted = app.handle_action(action, dsl_args.as_ref(), &action_meta).await;
            let reserved = match admitted {
                Ok(admitted) => settle_reserved(app, admitted).await,
                Err(fault) => Err(fault),
            };
            return settle_into_reporting_with_items(app, reserved, maximum_items).await;
        }
        let typed = app.dispatch_typed(Puzzle3dCommand::from_action(action, args.cloned(), Some(window_id.to_string())).unwrap_or_else(|| panic!("unknown puzzle3d action id in test: {action}")), &action_meta).await;
        settle_into_reporting_with_items(app, typed, maximum_items).await
    }
    
    /// 📤 `dispatch` up to the point the host has minted the typed operation and NOT one continuation
    /// turn further: the operation is mounted, its worker session holds one of the process-wide
    /// [`semio_framework_job::WORKER_JOB_SESSION_SLOTS`] admissions, and nothing has retired it. Dropping
    /// the fixture here is exactly what a closed tab, a cancelled command or a torn-down app does, which
    /// is the ONLY way a law can witness the retirement array a live app is expected to give back.
    pub async fn dispatch_unsettled(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
        let window_id = window_id.unwrap_or(main::WINDOW_KIND_ID);
        app.ensure_window(window_id);
        let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..meta("local") };
        let command = Puzzle3dCommand::from_action(action, args.cloned(), Some(window_id.to_string())).unwrap_or_else(|| panic!("unknown puzzle3d action id in test: {action}"));
        app.dispatch_typed(command, &action_meta).await
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
        let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..meta("local") };
        let dsl_args = args.map(json::to_dsl_value);
        app.handle_action(action, dsl_args.as_ref(), &action_meta).await
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
    /// `ViewModel::window_id` — a bare `ViewModel::default()` renders every window-owned field at its
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
                    let world3d = json::from_dsl_value(&dsl::ToValue::to_value(&scene));
                    rendered_scene = Some(object([("schema".to_string(), Value::from(surface.doc_schema.as_str())), ("world3d".to_string(), world3d)]));
                    if scene.interaction_json.is_some() {
                        break;
                    }
                }
            }
            stack.extend(node.children.iter());
        }
        let projected = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render projection");
        rendered_scene.unwrap_or_else(|| parse(&projected.to_string()).expect("rendered node JSON"))
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
        parse(&app.snapshot().expect("projection").value().to_string()).expect("snapshot JSON")
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
        node.pointer(&format!("/world3d/{field}")).and_then(Value::as_str).and_then(|raw| parse(raw).ok()).unwrap_or(Value::Null)
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
    
    pub fn brush_preview_of(node: &Value) -> Value {
        scene_field(node, "brushPreviewJson")
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
    
    pub fn find_measure_slider_max(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Slider { id, max, .. } if id == slider_id => Some(*max),
            WindowMeasure::Group { children, .. } => find_measure_slider_max(children, slider_id),
            _ => None,
        })
    }
    
    pub fn find_measure_slider_ready(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Slider { id, ready, .. } if id == slider_id => *ready,
            WindowMeasure::Group { children, .. } => find_measure_slider_ready(children, slider_id),
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
    
    pub fn find_measure_number_ready(measures: &[WindowMeasure], number_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Number { id, ready, .. } if id == number_id => *ready,
            WindowMeasure::Group { children, .. } => find_measure_number_ready(children, number_id),
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
    
    /// 🪣️ How many fill placements the DOCUMENT already holds, read off the fill tool's own count
    /// entry. Locking is committing, so the entry's `ready` extent is the locked count — there is no
    /// planned-but-invisible prefix to read any more.
    pub async fn fill_ready(app: &mut Puzzle3dApp) -> f64 {
        let view = app.window_view(main::WINDOW_KIND_ID);
        app.tool_measures(&view).await.get(fill_tool::TOOL_ID).and_then(|tool_measures| find_measure_number_ready(tool_measures, "puzzle3d-fill-count")).unwrap_or(0.0)
    }
    
    /// 🛑 The `(job, operation, generation)` triple the Fill panel's own `Cancel fill` affordance carries,
    /// read off the published measure exactly as the host reads it before dispatching `cancelFillBuild` —
    /// `None` while no run is planning, which is also when the affordance itself is absent.
    pub async fn fill_cancel_identity(app: &mut Puzzle3dApp) -> Option<(u64, u64, u64)> {
        let view = app.window_view(main::WINDOW_KIND_ID);
        let measures = app.tool_measures(&view).await;
        let toggle_id = format!("{}-fill-cancel", crate::editor::puzzle3d::PUZZLE3D_PLAY_CONTROLLER_ID);
        let descriptor = measures.get(fill_tool::TOOL_ID)?.iter().find_map(|measure| match measure {
            WindowMeasure::Toggle { id, on_change, .. } if *id == toggle_id => Some(on_change.clone()),
            _ => None,
        })?;
        let args = dsl::os_pack::json::from_dsl_value(descriptor.args.as_ref()?);
        let field = |key: &str| args.get(key).and_then(dsl::os_pack::json::Value::as_u64);
        Some((field("job")?, field("operation")?, field("generation")?))
    }
    
    /// ⚙️ Steps isolated `FILL_JOB_KIND` jobs the same way the React host does after a guest turn:
    /// `start_job` once per `SpawnJob`, then `step_job` until `Done`/`Failed` or the per-tick slice
    /// budget. `drive_enqueued_fill_job_for_test` is a different empty `Puzzle3dPlayApp` than the
    /// dispatch session and must not be used as a stand-in for this runtime.
    pub async fn step_spawned_fill_jobs(effects: &[Effect], live: &mut Vec<u64>) {
        use crate::editor::puzzle3d::precompute::FILL_JOB_KIND;
        use semio_framework_plugin::reactor::jobs::{self, JobBudget, JobStep};
        for effect in effects {
            if let Effect::SpawnJob { job, kind, input, .. } = effect {
                if kind == FILL_JOB_KIND {
                    jobs::start_job(*job, kind, input).await;
                    live.push(*job);
                }
            }
        }
        let mut still = Vec::new();
        for job in live.drain(..) {
            let mut terminal = false;
            for _ in 0..64 {
                match jobs::step_job(job, JobBudget { fuel: 1, deadline_ms: 1 }).await {
                    JobStep::Running(_) => {}
                    JobStep::Done(_) | JobStep::Failed(_) => {
                        terminal = true;
                        break;
                    }
                }
            }
            if !terminal {
                still.push(job);
            }
        }
        *live = still;
    }
    
    /// 📑️ Drives `fillBuildTick` plus the isolated bounded fill job until the document holds `target` locked placements (or the budget runs out).
    pub async fn drive_fill_until_ready(app: &mut Puzzle3dApp, target: f64) -> f64 {
        let mut live = Vec::new();
        for _ in 0..256 {
            let result = dispatch(app, "fillBuildTick", None, None).await.expect("fillBuildTick");
            step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
            if fill_ready(app).await >= target {
                break;
            }
        }
        fill_ready(app).await
    }
    
    /// 🧵️ The retained command completes the bounded fill delta before publishing one atomic result.
    pub async fn finish_fill_count(_app: &mut Puzzle3dApp, _result: InvocationResult) -> (usize, std::time::Duration) {
        (0, std::time::Duration::ZERO)
    }
    
    pub async fn set_fill_count_and_finish(app: &mut Puzzle3dApp, value: u32, window_id: Option<&str>) -> (usize, std::time::Duration) {
        let result = dispatch(app, "setFillCount", Some(&json!({ "value": value })), window_id).await.expect("begin setFillCount");
        finish_fill_count(app, result).await
    }
    //#endregion 🔖️MeasureProbes
    
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
    
    /// 🧮️ Retained bytes in the whole process right now — the reading a growth law differences. Signed,
    /// because a `realloc` shrink and a free of memory allocated before this counter existed both
    /// legitimately push it down; a law reads DIFFERENCES of it, never its absolute value.
    pub fn retained_heap_bytes() -> isize {
        semio_framework_trace::retained_heap_bytes()
    }
    //#endregion 🧮️HeapWitness
}

use context::*;
use super::*;
/// 🧰️ The framework-injected utility verb: the editor module itself no longer names it, so `use super::*`
/// cannot carry it into the tests. Imported from its owner instead of relying on a re-export.
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;

//#region 🕹️LocalInteractionRead
/// 🚧️ Runaway guard for the host's own continuation drain. Far above the host's real 4096-turn
/// budget so a law that fails here fails on the state machine, never on the guard.
const LOCAL_INTERACTION_READ_TURNS: usize = 65_536;

/// 📤️ The exact frame admission `advance_typed_operation_output` hands
/// `publish_local_interaction_query_reply` every continuation turn.
const LOCAL_INTERACTION_READ_FRAMES: usize = 4;

/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: plays the HOST half of one local-interaction
/// read exactly as `plugin_continue_typed_operations` → `advance_typed_operation_output` does —
/// one `advance_typed_operation_publication` unit, then `publish_local_interaction_query_reply`
/// into the same fixed frame admission — and stops draining the moment
/// `has_runnable_typed_operations` reports quiescence. That quiescence point is the ONLY place a
/// page acknowledgement can arrive: the shell's `settlePluginTurn(drainOperations=true)` owns the
/// call until the actor stops reporting more-work, so a query that stays runnable while it waits
/// on the host never sees its own ACK. Returns the exact capture bytes the shell assembles.
async fn read_local_interaction(app: &mut Puzzle3dApp, request_id: u64) -> Vec<u8> {
    read_local_interaction_while_rendering(app, request_id, false).await
}

/// 🖼️ The same host loop with the shell's own render pressure optionally interleaved, because a boot
/// shell renders on every turn while it waits for its surfaces — so a read must terminate while the
/// app is also doing its ordinary per-turn work, not only on an otherwise idle instance.
async fn read_local_interaction_while_rendering(app: &mut Puzzle3dApp, request_id: u64, render: bool) -> Vec<u8> {
    use semio_framework_plugin::PluginApp;
    assert!(app.begin_local_interaction_query(request_id, request_id).is_none(), "a live instance must admit local interaction read {request_id}");
    let (mut capture, mut started, mut closed) = (Vec::new(), false, false);
    let mut awaiting_ack: Option<protocol::LocalInteractionQueryToken> = None;
    let mut turns = 0_usize;
    for _ in 0..LOCAL_INTERACTION_READ_TURNS {
        turns += 1;
        if !app.has_runnable_typed_operations() {
            let Some(token) = awaiting_ack.take() else { break };
            assert!(app.acknowledge_local_interaction_query(&token), "the page the app itself published must accept its own exact token");
            continue;
        }
        if render {
            drop(render_composite(app).await);
        }
        app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("the live-cleanup clock keeps ticking during a read");
        app.advance_typed_operation_publication().await.expect("advance one local interaction publication unit");
        let mut frames = Vec::new();
        app.publish_local_interaction_query_reply(&mut frames, LOCAL_INTERACTION_READ_FRAMES);
        for frame in &frames {
            let protocol::AppFrame::LocalInteractionQuery { reply } = protocol::decode_app_frame(frame).await.expect("a published local interaction frame decodes") else {
                panic!("local interaction publication emitted a foreign app frame");
            };
            match reply {
                protocol::LocalInteractionQueryReply::Started { .. } => started = true,
                protocol::LocalInteractionQueryReply::Page { page } => {
                    assert!(awaiting_ack.is_none(), "a second page was published while the first still awaited its acknowledgement");
                    capture.extend_from_slice(&page.bytes);
                    awaiting_ack = Some(protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal });
                }
                protocol::LocalInteractionQueryReply::Closed { cancelled, .. } => {
                    assert!(!cancelled, "an acknowledged read closes uncancelled");
                    closed = true;
                }
                other => panic!("unexpected local interaction reply: {other:?}"),
            }
        }
        if closed {
            break;
        }
    }
    eprintln!("[DEBUG] local interaction read {request_id} turns={turns} bytes={}", capture.len());
    assert!(started, "local interaction read {request_id} never published its Started reply");
    assert!(closed, "local interaction read {request_id} never terminated: the actor stayed runnable without ever publishing Closed, or quiesced with nothing left for the host to answer");
    capture
}

/// 🩺️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: reproduces the browser boot stall — the shell's
/// very first `readLocalInteraction` over a document with NO selection, driven through the exact
/// host continuation loop. The actor must reach `Closed` and hand its three captured roots back,
/// and it must never stay runnable while the only thing it waits for is a host acknowledgement the
/// drain cannot deliver.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_of_an_unselected_document_terminates() {
    let mut app = app().await;
    let capture = read_local_interaction(&mut app, 1).await;
    let capture: protocol::LocalInteractionCapture = protocol::json::from_json_str(std::str::from_utf8(&capture).expect("a capture is canonical UTF-8 JSON")).expect("a terminated read yields a whole capture");
    assert!(capture.state.selection.values().all(|selection| selection.ids.is_empty()), "this law's document carries no selection: {:?}", capture.state.selection);
}

/// 🩺️ The boot shape: the shell renders while it waits for its UI surfaces, so the read runs against
/// a busy instance rather than an idle one. A terminal reply withheld while unrelated app-owned
/// maintenance is in flight — while the query keeps reporting runnable — is the livelock the boot
/// hit, and it cannot be seen on an instance that does nothing else.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_terminates_under_concurrent_render_pressure() {
    let mut app = app().await;
    read_local_interaction_while_rendering(&mut app, 4, true).await;
}

/// 🩺️ The shell re-reads the local interaction on every refresh, so the SECOND and third reads of
/// one live instance must terminate exactly like the first — a store lease registry that still
/// reports a reclaimed slot, or a live slot that outlives its own terminal reply, strands them.
#[semio_framework_async_macros::async_test]
async fn repeated_local_interaction_reads_of_one_instance_terminate() {
    let mut app = app().await;
    for request_id in 1..=3 {
        read_local_interaction(&mut app, request_id).await;
    }
}

/// 🩺️ The same read over a LIVE selection: a non-empty capture spans many more fixed pages, so it
/// exercises every intermediate page's acknowledgement round trip as well as the terminal one.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_of_a_selected_document_terminates() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let capture = read_local_interaction(&mut app, 2).await;
    let capture: protocol::LocalInteractionCapture = protocol::json::from_json_str(std::str::from_utf8(&capture).expect("a capture is canonical UTF-8 JSON")).expect("a terminated read yields a whole capture");
    assert!(
        capture.state.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| selection.ids.iter().any(|id| id == &object_id)),
        "the capture carries the live selection: {:?}",
        capture.state.selection
    );
}

/// 🩺️ ticket 26/09/02/PUZZLE-3D-END-TO-END: reproduces `runtime live cleanup faulted for
/// instance 1` — the vortex-picking local interaction query is the only path by which the
/// puzzle3d document store's `snapshot_read()` lease is ever taken and returned, so it is
/// driven to completion here (Started → pages → acknowledge → Closed) exactly as the
/// host does every actor turn, then one plain `maintenance_step` must not fault.
#[semio_framework_async_macros::async_test]
async fn local_interaction_query_return_does_not_fault_the_next_maintenance_step() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    read_local_interaction(&mut app, 1).await;
    app.maintenance_step(1, 4096).expect("maintenance step after a returned snapshot read lease must not fault");
}
//#endregion 🕹️LocalInteractionRead

/// 📏️ Budget the TYPICAL cooperative-maintenance unit of one stage must respect. A quarter of
/// `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000us): this runs at opt-level 0 where
/// every unit is far slower than the release wasm the ceiling actually guards, so a native unit
/// that already eats a quarter of the ceiling is the defect, not the noise. Applied to the
/// per-stage median for the reason `MaintenanceStageBudget` states.
const MAINTENANCE_UNIT_BUDGET_US: u64 = 2_000;

/// 🔁️ Full round-robin sweeps driven after the example lands, so every fixed stage runs its own
/// bounded unit several times (first unit does the real work, later ones prove it stays terminal).
const MAINTENANCE_UNIT_SWEEPS: usize = 8;

/// 🎲️ Independent repetitions of the whole measured scenario, folded per stage by
/// `MaintenanceStageBudget::keep_best_round`: an intrinsically over-budget unit costs the same in
/// every round, while a scheduler hiccup on a loaded machine hits one stage in one round.
const MAINTENANCE_BUDGET_ROUNDS: usize = 3;

/// 🚧️ Runaway guard for the measured host-turn loop; the flagship example loads need thousands of
/// publication turns, so this only has to be far above them, never tight.
const MAINTENANCE_BUDGET_TURNS: usize = 1_048_576;

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave R2: the OS runtime's cooperative-maintenance clock
/// (`RuntimeLiveCleanupJob::step` → `maintenance_step(1, 4096)`) faults its instance with
/// `plugin.internal.interactive-ceiling` the moment one unit overruns
/// `semio_framework_trace::INTERACTIVE_STEP_CEILING_US`, which is exactly how the browser boot
/// died. This law drives the REAL app — the real typed `setActiveExample` command, the real store,
/// the real store-replacement/envelope registries and the real fixed round robin — over both
/// flagship documents, and holds every stage's typical unit inside [`MAINTENANCE_UNIT_BUDGET_US`],
/// naming the offending stage when it does not. The measured turn deliberately does NOT assert the
/// operation's own result lane: a domain command that faults is a different law's subject, while
/// the clock must stay inside its budget either way.
#[semio_framework_async_macros::async_test]
async fn every_maintenance_unit_stays_inside_the_interactive_step_budget() {
    for example in [PUZZLE3D_EXAMPLE_CONCRETE_FOREST, PUZZLE3D_EXAMPLE_NAKAGIN] {
        let mut best = MaintenanceStageBudget::default();
        for _ in 0..MAINTENANCE_BUDGET_ROUNDS {
            let mut app = app().await;
            let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": example })), None).expect("setActiveExample is a declared puzzle3d command");
            app.dispatch_typed(command, &meta("local")).await.expect("setActiveExample mints its retained whole-document operation");
            for _ in 0..MAINTENANCE_BUDGET_TURNS {
                if !app.has_pending_typed_operations() {
                    break;
                }
                measured_host_turn(&mut app).await;
            }
            for _ in 0..MAINTENANCE_UNIT_SWEEPS * semio_framework_plugin::MAINTENANCE_STAGES as usize {
                app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("a live-cleanup maintenance unit never faults");
            }
            best.keep_best_round(&app.maintenance);
        }
        let (typical_stage, median_us) = best.worst();
        let (peak_stage, peak_us) = best.worst_unit();
        eprintln!("[DEBUG] maintenance budget example={example} typical_stage={typical_stage} median_us={median_us} peak_stage={peak_stage} peak_us={peak_us} breakdown={}", best.report());
        assert!(median_us <= MAINTENANCE_UNIT_BUDGET_US, "{example}: maintenance stage {typical_stage}'s typical unit cost {median_us}us in every round, over the {MAINTENANCE_UNIT_BUDGET_US}us typical-unit budget (per-stage median/worst/units: {})", best.report());
        assert!(u128::from(peak_us) < PUZZLE3D_INTERACTIVE_STEP_CEILING.as_micros(), "{example}: maintenance stage {peak_stage} ran one unit for {peak_us}us in every round, at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?} (per-stage median/worst/units: {})", best.report());
    }
}

/// 🔁️ One host actor turn with its cooperative-maintenance unit measured: the same
/// `maintenance_step` → `advance_typed_operation_publication` → present/ACK → drain shape the
/// runtime drives, minus any assertion on the operation's own outcome lane.
async fn measured_host_turn(app: &mut Puzzle3dApp) {
    app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("a live-cleanup maintenance unit never faults");
    app.advance_typed_operation_publication().await.expect("advance one typed operation publication unit");
    if let Some(page) = app.take_typed_operation_result_page(FIXTURE_INSTANCE_ID) {
        assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge one presented result page"), "the app's own presented result page must accept its exact token");
    }
    drop(app.take_typed_operation_effect());
    drop(app.take_typed_operation_event());
    drop(app.take_typed_operation_completion().await.expect("take one typed operation completion witness"));
    drop(app.take_typed_operation_ui_scope());
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            assert!(app.acknowledge_local_interaction_query(&token), "the app's own local-interaction page must accept its exact token");
        }
    }
}

#[test]
fn retained_publication_contracts_are_an_exact_nonempty_tool_bijection() {
    let fixture: Value = parse(include_str!("../../../🧫️fixtures/🗄️retained-jobs/🔣️.json")).expect("Puzzle3D retained route fixture");
    assert_eq!(fixture.get("toolIds"), Some(&Value::Array(PUZZLE3D_RETAINED_TOOL_IDS.iter().map(|id| Value::from(*id)).collect())));
    let manifest = create_puzzle3d_app();
    for tool_id in PUZZLE3D_RETAINED_TOOL_IDS {
        let actions = manifest.window_kinds.iter().flat_map(|window| &window.actions).filter(|action| action.id == *tool_id).collect::<Vec<_>>();
        assert_eq!(actions.len(), 1, "{tool_id} requires exactly one manifest declaration");
        assert_eq!(actions[0].semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{tool_id}");
    }
    let exact = |contracts: &[ArtifactToolPublicationContract]| {
        let ids = contracts.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>();
        ids == PUZZLE3D_RETAINED_TOOL_IDS.iter().copied().collect()
            && ids.len() == contracts.len()
            && contracts.iter().all(|contract| !contract.lanes.is_empty() && (!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1))
    };
    let contracts = <Puzzle3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert!(exact(contracts));
    assert!(!exact(&contracts[..contracts.len() - 1]));
    let mut duplicate = contracts.to_vec();
    let copied = duplicate[1];
    duplicate[0] = copied;
    assert!(!exact(&duplicate));
}

#[test]
fn retained_command_catalog_excludes_framework_owned_shared_actions() {
    assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_TOOL_ACTION_ID));
    assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
}

fn suggestion_and_precompute_routes_are_cursorized(source: &str) -> bool {
    [
        r#""acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default())"#,
        r#"| "suggestionsTick""#,
        "=> Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))",
        "Puzzle3dAcceptSuggestionStage::Target",
        "Puzzle3dAcceptSuggestionStage::Candidate",
        "Puzzle3dAcceptSuggestionStage::Representation",
        "Puzzle3dAcceptSuggestionStage::Vortices",
        "Puzzle3dAcceptSuggestionStage::ExistingAttractions",
        "Puzzle3dAcceptSuggestionStage::PublishObject",
        "Puzzle3dAcceptSuggestionStage::PublishAttraction",
        "Puzzle3dPrecomputeCommandStage::Objects",
        "Puzzle3dPrecomputeCommandStage::Vortices",
        "Puzzle3dPrecomputeCommandStage::Attractions",
        "Puzzle3dPrecomputeCommandStage::CatalogObjects",
        "Puzzle3dPrecomputeCommandStage::CatalogVortices",
        "Puzzle3dPrecomputeCommandStage::Positions",
        "Puzzle3dPrecomputeCommandStage::Indices",
        "PUZZLE3D_MESH_PAGE_SCAN_CHARS",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains(r#""acceptSuggestion" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""cycleBrushCandidate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""fillBuildTick" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""registerBrushMesh" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""setFillCount" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""suggestionsTick" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn suggestion_and_precompute_hostile_static_law_rejects_one_grant_reducers_and_missing_boundaries() {
    let source = include_str!("../../🦀️.rs");
    assert!(suggestion_and_precompute_routes_are_cursorized(source));
    let direct_accept = source.replace(
        r#""acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default())"#,
        r#""acceptSuggestion" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!suggestion_and_precompute_routes_are_cursorized(&direct_accept));
    let direct_precompute = source.replace("=> Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))", "=> Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))");
    assert!(!suggestion_and_precompute_routes_are_cursorized(&direct_precompute));
    for marker in [
        "Puzzle3dAcceptSuggestionStage::Target",
        "Puzzle3dAcceptSuggestionStage::Representation",
        "Puzzle3dAcceptSuggestionStage::Vortices",
        "Puzzle3dAcceptSuggestionStage::ExistingAttractions",
        "Puzzle3dAcceptSuggestionStage::PublishObject",
        "Puzzle3dAcceptSuggestionStage::PublishAttraction",
        "Puzzle3dPrecomputeCommandStage::Objects",
        "Puzzle3dPrecomputeCommandStage::Vortices",
        "Puzzle3dPrecomputeCommandStage::Attractions",
        "Puzzle3dPrecomputeCommandStage::CatalogObjects",
        "Puzzle3dPrecomputeCommandStage::CatalogVortices",
        "Puzzle3dPrecomputeCommandStage::Positions",
        "Puzzle3dPrecomputeCommandStage::Indices",
        "PUZZLE3D_MESH_PAGE_SCAN_CHARS",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ] {
        assert!(!suggestion_and_precompute_routes_are_cursorized(&source.replace(marker, "cursor-removed")), "missing retained boundary was falsely accepted: {marker}");
    }
}

fn selection_transforms_are_cursorized(source: &str) -> bool {
    source.contains("\"translateSelection\" | \"rotateSelection\" | \"scaleSelection\" => Box::new(Puzzle3dScaleWork::new(tool_id))")
        && source.contains("Puzzle3dScaleStage::ObjectSelection")
        && source.contains("Puzzle3dScaleStage::VolumeSelection")
        && source.contains("Puzzle3dScaleStage::Objects")
        && source.contains("Puzzle3dScaleStage::Volumes")
        && !source.contains("\"translateSelection\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork")
        && !source.contains("\"rotateSelection\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork")
}

#[test]
fn selection_transform_hostile_static_law_rejects_one_grant_reducers_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(selection_transforms_are_cursorized(source));
    let direct = source.replace(
        "\"translateSelection\" | \"rotateSelection\" | \"scaleSelection\" => Box::new(Puzzle3dScaleWork::new(tool_id))",
        "\"translateSelection\" | \"rotateSelection\" | \"scaleSelection\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))",
    );
    assert!(!selection_transforms_are_cursorized(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["Puzzle3dScaleStage::ObjectSelection", "Puzzle3dScaleStage::VolumeSelection", "Puzzle3dScaleStage::Objects", "Puzzle3dScaleStage::Volumes"] {
        assert!(!selection_transforms_are_cursorized(&source.replace(marker, "cursor-removed")), "missing transform cursor was falsely accepted: {marker}");
    }
}

fn focus_selection_is_cursorized(source: &str) -> bool {
    source.contains(r#""focusSelection" => Box::new(Puzzle3dFocusSelectionWork::default())"#)
        && source.contains("Puzzle3dFocusSelectionStage::Selection")
        && source.contains("Puzzle3dFocusSelectionStage::SumObjects")
        && source.contains("Puzzle3dFocusSelectionStage::DistanceObjects")
        && source.contains("Puzzle3dFocusSelectionStage::Publish")
        && !source.contains(r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn focus_selection_hostile_static_law_rejects_hidden_whole_collection_work() {
    let source = include_str!("../../🦀️.rs");
    assert!(focus_selection_is_cursorized(source));
    let direct = source
        .replace(r#""focusSelection" => Box::new(Puzzle3dFocusSelectionWork::default())"#, r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!focus_selection_is_cursorized(&direct));
    for marker in ["Puzzle3dFocusSelectionStage::Selection", "Puzzle3dFocusSelectionStage::SumObjects", "Puzzle3dFocusSelectionStage::DistanceObjects", "Puzzle3dFocusSelectionStage::Publish"] {
        assert!(!focus_selection_is_cursorized(&source.replace(marker, "cursor-removed")), "missing focus cursor was falsely accepted: {marker}");
    }
}

fn patch_inspector_is_cursorized(source: &str) -> bool {
    source.contains(r#""patchInspector" => Box::new(Puzzle3dPatchInspectorWork::default())"#)
        && source.contains("Puzzle3dPatchInspectorStage::Selection")
        && source.contains("Puzzle3dPatchInspectorStage::Objects")
        && source.contains("Puzzle3dPatchInspectorStage::Vortices")
        && source.contains("Puzzle3dPatchInspectorStage::Attractions")
        && source.contains("Puzzle3dPatchInspectorStage::AttractionReconnect")
        && source.contains("Puzzle3dPatchInspectorStage::References")
        && source.contains("Puzzle3dPatchInspectorStage::Volumes")
        && !source.contains(r#""patchInspector" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn patch_inspector_hostile_static_law_rejects_old_reducer_and_hidden_collection_loops() {
    let source = include_str!("../../🦀️.rs");
    assert!(patch_inspector_is_cursorized(source));
    let direct = source
        .replace(r#""patchInspector" => Box::new(Puzzle3dPatchInspectorWork::default())"#, r#""patchInspector" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!patch_inspector_is_cursorized(&direct));
    for marker in [
        "Puzzle3dPatchInspectorStage::Selection",
        "Puzzle3dPatchInspectorStage::Objects",
        "Puzzle3dPatchInspectorStage::Vortices",
        "Puzzle3dPatchInspectorStage::Attractions",
        "Puzzle3dPatchInspectorStage::AttractionReconnect",
        "Puzzle3dPatchInspectorStage::References",
        "Puzzle3dPatchInspectorStage::Volumes",
    ] {
        assert!(!patch_inspector_is_cursorized(&source.replace(marker, "cursor-removed")), "missing inspector cursor was falsely accepted: {marker}");
    }
}

fn world_relocate_is_cursorized(source: &str) -> bool {
    source.contains(r#""worldRelocate" => Box::new(Puzzle3dWorldRelocateWork::default())"#)
        && source.contains("Puzzle3dWorldRelocateStage::Object")
        && source.contains("Puzzle3dWorldRelocateStage::ExistingAttractions")
        && source.contains("Puzzle3dWorldRelocateStage::CandidateObject")
        && source.contains("Puzzle3dWorldRelocateStage::CandidateVortex")
        && source.contains("Puzzle3dWorldRelocateStage::PublishAttraction")
        && source.contains("PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT")
        && !source.contains(r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn world_relocate_hostile_static_law_rejects_whole_proximity_scans() {
    let source = include_str!("../../🦀️.rs");
    assert!(world_relocate_is_cursorized(source));
    let direct =
        source.replace(r#""worldRelocate" => Box::new(Puzzle3dWorldRelocateWork::default())"#, r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!world_relocate_is_cursorized(&direct));
    for marker in [
        "Puzzle3dWorldRelocateStage::Object",
        "Puzzle3dWorldRelocateStage::ExistingAttractions",
        "Puzzle3dWorldRelocateStage::CandidateObject",
        "Puzzle3dWorldRelocateStage::CandidateVortex",
        "Puzzle3dWorldRelocateStage::PublishAttraction",
        "PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT",
    ] {
        assert!(!world_relocate_is_cursorized(&source.replace(marker, "cursor-removed")), "missing relocate cursor was falsely accepted: {marker}");
    }
}

fn create_attraction_is_cursorized(source: &str) -> bool {
    source.contains(r#""createAttraction" => Box::new(Puzzle3dCreateAttractionWork::default())"#)
        && source.contains("Puzzle3dCreateAttractionStage::Existing")
        && source.contains("Puzzle3dCreateAttractionStage::Attracting")
        && source.contains("Puzzle3dCreateAttractionStage::Attracted")
        && source.contains("Puzzle3dCreateAttractionStage::Compatibility")
        && source.contains("Puzzle3dCreateAttractionStage::Publish")
        && !source.contains(r#""createAttraction" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn create_attraction_hostile_static_law_rejects_nested_whole_scans() {
    let source = include_str!("../../🦀️.rs");
    assert!(create_attraction_is_cursorized(source));
    let direct = source.replace(
        r#""createAttraction" => Box::new(Puzzle3dCreateAttractionWork::default())"#,
        r#""createAttraction" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!create_attraction_is_cursorized(&direct));
    for marker in ["Puzzle3dCreateAttractionStage::Existing", "Puzzle3dCreateAttractionStage::Attracting", "Puzzle3dCreateAttractionStage::Attracted", "Puzzle3dCreateAttractionStage::Compatibility", "Puzzle3dCreateAttractionStage::Publish"] {
        assert!(!create_attraction_is_cursorized(&source.replace(marker, "cursor-removed")), "missing attraction cursor was falsely accepted: {marker}");
    }
}

fn set_active_example_is_cursorized(source: &str) -> bool {
    source.contains(r#""setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default())"#)
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteAttractions")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteObjects")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteVolumes")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteReferences")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteCompatibility")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateObjects")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateAttractions")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateVolumes")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateReferences")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateCompatibility")
        && source.contains("Puzzle3dSetActiveExampleStage::Publish")
        && !source.contains(r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn set_active_example_hostile_static_law_rejects_whole_document_reset() {
    let source = include_str!("../../🦀️.rs");
    assert!(set_active_example_is_cursorized(source));
    let direct = source.replace(
        r#""setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default())"#,
        r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!set_active_example_is_cursorized(&direct));
    for marker in [
        "Puzzle3dSetActiveExampleStage::DeleteAttractions",
        "Puzzle3dSetActiveExampleStage::DeleteObjects",
        "Puzzle3dSetActiveExampleStage::DeleteVolumes",
        "Puzzle3dSetActiveExampleStage::DeleteReferences",
        "Puzzle3dSetActiveExampleStage::DeleteCompatibility",
        "Puzzle3dSetActiveExampleStage::CreateObjects",
        "Puzzle3dSetActiveExampleStage::CreateAttractions",
        "Puzzle3dSetActiveExampleStage::CreateVolumes",
        "Puzzle3dSetActiveExampleStage::CreateReferences",
        "Puzzle3dSetActiveExampleStage::CreateCompatibility",
        "Puzzle3dSetActiveExampleStage::Publish",
    ] {
        assert!(!set_active_example_is_cursorized(&source.replace(marker, "cursor-removed")), "missing example cursor was falsely accepted: {marker}");
    }
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dWorldRelocateWork::extent` used to charge
/// every object a flat `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT` (64) regardless of its real vortex
/// count, so Nakagin's 180 objects (358 real vortex instances) computed `objects * 66 + attractions`
/// = 11,880 and faulted preflight with "puzzle command exceeds fixed semantic work capacity" before
/// `step()` ever ran. The rewritten bound counts the document's actual vortices instead.
#[test]
fn world_relocate_extent_fits_within_cap_for_nakagin() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "nonexistent", "position": [0.0, 0.0, 0.0] })), None).expect("worldRelocate command decodes");
    let work = Puzzle3dWorldRelocateWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "worldRelocate extent {extent} must not exceed the fixed cap {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): the extent test above only
/// proves the bound is REALISTIC (fits under the cap) — it says nothing about whether the bound is
/// SOUND (real `step()` calls never outrun it). This drives the actual `CandidateVortex ⇄
/// PublishAttraction` cycle (the genuine ping-pong at editor `🦀️.rs` `Puzzle3dWorldRelocateStage::
/// CandidateVortex`/`PublishAttraction`) against a real Nakagin joint: object
/// `25b0dba0-8f81-423a-94a1-b911a6031010` ("Capsule With Balcony Backslash") is "relocated" to its
/// own current origin — a real, non-degenerate command, not a synthetic no-op — and its one vortex
/// (`…:link`, kind "door capsule right") sits, by the fixture's own real assembled geometry
/// (independently recomputed here with the same `quat_rotate_vector` Hamilton-product formula
/// `step()` uses), within `proximity_radius` (0.75, `default_proximity_radius()`) of four
/// "door tambour right" vortices already on neighbour object `5f0266bc-…`. That forces the
/// `PublishAttraction` loop-back more than once, closing the gap a trivial early-exit run leaves
/// open, and empirically exercises the `candidate_scan_stage`'s `object_vortices * 2` term this
/// ticket introduced.
#[test]
fn world_relocate_step_loop_stays_within_its_own_extent_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "25b0dba0-8f81-423a-94a1-b911a6031010", "position": [-8.85, -2.8499999999999996, 7.7] })), None).expect("worldRelocate command decodes");
    let mut work = Puzzle3dWorldRelocateWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "worldRelocate step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(iterations <= extent, "worldRelocate step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(
        iterations > 100,
        "worldRelocate's CandidateObject/CandidateVortex stages unconditionally walk every real object and vortex, so a real Nakagin run must take hundreds of steps, not a trivial early exit; observed only {iterations} iterations"
    );
    assert!(
        emit.artifact_mutations.len() >= 2,
        "the relocated vortex sits within proximity_radius of a real neighbour already present in the Nakagin fixture, so PublishAttraction must fire at least once beyond the move itself; observed {} mutations",
        emit.artifact_mutations.len()
    );
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dCreateAttractionWork::extent` doubled the
/// same flat per-object 64-vortex charge across two endpoint scans, computing `objects * 128 + ...`
/// = 23,055 on Nakagin's 180 objects — nearly 6x the cap. The rewritten bound counts the document's
/// actual vortices instead of assuming every object carries the worst-case vortex count.
#[test]
fn create_attraction_extent_fits_within_cap_for_nakagin() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("createAttraction", Some(json!({ "attracting": "nonexistent-a", "attracted": "nonexistent-b" })), None).expect("createAttraction command decodes");
    let work = Puzzle3dCreateAttractionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "createAttraction extent {extent} must not exceed the fixed cap {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): drives the real
/// `Attracting`/`Attracted` full-document vortex scans instead of only checking the bound fits
/// under the cap. `attracting`/`attracted` name a genuine compatible pair already present in the
/// Nakagin fixture — `25b0dba0-…:link` (kind "door capsule right", object index 27 in DSL
/// declaration order) and `5f0266bc-…:sl0_d0` (kind "door tambour right", object index 64) — which
/// `kind-compatibility` marks bidirectionally compatible, so `step()` runs every real stage
/// (`Existing → Attracting → Attracted → Compatibility → Publish`) to a genuine success instead of
/// one of the Work's several early-`Complete` short-circuits (duplicate/incompatible/empty-id).
#[test]
fn create_attraction_step_loop_stays_within_its_own_extent_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command =
        Puzzle3dCommand::from_action("createAttraction", Some(json!({ "attracting": "25b0dba0-8f81-423a-94a1-b911a6031010:link", "attracted": "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952:sl0_d0" })), None).expect("createAttraction command decodes");
    let mut work = Puzzle3dCreateAttractionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "createAttraction step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(iterations <= extent, "createAttraction step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(iterations > 50, "createAttraction must scan real objects and vortices to find both endpoints, not take a trivial early exit; observed only {iterations} iterations");
    assert_eq!(
        emit.artifact_mutations.len(),
        1,
        "a real, compatible, non-duplicate attracting/attracted pair must reach Publish and emit exactly one connect_vortices mutation, not one of the early-Complete short-circuits; observed {}",
        emit.artifact_mutations.len()
    );
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dAcceptSuggestionWork::extent`'s
/// `target_scans` term charged every scene object the same flat 64-vortex worst case
/// (`objects * 64`), computing 7,888+ on Nakagin's 180 objects. The rewritten bound counts the
/// document's actual vortices for the target scan while keeping the catalog-kind-cap terms
/// (`PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT`), which bound a real runtime-enforced invariant on
/// catalog kinds, not scene object instances.
#[test]
fn accept_suggestion_extent_fits_within_cap_for_nakagin() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", None, None).expect("acceptSuggestion command decodes");
    let work = Puzzle3dAcceptSuggestionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's catalogs and real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "acceptSuggestion extent {extent} must not exceed the fixed cap {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): drives the real `Target`
/// scan (the term this ticket rewrote) plus the full `Candidate → Representation → Vortices →
/// ExistingAttractions → PublishObject → PublishAttraction → PublishResult` chain to a genuine
/// success, using a real Nakagin vortex (`25b0dba0-…:link`, object index 27 in DSL declaration
/// order) as `fullId` instead of the "no target requested" early-`Complete` short-circuit.
#[test]
fn accept_suggestion_step_loop_stays_within_its_own_extent_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), None).expect("acceptSuggestion command decodes");
    let mut work = Puzzle3dAcceptSuggestionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's catalogs and real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "acceptSuggestion step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(iterations <= extent, "acceptSuggestion step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(iterations > 20, "acceptSuggestion must scan real objects to find the requested target vortex, not take a trivial early exit; observed only {iterations} iterations");
    assert_eq!(
        emit.artifact_mutations.len(),
        2,
        "a real target vortex and a real catalog kind must reach PublishResult and emit exactly the created object plus its connecting attraction, not one of the early-Complete short-circuits; observed {}",
        emit.artifact_mutations.len()
    );
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dPatchInspectorWork::extent`'s `"vortex"`
/// arm charged `objects * PUZZLE_COMMAND_DECODED_ITEMS` (objects * 512) regardless of real vortex
/// count, computing ~92,160 on Nakagin's 180 objects — a 512x-over-budget design error, not a
/// precision edge case. The rewritten bound counts the document's actual vortex instances plus one
/// `step()` per object, matching the `Vortices` stage's real per-object "owner advance" call.
#[test]
fn patch_inspector_vortex_extent_fits_within_cap_for_nakagin() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("patchInspector", Some(json!({ "entity": "vortex" })), None).expect("patchInspector command decodes");
    let work = Puzzle3dPatchInspectorWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "patchInspector vortex extent {extent} must not exceed the fixed cap {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): the `Vortices` stage walks
/// every object and every real vortex UNCONDITIONALLY (selection only gates whether a visited
/// vortex is mutated, never whether it is visited), so this is a genuine full-document run by
/// construction, not a trivial early exit — selecting one real vortex
/// (`25b0dba0-…:link`) and setting `field: "hidden"` proves the walk both completes within its own
/// extent AND actually mutates the one entity that was selected out of the full scan.
///
/// Writing this test surfaced a real off-by-one in `extent()`'s own `Selection` accounting: the
/// `Selection` stage's own exhaustion call (`source_id` returning `None`) is a real `step()` call
/// that returns `Progress` — a `+1` distinct from each entity arm's own terminal call (which
/// returns `Complete` instead, contributing zero to the `Progress` count `work_cursor` tracks). The
/// old `let items = source.checked_add(scan)?;` omitted that `Selection`-stage `+1` entirely; fixed
/// to `let items = source.checked_add(scan)?.checked_add(1)?;`, which this test's `iterations <=
/// extent` assertion is what actually catches — the extent-only test above could not have (it
/// never drives the real loop, so no test previously computed the true `Progress`-call count).
#[test]
fn patch_inspector_vortex_step_loop_stays_within_its_own_extent_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("patchInspector", Some(json!({ "entity": "vortex", "field": "hidden", "value": true, "ids": ["25b0dba0-8f81-423a-94a1-b911a6031010:link"] })), None).expect("patchInspector command decodes");
    let mut work = Puzzle3dPatchInspectorWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "patchInspector step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(iterations <= extent, "patchInspector step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(
        iterations > 400,
        "patchInspector's Vortices stage unconditionally walks every object and every real vortex regardless of selection, so a real Nakagin run must take hundreds of steps, not a trivial early exit; observed only {iterations} iterations"
    );
    assert_eq!(emit.artifact_mutations.len(), 1, "exactly the one selected vortex must be patched out of the full unconditional scan; observed {} mutations", emit.artifact_mutations.len());
}

/// 🧵️ ticket 26/09/02/PUZZLE-3D-END-TO-END: direct functional proof that the previously-dead
/// `Puzzle3dSetActiveExampleWork` state machine actually walks its own stages across MULTIPLE
/// bounded `step()` calls for a real fixture (not a single-shot reducer), accumulating exactly
/// one mutation per deleted/created item and only publishing them all in the final `Complete`
/// emit — the same turn-by-turn contract `crate::retained_command::RetainedPuzzleCommandJob`
/// relies on when it drives an app-owned `PuzzleCommandWork` (`🎮️commands/🧵️retained/🦀️.rs`,
/// `PuzzleCommandPhase::Work`). The hostile-static-law test above only proves this arm's source
/// text exists; this proves it runs.
#[test]
fn set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample command decodes");
    let mut work = Puzzle3dSetActiveExampleWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin example fits the fixed bounded work envelope");
    assert!(extent > 4, "nakagin example must require more than the handful of fixed transition steps alone");
    let mut progress_steps = 0usize;
    let emit = loop {
        assert!(progress_steps <= extent + 8, "setActiveExample work did not reach Complete within its own declared extent");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => progress_steps += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(progress_steps > 1, "setActiveExample must require multiple bounded step() calls for the nakagin example, not a single-shot reducer; observed {progress_steps}");
    assert!(emit.artifact_mutations.len() > 1, "the completed emit must carry every incrementally-collected mutation, one per deleted/created item; observed {}", emit.artifact_mutations.len());
    // 🏷️ Example loading publishes exactly ONE config row, and it moves exactly ONE field: the id of
    // the example just loaded, which is what `export_fixture` names its download after. The user's own
    // shared preferences (fill count, overlap budget, kind weights) ride through untouched.
    let Some(Puzzle3dConfigMutation::Snapshot { config: published }) = emit.config_mutations.first().cloned() else {
        panic!("example loading must stamp the active example id on the shared config: {:?}", emit.config_mutations);
    };
    assert_eq!(emit.config_mutations.len(), 1, "example loading publishes one config row, not many");
    assert_eq!(published.active_example_id, PUZZLE3D_EXAMPLE_NAKAGIN, "the stamped id must be the example that was loaded");
    assert_eq!(Puzzle3dConfig { active_example_id: config.active_example_id.clone(), ..published }, config, "example loading leaves shared app preferences untouched");
}

/// 🧲️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave T: the gumball bracket is an honest `Migrated`
/// `HostOnly` pair. `World3dHost` (`🧰️framework/…/🌐️World3dHost/🟦️.tsx:4718`) dispatches
/// `transformBegin` on drag start, ONE absolute start→end `translateSelection`/`rotateSelection`/
/// `scaleSelection` delta on drag end, then `transformEnd`; mid-drag ticks never leave the host. So
/// both brackets carry no document or config transition of their own and complete empty on
/// `NoopPuzzleCommandWork` — but they must still be `Migrated`, or `validate_ui_dispatch_classification`
/// (`🧰️framework/…/🔌️plugin/🦀️.rs`) rejects every real drag with `interactive-job.not-ui-safe`.
#[test]
fn transform_brackets_are_migrated_host_only_routes_that_complete_empty() {
    let manifest = create_puzzle3d_app();
    let contracts = <Puzzle3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    for action in ["transformBegin", "transformEnd"] {
        assert!(PUZZLE3D_RETAINED_TOOL_IDS.contains(&action), "{action} must be a retained tool id or no tool job is ever built for it");
        let declarations = manifest.window_kinds.iter().flat_map(|window| &window.actions).filter(|declared| declared.id == action).collect::<Vec<_>>();
        assert_eq!(declarations.len(), 1, "{action} requires exactly one manifest declaration");
        assert_eq!(declarations[0].semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{action} must pass the UI dispatch gate");
        let contract = contracts.iter().find(|contract| contract.tool_id == action).unwrap_or_else(|| panic!("{action} needs a publication contract"));
        assert_eq!(contract.lanes, &[ArtifactToolPublicationLane::HostOnly], "{action} publishes nothing: the drag itself is one absolute delta on another route");
        let command = Puzzle3dCommand::from_action(action, None, None).expect("command decodes");
        let emit = puzzle3d_retained_reduce(&command, &snapshot, &config, &interaction, &hover, None).expect("real dispatch");
        assert!(emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.effects.is_empty(), "{action} must stay a true no-op; got {}/{}/{}", emit.artifact_mutations.len(), emit.config_mutations.len(), emit.effects.len());
    }
}

/// 🌉️ ticket 26/09/02/PUZZLE-3D-END-TO-END: exercises the wiring this ticket adds — the
/// classification flip to `Migrated`, `PUZZLE3D_RETAINED_TOOL_IDS` registration, the
/// `ArtifactToolPublicationContract` for the Artifact+Config lanes, and
/// `Puzzle3dArtifactStorePreparationFactory` — by dispatching `setActiveExample` through the
/// real `InteractiveJob`/tool-job path (`Puzzle3dRetainedCommandJobFactory` ->
/// `RetainedPuzzleCommandJob` -> `ArtifactToolCompletion` -> the shared publication loop's
/// `self.store.begin_apply_batch(..., self.artifact_one_item_factory.as_ref())`), driving the
/// resulting typed operation to completion via repeated `maintenance_step` turns exactly as a
/// real host does every actor tick, then asserting the document was actually swapped. Uses
/// the registry-backed, instance-bound `app()`: this plugin declares
/// `bounded_first_step_tool_proofs!`, so the bare registry-less `artifact_app_laws::new_app` faults closed
/// with `interactive-job.catalog-authority` before any dispatch is even attempted.
#[semio_framework_async_macros::async_test]
async fn set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    let before_first_id = first_object_id(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("dispatch setActiveExample through the migrated tool-job path");
    let mut ticks = 0usize;
    while ticks < 5_000 && (object_count(&app) == 0 || first_object_id(&app) == before_first_id) {
        app.maintenance_step(1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
        ticks += 1;
    }
    assert!(object_count(&app) > 0, "nakagin document did not land after {ticks} maintenance turns");
    assert_ne!(first_object_id(&app), before_first_id, "setActiveExample did not actually swap the document's objects through the tool-job path");
}

/// 🌉️ ticket 26/09/02/PUZZLE-3D-END-TO-END: exercises the wiring this ticket adds for
/// `setFillCount` — the classification flip to `Migrated`, `PUZZLE3D_RETAINED_TOOL_IDS`
/// registration, and the `ArtifactToolPublicationContract` Config lane — by dispatching
/// `setFillCount` through the real `InteractiveJob`/tool-job path
/// (`Puzzle3dRetainedCommandJobFactory` -> `RetainedPuzzleCommandJob` ->
/// `Puzzle3dPrecomputeCommandWork`'s bounded `step()` cursor -> the config-store publication
/// loop), driving the resulting typed operation to completion via repeated `maintenance_step`
/// turns exactly as a real host does every actor tick, then asserting the fill tool's own count
/// slider actually observed the requested target land in the live config (`SetFillRequest`'s
/// `next.fill_count = *count`, `✏️s/…/🎚️config/🦀️.rs:539`). Uses the registry-backed,
/// instance-bound `app()`: this plugin declares `bounded_first_step_tool_proofs!`, so the bare
/// registry-less `artifact_app_laws::new_app` faults closed with `interactive-job.catalog-authority` before
/// any dispatch is even attempted. Deliberately reads only the config-side slider measure, not
/// the document — `fillBuildTick` (unmigrated; see its own blocker note in
/// `🔏️publication-authority/🔣️.json`) is what materializes actual fill objects, not this tool.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count() {
    use semio_framework_plugin::PluginApp;
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let ready = drive_fill_until_ready(&mut app, 3.0).await;
    assert!(ready >= 3.0, "the tool-job path needs a planned prefix before setFillCount can publish 3");
    async fn fill_count_slider(app: &mut Puzzle3dApp) -> Option<f64> {
        let view = app.window_view(main::WINDOW_KIND_ID);
        let measures = app.tool_measures(&view).await;
        find_measure_number(measures.get(fill_tool::TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count")
    }
    assert_eq!(fill_count_slider(&mut app).await, Some(0.0), "fill count starts at zero before any request");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).await.expect("dispatch setFillCount through the migrated tool-job path");
    let mut ticks = 0usize;
    while ticks < 5_000 && fill_count_slider(&mut app).await != Some(3.0) {
        app.maintenance_step(1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
        ticks += 1;
    }
    assert_eq!(fill_count_slider(&mut app).await, Some(3.0), "setFillCount did not update the live config's requested fill count through the tool-job path after {ticks} maintenance turns");
}

/// 📏️ Sizes this artifact's three reserved refresh sections against the retained section carrier that
/// now publishes them (`semio_framework_plugin::section_component_tree`): the engagements, window
/// measure and tool measure maps must all admit into the bounded chunk tree and round-trip
/// byte-exactly, with the fill tool's count slider present in the tool payload.
#[semio_framework_async_macros::async_test]
async fn reserved_refresh_section_payloads_admit_into_the_retained_section_carrier() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    let view = semio_framework_plugin::ViewModel { window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }], ..Default::default() };
    let payloads = [
        (semio_framework_plugin::UiRefreshSection::Engagements, serde_json::to_string(&app.window_engagements(&view).await).expect("engagements serialize")),
        (semio_framework_plugin::UiRefreshSection::Measures, serde_json::to_string(&app.window_measures(&view).await).expect("measures serialize")),
        (semio_framework_plugin::UiRefreshSection::Tools, serde_json::to_string(&app.tool_measures(&view).await).expect("tool measures serialize")),
    ];
    assert!(payloads[2].1.contains("puzzle3d-fill-count"), "the fill tool must contribute its count slider to the tools section");
    assert!(payloads[1].1.contains(main::WINDOW_KIND_ID), "the main window instance must key its own entry in the measures section");
    for (section, payload) in payloads {
        let tree = semio_framework_plugin::section_component_tree(section, &payload).expect("reserved section payload admits into the bounded carrier");
        let projected: serde_json::Value = serde_json::from_str(&semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("carrier projects")).expect("carrier projection is json");
        assert_eq!(projected["key"], section.body_key());
        assert_eq!(semio_framework_plugin::artifact_app_laws::fixture_carrier_text(&projected), payload, "{} carrier must round-trip byte-exactly", section.key());
        eprintln!("[DEBUG] puzzle3d {} section payload is {} bytes", section.key(), payload.len());
    }
}

fn add_brush_object_is_cursorized(source: &str) -> bool {
    source.contains(r#""addBrushObject" => Box::new(Puzzle3dAddBrushObjectWork::default())"#)
        && source.contains("Puzzle3dAddBrushObjectStage::Decode")
        && source.contains("Puzzle3dAddBrushObjectStage::Kind")
        && source.contains("Puzzle3dAddBrushObjectStage::Representation")
        && source.contains("Puzzle3dAddBrushObjectStage::Vortices")
        && source.contains("Puzzle3dAddBrushObjectStage::ExistingAttractions")
        && source.contains("Puzzle3dAddBrushObjectStage::PublishObject")
        && source.contains("Puzzle3dAddBrushObjectStage::PublishAttraction")
        && !source.contains(r#""addBrushObject" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_brush_object_hostile_static_law_rejects_engine_run_to_completion() {
    let source = include_str!("../../🦀️.rs");
    assert!(add_brush_object_is_cursorized(source));
    let direct = source
        .replace(r#""addBrushObject" => Box::new(Puzzle3dAddBrushObjectWork::default())"#, r#""addBrushObject" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!add_brush_object_is_cursorized(&direct));
    for marker in [
        "Puzzle3dAddBrushObjectStage::Decode",
        "Puzzle3dAddBrushObjectStage::Kind",
        "Puzzle3dAddBrushObjectStage::Representation",
        "Puzzle3dAddBrushObjectStage::Vortices",
        "Puzzle3dAddBrushObjectStage::ExistingAttractions",
        "Puzzle3dAddBrushObjectStage::PublishObject",
        "Puzzle3dAddBrushObjectStage::PublishAttraction",
    ] {
        assert!(!add_brush_object_is_cursorized(&source.replace(marker, "cursor-removed")), "missing brush cursor was falsely accepted: {marker}");
    }
}

fn add_object_kind_is_cursorized(source: &str) -> bool {
    source.contains(r#""addObjectKind" => Box::new(Puzzle3dAddObjectKindWork::default())"#)
        && source.contains("Puzzle3dAddObjectKindStage::Decode")
        && source.contains("Puzzle3dAddObjectKindStage::Kind")
        && source.contains("Puzzle3dAddObjectKindStage::Representation")
        && source.contains("Puzzle3dAddObjectKindStage::Vortex")
        && source.contains("Puzzle3dAddObjectKindStage::Publish")
        && source.contains("PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT")
        && !source.contains(r#""addObjectKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_object_kind_hostile_static_law_rejects_whole_catalog_conversion() {
    let source = include_str!("../../🦀️.rs");
    assert!(add_object_kind_is_cursorized(source));
    let direct =
        source.replace(r#""addObjectKind" => Box::new(Puzzle3dAddObjectKindWork::default())"#, r#""addObjectKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!add_object_kind_is_cursorized(&direct));
    for marker in ["Puzzle3dAddObjectKindStage::Kind", "Puzzle3dAddObjectKindStage::Representation", "Puzzle3dAddObjectKindStage::Vortex", "Puzzle3dAddObjectKindStage::Publish", "PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT"] {
        assert!(!add_object_kind_is_cursorized(&source.replace(marker, "cursor-removed")), "missing add-object-kind cursor was falsely accepted: {marker}");
    }
}

fn exact_window_routes_capture_instance_owners(source: &str) -> bool {
    source.contains("struct Puzzle3dWindowCommandWork")
        && source.contains("fn bind_window_owners")
        && source.contains("fn take_ephemeral")
        && source.contains("config_from_snapshot(self.window_config.as_ref())")
        && source.contains("transient_from_snapshot(self.window_transient.as_ref())")
        && source.contains("=> Box::new(Puzzle3dWindowCommandWork::new(tool_id))")
}

#[test]
fn exact_window_routes_reject_missing_owner_capture() {
    let source = include_str!("../../🦀️.rs");
    assert!(exact_window_routes_capture_instance_owners(source));
    for marker in ["fn bind_window_owners", "fn take_ephemeral", "config_from_snapshot(self.window_config.as_ref())", "transient_from_snapshot(self.window_transient.as_ref())"] {
        assert!(!exact_window_routes_capture_instance_owners(&source.replace(marker, "owner-capture-removed")));
    }
}

fn window_config_publish_does_not_silently_drop(source: &str) -> bool {
    source.contains("addressed_config_for(&wid, window_after)")
        && source.contains("addressed_transient_for(&wid, transient_after)")
        && !source.contains("addressed_config(view, window_after).ok()).into_iter()")
}

#[test]
fn window_config_publish_hostile_static_law_rejects_silent_ok_into_iter_drop() {
    let source = include_str!("../../🦀️.rs");
    assert!(window_config_publish_does_not_silently_drop(source));
    let dropped = source.replace(
        "vec![window_ownership::addressed_config_for(&wid, window_after)]",
        "view_state.and_then(|view| window_ownership::addressed_config(view, window_after).ok()).into_iter().collect()",
    );
    assert!(!window_config_publish_does_not_silently_drop(&dropped), "a silent .ok().into_iter() drop must fail the law");
}

#[test]
fn transform_lifecycle_is_an_explicit_bounded_retained_boundary() {
    let source = include_str!("../../🦀️.rs");
    let route = r#""worldPointerDown" | "transformBegin" | "transformEnd" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id))"#;
    assert!(source.contains(route));
    let direct = source.replace(
        route,
        r#""worldPointerDown" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id)),
            "transformBegin" | "transformEnd" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!direct.contains(route));
    assert!(direct.contains(r#""transformBegin" | "transformEnd" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#));
}

fn kind_weight_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle3dKindWeightWork::new(tool_id))"#)
        && source.contains("Puzzle3dKindWeightStage::Catalog")
        && source.contains("Puzzle3dKindWeightStage::Validate")
        && source.contains("Puzzle3dKindWeightStage::SumOthers")
        && source.contains("Puzzle3dKindWeightStage::Build")
        && source.contains("Puzzle3dConfigMutation::SetObjectKindWeights")
        && source.contains("Puzzle3dConfigMutation::SetVortexKindWeights")
        && !source.contains(r#""setObjectKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn kind_weight_hostile_static_law_rejects_whole_normalizer_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(kind_weight_route_is_cursorized(source));
    let direct = source.replace(
        r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle3dKindWeightWork::new(tool_id))"#,
        r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!kind_weight_route_is_cursorized(&direct));
    assert!(!source.contains("puzzle3d_normalize_kind_weight_group(self.weights"));
}

use crate::editor::puzzle3d::config::Puzzle3dCamera;
use protocol::MutationDiff;
use semio_framework_plugin::{EditorApp, PluginApp};

#[test]
fn app_config_serialization_excludes_operation_and_window_state() {
    let config = Puzzle3dConfig::default();
    let spr = to_json_string(&config);
    let oracle: serde_json::Value = serde_json::from_str(&spr).expect("third-party JSON oracle accepts Puzzle 3D config");
    // 🧮️ Five shared preferences: the four fill/distribution ones, plus `activeExampleId` — document
    // identity rather than a preference, and the only thing `export_fixture` can name its download after
    // (wave B26). Everything per-WINDOW or per-OPERATION still stays out, which is what this law is for.
    assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(5));
    assert!(spr.contains("activeExampleId"));
    for forbidden in ["fillCheckpoint", "fillApplyGeneration", "windowOptions", "camera", "suggestionMenu", "engagementInput"] {
        assert!(!spr.contains(forbidden));
    }
}

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn renders_world_scene() {
    let mut app = app().await;
    assert!(render_composite(&mut app).await.to_string().contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_the_concrete_forest_fixture() {
    let app = app().await;
    assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
    assert!(object_count(&app) > 0, "the concrete-forest default fixture ships with objects");
}

/// 📦️ `Puzzle3dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture.
#[semio_framework_async_macros::async_test]
async fn puzzle3d_play_projection_pack_round_trips() {
    let app = app().await;
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
}

/// 🧬️ Wave B9 lane 2: the TYPED half of the snapshot is the authority
/// `ArtifactApp::interaction_topology` reads, and `protocol::validate_state` prunes every selected id
/// the resulting topology does not contain — so a snapshot whose typed half decoded to the DEFAULT
/// (empty) document answers every pick with an empty selection while its `value()` projection still
/// renders the whole world. That is exactly the boundary wave B6 proved in the browser. `PartialEq`
/// compares the typed half alone, so [`assert_dsl_pack_equivalence`] above reads two empty documents
/// as equal and cannot state this; the object count on BOTH halves can.
#[semio_framework_async_macros::async_test]
async fn play_snapshot_typed_authority_survives_every_store_round_trip() {
    use store::{ArtifactDsl, ArtifactPack};
    let app = app().await;
    let snapshot = app.snapshot().expect("projection");
    let objects = snapshot.value().get("objects").and_then(serde_json::Value::as_array).map(Vec::len).unwrap_or(0);
    assert!(objects > 0, "the boot fixture ships objects");
    assert_eq!(snapshot.typed().objects.len(), objects, "the live snapshot's typed authority must carry the projection's objects");
    let via_pack = Puzzle3dPlaySnapshot::decode_pack(&snapshot.encode_pack()).expect("pack decode");
    assert_eq!(via_pack.typed().objects.len(), objects, "a store pack round trip must not empty the typed authority");
    let via_dsl = Puzzle3dPlaySnapshot::parse_dsl(&snapshot.print_dsl()).expect("dsl parse");
    assert_eq!(via_dsl.typed().objects.len(), objects, "a dsl round trip must not empty the typed authority");
    let via_value = Puzzle3dPlaySnapshot::new(snapshot.value().clone());
    assert_eq!(via_value.typed().objects.len(), objects, "rebuilding from the projection must not empty the typed authority");
}

/// 🕳️ Wave B9 lane 2: an ABSENT fixture-meta member must project as absent, never as `Null`. The
/// persisted twin types `meta.kindCompatibility` as a real array and refuses a `Null` for it, and
/// `Puzzle3dPlaySnapshot`'s decode is all-or-nothing — so one `null` member replaced the ENTIRE typed
/// authority with an empty document while the projection kept every object. `PartialEq` compares the
/// typed half alone, so no round-trip law above can see it; the object count on BOTH halves can.
#[test]
fn an_absent_fixture_meta_member_never_empties_the_typed_authority() {
    let mut seeded = empty_fixture();
    seeded.objects.clone_from(&CONCRETE_FOREST_EXAMPLE_FIXTURE.objects);
    assert!(seeded.meta.kind_catalogs.is_none() && seeded.meta.kind_compatibility.is_none(), "this law is about the meta-less fixture shape");
    let projection: serde_json::Value = (&dsl::ToValue::to_value(&seeded)).into();
    let meta = projection.get("meta").expect("the projection carries a meta object");
    assert!(!meta.get("kindCatalogs").is_some_and(serde_json::Value::is_null), "an absent kind catalog must be absent, not null: {meta}");
    assert!(!meta.get("kindCompatibility").is_some_and(serde_json::Value::is_null), "an absent compatibility table must be absent, not null: {meta}");
    let snapshot = Puzzle3dPlaySnapshot::new(projection);
    assert_eq!(snapshot.typed().objects.len(), seeded.objects.len(), "both halves of one snapshot must describe the same document");
}

/// 🔭️ Wave B9 lane 2: the pickable universe `protocol::validate_state` prunes against must name
/// every id the world lane paints. Measured against a document whose TYPED authority is empty while
/// its projection is whole — the exact divergence a refused meta decode produces, and the reason the
/// browser's first pick landed on the leftover and then vanished from the Inspection panel.
#[test]
fn interaction_topology_names_every_id_the_world_lane_paints() {
    let mut divergent = empty_fixture();
    divergent.objects.clone_from(&CONCRETE_FOREST_EXAMPLE_FIXTURE.objects);
    divergent.meta.kind_compatibility = Some(dsl::DslValue::Array(vec![dsl::DslValue::object([("target".to_string(), dsl::DslValue::String("b-l".to_string()))])]));
    let projection: serde_json::Value = (&dsl::ToValue::to_value(&divergent)).into();
    let snapshot = Puzzle3dPlaySnapshot::new(projection);
    assert!(snapshot.typed().objects.is_empty(), "this law needs the typed authority to have refused the document");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = Puzzle3dConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = Puzzle3dPlayApp::interaction_topology(&doc, &cfg);
    let domain = topology.domains.get(PUZZLE3D_INTERACTION_DOMAIN).expect("the vortex domain is declared");
    let ids: Vec<&str> = domain.ordered.iter().map(|node| node.id.as_str()).collect();
    for object in &divergent.objects {
        assert!(ids.contains(&object.id.as_str()), "a painted object must be pickable: {} missing from {ids:?}", object.id);
        for vortex in &object.vortices {
            let full = puzzle3d_vortex_full_id(&object.id, &vortex.id);
            assert!(ids.iter().any(|id| *id == full.as_str()), "a painted vortex marker must be pickable: {full} missing");
        }
    }
}


#[semio_framework_async_macros::async_test]
async fn open_add_object_dialog_emits_the_open_dialog_effect_with_no_document_change() {
    let mut app = app().await;
    let before = object_count(&app);
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("openAddObjectDialog");
    assert!(
        matches!(result.requested_effects.as_slice(), [Effect::OpenDialog { dialog_id, args, .. }] if dialog_id == "addObject" && args.is_none()),
        "expected a single OpenDialog effect for the addObject dialog, got {:?}",
        result.requested_effects,
    );
    assert_eq!(object_count(&app), before, "opening the dialog does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_the_document_and_undo_restores_it() {
    let mut app = app().await;
    let loaded = object_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0, "empty example clears the objects");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), loaded, "undo restores the concrete-forest objects");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_count(&app), 0);
}

/// 🛰️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B7: `undo` is a TWO-half framework-reserved gesture
/// and neither half may be silent. The ADMISSION half must change nothing and say so out loud — one
/// Isolated [`semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND`] spawn job and
/// `UiDirtyScope::None`, never a success that quietly committed nothing; the HOST half
/// (`complete_reserved_spawned_job` → `commit_framework_history_route`) is the only thing that may
/// move the document. Pins the regression wave B5 reported: a caller that stops at the admission
/// sees `dispatch("undo")` succeed while the store never moves, so every undo law degrades into a
/// no-op that still passes its dispatch.
#[semio_framework_async_macros::async_test]
async fn undo_admits_one_reserved_job_that_alone_restores_the_document() {
    let mut app = app().await;
    let loaded = object_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0, "empty example clears the objects");
    let admitted = dispatch_reserved_unsettled(&mut app, "undo", None, None).await.expect("undo admits");
    let spawned = admitted.requested_effects.iter().filter(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND)).count();
    assert_eq!(spawned, 1, "the admission half must ask the host for exactly one Isolated reserved job: {:?}", admitted.requested_effects);
    assert!(matches!(admitted.ui_scope, UiDirtyScope::None), "an undo that has not run yet must not claim a dirty scope: {:?}", admitted.ui_scope);
    assert_eq!(object_count(&app), 0, "the admission half must never move the document on its own");
    let committed = settle_reserved(&mut app, admitted).await.expect("the host half commits the history route");
    assert!(matches!(committed.ui_scope, UiDirtyScope::Full), "a committed undo republishes the whole document: {:?}", committed.ui_scope);
    assert_eq!(object_count(&app), loaded, "only the driven reserved job restores the concrete-forest objects");
}

#[semio_framework_async_macros::async_test]
async fn nakagin_example_loads_via_operations() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let projection = projection_of(&app);
    assert_eq!(projection.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
    assert!(projection.get("objects").and_then(|value| value.as_array()).is_some_and(|objects| !objects.is_empty()));
}

/// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: a mutating verb lands its OBJECT **and** its
/// history row inside the settle of its OWN dispatch — no second command. The browser measured the
/// opposite on wasm #47: an `addObjectKind` click settled in 23 continuations with
/// `frameKinds:["Invocation","Ephemeral"]` and `historyUpserts:0`, and the `create-object` row only
/// arrived inside the NEXT command's settle, which is why `catalogue-add-object-kind`,
/// `delete-selection`, `duplicate-selection` and `volume-brush-add-target-volume` all read
/// `before == after` at their own verdict (`📓️2026-09-12-wave-B19-mutation-lane-regression.md` §3).
///
/// The row rides [`Puzzle3dCompletion::history_patch`] — the `AppFrame::OperationCompleted` the
/// renderer applies through `subscribeOperationCompletions`. A typed operation never calls
/// `record_command`: its edit reaches the command log only through `backfill_command_log`, inside
/// `refresh_cache`, inside `history_patch` — so reading `history_dirty_sequences` BEFORE that
/// backfill answered "nothing dirty" on the very call whose publication had just landed.
#[semio_framework_async_macros::async_test]
async fn a_mutating_verb_lands_its_object_and_its_history_row_inside_its_own_settle() {
    for (action, args) in [("addObjectKind", json!({ "objectKind": "Object", "origin": [3.0, 0.0, 0.0] })), ("duplicateSelection", json!({}))] {
        let mut app = app().await;
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty example");
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("seed one object");
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the seed object");
        let before = object_count(&app);
        dispatch_unsettled(&mut app, action, Some(&args), None).await.unwrap_or_else(|fault| panic!("{action} must admit: {fault:?}"));
        let settled = settle(&mut app).await;
        assert_eq!(object_count(&app), before + 1, "{action} must land its object inside its own settle");
        let rows = settled.completions.iter().filter(|completion| completion.history_patch.as_ref().is_some_and(|patch| !patch.upserts.is_empty())).count();
        assert_eq!(rows, 1, "{action} must hand back exactly one completion carrying its history patch inside its own settle, got {:?}", settled.completions);
    }
}

#[semio_framework_async_macros::async_test]
async fn document_and_inspector_panels_render() {
    let mut app = app().await;
    for body in [document::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(!render_body(&mut app, body).await.to_string().is_empty());
    }
}
//#endregion 🔖️Operations

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`). Deliberately
/// dispatches through a standalone typed `Puzzle3dStore` — NOT through `Puzzle3dPlayApp`/
/// `Puzzle3dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app still uses)
/// — since `Puzzle3dMutation`'s canonical `Mutation<Puzzle3dSnapshot>` impl (not its
/// `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::binary::{close_puzzle3d_store, puzzle3d_store};
    use crate::{Puzzle3dObject as TypedObject, PUZZLE_3D_SCHEMA};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand};

    let mut store = puzzle3d_store(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None)).await.expect("store");
    let object = TypedObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_object(object, None)], description: None }).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle3dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    close_puzzle3d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️Inspector
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: with nothing selected the inspector
/// shows the document summary; a live `object`-granularity selection switches it to that object's own
/// field group, carrying its real id/origin plus the `hidden`/`locked` `patchInspector` toggles.
#[semio_framework_async_macros::async_test]
async fn selected_object_inspector_renders_that_object_field_group() {
    let mut app = app().await;
    let empty = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(empty.contains("puzzle3d-play-inspector.empty"), "an empty selection shows the document summary: {empty}");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let json = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!json.contains("puzzle3d-play-inspector.empty"), "a live selection must replace the document summary: {json}");
    for expected in ["puzzle3d-play-inspector.object.id", "puzzle3d-play-inspector.object.origin", "puzzle3d-play-inspector.object.hidden", "puzzle3d-play-inspector.object.locked"] {
        assert!(json.contains(expected), "inspector must render {expected}: {json}");
    }
    assert!(json.contains(object_id.as_str()), "the rendered field group carries the selected object's own id: {json}");
}

/// 🕹️ Wave B9 lane 2, end to end: the FIRST pick of a fresh session must already be visible to the
/// guest's own render — the object's id and its `hidden`/`locked` `flag_row`s — and a lock taken on
/// that same selection must show as a pressed row. In the browser this hop folded: the pick reached
/// the leftover, `validate_state` pruned it against a topology narrower than the painted document,
/// and the panel kept re-rendering the empty-document summary for 73 s.
#[semio_framework_async_macros::async_test]
async fn first_pick_of_a_fresh_session_renders_the_object_and_its_lock_row() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("first interactionSelect of the session");
    let picked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!picked.contains("puzzle3d-play-inspector.empty"), "the FIRST pick must replace the document summary: {picked}");
    assert!(picked.contains(object_id.as_str()), "the first pick's own object id must reach the panel: {picked}");
    assert!(picked.contains("puzzle3d-play-inspector.object.locked"), "the first pick must assemble the lock flag_row: {picked}");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), None).await.expect("lock the picked object");
    let locked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(locked.contains("puzzle3d-play-inspector.object.locked"), "a locked selection keeps its lock flag_row: {locked}");
    assert!(
        projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id.as_str())).cloned()).and_then(|object| object.get("locked").and_then(Value::as_bool))
            == Some(true),
        "the lock taken on the first pick's selection must land on that object"
    );
}

/// 🕹️ Wave B13: hover after the first pick must keep Inspection on that object — leftover hover
/// used to republish `selectedIds: []` and the panel folded back to the empty summary.
#[semio_framework_async_macros::async_test]
async fn hover_after_first_pick_keeps_the_object_and_its_lock_row() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("first interactionSelect");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(object_id.as_str())).await.expect("hover after first pick");
    let hovered = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!hovered.contains("puzzle3d-play-inspector.empty"), "hover after first pick must keep the object fields: {hovered}");
    assert!(hovered.contains(object_id.as_str()), "hover after first pick must keep the object id: {hovered}");
    assert!(hovered.contains("puzzle3d-play-inspector.object.locked"), "hover after first pick must keep the lock flag_row: {hovered}");
}

/// 🕹️ Background click: empty-target `interactionSelect` with `replace` clears selection; hover may remain.
#[semio_framework_async_macros::async_test]
async fn empty_target_interaction_select_clears_selection_while_hover_remains() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(object_id.as_str())).await.expect("hover names the object");
    let admitted = dispatch_reserved_unsettled(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": "[]", "merge": "replace", "method": "pick" })),
        None,
    )
    .await
    .expect("empty-target interactionSelect admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("empty-target interactionSelect leftover");
    let view = settled.output.get("interactionView").expect("leftover InteractionView");
    let ids = view.get("selectedIds").and_then(dsl::DslValue::as_array).expect("selectedIds");
    assert!(ids.is_empty(), "empty-target interactionSelect must clear selectedIds, got {ids:?} hover={:?}", view.get("hoverTarget"));
    let hover = view.get("hoverTarget").expect("hoverTarget on leftover");
    assert_eq!(hover.get("id").and_then(dsl::DslValue::as_str), Some(object_id.as_str()));
    assert!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "empty-target interactionSelect must clear the interaction store"
    );
    let picked = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(picked.contains("puzzle3d-play-inspector.empty"), "Inspection must fold to the empty summary after background deselect: {picked}");
}

/// 🪟️ WAVE B56 LAW: a window-addressed `interactionSelect` leftover encode carries the window
/// INSTANCE it was dispatched for, and a windowless dispatch carries none — never a synthetic window.
///
/// 🐛️ The leftover encode named no window at all, so the host had to infer the pane the overlay belonged
/// to; when it could not, the guest's own dirty fell back to the bare surface name `window` and the host
/// mounted a synthetic alias to give that name a context (wave W-G3 §8.29). That alias is the surface whose
/// reconcile reservation refusal starved every real one
/// (`reserve_refusal=1:window:registry-reservation-unavailable`, wave B54 §6.4). The instance rides the
/// encode now, so `publishLeftoverWorldSelectionV1` addresses a real pane and nothing has to be inferred
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
#[semio_framework_async_macros::async_test]
async fn interaction_select_leftover_window_instance_rides_the_encode() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), id: object_id.clone() }]);
    for window in [main::WINDOW_INSTANCE_PERSPECTIVE, main::WINDOW_INSTANCE_TOP] {
        let admitted = dispatch_reserved_unsettled(
            &mut app,
            "interactionSelect",
            Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets.clone(), "merge": "replace", "method": "pick" })),
            Some(window),
        )
        .await
        .expect("window-addressed interactionSelect admit");
        let settled = settle_reserved(&mut app, admitted).await.expect("window-addressed interactionSelect leftover");
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        assert_eq!(
            view.get("windowId").and_then(dsl::DslValue::as_str),
            Some(window),
            "the leftover encode must name the window instance the pick addressed, got {:?}",
            view.get("windowId")
        );
        eprintln!("[DEBUG] leftover encode for {window} carries windowId={:?}", view.get("windowId").and_then(dsl::DslValue::as_str));
    }
}

/// 🎮 Wave B23: leftover.ids that name an object must reach the snapshot `inspection::render` reads
/// when persist `selection(vortex)` is empty.
#[test]
fn leftover_ids_name_object_empty_persist_vortex_from_state_wires_inspection_snapshot() {
    let mut state = protocol::InteractionState::default();
    state.selection.insert("vortex".into(), protocol::DomainSelection { granularity: String::new(), ids: Vec::new(), anchor_id: None });
    state.selection.insert("leftover".into(), protocol::DomainSelection { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), ids: vec!["seed-left-001".into()], anchor_id: None });
    let interaction = Puzzle3dInteractionSnapshot::from_state(&state, &semio_framework_plugin::app::InteractionHoverState::default());
    assert_eq!(interaction.selected, vec!["seed-left-001".to_string()], "leftover.ids must land on the snapshot Inspection reads");
    assert_eq!(interaction.granularity, PUZZLE3D_GRANULARITY_OBJECT);
}

/// 🕹️ A `vortex`-granularity selection switches the inspector to the vortex field group instead — the
/// per-granularity switch the panel's `selected_section` performs.
#[semio_framework_async_macros::async_test]
async fn selected_vortex_inspector_renders_the_vortex_field_group() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let json = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(json.contains("puzzle3d-play-inspector.vortex.full-id"), "inspector must render the vortex field group: {json}");
    assert!(json.contains("puzzle3d-play-inspector.vortex.radius"), "inspector must render the vortex radius: {json}");
    assert!(!json.contains("puzzle3d-play-inspector.object.id"), "a vortex selection must not render the object group: {json}");
}

fn object_origin_x(app: &Puzzle3dApp, object_id: &str) -> f64 {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).and_then(|origin| origin.first()).and_then(Value::as_f64))
        .expect("origin.x")
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_sets_absolute_value_and_preserves_other_axes() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let before_y = projection_of(&app)
        .get("objects")
        .and_then(|value| value.as_array())
        .and_then(|objects| objects.first())
        .and_then(|object| object.get("origin"))
        .and_then(|value| value.as_array())
        .and_then(|origin| origin.get(1))
        .and_then(|value| value.as_f64())
        .expect("origin.y");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector");
    let projection = projection_of(&app);
    let objects = projection.get("objects").and_then(|value| value.as_array()).expect("objects");
    let object = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(object_id.as_str())).expect("patched object");
    let origin = object.get("origin").and_then(|value| value.as_array()).expect("origin");
    assert_eq!(origin[0].as_f64(), Some(42.5), "origin.x should be set to the absolute value");
    assert_eq!(origin[1].as_f64(), Some(before_y), "origin.y should be untouched by an origin.x edit");
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_delta_offsets_each_selected_object_from_its_own_current_value() {
    let mut app = app().await;
    let id_a = first_object_id(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [10.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let id_b = projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("added object id").to_string();
    assert_ne!(id_a, id_b, "the added object must be distinct from the first fixture object");
    let x_a_before = object_origin_x(&app, &id_a);
    let x_b_before = object_origin_x(&app, &id_b);
    assert_ne!(x_a_before, x_b_before, "the two objects must start at different x values for this test to prove per-object offset preservation");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [id_a.clone(), id_b.clone()], "field": "origin.x", "delta": 3.0 })), None).await.expect("patchInspector");
    assert_eq!(object_origin_x(&app, &id_a), x_a_before + 3.0, "a delta edit adds to each object's own current x");
    assert_eq!(object_origin_x(&app, &id_b), x_b_before + 3.0, "a delta edit preserves each object's own starting offset");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the inspector chrome no longer
/// renders per-entity stepper controls to inspect (see the sibling test's doc comment above), so
/// this now proves the same "resolve selection without embedding ids" contract one level down, at
/// `patchInspector`'s own `interaction.selection(vortex)` fallback (`commands::patch_inspector`) —
/// a bare `field`/`value` patch (no `ids` arg) must resolve against whatever the `vortex` domain's
/// `object` granularity currently holds.
#[semio_framework_async_macros::async_test]
async fn inspector_field_actions_resolve_selection_without_embedding_ids() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector without ids");
    assert_eq!(object_origin_x(&app, &object_id), 42.5, "patchInspector must resolve the patched object from the live selection, not an embedded id");
}
//#endregion 🔖️Inspector

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn app_definition_has_the_main_world_window() {
    let definition = create_puzzle3d_app();
    assert!(definition.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_the_add_object_dialog() {
    let definition = create_puzzle3d_app();
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    assert_eq!(dialog.submit_action.as_str(), "addObjectKind");
    assert_eq!(dialog.args.len(), 1);
}

/// 📌️ The four declared panel tabs are present (the framework injects its own tabs alongside, so
/// this asserts presence, never a total count).
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_its_four_panel_tabs() {
    let definition = create_puzzle3d_app();
    let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
    for expected in [document::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(body_keys.contains(&expected), "panel tab body {expected} must be declared, got {body_keys:?}");
    }
}

/// 🌉️ Every declared action must bridge through `command_from_action` and round-trip
/// `command_id` via the shared framework harness.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_bridges_to_a_command() {
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_tests).await;
    assert!(Puzzle3dPlayApp::command_from_action("noSuchAction", None).is_err());
}

/// 🌉️ Every declared app action (framework-injected verbs never reach `Puzzle3dCommand::from_action`
/// by design, so they simply fall through the `None` branch below) round-trips through the
/// macro-generated `Puzzle3dCommand::from_action`/`action_id` pair as well as the `ArtifactApp`
/// bridge asserted above.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_round_trips_through_the_command_enum() {
    let definition = create_puzzle3d_app();
    for action in definition.window_kinds.iter().flat_map(|window| window.actions.iter()) {
        let Some(command) = Puzzle3dCommand::from_action(&action.id, None, None) else {
            continue;
        };
        assert_eq!(command.action_id(), action.id.as_str(), "declared action {} must round-trip through Puzzle3dCommand", action.id);
    }
}

/// 🗣️ B1: manifest text is baked into `AppDefinition`/`App` as `LocalizedLabel` and resolved
/// directly via `.resolve(Terminology, Locale)` — no shell round-trip needed to assert on it.
#[semio_framework_async_macros::async_test]
async fn app_definition_labels_resolve_german_reuse_branded_for_aggregator() {
    use semio_framework_plugin::{Locale, Terminology};
    let definition = create_puzzle3d_app();
    let def = &definition;
    let (terminology, locale) = (Terminology::Reuse, Locale::De);
    let actions = || def.window_kinds.iter().flat_map(|window| window.actions.iter());
    let action = |id: &str| actions().find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
    assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Bearbeiten");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Aggregator");
    let dialog = def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog");
    assert_eq!(dialog.title.resolve(terminology, locale), "Baukomponente hinzufügen");
    assert_eq!(dialog.submit_label.resolve(terminology, locale), "Hinzufügen");
    // 🗂️ The select's own label is the terminology-aware half; its OPTIONS are the declared examples'
    // catalog rows (`puzzle3d_object_kind_options`), carried as `LocalizedLabel::data` — document data,
    // never authored UI text, so a row reads identically on every axis. The literal `"Object"` option
    // this law used to assert on was the hardcoded placeholder that catalog derivation replaced.
    let arg = dialog.args.iter().find(|entry| entry.id == "objectKind").expect("objectKind arg");
    assert_eq!(arg.label.resolve(terminology, locale), "Art");
    match arg.control() {
        semio_framework_plugin::ActionArgControl::Select { options } => {
            assert!(!options.is_empty() && options.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "the objectKind select offers the declared examples' bounded catalog rows; observed {}", options.len());
            for option in options {
                assert!(!option.value.is_empty(), "every offered object kind names a catalog row");
                assert_eq!(option.label.resolve(terminology, locale), option.label.resolve(semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::En), "catalog row {} is document data and must not be re-authored per axis", option.value);
            }
        }
        _ => panic!("objectKind arg is not a select"),
    }
    assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Baukomponente hinzufügen");
    assert_eq!(action("openVortexSuggestions").label.resolve(terminology, locale), "Verbindungspunkt-Vorschläge öffnen");
    assert_eq!(action("createAttraction").label.resolve(terminology, locale), "Verbindung erstellen");
    assert_eq!(def.utilities.iter().find(|entry| entry.id == utilities::transform::UTILITY_ID).expect("transform utility").label.resolve(terminology, locale), "Transformieren");
    // 🎭️✏️ `create_puzzle3d_app()` no longer registers examples (see its own doc comment — `Editor::builder`
    // has no `.example(...)` and `AppDefinition` carries no `examples` field), so the concrete-forest
    // example's German label can no longer be asserted here; dropped, not silently left stale.
    let framework_interaction_actions = [
        INTERACTION_SELECT_ACTION_ID,
        semio_framework_plugin::INTERACTION_HOVER_ACTION_ID,
        semio_framework_plugin::CLEAR_SELECTION_ACTION_ID,
        semio_framework_plugin::SELECT_ALL_ACTION_ID,
        semio_framework_plugin::SET_SELECTION_MODE_ACTION_ID,
        semio_framework_plugin::SET_INTERACTION_GRANULARITY_ACTION_ID,
    ];
    for entry in actions() {
        if framework_interaction_actions.contains(&entry.id.as_str()) {
            continue;
        }
        let text = entry.label.resolve(terminology, locale);
        assert!(!text.contains("Hover") && !text.contains("Pick") && !text.contains("hovern"), "leftover English/mistranslation in {}: {text}", entry.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn app_definition_labels_stay_english_native_without_brand_locks() {
    use semio_framework_plugin::{Locale, Terminology};
    let definition = create_puzzle3d_app();
    let def = &definition;
    let (terminology, locale) = (Terminology::Native, Locale::En);
    let action = |id: &str| def.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
    assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Edit");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Puzzle 3D");
    assert_eq!(def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog").title.resolve(terminology, locale), "Add Object");
    assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Add Object");
}

#[semio_framework_async_macros::async_test]
async fn document_and_kinds_trees_use_german_reuse_section_labels() {
    let mut app = app().await;
    // 🗣️ Panels resolve their label set from the host's own axes and fail closed on an unauthored one
    // (`puzzle3d_labels`), so the German reuse text this law is about only exists once the axes name it.
    app.set_label_axes(semio_framework_plugin::Locale::De, semio_framework_plugin::Terminology::Reuse);
    let document_json = render_body(&mut app, document::BODY_KEY).await.to_string();
    let kinds = render_body(&mut app, catalogue::BODY_KEY).await.to_string();
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_json = to_json_string(&app.window_measures(&view).await);
    assert!(document_json.contains("Baukomponenten"), "document tree objects section");
    assert!(document_json.contains("Verbindungen"), "document tree attractions section");
    assert!(document_json.contains("Referenzen"), "document tree references section");
    assert!(document_json.contains("Zielvolumina"), "document tree target volumes section");
    assert!(kinds.contains("Kabel"), "catalogue cables section");
    assert!(kinds.contains("Verbindungen"), "catalogue attractions section");
    assert!(!document_json.contains("\"Attractions\"") && !kinds.contains("\"Attractions\""), "English Attractions must not appear");
    assert!(!kinds.contains("\"Cables\""), "English Cables must not appear");
    assert!(measures_json.contains("Verbindungen"), "select measures attractions toggle");
    assert!(!measures_json.contains("\"Attractions\""), "select measures must not hardcode Attractions");
}

#[semio_framework_async_macros::async_test]
async fn main_window_utilities_lead_with_transform_without_select_tool_and_no_default_utility() {
    let definition = create_puzzle3d_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert!(!utility_ids.contains(&"select"), "puzzle 3d must not declare a select utility");
    assert!(!utility_ids.contains(&"scale"), "puzzle 3d must not declare a scale utility");
    assert!(!utility_ids.contains(&fill_tool::TOOL_ID), "fill is a mode-level tool, not a window utility");
    let window = definition.window_kinds.iter().find(|window| window.id == main::WINDOW_KIND_ID).expect("main window");
    let main_utilities: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(main_utilities.first().copied(), Some(utilities::transform::UTILITY_ID));
    assert!(!main_utilities.contains(&"select"));
    assert!(!main_utilities.contains(&fill_tool::TOOL_ID), "fill must not be bound to the main window as a utility");
    assert_eq!(PUZZLE3D_DEFAULT_UTILITY, "", "unset/cleared host utility must not impersonate transform");
}

/// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
#[semio_framework_async_macros::async_test]
async fn tool_registry_declares_fill_tool() {
    let definition = create_puzzle3d_app();
    let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
    assert_eq!(tool_ids, vec![fill_tool::TOOL_ID]);
    assert_eq!(definition.modes[0].tools, vec![ToolRef::new(fill_tool::TOOL_ID).await]);
    assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
}

/// 🛠️ Wave W-AB, restated by wave B31: an empty `setActiveTool` is a DISARM, so the only verb allowed to
/// emit one is the one whose whole meaning is "leave what is armed". Background and parameter traffic that
/// merely happens WHILE fill is armed must never emit it — a bounce there disarms the tool under the user's
/// hands mid-plan. `engagementAbort` is the exception and states it deliberately
/// (`🎮️commands/🛑️engagement-abort/🦀️.rs`, and
/// `escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool`): before B31 it was listed here as
/// a bounce too, which is exactly why Escape could never leave Fill in the browser.
#[semio_framework_async_macros::async_test]
async fn fill_flow_only_disarms_the_tool_when_the_user_aborts() {
    let empty_tool = |result: &semio_framework_plugin::InvocationResult| {
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty()))
    };
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("arm fill");
    let tick = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
    assert!(!empty_tool(&tick), "fillBuildTick must not bounce-disarm: {:?}", tick.requested_effects);
    let count = dispatch(&mut app, "setFillCount", Some(&json!({ "count": 3 })), None).await.expect("setFillCount");
    assert!(!empty_tool(&count), "adjusting the count mid-plan must not bounce-disarm: {:?}", count.requested_effects);
    let abort = dispatch(&mut app, "engagementAbort", None, None).await.expect("engagementAbort");
    assert!(empty_tool(&abort), "engagementAbort IS the disarm — it must emit the empty tool effect: {:?}", abort.requested_effects);
}
//#endregion 🔖️Manifest

//#region 🔖️Suggestions
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_vortex_and_prepends_suggest_objects() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("Suggest objects"), "menu should be {menu_json}");
    assert!(menu_json.contains("openVortexSuggestions"));
    assert!(menu_json.contains("sparkles"), "menu should include suggest icon: {menu_json}");
    assert!(menu_json.contains("Zoom to selection"), "menu should include zoom: {menu_json}");
    assert!(menu_json.contains("deleteSelection"), "menu should include delete: {menu_json}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden() {
    let mut app = app().await;
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("setTargetVolumeFlag"), "menu should be {menu_json}");
    assert!(menu_json.contains("menu.group.targets"), "hide/lock rows should be grouped under targets: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "destructive delete must be the last top-level row: {menu_json}");
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id.as_str(), "flag": "hidden", "value": true })), None).await.expect("setTargetVolumeFlag");
    let hidden = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("hidden")).and_then(Value::as_bool);
    assert_eq!(hidden, Some(true));
}

/// 🗂️ Grouped-disclosure contract for the object-selection branch: the top-level menu stays
/// scannable (leaves + groups + separator combined) and the destructive `deleteSelection` row is
/// the last top-level entry (`organize_context_menu` inserts the separator ahead of it).
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_object_groups_flags_and_keeps_delete_last() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    assert!(menu.len() <= 9, "top-level menu should stay scannable, got {} rows: {menu:?}", menu.len());
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("menu.group.hand"), "hide/lock rows should be grouped under hand: {menu_json}");
    assert!(menu_json.contains("duplicateSelection"), "menu should be {menu_json}");
    assert_eq!(menu.last().map(|item| item.id.as_str()), Some("delete"), "delete must be the last top-level row: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "delete must be marked destructive: {menu_json}");
}

/// 🎯️ Wave B11 (checklist §15, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 8): a right-click
/// on an object the document had NOT selected produced an empty plugin menu, so the shell's own fallback
/// ("Set Active Example", "Export", …) took over the viewport and none of `duplicate`/`select-same-kind`/
/// `zoom`/`hide-show`/`lock-unlock`/`delete` was ever reachable. `ContextMenuSurfaceTarget.hits` is the
/// pointer target the host already sends; the menu must read it, not only `selection`.
#[semio_framework_async_macros::async_test]
async fn right_clicking_an_unselected_object_opens_that_object_menu_not_an_empty_one() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let menu = context_menu_for_hit(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    let mut ids: Vec<String> = Vec::new();
    let mut walk: Vec<&semio_framework_plugin::ContextMenuItemSpec> = menu.iter().collect();
    while let Some(item) = walk.pop() {
        ids.push(item.id.clone());
        walk.extend(item.children.iter().flatten());
    }
    for row in ["duplicate", "select-same-kind", "zoom", "hide-show", "lock-unlock", "delete"] {
        assert!(ids.iter().any(|id| id == row), "the object vocabulary must be reachable from a bare hit, {row} missing from {ids:?}");
    }
}

/// 🎯️ …and a hit INSIDE the live selection never narrows the menu to the one row under the cursor: the
/// whole selection stays the subject, so a "Delete (3 objects)" row can never silently become a one-object
/// delete just because the press landed on a particular member.
#[semio_framework_async_macros::async_test]
async fn a_hit_inside_the_selection_keeps_the_whole_selection_as_the_menu_subject() {
    use semio_framework_plugin::{ContextMenuHit, ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [4.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).expect("objects").iter().filter_map(|object| object.get("id").and_then(Value::as_str)).map(str::to_string).collect();
    assert!(ids.len() >= 2, "this law needs a multi-object selection, got {ids:?}");
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget {
            surface_id: "world3d".into(),
            kind: "world3d".into(),
            hits: vec![ContextMenuHit { domain: PUZZLE3D_GRANULARITY_OBJECT.into(), id: ids[0].clone(), label: None }],
            selection: vec![ContextMenuSelectionGroup { domain: PUZZLE3D_GRANULARITY_OBJECT.into(), ids: ids.clone() }],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let view = app.window_view(main::WINDOW_KIND_ID);
    let menu = app.context_menu(&request, &view).await;
    let delete = menu.iter().find(|item| item.id == "delete").expect("the object menu carries a delete row");
    let label = delete.label.clone().unwrap_or_default();
    assert!(label.contains(&ids.len().to_string()), "the delete row must still name the whole selection, got {label:?} for {ids:?}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_opens_the_suggestion_popup() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let result = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 34.0 })), None).await.expect("openVortexSuggestions");
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "opening a one-shot suggestion must not switch the host-owned utility or tool: {:?}",
        result.requested_effects,
    );
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "context-menu suggestion stays in the current selection mode");
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("open").and_then(Value::as_bool), Some(true));
    assert_eq!(menu.get("x").and_then(Value::as_f64), Some(12.0));
    assert_eq!(menu.get("y").and_then(Value::as_f64), Some(34.0));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
    assert!(menu.get("windowId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "suggestion menu is scoped to the opening window: {menu}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_records_explicit_window_id() {
    // 🪟️ The popup is transient state of the pane the user acted in (the dispatch's own window
    // authority), and it RECORDS the explicit target pane the args named — so the assertion renders
    // the acting pane and reads the recorded `windowId` back out of it.
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 8.0, "y": 16.0, "windowId": main::WINDOW_INSTANCE_TOP })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("openVortexSuggestions");
    let interaction = interaction_of(&render_window(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await);
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("windowId").and_then(Value::as_str), Some(main::WINDOW_INSTANCE_TOP));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_with_full_id_places_even_if_selection_was_cleared() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    let before_count = object_count(&app);
    // 🧹️ Simulate the split-pane outside-dismiss race clearing vortex selection before accept.
    dispatch(&mut app, "clearSelection", None, None).await.expect("clearSelection");
    let result = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex.as_str() })), None).await.expect("acceptSuggestion");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accept must not switch utility/tool: {:?}", result.requested_effects);
    assert!(object_count(&app) > before_count, "accept with fullId must place even after selection clear");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_the_menu() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🖱️ Hovering a row in the suggestion popup must live-update the 3D brush preview (rendered by
/// `world_brush_preview_json`, which reads `runtime.brush_candidate_index`) to the hovered
/// candidate, so the UI can highlight it in 3D before the user clicks — without switching the
/// host-owned active utility into brush mode.
#[semio_framework_async_macros::async_test]
async fn hover_suggestion_updates_the_brush_candidate_index_and_live_preview() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    let composite = render_composite(&mut app).await;
    let interaction = interaction_of(&composite);
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "suggestion hover must not enter brush mode");
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(0), "opening suggestions starts hover at the first candidate");
    let candidates = interaction.pointer("/suggestionMenu/candidates").and_then(Value::as_array).cloned().unwrap_or_default();
    assert!(!candidates.is_empty(), "suggestion candidates should be present");
    assert!(candidates[0].get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "candidates carry object-kind color: {candidates:?}");
    assert!(candidates[0].get("icon").and_then(Value::as_str).is_some_and(|icon| !icon.is_empty()), "candidates carry icon: {candidates:?}");
    let preview = brush_preview_of(&composite);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the live preview must target the vortex the suggestion menu was opened on");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "the live preview must resolve to a real candidate object kind");
    assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "brush preview carries object-kind color: {preview}");

    dispatch(&mut app, "hoverSuggestion", Some(&json!({ "index": 1 })), None).await.expect("hoverSuggestion");
    let composite = render_composite(&mut app).await;
    let interaction = interaction_of(&composite);
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(1), "hovering a different row must move the tracked candidate index");
    let preview = brush_preview_of(&composite);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the preview must keep targeting the same vortex while only the hovered candidate changes");
    assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "hovered brush preview still carries color: {preview}");
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_appends_an_object_and_closes_the_menu() {
    let mut app = app().await;
    let object_count_before = object_count(&app);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    let result = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before + 1);
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "accepting a one-shot suggestion must leave the host-owned utility/tool unchanged: {:?}",
        result.requested_effects,
    );
    let composite = render_composite(&mut app).await;
    let interaction = interaction_of(&composite);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
    assert!(interaction.get("hoveredVortexFullId").is_none_or(|value| value.is_null()), "accept must clear sticky vortex hover");
    let selected_vortices = vortices_of(&composite).iter().filter(|entry| entry.get("selected").and_then(Value::as_bool) == Some(true)).count();
    assert_eq!(selected_vortices, 0, "one-shot accept must leave no sticky vortex selection");
}

/// 🧹️ A failed place (unknown vortex) must still close the suggestion menu — otherwise
/// `suggestionMenu.open` stays true and every split pane's regular context menu is gated shut.
#[semio_framework_async_macros::async_test]
async fn accept_suggestion_closes_menu_even_when_placement_fails() {
    // 🕹️ `hover_id` dispatches through the real `interactionHover` verb, which resolves the
    // `vortex` domain against `self.registry` — the fixture `app()` is registry-backed for exactly
    // this reason (see its own doc comment).
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
    let before = interaction_of(&render_composite(&mut app).await);
    assert_eq!(before.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    let object_count_before = object_count(&app);
    dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": "missing-object::missing-vortex.as_str()" })), None).await.expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before, "unknown-vortex accept must not place");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()), "failed accept must still dismiss the suggestion menu");
}

/// 📏️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave K: a world scene names its built-in meshes by
/// REFERENCE and never carries their tessellation, so mesh geometry cannot spend the fixed
/// `UiFixedBytes` surface payload. Before this wave `world3d_mesh_kind_entry` inlined
/// `mesh_from_kind`'s buffers, and `vortex-marker` alone (an 80-triangle ico sphere printed as JSON
/// floats) took ~26 KiB of the 32 KiB payload — so a window that also carried a resolved suggestion
/// popup failed `scene-surface.encode` outright. Measured on the live popup-open scene AND on the
/// Nakagin catalog, whose 180 objects are the widest mesh set this editor can name.
#[semio_framework_async_macros::async_test]
async fn the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
    let node = render_composite(&mut app).await;
    assert_eq!(interaction_of(&node).pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "the law only measures the popup-open scene");
    let live_meshes = scene_meshes_of(&node);
    let (payload, capacity) = world_surface_payload_bytes(&mut app, main::BODY_KEY).await;
    assert!(payload <= capacity, "the popup-open world scene packs to {payload} bytes, over the fixed surface capacity {capacity}");
    for meshes in [live_meshes, parse(&main::world_meshes_json(&nakagin_fixture())).expect("nakagin meshes").as_array().cloned().expect("mesh array")] {
        assert!(!meshes.is_empty(), "a world scene always declares its meshes");
        assert!(to_json_string(&meshes).len() <= PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET, "mesh references must stay a rounding error against the {capacity}-byte surface payload; observed {}", to_json_string(&meshes).len());
        for mesh in &meshes {
            assert!(mesh.get("id").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "every scene mesh is identified: {mesh}");
            assert!(mesh.get("data").is_none(), "a scene mesh must never carry inline geometry: {mesh}");
            assert!(mesh.get("kind").and_then(Value::as_str).is_some() || mesh.get("url").and_then(Value::as_str).is_some(), "every scene mesh resolves by kind or by url: {mesh}");
        }
        assert!(meshes.iter().any(|mesh| mesh.get("kind").and_then(Value::as_str) == Some("vortex-marker")), "the vortex marker is the kind that used to blow the payload and must still be declared");
    }
}

//#region 🚚️PagedSceneCarrier
/// 🚚️ The one language-neutral declaration of the world-3d lane carrier — the SAME file the scene
/// crate's Rust lane table and `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`'s `WORLD3D_SCENE_LANES` are
/// pinned against, read here so the real Nakagin publication is measured against the same bounds the
/// TypeScript assembler enforces.
const PUZZLE3D_WORLD3D_LANE_CONTRACT: &str = include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json");

/// 🗼️ Loads the Nakagin Capsule Tower example through the REAL typed `setActiveExample` command and
/// drives it to quiescence, exactly as the navbar picker does.
async fn nakagin_app() -> Puzzle3dApp {
    let mut app = app().await;
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample is a declared puzzle3d command");
    app.dispatch_typed(command, &meta("local")).await.expect("setActiveExample mints its retained whole-document operation");
    settle(&mut app).await;
    app
}

/// 🚚️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave P — LAW (a). The Nakagin Capsule Tower with its
/// vortex suggestion popup open is the document that made `scene-surface.encode` refuse the whole
/// refresh (`surface payload exceeds fixed capacity with 57281 bytes`, browser rebuild #25). It now
/// publishes as a small spine inside the fixed-capacity doc plus one individually paged carrier per
/// payload lane, and every one of those carriers must obey the SAME bounds the TypeScript assembler
/// enforces on the other side of the wire — leaf bytes, children per node, and a byte/hash manifest
/// that lets a partially arrived lane be told from a settled one.
#[semio_framework_async_macros::async_test]
async fn the_nakagin_world_scene_publishes_every_lane_under_the_page_cap_with_the_popup_open() {
    let contract: Value = parse(PUZZLE3D_WORLD3D_LANE_CONTRACT).expect("world-3d lane contract is json");
    let leaf_bytes = contract.pointer("/carrier/leafBytes").and_then(Value::as_u64).expect("leafBytes") as usize;
    let children_max = contract.pointer("/carrier/childrenMax").and_then(Value::as_u64).expect("childrenMax") as usize;
    let doc_bytes_max = contract.pointer("/carrier/docBytesMax").and_then(Value::as_u64).expect("docBytesMax") as usize;

    let mut app = nakagin_app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");

    let census = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    assert_eq!(census.capacity, doc_bytes_max, "the doc ceiling this law measures against is the contract's own");
    assert!(census.doc_bytes <= census.capacity, "the Nakagin spine packs to {} bytes, over the fixed surface capacity {}", census.doc_bytes, census.capacity);
    assert!(!census.lanes.is_empty(), "a published world scene always carries its payload lanes");

    let declared: Vec<&str> = contract["lanes"].as_array().expect("lanes").iter().map(|lane| lane["bodyKey"].as_str().expect("bodyKey")).collect();
    for lane in &census.lanes {
        assert!(declared.contains(&lane.key.as_str()), "lane carrier {} is not a declared world-3d lane", lane.key);
        assert!(lane.widest_leaf <= leaf_bytes, "lane {} has a {}-byte text leaf, over the {leaf_bytes}-byte cap", lane.key, lane.widest_leaf);
        assert!(lane.widest_children <= children_max, "lane {} has a node with {} children, over the {children_max} cap", lane.key, lane.widest_children);
        assert_eq!(lane.bytes, lane.declared_bytes as usize, "lane {} carrier text disagrees with the byte count its spine declared", lane.key);
        assert_eq!(lane.declared_hash.len(), 16, "lane {} must declare a 16-hex-digit content hash", lane.key);
        assert!(lane.leaves >= 1, "lane {} publishes at least one leaf", lane.key);
        assert_eq!(lane.leaves > children_max, lane.leaf_depth > 1, "lane {} with {} leaves must page into a nested carrier exactly when it outgrows one level of {children_max} children (observed leaf depth {})", lane.key, lane.leaves, lane.leaf_depth);
    }

    let interaction = parse(census.assembled.interaction_json.as_deref().expect("the interaction lane reassembles")).expect("interaction lane is json");
    assert_eq!(interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "the law only measures the popup-open scene");
    let instances = census.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).expect("the instances lane always publishes");
    assert!(parse(&census.assembled.instances_json).expect("instances lane is json").as_array().is_some_and(|array| array.len() > 100), "Nakagin publishes its whole catalog of objects through the instances lane");
    eprintln!(
        "[DEBUG] nakagin popup-open world scene: spine={}B of {}B, {} lanes carrying {}B total ({}), widest lane={}B in {} leaves",
        census.doc_bytes,
        census.capacity,
        census.lanes.len(),
        census.payload_bytes(),
        census.report(),
        instances.bytes,
        instances.leaves
    );
    eprintln!("[DEBUG] nakagin lane paging: {}", census.lanes.iter().map(|lane| format!("{}={}leaves@depth{}", lane.key.trim_start_matches(semio_framework_plugin::WORLD3D_SCENE_LANE_KEY_PREFIX), lane.leaves, lane.leaf_depth)).collect::<Vec<_>>().join(" "));
    assert!(census.payload_bytes() > census.capacity, "this law is only meaningful while the Nakagin payload is past what one fixed doc could ever hold");
}

/// 🚚️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave P — LAW (b). A partial refresh that changes only the
/// camera must leave every lane carrier byte-identical, so the reconciler emits no op for any of
/// them; `instances`, `interaction` and `lod` have very different change rates and the whole point of
/// splitting them is that an unchanged lane costs nothing.
#[semio_framework_async_macros::async_test]
async fn a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh() {
    let mut app = nakagin_app().await;
    let before = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [80.0, 80.0, 80.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), Some(main::WINDOW_KIND_ID)).await.expect("setCamera");
    let moved = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let republished: Vec<&str> = moved.lanes.iter().filter(|lane| before.lane(&lane.key).is_none_or(|previous| previous.declared_hash != lane.declared_hash)).map(|lane| lane.key.as_str()).collect();
    assert!(republished.is_empty(), "a camera move republished {republished:?}");
    assert_ne!(moved.assembled.camera_json, before.assembled.camera_json, "the camera itself must have moved for this law to mean anything");

    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("interactionSelect");
    let picked = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let changed: Vec<&str> = picked.lanes.iter().filter(|lane| before.lane(&lane.key).is_none_or(|previous| previous.declared_hash != lane.declared_hash)).map(|lane| lane.key.as_str()).collect();
    assert!(!changed.contains(&semio_framework_plugin::World3dSceneLane::Instances.body_key()), "a selection change must not republish the instances lane");
    assert_eq!(picked.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).map(|lane| lane.declared_hash.as_str()), before.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).map(|lane| lane.declared_hash.as_str()));
    eprintln!("[DEBUG] nakagin partial refresh: camera move republished 0 of {} lanes; a selection republished {changed:?}", before.lanes.len());
}
//#endregion 🚚️PagedSceneCarrier

/// 📏️ A whole scene's mesh declarations, as references, against the 32 KiB fixed surface payload:
/// generous enough for the widest catalog the editor can name, tight enough that a single inlined
/// primitive (26 KiB) fails it.
const PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET: usize = 4 * 1024;

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_sticky_hover() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🧰️ Context-menu / Alt+right-click suggestions are a one-shot placement: opening and accepting
/// must leave whatever host-owned utility was already active (e.g. transform) untouched.
#[semio_framework_async_macros::async_test]
async fn open_and_accept_vortex_suggestions_preserve_active_utility() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("activate transform");
    let vortex = first_vortex_full_id(&app);
    let open = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), Some(main::WINDOW_KIND_ID)).await.expect("openVortexSuggestions");
    assert!(open.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "opening suggestions must not emit utility/tool switches: {:?}", open.requested_effects);
    let open_node = render_window(&mut app, main::WINDOW_KIND_ID).await;
    let open_interaction = interaction_of(&open_node);
    assert_eq!(open_interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "transform remains non-brush scene mode during suggestions");
    assert_eq!(open_interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    assert!(brush_preview_of(&open_node).get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "one-shot suggestions still emit a placement preview without entering brush mode");
    let accept = dispatch(&mut app, "acceptSuggestion", None, Some(main::WINDOW_KIND_ID)).await.expect("acceptSuggestion");
    assert!(accept.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accepting suggestions must not emit utility/tool switches: {:?}", accept.requested_effects);
    let accept_interaction = interaction_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert!(accept_interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(accept_interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
}

/// 🎰️ Wave B6: `pending_reserved` is direct-mapped on `job % 64`, and a reserved verb that is NOT an
/// interaction (`noteShellCommand` — the shell records one for every user command) holds its residue
/// class until it settles. `retire_pending_reserved_latest_wins` may only retire interaction pendings,
/// so the colliding class used to fault the whole pick with `framework route 'interactionSelect' has no
/// exact pending spawn slot` (browser battery #44-pre) while 63 classes stood empty. 64 rounds cover
/// every residue class, so at least one of them mints into the occupied one.
#[semio_framework_async_macros::async_test]
async fn interaction_admits_in_every_residue_class_while_a_reserved_verb_stays_pending() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    dispatch_reserved_unsettled(&mut app, "noteShellCommand", Some(&json!({ "commandId": "framework.shell.b6-probe", "label": "Probe" })), None).await.expect("a non-interaction reserved verb admits and holds its residue class");
    for round in 0..64 {
        let admitted = select_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.unwrap_or_else(|error| panic!("round {round} must mint into a vacant residue class: {error:?}"));
        settle_reserved(&mut app, admitted).await.unwrap_or_else(|error| panic!("round {round} must commit: {error:?}"));
    }
}

/// 🖱️ Wave W-AB: a vortex pointermove storm admits 70 `interactionHover`s (then a click
/// `interactionSelect`) before Isolated reserved jobs finish — the #38 spawn-admit drop. Latest-wins
/// keeps one pending hover so the last target commits, brush preview publishes, and place lands.
#[semio_framework_async_macros::async_test]
async fn vortex_hover_storm_admits_then_brush_preview_and_place() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let vortex = first_vortex_full_id(&app);
    let before = object_count(&app);
    let mut last = None;
    for _ in 0..70 {
        last = Some(hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("vortex hover storm must keep admitting"));
    }
    settle_reserved(&mut app, last.expect("storm admitted at least one hover")).await.expect("latest hover must commit");
    let select_admit = select_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("vortex click select must admit after the hover storm");
    settle_reserved(&mut app, select_admit).await.expect("select commit");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("suggestionsTick");
    }
    let node = render_composite(&mut app).await;
    let preview = brush_preview_of(&node);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "hover storm must still publish a brush preview: {preview}");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "brush preview must name a kind: {preview}");
    dispatch(&mut app, "addBrushObject", Some(&preview), None).await.expect("addBrushObject from published preview");
    assert!(object_count(&app) > before, "vortex click place must land an object from the published preview");
}

/// 🪟️ Wave B9 lane 1: `ViewModel.active_utility_by_window_id` is keyed by window INSTANCE
/// (`puzzle3d-main-perspective`), and `plugin_refresh_ui` renders EVERY instance under the window
/// KIND's one body key — the instance identity reaches the guest only through
/// `ViewModel::for_window_instance`'s `window_id`. Resolving the roster's first pane instead
/// published that pane's armed utility into every other pane's world lane, which is why the browser
/// read `utility=select preview=null` in the pane the user had armed Brush in.
#[semio_framework_async_macros::async_test]
async fn each_window_instance_publishes_its_own_armed_utility_into_its_world_lane() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    let top = "puzzle3d-main-top";
    let arm = |utility: &'static str, window: &'static str| json!({ "utilityId": utility, "windowId": window });
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&arm(utilities::volume_brush::UTILITY_ID, main::WINDOW_KIND_ID)), Some(main::WINDOW_KIND_ID)).await.expect("arm the base pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&arm(utilities::brush::UTILITY_ID, perspective)), Some(perspective)).await.expect("arm brush in the perspective pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "", "windowId": top })), Some(top)).await.expect("the top pane stays unarmed");
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(perspective)).await.expect("suggestionsTick");
    }
    let armed = render_window_refresh(&mut app, main::BODY_KEY, perspective).await;
    let unarmed = render_window_refresh(&mut app, main::BODY_KEY, top).await;
    let utility_of = |node: &Value| interaction_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(utility_of(&armed), "brush", "the pane the user armed must publish its OWN utility: {}", interaction_of(&armed));
    assert_eq!(utility_of(&unarmed), "select", "an unarmed pane must not inherit another pane's utility: {}", interaction_of(&unarmed));
    let preview = brush_preview_of(&armed);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the armed pane's world lane must carry a brush preview: {preview}");
    assert!(brush_preview_of(&unarmed).is_null(), "an unarmed pane publishes no brush preview: {}", brush_preview_of(&unarmed));
}

/// 🪟️ Wave B39: the OTHER host route — `plugin_render_surface`'s `<body>:<windowInstanceId>` form,
/// the one a mounted surface is rendered through (`🪟️surfaces/🦀️.rs`'s per-surface binding). Two
/// mounted instances of one window kind must publish two DISTINCT world bodies, each carrying its
/// own armed utility, so the host's two `📃️UiDocumentStore`s never hold the same record: a body key
/// that resolved the roster's first pane made both surfaces publish one tree, and the host cannot
/// tell two identical trees apart.
#[semio_framework_async_macros::async_test]
async fn two_mounted_instances_publish_two_distinct_world_bodies_with_their_own_utilities() {
    let mut app = app().await;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    let top = main::WINDOW_INSTANCE_TOP;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID, "windowId": perspective })), Some(perspective)).await.expect("arm brush in the perspective pane");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "", "windowId": top })), Some(top)).await.expect("the top pane stays unarmed");
    let armed = render_window(&mut app, perspective).await;
    let unarmed = render_window(&mut app, top).await;
    let utility_of = |node: &Value| interaction_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(utility_of(&armed), utilities::brush::UTILITY_ID, "the armed pane's own surface body must carry its utility: {}", interaction_of(&armed));
    assert_eq!(utility_of(&unarmed), "select", "the sibling pane's surface body must stay unarmed: {}", interaction_of(&unarmed));
    assert_ne!(armed, unarmed, "two mounted instances must publish two distinct world bodies, never one shared tree");
}

/// 🖱️ Wave W-AB: after a committed vortex hover, Alt+right-click (`openVortexSuggestions` with the
/// hovered `fullId`) publishes `suggestionMenu.open` — the guest half of the host gesture.
#[semio_framework_async_macros::async_test]
async fn vortex_hover_then_open_vortex_suggestions_publishes_menu() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let mut last = None;
    for _ in 0..70 {
        last = Some(hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("suggestion hover storm must keep admitting"));
    }
    settle_reserved(&mut app, last.expect("storm")).await.expect("latest hover commits for suggestions");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 24.0 })), None).await.expect("openVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "vortex hover + openVortexSuggestions must publish the menu: {interaction}");
    assert_eq!(interaction.pointer("/suggestionMenu/vortexFullId").or_else(|| interaction.pointer("/suggestionMenu/vortex_full_id")).and_then(Value::as_str), Some(vortex.as_str()));
}

/// Hover-committed leftover in brush mode publishes `brushPreviewJson` without a click-select.
/// Preview is a world scene lane (not an Effect); leftover InteractionView cannot carry it —
/// `suggestionsTick` must finish the hovered target so the dirty world body encodes the lane.
#[semio_framework_async_macros::async_test]
async fn hover_committed_in_brush_publishes_preview() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("suggestionsTick");
    }
    let preview = brush_preview_of(&render_composite(&mut app).await);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "hover-committed brush must publish preview for the hovered vortex: {preview}");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "hover-committed preview must name a kind: {preview}");
}

/// 🎯️ Wave B18: the browser `suggestionsTick` addresses the window KIND (`puzzle3d-main`) while
/// `SET_ACTIVE_UTILITY` keys the map by instance. Kind-valued `view.window_id` must still hit the
/// armed pane so the tick warms `brush_live_target` and `render_body` publishes `brushPreviewJson`.
#[semio_framework_async_macros::async_test]
async fn suggestions_tick_at_the_window_kind_still_publishes_the_instance_brush_preview() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID, "windowId": perspective })), Some(perspective)).await.expect("arm brush on the instance");
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("kind-addressed suggestionsTick");
    }
    let preview = brush_preview_of(&render_window_refresh(&mut app, main::BODY_KEY, perspective).await);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "kind-addressed tick must still publish the instance pane preview: {preview}");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "kind-addressed tick preview must name a kind: {preview}");
}

/// 🎯️ Wave B24: leftover `refresh-ui` / `plugin_render_surface` builds a fresh `Puzzle3dPlayApp`
/// and checks the session slot back out. `brush_live_target` used to die on check-in (collision/fill
/// only), so SurfaceVisible projected the boot spine (`preview=0`) after the same tick logged
/// `preview=252…313`. Clearing guest hover reproduces the leftover skip-clear; the latched target
/// must still publish the instance preview.
#[semio_framework_async_macros::async_test]
async fn leftover_refresh_after_suggestions_tick_still_publishes_latched_brush_preview() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID, "windowId": perspective })), Some(perspective)).await.expect("arm brush on the instance");
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("kind-addressed suggestionsTick");
    }
    let cleared = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, None).await.expect("leftover skip-clear hover");
    settle_reserved(&mut app, cleared).await.expect("clear leftover hover");
    let preview = brush_preview_of(&render_window_refresh(&mut app, main::BODY_KEY, perspective).await);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "leftover refresh-ui must republish the latched tick preview without guest hover: {preview}");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "latched leftover preview must name a kind: {preview}");
}

/// ⏰️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B35: the bounded warm-up budget a brush preview may
/// cost. One tick spends up to eight `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US` slices on the ONE target the
/// render is asking for, and the live browser warmed a cold rim vortex in exactly one tick / three
/// slices (wave B33, `tick-before free=0 pending=true → tick-after free=4 … slices=3`). Two is that
/// measurement plus one tick of headroom, deliberately far below
/// [`PUZZLE3D_BRUSH_PICKER_TICKS`]: a preview that needs more than a couple of the host's 120 ms ticks
/// is not a slow preview, it is a lane that stopped asking.
const PUZZLE3D_BRUSH_WARM_TICKS: usize = 2;

/// 🖌️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B35: hovering ONE vortex on a freshly armed pane must
/// publish `brushPreviewJson` within [`PUZZLE3D_BRUSH_WARM_TICKS`] host ticks. The live browser gate
/// (`brushPreview.gate reason=no-free-candidate vortex=… free=0 pending=true index=0`) is a cold
/// candidate cache, and a cold cache is only ever a *number of ticks* — this law is what makes that
/// number small and stated instead of "eventually".
#[semio_framework_async_macros::async_test]
async fn one_hover_warms_the_brush_preview_within_the_declared_tick_budget() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID, "windowId": perspective })), Some(perspective)).await.expect("arm brush on the instance");
    // 🖱️ The LAST vortex, never the first: the brush lane's background round-robin enumerates the
    // document in order, so a preview for vortex 0 is warmed by the queue no matter what the tick asks
    // for, and a law written on it cannot tell a targeted warm-up from the enumeration walking past. The
    // browser hovers rim vortices deep in that enumeration (`seed-left-001:v3`, `:v8`), which is why it
    // sees the gate at all.
    let vortex = vortex_full_ids(&app).last().cloned().expect("the fixture publishes vortices");
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    let mut ticks = 0;
    let mut preview = brush_preview_of(&render_window_refresh(&mut app, main::BODY_KEY, perspective).await);
    while preview.get("objectKindId").and_then(Value::as_str).is_none_or(str::is_empty) && ticks < PUZZLE3D_BRUSH_WARM_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("kind-addressed suggestionsTick");
        ticks += 1;
        preview = brush_preview_of(&render_window_refresh(&mut app, main::BODY_KEY, perspective).await);
    }
    eprintln!("[DEBUG] brush warm ticks={ticks} budget={PUZZLE3D_BRUSH_WARM_TICKS} preview={preview}");
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "one hover must warm the hovered vortex within {PUZZLE3D_BRUSH_WARM_TICKS} ticks, got {preview} after {ticks}");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "a warmed preview names a kind: {preview} after {ticks} ticks");
}

/// 🧊️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B35: the forever-gate. The render resolves its preview
/// target through the LATCH (`brush_live_target`) whenever the leftover `refresh-ui` shape has no guest
/// hover, and every mesh upload / document edit drops `brush_cache` — so the pair "hover cleared, cache
/// invalidated" is the live browser's normal state, not an edge. `suggestionsTick` used to resolve its
/// own target from the menu and the hover ONLY, so in exactly that state it ran with `target=None`
/// while the render asked for the latched vortex, and the gate printed
/// `reason=no-free-candidate free=0 pending=true` for as long as the pane stayed armed. The tick must
/// warm whatever the render asks for, and it must do so inside the same declared tick budget.
#[semio_framework_async_macros::async_test]
async fn the_tick_rewarms_the_latched_brush_target_after_an_edit_invalidated_its_candidates() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID, "windowId": perspective })), Some(perspective)).await.expect("arm brush on the instance");
    // 🖱️ A LATE vortex, for the same reason the sibling law above states: the background
    // round-robin warms vortex 0 on its own.
    let vortex = vortex_full_ids(&app).last().cloned().expect("the fixture publishes vortices");
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("warm the latch");
    }
    let cleared = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, None).await.expect("leftover skip-clear hover");
    settle_reserved(&mut app, cleared).await.expect("clear leftover hover");
    // 🥽️ The browser's own invalidator, not a stand-in: `install_collision_mesh` clears `brush_cache`
    // and re-arms the broad phase on EVERY accepted upload, and the shell fires hundreds of them per
    // example (wave B22 measured 202 `registerBrushMesh` commands for one). Geometry no other law in
    // this binary derives, so the content-addressed store cannot adopt it by id and the install really
    // runs.
    let (positions, indices) = crate::standards::v1::subsets::any::schema::precompute_model_tests::context::seeded_cube_mesh_buffers(41.0);
    let position_bytes: Vec<u8> = positions.iter().flat_map(|value| value.to_le_bytes()).collect();
    let index_bytes: Vec<u8> = indices.iter().flat_map(|value| value.to_le_bytes()).collect();
    let upload = json!({
        "surfaceId": "world-3d",
        "url": "/test/b35-relatch-invalidator.glb",
        "digest": crate::editor::puzzle3d::precompute::brush_mesh_digest(&positions, &indices),
        "page": 0,
        "pageCount": 1,
        "positionsB64": semio_framework_io_base64::base64_standard_encode(&position_bytes),
        "indicesB64": semio_framework_io_base64::base64_standard_encode(&index_bytes),
    });
    dispatch(&mut app, "registerBrushMesh", Some(&upload), Some(perspective)).await.expect("one accepted mesh upload invalidates the candidate cache");
    let mut ticks = 0;
    let mut preview = brush_preview_of(&render_window_refresh(&mut app, main::BODY_KEY, perspective).await);
    while preview.get("objectKindId").and_then(Value::as_str).is_none_or(str::is_empty) && ticks < PUZZLE3D_BRUSH_WARM_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("kind-addressed suggestionsTick");
        ticks += 1;
        preview = brush_preview_of(&render_window_refresh(&mut app, main::BODY_KEY, perspective).await);
    }
    eprintln!("[DEBUG] brush relatch ticks={ticks} budget={PUZZLE3D_BRUSH_WARM_TICKS} preview={preview}");
    assert_eq!(
        preview.get("targetVortexFullId").and_then(Value::as_str),
        Some(vortex.as_str()),
        "a tick with no hover must still warm the vortex the render latched, got {preview} after {ticks} ticks",
    );
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "the re-warmed latched preview names a kind: {preview} after {ticks} ticks");
}

/// Click place after hover-committed preview dispatches real `addBrushObject` with that vortex id.
#[semio_framework_async_macros::async_test]
async fn hover_committed_click_places_via_published_preview() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let vortex = first_vortex_full_id(&app);
    let before = object_count(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("suggestionsTick");
    }
    let preview = brush_preview_of(&render_composite(&mut app).await);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "place must use the hovered vortex preview: {preview}");
    dispatch(&mut app, "addBrushObject", Some(&preview), None).await.expect("addBrushObject from published hover preview");
    assert!(object_count(&app) > before, "hover-committed click must land an object via addBrushObject");
}


/// leftover InteractionView after vortex-domain interactionHover must carry the vortex full id.
#[semio_framework_async_macros::async_test]
async fn interaction_hover_leftover_carries_vortex_full_id() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("vortex interactionHover admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("vortex interactionHover leftover");
    let view = settled.output.get("interactionView").expect("leftover InteractionView");
    let hover = view.get("hoverTarget").expect("hoverTarget on leftover");
    assert_eq!(hover.get("id").and_then(dsl::DslValue::as_str), Some(vortex.as_str()), "leftover hoverTarget must be the vortex full id, got {hover:?}");
    assert_eq!(hover.get("domain").and_then(dsl::DslValue::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "next scene render must project leftover hover onto hoveredVortexFullId");
}


//#endregion 🔖️Suggestions

//#region 🔖️WindowOptions
#[semio_framework_async_macros::async_test]
async fn grid_window_options_control_one_visible_grid_spacing() {
    let mut app = app().await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), None).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 7.5 })), None).await.expect("setGridSpacing");
    let lod = lod_of(&render_composite(&mut app).await);
    assert_eq!(lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(lod.get("gridFactor").and_then(Value::as_f64), Some(7.5));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid")), Some(None));
    assert_eq!(find_measure_slider(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(7.5));
}

/// 🪟️ Two window instances of the same kind (a split top/perspective pane pair) must never share
/// window options — toggling grid visibility in one instance must leave every other instance's
/// grid untouched, both in its measures chrome and in its own rendered scene.
#[semio_framework_async_macros::async_test]
async fn window_options_are_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut reopened = app().await;
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let base_view = app.window_view(main::WINDOW_KIND_ID);
    let second_view = app.window_view(second_window);
    let document_before = projection_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before exact window publications");

    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 2.5 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing on base window");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(second_window)).await.expect("setGridVisible on second window");

    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_after = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures_after.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true), "the base window instance's grid must stay visible");
    assert_eq!(find_measure_toggle(measures_after.get(second_window).expect("second measures"), &toggle_id), Some(false), "only the targeted window instance's grid toggles off");
    let base_render = render_window(&mut app, main::WINDOW_KIND_ID).await;
    let second_render = render_window(&mut app, second_window).await;
    assert_eq!(lod_of(&base_render).get("showLodGrid").and_then(Value::as_bool), Some(true));
    assert_eq!(lod_of(&base_render).get("gridFactor").and_then(Value::as_f64), Some(2.5));
    assert_eq!(lod_of(&second_render).get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(projection_of(&app), document_before);
    let app_config_after = app.config_pack().await.expect("app config after exact window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    assert_eq!(app.window_config_generation(&base_view).await.expect("base generation"), Some(1));
    assert_eq!(app.window_config_generation(&second_view).await.expect("second generation"), Some(1));
    let packs = app.window_config_packs().await.expect("exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact window pack");
    }
    assert_eq!(render_window(&mut reopened, main::WINDOW_KIND_ID).await, base_render);
    assert_eq!(render_window(&mut reopened, second_window).await, second_render);
    assert!(close_witness(reopened).expect("reopened app close"));
    assert!(close_witness(app).expect("source app close"));
    eprintln!("[DEBUG] two Puzzle 3D windows isolated and rendered persisted options, preserved document and app config, reloaded exact packs, and reached terminal-empty close");
}

/// 📏 Wave B12: grid / spacing / LOD / vortex-show must publish into Puzzle3dWindowConfig and the
/// measures rail must read the new values. A silent `.ok().into_iter()` drop would leave generation
/// unchanged and the rail bound to the previous snapshot.
#[semio_framework_async_macros::async_test]
async fn window_option_rail_round_trips_grid_lod_and_vortex_into_published_config() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 25.0 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing");
    dispatch(&mut app, "setLodAutomatic", Some(&json!({ "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("setLodAutomatic");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow");
    assert!(!app.has_pending_typed_operations(), "window-option gestures must quiesce");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 4, "each window-option change must publish a WindowConfig generation, never drop");
    let measures = app.window_measures(&view).await;
    let rail = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible")), Some(false));
    assert_eq!(find_measure_slider(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(25.0));
    assert_eq!(find_measure_toggle(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-auto")), Some(false));
    assert_eq!(find_measure_select(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
    eprintln!("[DEBUG] window-option rail round-tripped grid/lod/vortex into published WindowConfig generations={}", after - before);
}

/// 🪟️ The whole per-window option round trip, on the two instances the default layout actually opens
/// (`puzzle3d-main-top` / `puzzle3d-main-perspective`), driven exactly as the measures rail drives it:
/// the rail's own `WindowMeasure::Toggle` args (`{"pressed": …}`) tagged with the instance the rail
/// belongs to. One toggle must (a) advance THAT instance's window-config generation by exactly one,
/// (b) come back through the same instance's measures rail as the new value, (c) reach the world body
/// the instance renders, and (d) leave the sibling instance's rail, body and generation untouched.
///
/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12: the browser symptom was a checkbox that never
/// moved. Every hop below is green, which is what pinned the defect to the two host-side halves — the
/// control rendering the published value with no state of its own while the round trip is in flight,
/// and `Puzzle3dScopeClass::Chrome` making that round trip a full-shell repaint.
#[semio_framework_async_macros::async_test]
async fn window_option_toggle_round_trips_on_its_own_instance_and_leaves_the_sibling_alone() {
    let mut app = app().await;
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let top = app.window_view(main::WINDOW_INSTANCE_TOP);
    let perspective = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": true })), Some(main::WINDOW_INSTANCE_TOP)).await.expect("register the top pane's own config lane");
    let top_generation_before = app.window_config_generation(&top).await.expect("top generation");

    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setGridVisible on the perspective pane");

    let perspective_generation = app.window_config_generation(&perspective).await.expect("perspective generation");
    assert_eq!(perspective_generation, Some(1), "one rail toggle publishes exactly one generation on the instance it was dispatched from");
    assert_eq!(app.window_config_generation(&top).await.expect("top generation"), top_generation_before, "the sibling instance's own config lane must not advance");

    let view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let measures = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective measures"), &toggle_id), Some(false), "the rail re-reads the value its own toggle just published");
    assert_eq!(find_measure_toggle(measures.get(main::WINDOW_INSTANCE_TOP).expect("top measures"), &toggle_id), Some(true), "the sibling rail keeps the value IT published");
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await).get("showLodGrid").and_then(Value::as_bool), Some(false), "the world body the instance renders consumes the published option");
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_INSTANCE_TOP).await).get("showLodGrid").and_then(Value::as_bool), Some(true), "the sibling's world body keeps its own grid");
}

/// ⚙️ The Settings panel's four steppers are addressable, carry the value they configure, and declare
/// the trigger the control they are built from actually fires.
///
/// 🔢️ `uniform` is the load-bearing half: a `NumberStepper` declared `uniform: false` renders MIXED —
/// a blank box with a placeholder instead of the value, and a +/− bump computed from the widget's own
/// `defaultValue` (0) rather than from the setting. These are one window's own scalars, never a
/// multi-selection aggregate, so the honest declaration is `true` (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B12, browser-measured: all four boxes rendered empty).
#[semio_framework_async_macros::async_test]
async fn settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on() {
    fn stepper(node: &Value, id: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(id) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| stepper(child, id)),
            Value::Array(items) => items.iter().find_map(|item| stepper(item, id)),
            _ => None,
        }
    }
    let mut app = app().await;
    let panel: Value = from_json_str(&to_json_string(&render_body(&mut app, settings_panel::BODY_KEY).await)).expect("the settings panel renders parseable ui json");
    for (field, action) in [
        ("overlap-budget", "setBrushPlacementOverlapBudget"),
        ("proximity-radius", "setProximityRadius"),
        ("chunk-size", "setChunkSize"),
        ("grid-spacing", "setGridSpacing"),
    ] {
        let row = stepper(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}")).unwrap_or_else(|| panic!("the settings panel must declare a {field} field row: {panel}"));
        let control = stepper(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}.control")).unwrap_or_else(|| panic!("the {field} row must own the stepper control it labels: {row}"));
        let props = control.get("component").cloned().unwrap_or_else(|| panic!("{field} control is a number stepper: {control}"));
        assert_eq!(props.get("type").and_then(Value::as_str), Some("numberStepper"), "{field} control is a number stepper: {control}");
        assert_eq!(props.get("uniform").and_then(Value::as_bool), Some(true), "{field} configures ONE window's own scalar, so it can never render as a mixed selection: {props}");
        assert!(props.get("value").and_then(Value::as_f64).is_some(), "{field} must carry the value it configures: {props}");
        let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
        assert_eq!(bindings.len(), 1, "{field} declares exactly the one trigger it handles: {bindings:?}");
        assert_eq!(bindings[0].get("trigger").and_then(Value::as_str), Some("change"), "{field} handles absolute values, so it binds Change and NOT Delta: {bindings:?}");
        assert_eq!(bindings[0].pointer("/action/name").and_then(Value::as_str), Some(action), "{field} dispatches {action}: {bindings:?}");
        assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(main::WINDOW_KIND_ID), "{field} is scoped at the window the panel was rendered for, so the value it publishes reaches THAT window's rail: {bindings:?}");
    }
}

/// 🐢️ Every control the measures rail and the Settings panel bind to a per-window option resolves to
/// [`Puzzle3dScopeClass::WindowOption`], and that class paints exactly the three surfaces those options
/// can move: the world body that re-derives from them, the measures rail that renders them, and the
/// Settings panel that mirrors four of the same fields. Never the outliner, catalogue, inspector,
/// history, labels, utilities or tool measures — and never `Full`, which is what these verbs used to
/// fall through to (one grid toggle re-rendered the whole shell before its own checkbox could move).
#[semio_framework_async_macros::async_test]
async fn window_option_verbs_paint_only_their_window_their_rail_and_the_settings_panel() {
    use crate::editor::puzzle3d::{puzzle3d_command_scope_class, puzzle3d_scope, Puzzle3dScopeClass};
    for action in [
        "setGridVisible",
        "setGridSnapEnabled",
        "setGridSpacing",
        "setLodAutomatic",
        "setLodDepthVariable",
        "setLodManual",
        "setVortexShow",
        "setVortexDirection",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setSelectableKind",
        "setTransformGumballFlag",
        "setProximityRadius",
        "setChunkSize",
        "setVoxelDims",
        "setProjection",
        "setProjectionParam",
    ] {
        assert_eq!(puzzle3d_command_scope_class(action), Puzzle3dScopeClass::WindowOption, "{action} is a per-window option and must be classified as one");
    }
    let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = puzzle3d_scope(Puzzle3dScopeClass::WindowOption) else {
        panic!("the window-option class is a narrowed scope, never Full");
    };
    assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
    assert_eq!(panel_bodies, vec![settings_panel::BODY_KEY.to_string()], "the Settings panel mirrors four window options and must never disagree with the rail");
    assert!(measures, "the rail's own controls are bound to the value this publishes");
    assert!(!utilities && !tools && !engagements && !labels, "a window option moves no other chrome");
    for body in [inspection::BODY_KEY, document::BODY_KEY, catalogue::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        assert!(!panel_bodies.iter().any(|named| named == body), "a window option cannot move the {body} panel");
    }
}

#[semio_framework_async_macros::async_test]
async fn window_transient_is_exact_instance_local_and_resets_on_reload() {
    let mut reopened = app().await;
    let mut app = app().await;
    let window_a = "puzzle3d-transient-a";
    let window_b = "puzzle3d-transient-b";
    let view_a = app.window_view(window_a);
    let view_b = app.window_view(window_b);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 10.0, "y": 20.0 })), Some(window_a)).await.expect("open suggestions in window a");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window_ownership::Puzzle3dWindowTransientOwner>().and_then(|value| value.suggestion_menu.as_ref()).map(|menu| menu.window_id.as_str()), Some(window_a));
    assert!(transient_b.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    dispatch(&mut app, "closeVortexSuggestions", None, Some(window_a)).await.expect("close suggestions in window a");
    let closed = app.window_transient_snapshot(&view_a).expect("closed transient").expect("closed owner");
    assert!(closed.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    let vortex_again = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex_again })), Some(window_a)).await.expect("open suggestions in window a again");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert!(reset.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(3));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    // 🧷️ Every `window_transient_snapshot` above is a live READ of the partition store, and close is
    // blocked — by design — while one is outstanding. The reads have to be surrendered before the
    // close witness, exactly as a host surrenders a rendered body before retiring its app.
    drop((transient_a, transient_b, closed));
    drop(reset);
    assert!(close_witness(reopened).expect("reopened app close"));
    assert!(close_witness(app).expect("source app close"));
    eprintln!("[DEBUG] Puzzle 3D transient suggestions stayed exact-window isolated, close cleared their owner, reload reset ephemeral state, and both registered apps reached terminal-empty close");
}

/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S. A mounted worker job session is admitted out of a
/// FIXED process-wide array of `semio_framework_job::WORKER_JOB_SESSION_SLOTS` retirement slots, and an
/// app dropped while an operation is still mounted parks its node there instead of freeing it. ONLY a
/// live app's cooperative-maintenance round robin hands that slot back. Without that pump the array
/// filled up for the life of the process: from then on EVERY typed operation had its worker session
/// refused, and a refused session never runs the job, so the operation reached publication with no
/// completion value and the host's continuation loop spun on it forever — which is exactly how one
/// abandoned command turned a whole test binary into `pending typed operations outlived the settle
/// budget` for every later command, including plain view actions.
#[semio_framework_async_macros::async_test]
async fn an_abandoned_app_hands_its_worker_session_admission_back_to_the_next_app() {
    let mut abandoned = app().await;
    dispatch_unsettled(&mut abandoned, "setCamera", Some(&json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("setCamera mounts one typed operation");
    drop(abandoned);
    let mut live = app().await;
    for _ in 0..RETIREMENT_RECLAIM_TURNS {
        if !semio_framework_job::worker_job_retirements_are_parked() {
            break;
        }
        live.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("one cooperative maintenance unit");
    }
    assert!(
        !semio_framework_job::worker_job_retirements_are_parked(),
        "a live app's maintenance round robin must reclaim every parked worker-job retirement slot, or the process runs out of session admissions"
    );
    dispatch(&mut live, "setCamera", Some(&json!({ "position": [4.0, 5.0, 6.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("the next app still quiesces on a plain view action");
}

/// 🔁️ Maintenance units the reclaim law grants: every stage of the round robin, many times over, so the
/// law fails on a missing pump rather than on a tight budget.
const RETIREMENT_RECLAIM_TURNS: usize = semio_framework_plugin::MAINTENANCE_STAGES as usize * 64;

/// 🎥️ `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection` moved off the document —
/// they are View-kind and must never emit VCS operations, no matter what they mutate.
#[semio_framework_async_macros::async_test]
async fn camera_actions_are_view_actions_that_emit_no_artifact_mutations() {
    let app_definition = create_puzzle3d_app();
    for action_id in ["setCamera", "setProjection", "setProjectionParam", "focusSelection"] {
        let def = app_definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("{action_id} declared"));
        assert_eq!(def.kind, ActionKind::View, "{action_id} must be a View action — camera is session-only, never a VCS edit");
    }
    let mut live = app().await;
    let before = projection_of(&live);
    let result = dispatch(&mut live, "setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.5 } })), None).await.expect("setCamera");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&live), before, "setCamera must not mutate the document");
}

/// 🎥️ Wave B11 (checklist §22, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 14): `f` with
/// nothing selected completed with a bare `Emit::default()` — the camera never moved, no notice was
/// raised, and the key read as dead on every boot before the user had picked anything. A camera verb
/// with no selection has an obvious subject (the whole document), so focus must FRAME it — while still
/// writing only the per-window camera, never a document operation.
#[semio_framework_async_macros::async_test]
async fn focus_selection_frames_the_whole_document_when_nothing_is_selected() {
    let mut app = app().await;
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [900.0, 900.0, 900.0], "target": [900.0, 900.0, 900.0], "zoom": 1.0 } })), Some(main::WINDOW_KIND_ID)).await.expect("park the camera far away");
    let parked = camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    let document_before = projection_of(&app);
    let result = dispatch(&mut app, "focusSelection", None, Some(main::WINDOW_KIND_ID)).await.expect("focusSelection with an empty selection must complete");
    let framed = camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_ne!(framed, parked, "focus with nothing selected must frame the document instead of leaving the camera parked");
    assert!(result.mutations.is_empty(), "focus is camera-only — it must emit no artifact history entry: {:?}", result.mutations);
    assert_eq!(projection_of(&app), document_before, "focus must not touch the document");
    let objects = document_before.get("objects").and_then(Value::as_array).expect("the example carries objects");
    let first = objects.first().and_then(|object| object.get("origin")).cloned().expect("an object origin");
    assert_eq!(framed.pointer("/target/0").and_then(Value::as_f64), first.pointer("/0").and_then(Value::as_f64), "a one-object document frames that object");
}

/// 🪟️ ONE per-window setting is ONE window-config publication that QUIESCES. The lane's own bounded
/// preparation used to demand the owner's whole declared publication ceiling
/// (`Puzzle3dWindowConfigOwner::MAXIMUM_PUBLICATION_BYTES`, 65 536) out of the publisher's per-turn
/// byte grant (`TYPED_OPERATION_RESULT_PAGE_BYTES`, 4 096) and answered `Blocked` on every single
/// turn, so `setCamera` — and every projection/grid/vortex/window option behind the same lane — sat
/// in stage `Publishing` forever and nothing queued behind it could ever run. The law is exactly:
/// one mutation, one generation, no pending operation left over.
#[semio_framework_async_macros::async_test]
async fn one_window_config_mutation_publishes_exactly_one_generation_and_quiesces() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [7.0, 8.0, 9.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), None).await.expect("setCamera");
    assert!(!app.has_pending_typed_operations(), "one window-config gesture must leave no typed operation pending");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 1, "one window-config mutation publishes exactly one store generation, never zero and never a spin");
    assert_eq!(camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).pointer("/position/0").and_then(Value::as_f64), Some(7.0), "the published window config is what the window renders");
}

/// 🛰️📷️ Wave B50: `setCamera` must move the pose the REFRESH route publishes, not only the one the
/// `<body>:<windowInstanceId>` surface route does. `plugin_refresh_ui` renders a window section as
/// `app.render(body_key, None, view.for_window_instance(key))` (`🔌️plugin/🦀️.rs:33790`) — the
/// instance reaches the guest through the VIEW alone, with no instance-suffixed body key — and the
/// section hash it answers is what `uiRefreshSectionUnchanged` compares. Every pre-existing camera
/// law measured [`render_window`] (the surface route), so a refresh route that publishes the opening
/// pose forever read green while the browser froze: wasm #59's battery reported
/// `camera settle puzzle3d-main-perspective moved=false waitedMs=30184` for orbit, pan AND zoom with
/// `refreshUi sections asked=["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"]
/// changed=[] hashes={…"puzzle3d-main-perspective":"aba169d7:1"}` — the same hash after every
/// `setCamera`, on a `full` scope included. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B50.
#[semio_framework_async_macros::async_test]
async fn set_camera_moves_the_pose_the_window_refresh_route_publishes() {
    let mut app = app().await;
    let window = main::WINDOW_INSTANCE_PERSPECTIVE;
    let before = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, window).await);
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window)).await.expect("setCamera on the perspective instance");
    let after = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, window).await);
    assert_ne!(after, before, "the refresh route must publish the gestured pose, not the opening one: before={before} after={after}");
    assert_eq!(after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "the refresh route's published camera is the one `setCamera` wrote: {after}");
    let sibling = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_TOP).await);
    assert_ne!(sibling.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "one instance's gesture must not move its sibling's refresh pose: {sibling}");
}

/// 🪟️📷️ Orbiting one window instance's camera must never move any sibling instance's camera, and
/// must never touch the shared document.
#[semio_framework_async_macros::async_test]
async fn set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched() {
    let mut app = app().await;
    let window_a = "puzzle3d-main-a";
    let window_b = "puzzle3d-main-b";
    dispatch(&mut app, "worldPointerDown", None, Some(window_a)).await.expect("register a");
    dispatch(&mut app, "worldPointerDown", None, Some(window_b)).await.expect("register b");

    let before_document = projection_of(&app);
    let camera_b_before = camera_of(&render_window(&mut app, window_b).await);

    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window_a)).await.expect("setCamera on window A");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&app), before_document, "setCamera must never mutate the shared document");

    let camera_a_after = camera_of(&render_window(&mut app, window_a).await);
    assert_eq!(camera_a_after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "window A's own rendered camera picks up the new pose");
    assert_eq!(camera_of(&render_window(&mut app, window_b).await), camera_b_before, "window B's rendered camera must be unaffected by window A's setCamera");
}

#[semio_framework_async_macros::async_test]
async fn vortex_show_window_option_defaults_to_selected_and_switches_to_always() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_SELECTED));

    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode must hide vortices while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_always = app.window_measures(&view).await;
    let window_measures_always = measures_always.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_always, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
    assert_eq!(vortices_of(&render_composite(&mut app).await).len(), all_vortex_ids.len(), "Always mode must emit every vortex while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_SELECTED })), None).await.expect("setVortexShow selected");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "switching back to Selected must hide idle vortices");
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS));

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let outwards_vortices = vortices_of(&render_composite(&mut app).await);
    assert!(!outwards_vortices.is_empty(), "fixture must expose vortices");
    assert!(outwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), None).await.expect("setVortexDirection inwards");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_inwards = app.window_measures(&view).await;
    let window_measures_inwards = measures_inwards.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_inwards, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS));
    assert!(vortices_of(&render_composite(&mut app).await).iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_option_is_local_to_the_window_instance() {
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).await.expect("register base window");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow always on base");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(second_window)).await.expect("setVortexShow always on second");
    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), Some(second_window)).await.expect("setVortexDirection inwards on second window");

    let base_vortices = vortices_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert!(!base_vortices.is_empty(), "the base window must still emit vortices");
    assert!(base_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    let second_vortices = vortices_of(&render_window(&mut app, second_window).await);
    assert!(second_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}
//#endregion 🔖️WindowOptions

//#region 🔖️Fill
/// 🛠️ Wave B14: Fill is a mode-level tool. A leftover/window `select` utility must not starve
/// `fillBuildTick` — the guest world lane publishes `fill` while `active_tool_id` is fill.
#[test]
fn fill_tool_wins_the_world_lane_over_a_select_window_utility() {
    let mut runtime = Puzzle3dRuntime::default();
    runtime.active_tool_id = Some(fill_tool::TOOL_ID.into());
    let mut view = semio_framework_plugin::ViewModel::default();
    view.active_utility_id = Some("select".into());
    view.active_utility_by_window_id.insert("pane".into(), "select".into());
    assert!(puzzle3d_fill_tool_active(&runtime), "the Fill tab arms the mode tool, not a window utility");
    assert_eq!(puzzle3d_scene_active_utility(&runtime, Some(&view), Some("pane")), fill_tool::TOOL_ID, "guest world lane must stay fill while the tool is armed");
    runtime.active_tool_id = None;
    assert!(!puzzle3d_fill_tool_active(&runtime));
    assert_eq!(puzzle3d_scene_active_utility(&runtime, Some(&view), Some("pane")), "select");
    eprintln!("[DEBUG] fill tool world-lane hop tool=fill utility=select -> {}", fill_tool::TOOL_ID);
}

#[semio_framework_async_macros::async_test]
async fn fill_build_tick_is_ignored_when_fill_tool_is_inactive() {
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `handle_action_impl` now takes
    // an `InteractionView`, which no external crate can construct directly (its fields are
    // `pub(crate)` to `semio_framework_plugin`, no public constructor exists — flagged to the
    // coordinator as a testability gap). Routed through the real `dispatch()`/`with_puzzle3d_app`
    // machinery instead, which builds one internally.
    //
    // 🕹️ Pre-existing (unrelated to this ticket) framework testability gap in
    // `VcsArtifactApp::dispatch_typed`/`finish_recorded`: `dispatch_typed` always tags its call to
    // `finish_recorded` with the literal verb `"typed-command"` (never the real action id), so
    // `finish_recorded`'s `self.registry.get(verb)` lookup can never resolve `fillBuildTick`'s
    // declared `ActionKind::View` — `skip_history_panel` is unreachable via this path regardless of
    // registry population, and every `dispatch_typed` call that logs anything (`log_generation`
    // advances) picks up a `Partial { panel_bodies: ["framework.body.history"] }` refresh. Confirmed
    // present before this ticket too (`finish_recorded`'s registry lookup, not its
    // `ActionKind::View | ActionKind::Interaction` match arm, is what fails) — flagged to the
    // coordinator, not fixed here (framework file, out of this crate's remit). Asserts the real
    // regression guard (no progression while inactive) plus the weaker-but-true scope bound (never
    // a `Full` refresh) instead of the unreachable exact `None`.
    // 🔒️ `fill_progress_summary` falls back to the PROCESS-WIDE envelope registry when this app has no
    // plan of its own, so without the shared guard this law reads whatever a concurrent fill law left
    // admitted and calls it this app's progression. W-F4 §7.6 named exactly this leak.
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": Value::Null })), None).await.expect("deactivate fill");
    let before = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
    for _ in 0..64 {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "an inactive fill tick must never force a full app refresh");
        assert!(result.history_patch.is_none(), "a View-kind verb never dirties the history panel — `finish_recorded` skips the patch by declared kind");
    }
    let after = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
    assert_eq!(after, before, "stale or queued fill ticks must not advance planning after the Fill tool is deactivated");
}

#[semio_framework_async_macros::async_test]
async fn fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    // ⏳️ The document reaches the precompute session through the BOUNDED sync prologue and the
    // admission census spends a bounded unit budget per turn, so the tick that finally hands the
    // envelope to its job is not necessarily the first one — what the law states is that exactly ONE
    // tick out of the run spawns, and that it spawns an isolated `FILL_JOB_KIND` job.
    let mut spawns = 0_usize;
    for _ in 0..FILL_TICK_ADMISSION_TICKS {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        for effect in &result.requested_effects {
            assert!(
                matches!(effect, Effect::SpawnJob { kind, placement: semio_framework_plugin::kernel::JobPlacement::Isolated, .. } if kind == crate::editor::puzzle3d::precompute::FILL_JOB_KIND),
                "the only effect a fill tick may request is one isolated fill job"
            );
            spawns += 1;
        }
    }
    // 🔬️ Read the plan through the SAME path the slider does. A `with_puzzle3d_app` probe builds a
    // fresh, session-less app, so its `fill_progress_summary` answers off the process-wide envelope
    // registry rather than off this app's cursor — it states nothing about this dispatch at all.
    assert_eq!(fill_ready(&mut app).await, 0.0, "the view action must not execute a solver transition inline: nothing is planned until the isolated job is stepped");
    assert_eq!(spawns, 1, "a live fill request is enqueued exactly once, never once per tick");
}

/// ⏱️ Ticks the admission law grants the bounded sync prologue plus the admission census before it
/// expects the one and only `SpawnJob`.
const FILL_TICK_ADMISSION_TICKS: usize = 16;

/// 🔁️ How many host `fillBuildTick` cycles the growth law drives. The production loop runs one tick
/// per completed turn — 140–250 ms — and the guest died after 173 of them, so a law that means to
/// state the retention bound has to outlast that.
const FILL_TICK_GROWTH_CYCLES: usize = 320;

/// 🧾️ How many fill envelopes the whole run may admit. A fill plan is admitted ONCE and then driven
/// by its own bounded job; a completed plan may legitimately be superseded and re-admitted a handful
/// of times as the document changes underneath it. What this number rules out is the failure this
/// law exists for: one fresh admission per tick, each carrying a whole `FillBuilder` (its cloned
/// mesh roots included) and a mounted worker session into the process-wide registry.
const FILL_TICK_ADMISSION_CEILING: u64 = 8;

/// 🧮️ Process-wide retained-heap growth the 320-tick run may add after Fill activation. One live
/// `FillBuilder` plus its bounded job is a few megabytes; a fresh preparation every tick is
/// ~2.8 MiB * 320 = ~896 MiB. Sixty-four mebibytes is enough for one plan and its job slices,
/// and far below the per-tick guest leak this law exists to catch.
const FILL_TICK_HEAP_GROWTH_CEILING: isize = 64 * 1024 * 1024;

/// 🪣️ Activating Fill and letting the host tick must converge on ONE admitted plan that the bounded
/// job actually advances — not on an envelope per tick.
///
/// 🧮️ Retention is stated two ways: registry OCCUPANCY plus admission count (exact under a parallel
/// suite) AND the process-wide [`retained_heap_bytes`] delta from [`Puzzle3dHeapWitness`]. An
/// admitted envelope holds the megabyte-scale owners (`FillBuilder` + `MountedFillWorker`). The
/// guest heap is a fixed 512 MiB wasm linear memory (`.cargo/config.toml`'s `--max-memory=536870912`),
/// so one more envelope per tick is a hard `memory allocation of 16384 bytes failed` trap in
/// production. The witness makes that leak visible natively.
///
/// 🚚️ The jobs are driven through the REAL `reactor::jobs` runtime — `start_job` on the effect the
/// tick requested, then bounded `step_job` slices — never through
/// `drive_enqueued_fill_job_for_test`. That bypass reaches into the registry directly and therefore
/// reports a healthy plan even when no job is ever spawned at all, which is exactly the state the
/// shipped guest was in.
#[semio_framework_async_macros::async_test]
async fn fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    let baseline_heap = retained_heap_bytes();
    let (_, baseline_admissions) = crate::editor::puzzle3d::precompute::fill_envelope_occupancy();
    let mut live: Vec<u64> = Vec::new();
    let mut spawned = 0_usize;
    let mut completed = 0_usize;
    let mut faults: Vec<String> = Vec::new();
    let mut peak_ready = 0.0_f64;
    for _ in 0..FILL_TICK_GROWTH_CYCLES {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        for effect in &result.requested_effects {
            let Effect::SpawnJob { job, kind, input, .. } = effect else { continue };
            if kind != crate::editor::puzzle3d::precompute::FILL_JOB_KIND {
                continue;
            }
            spawned += 1;
            semio_framework_plugin::reactor::jobs::start_job(*job, kind, input).await;
            live.push(*job);
        }
        let mut still = Vec::new();
        for job in live.drain(..) {
            let mut terminal = None;
            for _ in 0..64 {
                match semio_framework_plugin::reactor::jobs::step_job(job, FILL_TICK_JOB_BUDGET).await {
                    semio_framework_plugin::reactor::jobs::JobStep::Running(_) => {}
                    semio_framework_plugin::reactor::jobs::JobStep::Done(_) => {
                        terminal = Some(true);
                        break;
                    }
                    semio_framework_plugin::reactor::jobs::JobStep::Failed(bytes) => {
                        faults.push(String::from_utf8_lossy(&bytes).to_string());
                        terminal = Some(false);
                        break;
                    }
                }
            }
            match terminal {
                Some(true) => completed += 1,
                Some(false) => {}
                None => still.push(job),
            }
        }
        live = still;
        let tick_ready = fill_ready(&mut app).await;
        if tick_ready > peak_ready {
            peak_ready = tick_ready;
        }
    }
    let (occupied, admissions) = crate::editor::puzzle3d::precompute::fill_envelope_occupancy();
    let registry_available = crate::editor::puzzle3d::precompute::fill_envelope_available_count();
    let admitted = admissions.saturating_sub(baseline_admissions);
    let census = format!(
        "{FILL_TICK_GROWTH_CYCLES} ticks admitted {admitted} envelopes ({occupied} still live), spawned {spawned} jobs, completed {completed}, faulted {} ({}), moved {} heap bytes",
        faults.len(),
        faults.first().map_or("none", String::as_str),
        retained_heap_bytes() - baseline_heap
    );
    assert!(spawned > 0, "the fill tick loop never requested a single background plan job: {census}");
    assert!(
        admitted <= FILL_TICK_ADMISSION_CEILING,
        "every tick admitted its own fill envelope instead of advancing the one already admitted — this is the retained megabyte per tick that traps the 512 MiB guest heap: {census}"
    );
    assert!(occupied <= crate::editor::puzzle3d::precompute::FILL_ENVELOPE_MAX_OPERATIONS, "the fill registry may never hold more envelopes than it has slots: {census}");
    let heap_delta = retained_heap_bytes() - baseline_heap;
    assert!(
        heap_delta < FILL_TICK_HEAP_GROWTH_CEILING,
        "retained heap grew {heap_delta} bytes across {FILL_TICK_GROWTH_CYCLES} ticks (ceiling {FILL_TICK_HEAP_GROWTH_CEILING}) — a fresh FillBuilder every tick: {census}"
    );
    let ready = fill_ready(&mut app).await;
    let census = format!("{census}; peak_ready={peak_ready} final_ready={ready} registry_available={registry_available}");
    assert!(
        peak_ready > 0.0 || ready > 0.0 || registry_available > 0,
        "the fill-count slider never left `ready: 0` — background planning produced nothing a user could commit: {census}"
    );
    assert!(ready > 0.0, "the fill-count slider ended at `ready: 0` after planning: {census}");
    // 🧹️ Four process-wide envelope slots: a 320-tick law that walks away from its live plan starves
    // every later fill law of an admission.
    drop(app);
    crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
}

/// ⛽️ One bounded slice per tick, exactly as the host's isolated worker grants it: the point of the
/// law is that the plan advances under the SAME per-turn budget production gives it, not that a test
/// can grind the solver to completion inside one tick.
const FILL_TICK_JOB_BUDGET: semio_framework_plugin::reactor::jobs::JobBudget = semio_framework_plugin::reactor::jobs::JobBudget { fuel: 1, deadline_ms: 1 };

//#region 🪣️FillJobLifetime
/// 🧵️ Slices the long-plan owner spends before it terminalizes on its OWN report. Deliberately above
/// the 65 536-step lifetime cap the React host used to impose on every bounded job
/// (`🔌️PluginRuntime/🟦️.tsx`'s deleted `PLUGIN_JOB_STEP_LIMIT`): a real fill plan places however many
/// pieces the user asked for over the census of a whole document, and the host that cancels it at a
/// constant is the host that trapped the guest mid-run.
const LONG_PLAN_SLICES: u32 = 70_000;

/// 🧪️ A bounded owner whose only property is length — the shortest statement of "larger than one host
/// slice budget" that does not depend on the fill solver's own cost model.
const LONG_PLAN_JOB_KIND: &str = "puzzle3d.test.long-plan";

struct LongPlanBoundedJob {
    remaining: u32,
    cancelled: bool,
}

impl semio_framework_plugin::reactor::jobs::BoundedJob for LongPlanBoundedJob {
    fn step(&mut self, _budget: semio_framework_plugin::reactor::jobs::JobBudget) -> semio_framework_plugin::reactor::jobs::JobStep {
        if self.cancelled {
            return semio_framework_plugin::reactor::jobs::JobStep::Failed(b"long-plan.cancelled".to_vec());
        }
        self.remaining = self.remaining.saturating_sub(1);
        if self.remaining == 0 {
            return semio_framework_plugin::reactor::jobs::JobStep::Done(b"long-plan.done".to_vec());
        }
        semio_framework_plugin::reactor::jobs::JobStep::Running(None)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        None
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        true
    }
}

fn long_plan_job_factory(_job: u64, _input: &[u8]) -> Result<Box<dyn semio_framework_plugin::reactor::jobs::BoundedJob>, Vec<u8>> {
    Ok(Box::new(LongPlanBoundedJob { remaining: LONG_PLAN_SLICES, cancelled: false }) as Box<dyn semio_framework_plugin::reactor::jobs::BoundedJob>)
}

/// 🪣️ A plan bigger than one host slice budget must reach its OWN terminal — no host may end it at a
/// step count of its own choosing.
///
/// The native shard host already obeys this: `ShardLoop::pump` walks `running_jobs` granting one
/// `job_budget_from_grant` per turn and has no lifetime counter at all
/// (`🔌️plugin/🖥️host/🧵️shard/🦀️.rs`). The React host did not — it cancelled at 65 536 steps, which a
/// 1 000-placement fill plan reaches in about forty seconds of ticking, and the cancel then trapped
/// the guest. The law states the reactor-side half of the contract both hosts share: an unchanged
/// per-slice [`JobBudget`] over [`LONG_PLAN_SLICES`] slices is NOT a stall, and the owner alone says
/// when the run is over. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-F5.
#[semio_framework_async_macros::async_test]
async fn a_plan_larger_than_one_host_slice_completes_without_a_host_imposed_cancellation() {
    use semio_framework_plugin::reactor::jobs::{self, JobStep};
    jobs::register_bounded_job_kind(LONG_PLAN_JOB_KIND, long_plan_job_factory as semio_framework_plugin::reactor::jobs::BoundedJobFactory);
    let job = 0xF5_00_00_01_u64;
    jobs::start_job(job, LONG_PLAN_JOB_KIND, &[]).await;
    let mut slices = 0_u32;
    let mut done = None;
    while slices < LONG_PLAN_SLICES.saturating_add(1) {
        slices += 1;
        match jobs::step_job(job, FILL_TICK_JOB_BUDGET).await {
            JobStep::Running(_) => {}
            JobStep::Done(bytes) => {
                done = Some(bytes);
                break;
            }
            JobStep::Failed(bytes) => panic!("a bounded owner that is still running must never be failed by its runtime after {slices} slices: {}", String::from_utf8_lossy(&bytes)),
        }
    }
    assert_eq!(done.as_deref(), Some(b"long-plan.done".as_slice()), "the owner's own terminal is the only terminal: {slices} slices");
    assert_eq!(slices, LONG_PLAN_SLICES, "a bounded owner reaches its terminal on exactly the slices it declared, whatever the host's own step counter says");
}

/// 🛑 Cancelling a fill job that is mid-flight must never trap the guest.
///
/// Production sequence this reproduces, verbatim: the Fill panel publishes `Cancel fill` with the live
/// `(job, operation, generation)`; the host dispatches `cancelFillBuild`; the guest answers
/// `Effect::CancelJob`; the host calls `jobs::cancel-job`, which drops the owner. Before W-F5 that drop
/// released a still-armed [`FillEnvelopeWorkerFaultGuard`], so a user cancel reported the envelope as a
/// FAULT — the `fill_failed` notice on a run the user stopped on purpose — and the guest carried on
/// against a torn envelope. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-F5.
#[semio_framework_async_macros::async_test]
async fn cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    let mut live: Vec<u64> = Vec::new();
    let mut identity = None;
    for _ in 0..FILL_TICK_GROWTH_CYCLES {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        if fill_ready(&mut app).await > 0.0 {
            identity = fill_cancel_identity(&mut app).await;
            if identity.is_some() {
                break;
            }
        }
    }
    let (job, operation, generation) = identity.expect("the Cancel fill affordance publishes the live job identity once planning has produced readiness");
    let cancel = dispatch(&mut app, "cancelFillBuild", Some(&json!({ "job": job, "operation": operation, "generation": generation })), None).await.expect("cancelFillBuild");
    let cancelled: Vec<u64> = cancel
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::CancelJob { job } => Some(*job),
            _ => None,
        })
        .collect();
    assert_eq!(cancelled, vec![job], "a cancel naming the live run asks the host to stop exactly that job: {:?}", cancel.requested_effects);
    for job in &cancelled {
        semio_framework_plugin::reactor::jobs::cancel_job(*job).await;
        live.retain(|live_job| live_job != job);
    }
    let mut notices: Vec<String> = Vec::new();
    for _ in 0..FILL_TICK_ADMISSION_TICKS {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick after cancel");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        notices.extend(result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }));
    }
    assert!(
        !notices.iter().any(|message| message == Puzzle3dLabels::NATIVE_EN.fill_failed.as_str()),
        "a run the user cancelled is not a failure — the fault guard must not outlive a deliberate cancel: {notices:?}"
    );
    // 🔁️ Not `is_none`: once the cancelled envelope has given its slot back the tick loop legitimately
    // admits a FRESH plan, and the panel offers to cancel THAT one. What may never happen is the
    // affordance still naming the run the user already stopped.
    assert_ne!(fill_cancel_identity(&mut app).await, Some((job, operation, generation)), "the Cancel fill affordance must stop naming the run it already cancelled");
    let (occupied, _) = crate::editor::puzzle3d::precompute::fill_envelope_occupancy();
    assert!(occupied < crate::editor::puzzle3d::precompute::FILL_ENVELOPE_MAX_OPERATIONS, "a cancelled envelope must give its slot back: {occupied} still live");
    drop(app);
    crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
}

/// 🛑 The other cancel order, and the one the browser actually took: the HOST cancels a job the guest
/// never asked it to, so `jobs::cancel-job` arrives with no `cancelFillBuild` before it and no live
/// cancel token already tripped. That is what the deleted `PLUGIN_JOB_STEP_LIMIT` did at 65 536 steps,
/// and it must be survivable on its own terms — an actor whose instance goes away mid-plan takes the
/// same route. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-F5.
#[semio_framework_async_macros::async_test]
async fn a_host_initiated_cancel_of_a_stepping_fill_job_leaves_the_guest_serving_further_ticks() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    let mut live: Vec<u64> = Vec::new();
    let mut stepping = None;
    for _ in 0..FILL_TICK_GROWTH_CYCLES {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        if fill_ready(&mut app).await > 0.0 {
            stepping = live.first().copied();
            if stepping.is_some() {
                break;
            }
        }
    }
    let job = stepping.expect("a fill job that has stepped far enough to publish readiness");
    semio_framework_plugin::reactor::jobs::cancel_job(job).await;
    live.retain(|live_job| *live_job != job);
    let mut notices: Vec<String> = Vec::new();
    for _ in 0..FILL_TICK_ADMISSION_TICKS {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("the guest must keep answering ticks after a host-side cancel");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        notices.extend(result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }));
    }
    assert!(
        !notices.iter().any(|message| message == Puzzle3dLabels::NATIVE_EN.fill_failed.as_str()),
        "a host that stops a job it started is not reporting a plan failure to the user: {notices:?}"
    );
    let (occupied, _) = crate::editor::puzzle3d::precompute::fill_envelope_occupancy();
    assert!(occupied < crate::editor::puzzle3d::precompute::FILL_ENVELOPE_MAX_OPERATIONS, "a host-cancelled envelope must give its slot back: {occupied} still live");
    drop(app);
    crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
}

/// 📏️ Teardown units ONE turn may spend. The reactor grants a turn an 8 ms slice
/// (`⚛️reactor/🔄️turn/🦀️.rs`'s `run_until_deadline(64, …, 8 ms)`) and the framework's maintenance
/// ladder grants `ArtifactEditor::mounted_job_maintenance_step` exactly `maximum_items.min(1)` — ONE
/// item. Eight is that one item plus the close cursor's own fixed stages; anything above it is work
/// the turn took without yielding, which is what the host watchdog reports as
/// `the worker was silent for 18603 ms; outstanding: cancelJob`.
const CANCEL_TEARDOWN_UNITS_PER_TURN: u64 = 8;

/// 📏️ Granted teardown calls the whole post-cancel retirement may take before the registry is quiet —
/// `FILL_ENVELOPE_SESSION_CLOSE_TURNS`, the plugin's own derived budget: every envelope slot, each at
/// most `FILL_ENVELOPE_MAX_ITEMS` retained owners (the ceiling `finish_measurement` admits against),
/// plus the close cursor's stages.
const CANCEL_TEARDOWN_TURNS: usize = crate::editor::puzzle3d::precompute::FILL_ENVELOPE_SESSION_CLOSE_TURNS;

/// 📏️ Maintenance turns the law spends proving the FRAMEWORK ladder reaches the app's own teardown
/// stage. It has to be more than one round of the ladder's round-robin: `mounted_job_maintenance_step`
/// is one stage of `MAINTENANCE_STAGES`, so a plan-sized retirement driven only through this path takes
/// that many turns per unit — which is production's own cadence, and far too slow for a law to grind a
/// whole Nakagin plan through. The law proves reachability here and bounded completeness below.
const CANCEL_TEARDOWN_LADDER_TURNS: usize = semio_framework_plugin::MAINTENANCE_STAGES as usize * 4;

/// 🛑 Escape while the Fill tool is armed on a Nakagin-scale document. The census this law reads is the
/// plugin's own `fill_close_unit_census`: one unit is one `FillEnvelopeTerminalHandle::close_step` —
/// one retired plan owner, or one close-cursor stage. So "the cancel turn stays within the reactor
/// slice" is stated as a unit count, not a wall clock: reset, run ONE turn, read, and the number IS what
/// that turn charged to the guest's single thread. No dispatch, no `jobs::cancel-job` and no maintenance
/// turn may charge more than [`CANCEL_TEARDOWN_UNITS_PER_TURN`], and the teardown the cancel starts must
/// still finish — the registry quiet — within [`CANCEL_TEARDOWN_TURNS`] ordinary maintenance turns.
///
/// 🧊️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B42, battery wasm #56: right after `engagement-abort`
/// PASSed, `shard 0` was terminated by the host watchdog with `outstanding: cancelJob puzzle#1 started
/// 18603 ms ago`, then `shard 0 lost, restoring actors: puzzle#1` and ten guest-death faults for every
/// later verdict.
#[semio_framework_async_macros::async_test]
async fn engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one() {
    use crate::editor::puzzle3d::precompute::{fill_close_unit_census, fill_envelope_close_debt, fill_envelope_close_diagnostics, fill_envelope_occupancy, reset_fill_close_unit_census};
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the Nakagin example through the real typed command");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("arm fill");
    let mut live: Vec<u64> = Vec::new();
    for _ in 0..FILL_TICK_GROWTH_CYCLES {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        if !live.is_empty() && fill_ready(&mut app).await > 0.0 {
            break;
        }
    }
    let identity = fill_cancel_identity(&mut app).await.expect("an armed, planning fill run publishes its cancel identity");
    reset_fill_close_unit_census();
    let result = dispatch(&mut app, "engagementAbort", None, None).await.expect("engagementAbort");
    let abort_units = fill_close_unit_census();
    let cancelled: Vec<u64> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::CancelJob { job } => Some(*job),
            _ => None,
        })
        .collect();
    assert_eq!(cancelled, vec![identity.0], "Escape on an armed Fill tool cancels exactly the run the panel named");
    reset_fill_close_unit_census();
    for job in &cancelled {
        semio_framework_plugin::reactor::jobs::cancel_job(*job).await;
    }
    let cancel_units = fill_close_unit_census();
    let mut worst_turn = 0_u64;
    let mut ladder_units = 0_u64;
    for _ in 0..CANCEL_TEARDOWN_LADDER_TURNS {
        reset_fill_close_unit_census();
        app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("the live-cleanup ladder keeps ticking while the cancelled plan retires");
        let spent = fill_close_unit_census();
        worst_turn = worst_turn.max(spent);
        ladder_units += spent;
    }
    let mut turns = 0_usize;
    let mut quiet = false;
    while turns < CANCEL_TEARDOWN_TURNS {
        turns += 1;
        reset_fill_close_unit_census();
        let step = crate::editor::puzzle3d::precompute::fill_envelope_maintenance_step(1);
        worst_turn = worst_turn.max(fill_close_unit_census());
        if matches!(step, semio_framework_plugin::PluginCloseStep::Complete) && fill_envelope_occupancy().0 == 0 {
            quiet = true;
            break;
        }
    }
    let census = format!(
        "abort_turn={abort_units} units, cancel_job={cancel_units} units, ladder_units={ladder_units} over {CANCEL_TEARDOWN_LADDER_TURNS} maintenance turns, teardown grants={turns} worst_turn={worst_turn} quiet={quiet}, occupancy={:?}, close_position={:?}, close_debt={:?}",
        fill_envelope_occupancy(),
        fill_envelope_close_diagnostics(),
        fill_envelope_close_debt()
    );
    assert!(abort_units <= CANCEL_TEARDOWN_UNITS_PER_TURN, "the engagementAbort turn tore the plan down inline instead of marking it cancelled: {census}");
    assert!(cancel_units <= CANCEL_TEARDOWN_UNITS_PER_TURN, "`jobs::cancel-job` tore the plan down inline — this is the guest turn that never yields: {census}");
    assert!(worst_turn <= CANCEL_TEARDOWN_UNITS_PER_TURN, "one granted teardown call spent more than its granted unit: {census}");
    assert!(ladder_units > 0, "the framework maintenance ladder never reached this app's own teardown stage, so nothing would ever retire the cancelled plan: {census}");
    assert!(quiet, "the teardown the cancel started never finished: {census}");
    println!("puzzle3d.cancel-teardown-census {census}");
    drop(app);
    crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
}

/// 🧾️ The OTHER turns in the same family, measured with the same close-unit census. Every one of them
/// supersedes or abandons a live fill plan, and a session that abandons one is dropped — by
/// `with_puzzle3d_app_for`'s refused check-in, or by `Puzzle3dSessionRegistry::retire`, which drops it
/// while holding the registry mutex. Before wave B42 that drop drained the plan's whole close ladder
/// inline, so ANY of these could be the unyielding turn, not only the cancel.
///
/// 📏️ What is measured is the ADMISSION turn of each command — the one that runs
/// `with_puzzle3d_app_for` against the live session and so is the one that can drop it — plus a round
/// of the maintenance ladder after it. None may charge more than [`CANCEL_TEARDOWN_UNITS_PER_TURN`]
/// close units. Each command gets its own app: the point is the turn's own cost, not what three
/// document-scale commands do to one document in a row.
#[semio_framework_async_macros::async_test]
async fn no_document_scale_turn_retires_a_live_fill_plan_inside_itself() {
    use crate::editor::puzzle3d::precompute::{fill_close_unit_census, reset_fill_close_unit_census};
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut census: Vec<(&str, u64, u64)> = Vec::new();
    for label in ["example-switch", "import-fixture", "delete-selection"] {
        let mut app = app().await;
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the Nakagin example");
        let payload = to_json_string(&projection_of(&app));
        let target = first_object_id(&app);
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("arm fill");
        let mut live: Vec<u64> = Vec::new();
        for _ in 0..FILL_TICK_GROWTH_CYCLES {
            let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
            step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
            if !live.is_empty() && fill_ready(&mut app).await > 0.0 {
                break;
            }
        }
        let (action, args) = match label {
            "example-switch" => ("setActiveExample", Some(json!({ "exampleId": "" }))),
            "import-fixture" => ("importFixture", Some(json!({ "payload": payload }))),
            _ => {
                select_id(&mut app, "object", &target).await.expect("select one object before deleting it");
                ("deleteSelection", None)
            }
        };
        reset_fill_close_unit_census();
        drop(dispatch_unsettled(&mut app, action, args.as_ref(), None).await);
        let admission = fill_close_unit_census();
        let mut worst_ladder = 0_u64;
        for _ in 0..CANCEL_TEARDOWN_LADDER_TURNS {
            reset_fill_close_unit_census();
            drop(app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP));
            worst_ladder = worst_ladder.max(fill_close_unit_census());
        }
        census.push((label, admission, worst_ladder));
        drop(app);
        crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
    }
    let report = census.iter().map(|(label, admission, ladder)| format!("{label}=admission:{admission}/ladder:{ladder}")).collect::<Vec<_>>().join(" ");
    for (label, admission, ladder) in &census {
        assert!(*admission <= CANCEL_TEARDOWN_UNITS_PER_TURN, "the {label} turn retired a live fill plan inside itself instead of leaving it to the maintenance ladder: {report}");
        assert!(*ladder <= CANCEL_TEARDOWN_UNITS_PER_TURN, "a maintenance turn after {label} spent more than its granted teardown unit: {report}");
    }
    println!("puzzle3d.turn-close-units {report}");
}
//#endregion 🪣️FillJobLifetime

/// 🪣️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS §1 decision 2 — locking IS committing. Every
/// candidate the planner accepts becomes a real `create_object`/`connect_vortices` on the NEXT
/// `fillBuildTick`, at most [`FILL_LOCK_PLACEMENTS_PER_TICK`] per tick, and the viewport shows exactly
/// the document: there is no planned-but-ungrafted tail and no render-time ghost any more.
#[semio_framework_async_macros::async_test]
async fn fill_build_tick_locks_planned_placements_into_the_document_in_bounded_chunks() {
    use crate::editor::puzzle3d::precompute::FILL_LOCK_PLACEMENTS_PER_TICK;
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    set_fill_count_and_finish(&mut app, 12, None).await;
    let mut live = Vec::new();
    let mut previous = object_count(&app);
    let mut biggest_step = 0_usize;
    for _ in 0..256 {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        let now = object_count(&app);
        assert!(now >= previous, "a raising fill run may only grow the document: {previous} -> {now}");
        biggest_step = biggest_step.max(now - previous);
        previous = now;
        if now >= object_count_before + 12 {
            break;
        }
    }
    let locked = object_count(&app) - object_count_before;
    assert!(locked > 0, "the fill tick never committed a single locked placement into the document");
    assert!(locked <= 12, "the tick may never commit more than the requested count: {locked}");
    assert!(biggest_step <= FILL_LOCK_PLACEMENTS_PER_TICK, "one tick committed {biggest_step} placements, above the {FILL_LOCK_PLACEMENTS_PER_TICK} per-tick lock chunk");
    assert_eq!(fill_ready(&mut app).await as usize, locked, "the count entry's ready extent is the LOCKED count — what the document actually holds");
    let rendered = render_composite(&mut app).await;
    assert_eq!(instance_count(&rendered), object_count(&app), "what the viewport shows is exactly the document — no render-time ghost tail");
}

/// 🧺️ One fill run is ONE undo entry: `setFillCount` and every `fillBuildTick` chunk carry the same
/// `fill-count` coalesce key, so the whole run amends a single edit.
#[semio_framework_async_macros::async_test]
async fn a_whole_fill_run_coalesces_into_one_undo_entry() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    set_fill_count_and_finish(&mut app, 6, None).await;
    drive_fill_until_ready(&mut app, 2.0).await;
    assert!(object_count(&app) > object_count_before, "the run must have committed something to have anything to undo");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), object_count_before, "one undo restores the whole coalesced fill run");
}

/// 🔽️ Lowering the count deletes the document tail immediately — the command itself emits the
/// `delete_object` mutations through `take_fill_locked_chunk`, so the document visibly shrinks instead
/// of waiting for a background tick — and publishes the new requested count on the config lane.
#[semio_framework_async_macros::async_test]
async fn lowering_the_fill_count_deletes_the_committed_tail_and_publishes_the_count() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    set_fill_count_and_finish(&mut app, 8, None).await;
    let locked = drive_fill_until_ready(&mut app, 4.0).await as usize;
    assert!(locked >= 2, "need a committed tail to delete, got {locked}");
    let target = locked / 2;
    set_fill_count_and_finish(&mut app, target as u32, None).await;
    for _ in 0..64 {
        if object_count(&app) <= object_count_before + target {
            break;
        }
        set_fill_count_and_finish(&mut app, target as u32, None).await;
    }
    assert_eq!(object_count(&app), object_count_before + target, "lowering must delete the committed tail from the document");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(target as f64), "the config lane carries the lowered request verbatim");
    let rendered = render_composite(&mut app).await;
    assert_eq!(instance_count(&rendered), object_count(&app), "a lowered run leaves no ghost behind in the viewport either");
}

/// ♾️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS §1 decisions 1 and 4 — the count is any `u32`.
/// Nothing clamps a request against a planner ceiling; a document that cannot hold the ask reports a
/// stall reason instead.
#[semio_framework_async_macros::async_test]
async fn the_fill_count_has_no_ceiling_and_the_entry_declares_none() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    set_fill_count_and_finish(&mut app, 5_000, None).await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(5_000.0), "a request far above anything planned is kept verbatim — no clamp");
    assert_eq!(find_measure_number_max(tool_measures, "puzzle3d-fill-count"), Some(None), "the count entry declares no maximum");
    assert_eq!(crate::editor::puzzle3d::commands::set_fill_count::parse_count(Some(&json!({ "value": 4_000_000_000u32 }))), 4_000_000_000, "parse_count carries the whole u32 range");
}

/// 🪣️ Fill is document-global: split top/perspective panes must show the exact same committed objects
/// after a commit on either pane.
#[semio_framework_async_macros::async_test]
async fn fill_count_is_shared_across_split_panes() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), Some(top)).await.expect("select fill tool");
    set_fill_count_and_finish(&mut app, 6, Some(top)).await;
    let locked = drive_fill_until_ready(&mut app, 3.0).await as u32;
    assert!(locked >= 1, "need a committed fill prefix to assert cross-pane sync");

    let top_render = render_window(&mut app, top).await;
    let perspective_render = render_window(&mut app, perspective).await;
    assert_eq!(instance_count(&top_render), instance_count(&perspective_render), "both panes must emit the same instance list for the shared document");
    let instance_ids = |node: &Value| -> Vec<String> { instances_of(node).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect() };
    assert_eq!(instance_ids(&top_render), instance_ids(&perspective_render), "top and perspective must show the exact same object ids after a fill commit");

    let reduced = locked.saturating_sub(1);
    set_fill_count_and_finish(&mut app, reduced, Some(perspective)).await;
    let top_after = render_window(&mut app, top).await;
    let perspective_after = render_window(&mut app, perspective).await;
    assert_eq!(instance_count(&top_after), instance_count(&perspective_after));
    assert_eq!(instance_ids(&top_after), instance_ids(&perspective_after));
}

/// 🔢️ The count control is an unbounded `Number` entry whose `ready` extent is the locked count and
/// whose loading ring tracks a live run — the slider's fixed `max` and its client-side `reveal` key are
/// both gone.
#[semio_framework_async_macros::async_test]
async fn fill_count_measure_is_an_unbounded_number_entry_reporting_the_locked_count() {
    let mut session = Puzzle3dPrecomputeSession::new();
    let scene = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
    sync_precompute_session(&mut session, &scene);
    session.precompute_step(1);
    match fill_tool::count_measure(&scene, &session, &Puzzle3dLabels::NATIVE_EN) {
        WindowMeasure::Number { label: Some(label), value, min, max, ready, .. } => {
            assert_eq!(label, Puzzle3dLabels::NATIVE_EN.count.as_str(), "fill count label stays fixed as Count while planning");
            assert_eq!(value, scene.runtime.fill_count as f64, "the entry shows the REQUESTED count, verbatim");
            assert_eq!(min, Some(0.0));
            assert_eq!(max, None, "the count has no ceiling — a document that cannot hold the ask stalls visibly instead");
            let ready = ready.expect("the entry must expose the locked extent");
            assert!(ready >= 0.0 && ready <= value, "the locked extent can never exceed what was requested");
        }
        other => panic!("expected a Number measure, got {other:?}"),
    }
}

/// 🎚️ `runtime.fill_count` is the single source of truth for what the planner is held to, and
/// `sync_precompute_session` is the ONE place it reaches the live session (wave B1 report §2/§6):
/// `SceneConfig` carries no count, so a session nobody tells plans toward the seed default forever and
/// never learns a persisted per-document ask. Idempotent, because every render, every measure pass and
/// every `drive_precompute` re-asserts it.
#[semio_framework_async_macros::async_test]
async fn scene_sync_holds_the_live_planner_to_the_config_fill_count() {
    let mut session = Puzzle3dPrecomputeSession::new();
    let mut scene = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
    assert_eq!(scene.runtime.fill_count, 100, "a fresh runtime asks for the schema default");
    sync_precompute_session(&mut session, &scene);
    assert_eq!(session.fill_requested_count(), 100, "the sync pushes the config count into the live session");
    scene.runtime.fill_count = 250;
    sync_precompute_session(&mut session, &scene);
    assert_eq!(session.fill_requested_count(), 250, "a raised persisted count reaches the planner on the next sync");
    sync_precompute_session(&mut session, &scene);
    assert_eq!(session.fill_requested_count(), 250, "re-asserting the same count is a no-op, not a retarget");
    scene.runtime.fill_count = 40;
    sync_precompute_session(&mut session, &scene);
    assert_eq!(session.fill_requested_count(), 40, "a lowered persisted count reaches the planner too");
}

/// 🔁️ A committed `setFillCount` survives the renders that follow it: the config mutation IS the count,
/// and every later sync re-asserts the same number rather than downgrading it to a stale one.
#[semio_framework_async_macros::async_test]
async fn a_committed_fill_count_survives_the_renders_that_follow_it() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    set_fill_count_and_finish(&mut app, 250, None).await;
    for _ in 0..3 {
        render_composite(&mut app).await;
    }
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(250.0), "three renders later the requested count is still what the user asked for");
}

//#endregion 🔖️Fill

//#region 🔖️Distribution
#[semio_framework_async_macros::async_test]
async fn puzzle3d_normalize_kind_weight_group_redistributes_siblings_proportionally() {
    let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut weights = HashMap::from([("a".to_string(), 0.2), ("b".to_string(), 0.3), ("c".to_string(), 0.5)]);
    weights = puzzle3d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
    let sum: f64 = ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    assert!((sum - 1.0).abs() < 1e-9, "simplex must stay at 1, got {sum}");
    assert!((weights.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 1e-9);
    // b:c were 0.3:0.5 — remainder 0.5 splits 0.3/0.8 and 0.5/0.8
    assert!((weights.get("b").copied().unwrap_or(0.0) - 0.5 * 0.3 / 0.8).abs() < 1e-9);
    assert!((weights.get("c").copied().unwrap_or(0.0) - 0.5 * 0.5 / 0.8).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_vortex_measure_exposes_joint_weight_scaled_by_object() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_weights = HashMap::from([("c-b".to_string(), 0.75), ("b-s".to_string(), 0.25)]);
    let object_weight = *object_weights.get("Object").unwrap();
    let measures = puzzle3d_joint_vortex_measures("Object", object_weight, &vortex_ids, &vortex_weights);
    match &measures[0] {
        WindowMeasure::Slider { value, max, step, disabled, .. } => {
            let expected_joint = puzzle3d_joint_vortex_weight(object_weight, 0.75);
            assert!((*value - expected_joint).abs() < 1e-9, "slider must show P(object)×P(vortex), got {value}");
            assert!((*max - object_weight).abs() < 1e-9, "joint range max is P(object)");
            assert_eq!(*step, Some(object_weight * 0.01), "step tracks 1% of P(object)");
            assert_eq!(*disabled, None);
        }
        other => panic!("expected vortex slider, got {other:?}"),
    }
    let raised = puzzle3d_normalize_kind_weight_group(&object_weights, &object_ids, "Object", 0.8);
    let raised_weight = *raised.get("Object").unwrap();
    let raised_measures = puzzle3d_joint_vortex_measures("Object", raised_weight, &vortex_ids, &vortex_weights);
    match (&measures[0], &raised_measures[0]) {
        (WindowMeasure::Slider { value: before, .. }, WindowMeasure::Slider { value: after, .. }) => {
            assert!(*after > *before, "raising P(object) must raise joint vortex percentages");
            assert!((*after - raised_weight * 0.75).abs() < 1e-9);
        }
        _ => panic!("expected vortex sliders"),
    }
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_distribution_lists_global_vortices_and_joints_sum_to_one() {
    let fixture = nakagin_fixture();
    let object_ids = puzzle3d_kind_ids(&fixture, "objects");
    let vortex_ids = puzzle3d_kind_ids(&fixture, "vortices");
    assert!(object_ids.len() >= 2, "default fixture needs multiple object kinds");
    assert!(vortex_ids.len() >= 2, "default fixture needs multiple vortex kinds");
    let object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_kind_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
    let scene = Puzzle3dScene { fixture, runtime: Puzzle3dRuntime { object_kind_weights, vortex_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: fill_tool::TOOL_ID.into() };
    let distribution_children = puzzle3d_distribution_children(&scene, Some(true));
    assert_eq!(distribution_children.len(), object_ids.len());
    let mut joint_sum = 0.0;
    for measure in &distribution_children {
        let WindowMeasure::Group { children, value: Some(object_weight), .. } = measure else {
            panic!("expected object-kind group");
        };
        assert_eq!(children.len(), vortex_ids.len(), "each object must list the full global vortex catalog");
        let local_sum: f64 = children
            .iter()
            .map(|child| match child {
                WindowMeasure::Slider { value, .. } => *value,
                _ => panic!("expected vortex slider"),
            })
            .sum();
        assert!((local_sum - object_weight).abs() < 1e-6, "under one object joints sum to P(object), not 1");
        joint_sum += local_sum;
    }
    assert!((joint_sum - 1.0).abs() < 1e-6, "all nested joint percentages across objects must sum to 1, got {joint_sum}");
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_object_weight_change_scales_joint_sampling_product() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
    object_weights = puzzle3d_normalize_kind_weight_group(&object_weights, &object_ids, "Object", 0.6);
    let object_weight = *object_weights.get("Object").unwrap();
    let vortex_weight = *vortex_weights.get("c-b").unwrap();
    let joint_before = puzzle3d_joint_vortex_weight(0.5, vortex_weight);
    let joint_after = puzzle3d_joint_vortex_weight(object_weight, vortex_weight);
    assert!(joint_after > joint_before);
}

/// 🚫️ Zero object-kind weight disables every vortex slider under that kind — anything × 0 is 0.
#[semio_framework_async_macros::async_test]
async fn zero_object_kind_weight_disables_joint_vortex_sliders() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let session = Puzzle3dPrecomputeSession::new();
    let fixture = nakagin_fixture();
    let object_ids = puzzle3d_kind_ids(&fixture, "objects");
    assert!(!object_ids.is_empty(), "default fixture must expose object kinds");
    let zeroed_id = object_ids[0].clone();
    let mut object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    object_kind_weights = puzzle3d_normalize_kind_weight_group(&object_kind_weights, &object_ids, &zeroed_id, 0.0);
    assert!(object_kind_weights.get(&zeroed_id).copied().unwrap_or(1.0) <= f64::EPSILON);
    let scene = Puzzle3dScene { fixture, runtime: Puzzle3dRuntime { object_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: fill_tool::TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&scene, &session, labels);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    let zeroed_group_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{zeroed_id}");
    let zeroed_group = distribution_children.iter().find(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == &zeroed_group_id)).expect("zeroed object kind must appear in distribution");
    match zeroed_group {
        WindowMeasure::Group { value: Some(value), children, .. } => {
            assert!(*value <= f64::EPSILON, "object-kind header must read 0%");
            assert!(!children.is_empty(), "object kind must still list vortex sliders");
            assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: Some(true), value, .. } if *value <= f64::EPSILON)), "every joint vortex slider under a 0% object kind must be disabled at 0%");
        }
        other => panic!("expected object-kind group, got {other:?}"),
    }
    let live_group = distribution_children.iter().find(|measure| match measure {
        WindowMeasure::Group { id, value: Some(value), .. } if id != &zeroed_group_id => *value > f64::EPSILON,
        _ => false,
    });
    if let Some(WindowMeasure::Group { children, .. }) = live_group {
        assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: None | Some(false), .. })), "joint vortex sliders under a non-zero object kind must stay enabled");
    }
}

/// 🎯️ Fill tool measures expose count + nested distribution tree under the Fill toggle; the Volume
/// Brush voxel dims live in a utility-options group in the window's own measures.
#[semio_framework_async_macros::async_test]
async fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
    {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let session = Puzzle3dPrecomputeSession::new();
    let fill_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&fill_scene, &session, labels);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    assert!(!fill_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle3d-play-tool-options-fill")), "fill must not wrap its options in a nested Fill group — the tool toggle already owns that row");
    assert_eq!(measure_group_tag(&fill_measures, &distribution_id), Some(None));
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    assert!(!distribution_children.is_empty(), "distribution must list object-kind groups");
    assert!(distribution_children.iter().all(|measure| matches!(measure, WindowMeasure::Group { value: Some(_), on_change: Some(_), .. })), "each object-kind group must carry a header weight slider");
    assert!(
        distribution_children.iter().all(|measure| match measure {
            WindowMeasure::Group { label, value: Some(_), on_change: Some(_), .. } => !label.contains('%'),
            _ => false,
        }),
        "object-kind group labels must not embed percentages — the header slider owns the value readout"
    );
    assert!(
        distribution_children.iter().any(|measure| match measure {
            WindowMeasure::Group { children, .. } => children.iter().any(|child| matches!(child, WindowMeasure::Slider { label: Some(label), .. } if !label.contains('%'))),
            _ => false,
        }),
        "vortex joint sliders must label kinds without embedding percentages"
    );
    assert!(find_measure_toggle(&fill_measures, "puzzle3d-edit-volumes").is_none(), "fill must not carry edit-volumes toggle");
    assert!(find_measure_slider(&fill_measures, "puzzle3d-voxel-w").is_none(), "fill must not carry voxel-dimension sliders");
    assert!(find_measure_number(&fill_measures, "puzzle3d-fill-count").is_some(), "the fill-count entry always lives in the fill tool measures");
    assert!(
        !main::window_measures(&fill_scene, &session, labels, &Puzzle3dInteractionSnapshot::default()).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id.contains("fill"))),
        "fill must no longer surface in window_measures — it is a mode-level tool, not a window utility"
    );
    let volume_brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::volume_brush::UTILITY_ID.into() };
    let volume_brush_measures = main::window_measures(&volume_brush_scene, &session, labels, &Puzzle3dInteractionSnapshot::default());
    assert_eq!(measure_group_tag(&volume_brush_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush")), Some(Some(utilities::volume_brush::UTILITY_ID.into())));
    assert!(find_measure_slider(&volume_brush_measures, "puzzle3d-voxel-w").is_some(), "volume brush utility exposes voxel width slider");
    let fill_engagement = main::engagement(&fill_scene, &Puzzle3dLabels::NATIVE_EN);
    assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
    let brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(measure_group_tag(&main::window_measures(&brush_scene, &session, labels, &Puzzle3dInteractionSnapshot::default()), &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())));
    let brush_engagement = main::engagement(&brush_scene, &Puzzle3dLabels::NATIVE_EN);
    assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
    }
    assert_brush_options_after_open_vortex_suggestions().await;
}

#[inline(never)]
async fn assert_brush_options_after_open_vortex_suggestions() {
    let mut app = app().await;
    activate_window_utility(&mut app, utilities::brush::UTILITY_ID);
    let view = app.window_view(main::WINDOW_KIND_ID);
    let brush_app_measures = app.window_measures(&view).await;
    let window_measures = brush_app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())), "the brush Utility Options group surfaces on the live window-measures path when brush is the active utility");
}
//#endregion 🔖️Distribution

//#region 🔖️UiScope
/// @emoji 🐢️ THE scope law: every declared MUTATING command's `UiDirtyScope` names the panels its
/// mutation changes. A document edit moves the artifact outliner (the object roster), the field
/// inspector (the selected entity's own fields) and the framework history panel (one command row per
/// dispatch) — so a mutation whose scope is `Partial` and omits any of those three leaves that panel
/// showing pre-edit content until some unrelated later action happens to repaint it.
///
/// 🧯️ Why this is a law and not a review note: before wave N every hand-written `Partial` in this app
/// carried `panel_bodies: Vec::new()` (`📓️2026-09-09-wave-L-…md` §5.3) and the shared dispatcher's
/// default was `Full`, so the only two states a command could be in were "repaint the whole shell" and
/// "silently stale panels". `puzzle3d_command_scope_class` is now the one place that decides, and this
/// law is what stops a narrower class from being handed to a verb that edits the document.
///
/// 🈳️ `Chrome` (i.e. `Full`) always satisfies this law — it is the widest, always-correct answer, and a
/// newly declared command that nobody has classified yet is slow rather than wrong.
#[semio_framework_async_macros::async_test]
async fn command_scope_classes_name_the_panels_they_change() {
    use crate::editor::puzzle3d::{puzzle3d_command_scope_class, puzzle3d_scope, Puzzle3dScopeClass};
    let definition = create_puzzle3d_app();
    let mut mutating = 0_usize;
    let mut narrowed = 0_usize;
    for action in definition.window_kinds.iter().flat_map(|window| window.actions.iter()) {
        let class = puzzle3d_command_scope_class(&action.id);
        let scope = puzzle3d_scope(class);
        // 🧾️ The declared `ActionKind` is a HISTORY/undo classification, not a paint one. Both
        // `relocateTargetVolume` and `worldRelocate` are Mutations (they emit artifact edits), so this
        // law requires them to name inspector, outliner and history. A `View` verb that happens to carry
        // the document class is over-painting at worst, never stale.
        if action.kind != ActionKind::Mutation {
            continue;
        }
        mutating += 1;
        match scope {
            UiDirtyScope::Full => {}
            UiDirtyScope::None => panic!("{} is a declared Mutation, so it cannot paint nothing", action.id),
            UiDirtyScope::Partial { ref panel_bodies, ref window_bodies, .. } => {
                narrowed += 1;
                assert_eq!(window_bodies, &vec![main::BODY_KEY.to_string()], "{} edits the document, so the world body is dirty", action.id);
                for body in [inspection::BODY_KEY, document::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
                    assert!(panel_bodies.iter().any(|named| named == body), "{} is a declared Mutation with a narrowed scope, so it must name the {body} panel body: {panel_bodies:?}", action.id);
                }
            }
        }
    }
    assert!(mutating > 0, "the app declares mutating commands");
    assert!(narrowed >= 10, "the scope table must actually narrow the mutating verbs, not fall back to Full for all of them: {narrowed}/{mutating}");
}

/// 🕹️ A pure SELECTION change repaints the field inspector — the panel that renders the selected
/// entity's own fields — and the outliner that marks the selected rows, but never the catalogue, which
/// lists object KINDS and cannot move when only the selection moves.
#[semio_framework_async_macros::async_test]
async fn selection_scope_names_the_inspector_and_not_the_catalogue() {
    use crate::editor::puzzle3d::{puzzle3d_scope, Puzzle3dScopeClass};
    let UiDirtyScope::Partial { panel_bodies, measures, utilities, engagements, labels, .. } = puzzle3d_scope(Puzzle3dScopeClass::Selection) else {
        panic!("the selection class is a narrowed scope");
    };
    assert!(panel_bodies.iter().any(|body| body == inspection::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == document::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == FRAMEWORK_HISTORY_BODY_KEY), "{panel_bodies:?}");
    assert!(!panel_bodies.iter().any(|body| body == catalogue::BODY_KEY), "a selection cannot change the kind roster: {panel_bodies:?}");
    assert!(measures, "selection-dependent window measures are re-read");
    assert!(!utilities);
    assert!(!engagements);
    assert!(!labels);
}

/// 🎮️ Wave B34: the six framework interaction verbs are declared out of this app's own scope table
/// through `ArtifactEditor::interaction_scope`, and each one names EXACTLY the lanes it moves. Before
/// this wave the framework answered `UiDirtyScope::Full` for all six — including `interactionHover`,
/// which fires on pointer motion — so every mouse move over the viewport repainted every window body,
/// every panel body, the utilities/tools/engagements rails, the labels and the measures
/// (`📓️2026-09-12-wave-B32-world-lane-after-completion.md` §3.2: 24 interaction ingresses and 3
/// whole-shell completions per 80 s lane, which is what a document mutation's own refresh pass then
/// queued behind for 10–14 s).
#[semio_framework_async_macros::async_test]
async fn interaction_verbs_declare_exactly_the_lanes_they_move() {
    use crate::editor::puzzle3d::{puzzle3d_interaction_chrome_scope, puzzle3d_selection_scope, puzzle3d_viewport_scope, PUZZLE3D_INTERACTION_DOMAIN};
    use semio_framework_plugin::InteractionVerb;
    let domain = [PUZZLE3D_INTERACTION_DOMAIN];
    assert_eq!(Puzzle3dPlayApp::interaction_scope(InteractionVerb::Hover, &domain), Some(puzzle3d_viewport_scope()), "a hover moves the world body's hover lane and nothing else");
    for verb in [InteractionVerb::Select, InteractionVerb::ClearSelection, InteractionVerb::SelectAll] {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &domain), Some(puzzle3d_selection_scope()), "{verb:?} moves the declared selection lanes");
    }
    for verb in [InteractionVerb::SetSelectionMode, InteractionVerb::SetGranularity] {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &domain), Some(puzzle3d_interaction_chrome_scope()), "{verb:?} moves the window's Select chrome");
    }
    for verb in InteractionVerb::ALL {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &["mesh"]), None, "{verb:?} on a domain this app never declared must fall back to the framework scope");
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &[]), None, "{verb:?} touching no domain must fall back to the framework scope");
    }
}

/// 🐁️ The hover half, pinned field by field: a pointer move paints the world body ALONE — no panel, no
/// rail, no label, not even the window measures. The pick half names exactly the three selection panels
/// (`selection_scope_names_the_inspector_and_not_the_catalogue` pins their identity) and no fourth one,
/// so widening either answer has to widen this law with it.
#[semio_framework_async_macros::async_test]
async fn a_hover_paints_only_the_world_body_and_a_pick_adds_exactly_the_selection_panels() {
    use crate::editor::puzzle3d::{puzzle3d_interaction_chrome_scope, puzzle3d_selection_scope, puzzle3d_viewport_scope};
    let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = puzzle3d_viewport_scope() else {
        panic!("the hover scope is a narrowed scope");
    };
    assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
    assert!(panel_bodies.is_empty(), "a hover cannot move a panel: {panel_bodies:?}");
    assert!(!utilities && !tools && !engagements && !measures && !labels, "a hover cannot move the shell chrome");
    let UiDirtyScope::Partial { panel_bodies, .. } = puzzle3d_selection_scope() else {
        panic!("the selection scope is a narrowed scope");
    };
    assert_eq!(panel_bodies.len(), 3, "a pick moves exactly the inspector, the outliner and the history: {panel_bodies:?}");
    for body in [inspection::BODY_KEY, document::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        assert!(panel_bodies.iter().any(|named| named == body), "a pick moves {body}: {panel_bodies:?}");
    }
    let UiDirtyScope::Partial { panel_bodies, measures, .. } = puzzle3d_interaction_chrome_scope() else {
        panic!("the interaction chrome scope is a narrowed scope");
    };
    assert!(panel_bodies.is_empty(), "no panel renders the active selection mode or granularity: {panel_bodies:?}");
    assert!(measures, "the measures rail binds both Select controls");
}

/// 🈳️ The four narrow non-document classes deliberately name NO panel body — a camera move, a fill
/// planning tick, a distribution slider and a suggestion tick each touch the world body (and at most
/// the tool/window measures) and nothing else. Pins the claim their doc comments make, so widening one
/// of them has to widen this law too.
#[semio_framework_async_macros::async_test]
async fn viewport_and_tick_scope_classes_name_no_panel_body() {
    use crate::editor::puzzle3d::{puzzle3d_scope, Puzzle3dScopeClass};
    for class in [Puzzle3dScopeClass::Viewport, Puzzle3dScopeClass::FillBuild, Puzzle3dScopeClass::FillOptions, Puzzle3dScopeClass::SuggestionsTick] {
        let UiDirtyScope::Partial { panel_bodies, window_bodies, .. } = puzzle3d_scope(class) else {
            panic!("{class:?} is a narrowed scope");
        };
        assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()], "{class:?}");
        assert!(panel_bodies.is_empty(), "{class:?} claims to paint no panel: {panel_bodies:?}");
    }
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Quiet), UiDirtyScope::None));
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Chrome), UiDirtyScope::Full));
}

#[semio_framework_async_macros::async_test]
async fn fill_build_tick_is_a_view_action_with_narrow_ui_scope() {
    let definition = create_puzzle3d_app();
    let def = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == "fillBuildTick").expect("fillBuildTick declared");
    assert_eq!(def.kind, ActionKind::View, "fillBuildTick must stay a View action — it only advances background planning");
    let mut live = app().await;
    dispatch(&mut live, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let result = dispatch(&mut live, "fillBuildTick", None, None).await.expect("fillBuildTick");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty());
            assert!(tools, "fill planning must refresh the fill-count slider range in the fill tool's measures");
            assert!(!measures);
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for fillBuildTick, got {other:?}"),
    }
}

/// 🪣️ `setFillCount` APPLIES planned placements, so it is a document edit that also moves the
/// fill-count slider range: narrow (no utilities, no engagements, no labels) but panel-naming. It used
/// to declare the pure fill-planning scope, which named no panel at all — so committing a fill left the
/// outliner, the inspector, the catalogue and the history panel showing the pre-fill document.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_declares_a_narrow_document_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let result = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 1 })), None).await.expect("setFillCount");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            for body in [inspection::BODY_KEY, document::BODY_KEY, catalogue::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
                assert!(panel_bodies.iter().any(|named| named == body), "a committed fill changes the {body} panel: {panel_bodies:?}");
            }
            assert!(tools, "the fill-count slider range lives in the fill tool's measures");
            assert!(measures);
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setFillCount, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn set_object_kind_weight_declares_fill_options_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let object_ids = puzzle3d_kind_ids(&nakagin_fixture(), "objects");
    let kind_id = object_ids.first().expect("object kind");
    let result = dispatch(&mut app, "setObjectKindWeight", Some(&json!({ "kindId": kind_id.as_str(), "value": 0.75 })), None).await.expect("setObjectKindWeight");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty());
            assert!(tools);
            assert!(measures, "distribution sliders live in tool + window measures");
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setObjectKindWeight, got {other:?}"),
    }
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `set_hover_is_a_view_action_with_no_ops_after_document_mutation`
// and `world_pick_declares_selection_ui_scope` deleted — both dispatched the now-deleted
// `setHover`/`worldPick` actions and asserted on the deleted `Effect::PatchWorld3dChrome`
// push-setter effect (selection/hover are framework-owned actions now, dispatched exclusively
// through the six reserved `interactionSelect`-family verbs; see `select_id`/`hover_id`).
//#endregion 🔖️UiScope

//#region 🔖️Utilities
#[semio_framework_async_macros::async_test]
async fn add_object_kind_honors_drop_origin() {
    let mut app = app().await;
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.5, 3.5, 0.0] })), None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1);
    let projection = projection_of(&app);
    let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("added object");
    let origin = object.get("origin").and_then(Value::as_array).expect("origin array");
    assert_eq!(origin.first().and_then(Value::as_f64), Some(2.5));
    assert_eq!(origin.get(1).and_then(Value::as_f64), Some(3.5));
    assert_eq!(origin.get(2).and_then(Value::as_f64), Some(0.0));
}

#[semio_framework_async_macros::async_test]
async fn add_object_kind_materializes_the_declared_kind_default() {
    // 📝️ P1 arg form: firing addObjectKind with no args on a document whose catalogs were cleared must
    // materialize the declared `objectKind` default — the catalog row AND the object referencing it —
    // in ONE gesture. A retained tool job never publishes inline (`settle`'s own docstring), so the
    // settled document, not the dispatch's `mutations` vector, is where the gesture is observed.
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", None, None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
    let projection = projection_of(&app);
    let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
    assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
    let catalog = projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("the declared default kind catalog was materialized");
    assert!(catalog.iter().any(|row| row.get("id").and_then(Value::as_str) == Some("Object")), "the created object references a catalogued kind, never a dangling one: {catalog:?}");
}

/// 🧊️ Wave B11 (checklist §10, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 12): the Volume
/// Brush armed and its W/D/H sliders moved, but Alt+click added nothing. This is the guest half of that
/// gesture — an `addTargetVolume` carrying the host's grid-snapped ground origin must land ONE volume
/// there, sized by the utility's own voxel dimensions times the grid spacing.
#[semio_framework_async_macros::async_test]
async fn alt_click_adds_one_target_volume_sized_by_the_utility_voxel_dims() {
    let volumes = |app: &Puzzle3dApp| projection_of(app).get("targetVolumes").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::volume_brush::UTILITY_ID })), None).await.expect("arm the volume brush");
    for (axis, value) in [("w", 2.0), ("d", 3.0), ("h", 4.0)] {
        dispatch(&mut app, "setVoxelDims", Some(&json!({ "axis": axis, "value": value })), Some(main::WINDOW_KIND_ID)).await.expect("voxel dimension");
    }
    let before = volumes(&app);
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [3.0, -2.0, 0.0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    assert_eq!(volumes(&app), before + 1, "one Alt+click places exactly one target volume");
    let projection = projection_of(&app);
    let volume = projection.get("targetVolumes").and_then(Value::as_array).and_then(|entries| entries.last()).expect("the placed volume");
    let scale: Vec<f64> = volume.get("scale").and_then(Value::as_array).expect("the volume carries a scale").iter().filter_map(Value::as_f64).collect();
    assert_eq!(scale.len(), 3, "a voxel volume is a three-axis box: {scale:?}");
    assert!(scale[1] > scale[0] && scale[2] > scale[1], "the W/D/H steppers must size the placed box in their own order: {scale:?}");
    let origin: Vec<f64> = volume.get("origin").and_then(Value::as_array).expect("the volume carries an origin").iter().filter_map(Value::as_f64).collect();
    assert_eq!(origin.len(), 3, "the volume lands at the pointed ground point: {origin:?}");
    // 🔢️ A grid-snapped ground point is USUALLY whole (`[0,10,0]` was the live browser payload), and JSON
    // serializes a whole f64 as an integer — so the decode must admit integer carriers, not only floats.
    let whole = volumes(&app);
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0, 10, 0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume with integer coordinates");
    assert_eq!(volumes(&app), whole + 1, "an integer-valued ground point must place a volume exactly like a fractional one");
}

/// 🧯️ …and a dispatch that carries no usable ground point answers with ONE localized notice instead of
/// returning silently, so a miss is distinguishable from a dead gesture.
#[semio_framework_async_macros::async_test]
async fn add_target_volume_without_a_ground_point_raises_one_notice_and_places_nothing() {
    let volumes = |app: &Puzzle3dApp| projection_of(app).get("targetVolumes").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    let mut app = app().await;
    let before = volumes(&app);
    let result = dispatch(&mut app, "addTargetVolume", None, Some(main::WINDOW_KIND_ID)).await.expect("a missing origin is a refusal, not a fault");
    let notices: Vec<&String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an origin-less placement must say why: {:?}", result.requested_effects);
    assert_ne!(notices[0].as_str(), PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so the refusal must be real prose");
    assert_eq!(volumes(&app), before, "a refused placement adds nothing");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    // 🧰️ Switching utilities is the framework-injected View action: no document operations, no undo
    // entry, no re-emitted utility-switch effect (the command IS the direct switch).
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), None).await.expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[test]
fn set_active_utility_dirties_the_world_body() {
    use crate::editor::puzzle3d::{puzzle3d_command_scope_class, puzzle3d_scope, Puzzle3dScopeClass};
    assert_eq!(puzzle3d_command_scope_class(SET_ACTIVE_UTILITY_ACTION_ID), Puzzle3dScopeClass::Viewport, "utility switch must republish the world body (interaction + vortices), not Full chrome and not None");
    match puzzle3d_scope(Puzzle3dScopeClass::Viewport) {
        UiDirtyScope::Partial { window_bodies, .. } => assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]),
        other => panic!("utility switch scope must be the world body, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_exposes_no_utility_switch_options() {
    // 🧰️ select/brush/fill switching lives only on the framework utility bar; the engagement HUD
    // must not duplicate it as options. The Add Object dialog opener is a create verb, not a utility.
    let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let engagement = main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN);
    let options = engagement.options.as_ref().expect("add-object opener lives on the engagement HUD");
    assert!(options.iter().all(|option| !matches!(option.id.as_str(), "select" | "brush" | "fill" | "volumeBrush" | "worldRelocate")), "the puzzle3d engagement must not re-expose utility switching as options");
    assert!(options.iter().any(|option| option.id == "shell-menu.action.openAddObjectDialog" && option.action.as_ref().is_some_and(|action| action.action == "openAddObjectDialog")), "engagement must expose the Add Object dialog opener");
}

#[semio_framework_async_macros::async_test]
async fn transform_engagement_does_not_block_background_deselect() {
    let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    assert_eq!(main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN).session_active, Some(false));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the Brush utility's "Placement"
/// picker exists only for a live brush target — an explicitly selected vortex, else the hovered one.
/// It reaches `window_measures` through `window_measures_with_request_context`, so this drives the
/// real app chrome rather than calling `main::window_measures` with a fabricated snapshot.
///
/// ⏰️ Host ticks the brush lane needs before the picker has rows: each `suggestionsTick` spends ONE
/// bounded `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US` slice, so on an opt-level-0 build a target costs a
/// couple of them. Fixed, never "until it resolves" — an unbounded loop here would turn the assertion
/// into a machine-load reading.
const PUZZLE3D_BRUSH_PICKER_TICKS: usize = 8;

#[semio_framework_async_macros::async_test]
async fn brush_placement_picker_appears_only_for_a_live_brush_target() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let idle = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(idle, "puzzle3d-brush-placement"), None, "no brush target means no placement picker");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    // ⏰️ The brush lane is warmed by the host's own 120 ms `suggestionsTick`, never by the render or
    // the measures call: `setActiveTool`/`setActiveUtility` are answered by the framework with an empty
    // `Emit` and reach no app reducer at all, so nothing in this app runs at the moment a utility is
    // activated (ticket 26/09/02/PUZZLE-3D-END-TO-END wave D3). Playing that clock here is what a
    // running host does between the click and the next frame.
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("suggestionsTick");
    }
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let targeted = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert!(find_measure_select(targeted, "puzzle3d-brush-placement").is_some(), "an explicitly selected vortex is a brush target, so the placement picker must render");
}
//#endregion 🔖️Utilities

//#region 🔖️WorldSelection
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the app-owned `worldSelect`
/// command is deleted — selection now goes exclusively through the framework's `interactionSelect`
/// verb (`select_id`), which is view-only by construction (`dispatch_interaction_action` never
/// touches `self.store`). Proves the `vortex` domain wiring reaches that same guarantee.
#[semio_framework_async_macros::async_test]
async fn world_select_emits_no_artifact_mutations() {
    let mut app = app().await;
    let before = projection_of(&app);
    let object_id = first_object_id(&app);
    let result = select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    assert!(result.mutations.is_empty(), "interactionSelect is framework-owned and view-only, must not diff the document");
    assert_eq!(projection_of(&app), before);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection paint rides on
/// `selectionJson` only — instance GEOMETRY stays byte-stable across a pick, so the host never has to
/// re-upload meshes just because something was selected.
#[semio_framework_async_macros::async_test]
async fn world_pick_keeps_instances_geometry_json_stable() {
    let mut app = app().await;
    let instances_before = instances_of(&render_composite(&mut app).await);
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let after = render_composite(&mut app).await;
    assert_eq!(instances_of(&after), instances_before, "picking must never perturb instance geometry");
    assert_eq!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()), Some(vec![object_id.clone()]));
    assert_eq!(
        selection_of(&after).get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![object_id]),
        "`selectionJson.ids` is what World3dHost paints instance selection from"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selectionJson` carries the exact
/// field names `World3dHost`'s `parseSelection`/`WorldSelectionRecord` reads — `ids` for object
/// instances, `targetVolumeIds`, `referenceSelectedId`, `hoveredId`. Vortex marks deliberately do NOT
/// appear here (that record has no vortex field; see `world_selection_json`'s doc comment).
#[semio_framework_async_macros::async_test]
async fn world_selection_json_carries_the_host_field_names_per_granularity() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(&object_id)).await.expect("hover object");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(selection.get("hoveredId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(selection.get("targetVolumeIds").and_then(Value::as_array).map(Vec::len), Some(0));
    assert!(selection.get("vortexIds").is_none(), "the host's WorldSelectionRecord has no vortexIds field");

    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0.0, 0.0, 0.0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.last().cloned()).and_then(|volume| volume.get("id").and_then(Value::as_str).map(str::to_string)).expect("target volume id");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select target volume");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(
        selection.get("targetVolumeIds").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![volume_id]),
        "a target-volume granularity selection paints through `targetVolumeIds`"
    );
    assert_eq!(selection.get("ids").and_then(Value::as_array).map(Vec::len), Some(0), "object `ids` stay empty while a target volume is the live granularity");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the world scene binds `domainId` (and
/// its own `domainGranularityId`) so `World3dHost` dispatches the generic `interactionSelect`/
/// `interactionHover` verbs — this crate has no handler for the legacy `worldPick`/`worldSelect`/
/// `setHover` fallbacks the host would otherwise use.
#[semio_framework_async_macros::async_test]
async fn world_scene_binds_the_vortex_interaction_domain_and_its_granularity() {
    let mut app = app().await;
    let scene = render_composite(&mut app).await.get("world3d").cloned().unwrap_or(Value::Null);
    assert_eq!(scene.get("domainId").and_then(Value::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(scene.get("domainGranularityId").and_then(Value::as_str), Some(PUZZLE3D_GRANULARITY_OBJECT));
}

#[semio_framework_async_macros::async_test]
async fn world_pick_null_clears_without_reselecting_first_object() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| !selection.ids.is_empty()));
    dispatch(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": "[]", "merge": "replace", "method": "pick" })),
        None,
    )
    .await
    .expect("empty-target interactionSelect");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "clicking empty background must clear, never fall back to reselecting the first object");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove that
/// world-picking a LOCKED object clears the selection instead of selecting it — the app-owned
/// `worldPick` command, which could read `object.locked` before deciding, is deleted.
/// `interactionSelect` is a framework-generic id/merge verb blind to app data, and this app's
/// `interaction_topology` does not filter locked ids out of the `vortex` domain either (see that
/// function's doc) — "never select a locked object" is now entirely a host click→pick
/// translation concern, not reachable from this crate. Proves the Rust-side floor instead: the
/// `vortex` domain has no lock awareness, so selecting a locked object's id still succeeds.
#[semio_framework_async_macros::async_test]
async fn world_pick_locked_object_clears_like_background() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "flag": "locked", "value": true })), None).await.expect("lock");
    let instances = instances_of(&render_composite(&mut app).await);
    assert_eq!(instances.first().and_then(|entry| entry.get("disabled")).and_then(Value::as_bool), Some(true));
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select locked object");
    assert_eq!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()),
        Some(vec![object_id]),
        "the vortex domain has no lock awareness — this now succeeds, the host must gate locked picks itself"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `PUZZLE3D_VORTEX_SHOW_SELECTED`
/// reveals a marker only while its own object (or one of its markers) is selected/hovered — the
/// live read `render_with_request_context` threads in as a `Puzzle3dInteractionSnapshot`.
#[semio_framework_async_macros::async_test]
async fn world_vortices_reveal_in_selected_mode_only_for_the_selected_object() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode with nothing selected reveals no vortex marker");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let revealed = vortices_of(&render_composite(&mut app).await);
    assert!(!revealed.is_empty(), "Selected mode must reveal the selected object's own vortex markers");
    assert!(revealed.iter().all(|vortex| vortex.get("objectId").and_then(Value::as_str) == Some(object_id.as_str())), "Selected mode must reveal ONLY the selected object's markers");
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "clearing the selection hides the markers again");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    assert!(!vortices_of(&render_composite(&mut app).await).is_empty(), "Always mode must still reveal every vortex marker");
}

/// 🕹️ A selected vortex marker carries its own `selected` flag on `vorticesJson` — the host's
/// `WorldVortexMarkers` reads it off each record (`WorldSelectionRecord` has no vortex field), and a
/// hovered marker carries `hovered` the same way.
#[semio_framework_async_macros::async_test]
async fn world_vortices_carry_their_own_selected_and_hovered_flags() {
    let mut app = app().await;
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selected = vortices_of(&render_composite(&mut app).await);
    let record = selected.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("selected vortex record");
    assert_eq!(record.get("selected").and_then(Value::as_bool), Some(true));
    assert!(selected.iter().filter(|entry| entry.get("fullId").and_then(Value::as_str) != Some(vortex.as_str())).all(|entry| entry.get("selected").and_then(Value::as_bool) == Some(false)), "only the picked marker is selected");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover vortex");
    let hovered = vortices_of(&render_composite(&mut app).await);
    let record = hovered.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("hovered vortex record");
    assert_eq!(record.get("hovered").and_then(Value::as_bool), Some(true));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `worldVortexSelect`/`worldPick`
/// are deleted; a `vortex`-domain `DomainSelection` only ever carries one granularity at a time
/// (see `Puzzle3dActionCtx::selected_ids`'s doc), so a `merge: "replace"` pick at a different
/// granularity inherently replaces the whole prior selection — verified against
/// `interaction_state()` (the render-time `selectionJson`/`vorticesJson` fields carry no live ids
/// any more, per `world_selection_json`'s known-gap doc comment).
#[semio_framework_async_macros::async_test]
async fn world_pick_object_replaces_vortex_selection() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids, vec![object_id]);
}

#[semio_framework_async_macros::async_test]
async fn world_vortex_select_clears_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_VORTEX);
    assert_eq!(selection.ids, vec![vortex]);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove a
/// PERSISTED "default merge mode" (`selection_mode_default`, a config field — on this crate's
/// DELETE list) flips `worldVortexSelect`'s implicit merge between replace/invertive. The
/// framework has no equivalent persisted concept: `interactionSelect`'s `merge` arg is supplied
/// explicitly on every dispatch (the host decides per click — e.g. a held modifier key — never
/// defaulted from stored state). Proves the underlying `merge: "invertive"` primitive still
/// toggles a second target back into the selection instead.
#[semio_framework_async_macros::async_test]
async fn world_vortex_click_replaces_until_invertive_mode_is_selected() {
    let mut app = app().await;
    let vortices = vortex_full_ids(&app);
    assert!(vortices.len() >= 2, "fixture must expose two vortices");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[0]).await.expect("select first vortex");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[1]).await.expect("replace with second vortex");
    let replaced = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(replaced.ids, vec![vortices[1].clone()]);

    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), id: vortices[0].clone() }]);
    dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "invertive", "method": "pick" })), None).await.expect("invertive toggle");
    let invertive = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(invertive.ids.len(), 2, "invertive merge toggles the first vortex back into the selection alongside the second");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "Select Same Kind" emits a real
/// `Emit.interaction_writes` entry, which `VcsArtifactApp` applies through the same `next_selection`
/// machine the reserved `interactionSelect` verb uses — so the widened selection is observable on
/// `interaction_state()` and painted into `selectionJson` on the next render.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_widens_the_selection_to_every_object_of_that_kind() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    for _ in 0..3 {
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    }
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(ids.len(), 3, "three same-kind objects must exist");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &ids[0]).await.expect("select one");
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    let mut widened = selection.ids.clone();
    widened.sort();
    let mut expected = ids.clone();
    expected.sort();
    assert_eq!(widened, expected, "every object of the clicked object's kind is selected");
    let painted = selection_of(&render_composite(&mut app).await).get("ids").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
    let mut painted_sorted = painted.clone();
    painted_sorted.sort();
    assert_eq!(painted_sorted, expected, "the widened selection reaches the host through selectionJson");
}

/// 🕹️ "Select Same Kind" with nothing selected still aborts (nothing to widen from) and leaves the
/// selection untouched.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_with_no_selection_leaves_the_selection_untouched() {
    let mut app = app().await;
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "widening from nothing must not invent a selection");
}

/// 🎯️ A selection-scoped command dispatched with NOTHING selected must refuse VISIBLY: exactly one
/// `Effect::Notify`, no document edit, and no history row. This is the browser defect measured
/// 2026-09-09 21:05 — "Duplicate Selection" from the context menu completed with no clone, no command
/// row and no notice of any kind, which is indistinguishable from a dead menu entry. Every arm that
/// reads the framework-owned selection is covered here, so the refusal cannot be reintroduced one
/// command at a time.
///
/// 🧾️ A command-log ROW is deliberately not asserted absent: `dispatch_emit` records one row per
/// dispatch by design, mutation-kind actions that produced zero operations included (see its own doc
/// in `🔌️plugin/🦀️.rs`), so a refused `Mutation` still shows up in history with no edit id. What must
/// never happen is a silent nothing — that is what the notice below pins.
#[semio_framework_async_macros::async_test]
async fn selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    // 🌱️ The seed gesture SELECTS what it adds (`add_object_kind_selects_the_object_it_adds`), so the
    // empty precondition this law is about has to be established explicitly — otherwise every arm below
    // measures a live selection and refuses nothing (wave B36).
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear the seed gesture's own selection");
    let objects_before = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    for action in ["duplicateSelection", "deleteSelection", "selectSameKindSelection"] {
        let result = dispatch(&mut app, action, None, None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "{action} painted nothing, so it must not force a full refresh");
    }
    // 🧲️ The three gumball verbs carry a `coalesce_key`, so they enter the latest-wins channel whose
    // accepted invocation answers BEFORE the command runs — their whole outcome (notice, scope) lives on
    // the completion lane, which `context::settle` now drains into `InvocationResult` instead of
    // dropping. They also never reach `refuse_without_selection`: `build_tool_job` routes them to
    // `Puzzle3dScaleWork`, not through `dispatch_step`, which is why they used to complete with an empty
    // edit, a coalesce key and `UiDirtyScope::Full` — measured 2026-09-09 in-process as
    // `completion scope=Full, effects=[]` while `duplicateSelection` on the same fixture refused.
    for (action, args) in [
        ("translateSelection", json!({ "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
        ("rotateSelection", json!({ "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 1.0 })),
        ("scaleSelection", json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
    ] {
        let result = dispatch(&mut app, action, Some(&args), None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice on its completion lane: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(matches!(result.ui_scope, UiDirtyScope::None), "{action} painted nothing, so its completion must carry UiDirtyScope::None, got {:?}", result.ui_scope);
    }
    let objects_after = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    assert_eq!(objects_after, objects_before, "a refused selection command must leave the document untouched");
    assert!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "a refused selection command must not invent a selection either"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `duplicateSelection` re-selects the
/// clones it created — the interaction write is applied AFTER the document mutations land, so the new
/// ids are already in `interaction_topology` and survive `validate_state`'s pruning.
#[semio_framework_async_macros::async_test]
async fn duplicate_selection_reselects_the_created_clones() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "duplicateSelection", None, None).await.expect("duplicateSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids.len(), 1, "exactly the one clone is selected");
    assert_ne!(selection.ids.first().map(String::as_str), Some(object_id.as_str()), "the CLONE is selected, not the original");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert!(live_ids.contains(selection.ids.first().expect("clone id")), "the re-selected id must exist in the document");
}

/// 🌱️ `addObjectKind` SELECTS the object it adds — the same `Emit.interaction_writes` contract
/// `duplicateSelection` and `addBrushObject` already honour, on both of its implementations: the
/// cursorized `Puzzle3dAddObjectKindWork` the interactive job runs and the direct reducer arm. Adding an
/// object the user must then hunt for is half a gesture: the catalogue row, the drag-drop and the Add
/// Object dialog all end with the new object nowhere in the inspector, the gumball or the outliner
/// (battery #53 `catalogue-add-selects-new-object added=1`, ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B36). Every id is asserted against the live document, so a write naming a phantom fails here.
#[semio_framework_async_macros::async_test]
async fn add_object_kind_selects_the_object_it_adds() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("first object");
    let first = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(first.granularity, PUZZLE3D_GRANULARITY_OBJECT, "the add selects at OBJECT granularity: {first:?}");
    assert_eq!(first.ids.len(), 1, "exactly the added object is selected: {first:?}");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [4.0, 0.0, 0.0] })), None).await.expect("second object");
    let second = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(second.ids.len(), 1, "the second add REPLACES the selection with its own object, it does not widen it: {second:?}");
    assert_ne!(second.ids, first.ids, "the second add selects the object IT created: {second:?} vs {first:?}");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(live_ids.len(), 2, "both adds landed in the document: {live_ids:?}");
    assert!(live_ids.contains(second.ids.first().expect("added id")), "the selected id is the one the document carries: {live_ids:?}");
}

/// 🗑️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31: one `deleteSelection` on a live object selection
/// removes that object from the RENDERED world census within the same settle, records exactly one
/// command-log row, and leaves the deleted id out of the framework-owned selection — so pressing
/// `Delete` twice cannot land a second, empty edit on an id the document no longer carries. The
/// browser red this states was `before=2 after=2 waitedMs=30299` with
/// `history patch applied labels=["delete-object id=object-1"]` in the same run: the document moved
/// and the selection did not, so every later delete was a no-op against a phantom.
#[semio_framework_async_macros::async_test]
async fn delete_selection_shrinks_the_world_census_and_drops_the_deleted_id() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("first object");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("second object");
    let census_before = instance_count(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(census_before, 2, "two added objects must both be in the rendered world census");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let (deleted, settled) = dispatch_reporting(&mut app, "deleteSelection", None, None).await;
    deleted.expect("deleteSelection");
    assert_eq!(history_rows(&settled), 1, "a delete that removed an object must publish exactly one command-log row");
    assert_eq!(object_count(&app), 1, "the document must carry one object fewer");
    assert_eq!(instance_count(&render_window(&mut app, main::WINDOW_KIND_ID).await), census_before - 1, "the rendered world census must shrink by one within the same settle");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert!(!selection.ids.contains(&object_id), "the deleted id must be gone from the selection, else a second Delete is a silent no-op: {:?}", selection.ids);
    let (again, settled_again) = dispatch_reporting(&mut app, "deleteSelection", None, None).await;
    again.expect("second deleteSelection");
    assert_eq!(history_rows(&settled_again), 0, "a delete with nothing left selected must record no row");
    assert_eq!(object_count(&app), 1, "and must not touch the document");
}
//#endregion 🔖️WorldSelection

//#region 🔖️Gumball
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `gumballActive` requires BOTH the
/// transform utility active AND a live object (or target-volume) selection — the live read
/// `render_with_request_context` threads in. `transformMode`/`gumballConfig` depend only on the
/// active utility, per window.
#[semio_framework_async_macros::async_test]
async fn gumball_active_only_for_transform_utilities_with_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let idle_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(idle_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "selection alone must not show the gumball");
    assert!(idle_selection.get("transformMode").is_none(), "non-transform utility must not emit transformMode");

    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    let transform_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(transform_selection.get("gumballActive").and_then(Value::as_bool), Some(true), "transform utility plus a live object selection shows the gumball");
    assert_eq!(transform_selection.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()), Some(vec![object_id.clone()]));
    assert_eq!(transform_selection.get("activeObjectId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(transform_selection.get("transformMode").and_then(Value::as_str), Some("transform"));
    assert_eq!(transform_selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(transform_selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(true));

    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert_eq!(selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).get("gumballActive").and_then(Value::as_bool), Some(false), "an unattached gumball must never render");

    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("reselect");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let brush_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(brush_selection.get("gumballActive").and_then(Value::as_bool), Some(false));
    assert!(brush_selection.get("transformMode").is_none());
}

/// 🕹️ Both handle flags off leaves nothing to grab, so the gumball must not render even with a live
/// selection and the transform utility active (`setTransformGumballFlag` is what the user toggles).
#[semio_framework_async_macros::async_test]
async fn gumball_inactive_when_every_handle_flag_is_off() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "move", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("no move");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("no rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(selection.get("gumballActive").and_then(Value::as_bool), Some(false), "a gumball with no handles must not render");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `transformMode` depends only on
/// each window instance's host-owned `ViewModel` utility map, so it is the per-window-isolation proof.
#[semio_framework_async_macros::async_test]
async fn transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(top)).await.expect("transform on top");
    let top_selection = selection_of(&render_window(&mut app, top).await);
    assert_eq!(top_selection.get("transformMode").and_then(Value::as_str), Some("transform"), "transform on top pane must switch that pane's own scene mode");
    let perspective_selection = selection_of(&render_window(&mut app, perspective).await);
    assert!(perspective_selection.get("transformMode").is_none(), "perspective pane must not inherit top pane's transform utility");
}

#[semio_framework_async_macros::async_test]
async fn transform_utility_options_expose_move_and_rotate_flags() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let session = Puzzle3dPrecomputeSession::new();
    let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    let measures = main::window_measures(&scene, &session, labels, &Puzzle3dInteractionSnapshot::default());
    assert_eq!(measure_group_tag(&measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform")), Some(Some(utilities::transform::UTILITY_ID.into())));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-move"), Some(true));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-rotate"), Some(true));
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("disable rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(false));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let app_measures = app.window_measures(&view).await;
    let window_measures = app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(window_measures, "puzzle3d-transform-rotate"), Some(false));
}

fn object_origin(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn gumball_translate_drag_coalesces_into_one_edit() {
    // 🌀️ Repeated translate dispatches coalesce into one undo entry via AmendLast.
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    for dx in [1.0, 2.0, 3.0] {
        dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": dx, "dy": 0.0, "dz": 0.0 })), None).await.expect("drag tick");
    }
    let dragged = object_origin(&app, &object_id);
    assert!((dragged[0] - start[0] - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole coalesced gumball drag");
}

fn object_orientation(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("orientation").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

fn object_scale(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("scale").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn gumball_rotate_drag_coalesces_into_one_edit() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_orientation(&app, &object_id);
    for angle in [0.25, 0.50, 0.75] {
        dispatch(&mut app, "rotateSelection", Some(&json!({ "ids": [object_id.as_str()], "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": angle })), None).await.expect("rotate tick");
    }
    assert_ne!(object_orientation(&app, &object_id), start, "three rotate ticks must move the pose");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_orientation(&app, &object_id), start, "one undo restores the whole coalesced rotate drag");
}

#[semio_framework_async_macros::async_test]
async fn gumball_scale_drag_coalesces_into_one_edit() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_scale(&app, &object_id);
    for sx in [1.1, 1.2, 1.3] {
        dispatch(&mut app, "scaleSelection", Some(&json!({ "ids": [object_id.as_str()], "sx": sx, "sy": 1.0, "sz": 1.0 })), None).await.expect("scale tick");
    }
    assert_ne!(object_scale(&app, &object_id), start, "three scale ticks must change the scale");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_scale(&app, &object_id), start, "one undo restores the whole coalesced scale drag");
}

/// 🧲️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave T: the real gumball gesture — `transformBegin`, ONE
/// absolute start→end delta, `transformEnd` — moves the object by exactly that delta and undoes as
/// one edit. The brackets themselves contribute nothing; a second gesture starts from the pose the
/// first one left, with no app-side drag session to carry between them.
#[semio_framework_async_macros::async_test]
async fn gumball_gesture_commits_one_absolute_delta_between_its_host_brackets() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "transformBegin", None, None).await.expect("begin");
    dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 6.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("drag-end delta");
    dispatch(&mut app, "transformEnd", None, None).await.expect("end");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 6.0).abs() < 1e-9, "the one absolute delta lands verbatim on the document");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole gumball gesture");
    dispatch(&mut app, "transformBegin", None, None).await.expect("begin again");
    dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 2.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("second gesture delta");
    dispatch(&mut app, "transformEnd", None, None).await.expect("second end");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 2.0).abs() < 1e-9, "a second gesture works from the restored pose");
}
//#endregion 🔖️Gumball

//#region 🔖️KitInPort
/// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like block3d's
/// `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
/// `Puzzle3dPlayApp::import_media` must normalize `objectKinds` → `objects` / `vortexKinds` →
/// `vortices` and, after applying the returned operations, land that object kind inside
/// `meta.kind_catalogs.objects` (and the vortex kind inside `.vortices`).
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_upserts_object_and_vortex_kinds_into_meta_kind_catalogs() {
    let projection = Puzzle3dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);

    let fragment = json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "capsule",
            "name": "capsule",
            "label": "Capsule",
            "meshUrl": "/mesh/capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "door", "name": "door", "label": "Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "door", "target": "door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
    assert!(!emit.artifact_mutations.is_empty(), "importing a non-empty fragment must emit real operations");

    let mut next_projection = projection.value().clone();
    for operation in &emit.artifact_mutations {
        next_projection = protocol::Mutation::<serde_json::Value>::diff(operation, &next_projection).diff().apply(&next_projection).expect("valid mutation diff");
    }

    let next_projection = parse(&next_projection.to_string()).expect("mutated snapshot JSON");
    let objects = next_projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert!(objects.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")), "the imported object kind must appear in meta.kind_catalogs.objects");
    let capsule = objects.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).unwrap();
    assert_eq!(capsule.pointer("/representations/0/url").and_then(Value::as_str), Some("/mesh/capsule.glb"));
    assert_eq!(capsule.pointer("/vortices/0/vortexKind").and_then(Value::as_str), Some("door"), "the per-object vortex template keeps its vortexKind after normalization");

    let vortices = next_projection.pointer("/meta/kindCatalogs/vortices").and_then(Value::as_array).expect("vortices catalog present");
    assert!(vortices.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("door")), "the imported vortex kind must appear in meta.kind_catalogs.vortices");

    let compatibility = next_projection.pointer("/meta/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
    assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
}

/// 🔁️ Re-importing the SAME fragment (a second producer edge, or a redelivered message on a
/// `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_is_idempotent_on_repeated_delivery() {
    let projection = Puzzle3dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let mut current = projection.value().clone();

    let fragment = json!({
        "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    for _ in 0..2 {
        let doc_projection = Puzzle3dPlaySnapshot::new(current.clone());
        let doc = ArtifactView::new(&doc_projection, &history);
        let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
        for operation in &emit.artifact_mutations {
            current = protocol::Mutation::<serde_json::Value>::diff(operation, &current).diff().apply(&current).expect("valid mutation diff");
        }
    }

    let current = parse(&current.to_string()).expect("mutated snapshot JSON");
    let objects = current.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert_eq!(objects.iter().filter(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
}

#[semio_framework_async_macros::async_test]
async fn kit_in_port_is_declared_on_the_app_io() {
    let io = Puzzle3dPlayApp::io().expect("puzzle3d declares an AppIo");
    let port = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
    assert_eq!(port.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(port.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    assert!(matches!(port.multiplicity, PortMultiplicity::Many));
}
//#endregion 🔖️KitInPort

//#region 🔖️Convergence
/// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT object edits and,
/// after exchanging operations, both converge to contain BOTH objects — impossible under
/// whole-document `setSnapshot` snapshots, which would clobber one side.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_object_edits_via_backbone() {
    use store::MemoryBackbone;
    let mut instance_a = app().await;
    let mut instance_b = app().await;
    let seeded = object_count(&instance_a);
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle3d-convergence", "mem://puzzle3d-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("a adds object");
    dispatch(&mut instance_b, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("b adds object");

    // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
    dispatch(&mut instance_a, "commitCheckpoint", None, None).await.expect("pump a");
    dispatch(&mut instance_b, "commitCheckpoint", None, None).await.expect("pump b");

    assert_eq!(object_count(&instance_a), seeded + 2, "instance A must contain both objects");
    assert_eq!(object_count(&instance_b), seeded + 2, "instance B must contain both objects");
    let _ = Puzzle3dCamera::default();
}

//#endregion 🔖️Convergence

//#region 🔖️Close
/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END: the app-close contract, stated exactly once and asserted
/// by a test rather than by a destructor. `VcsArtifactApp::close_step` walks seven owned lanes in a
/// fixed order (`document-store`, `config-store`, `draft-store`, `presence-store`, `transient-store`,
/// window-transient, `interaction-store`) and every lane is mandatory: `drive_artifact_owned_disposer`
/// faults `interactive-job.close-owned-disposer-missing` the moment `A::build_<lane>_disposer()`
/// returned `None`. Until that holds for every lane, no puzzle3d instance can ever reach its
/// terminal-empty witness — in a test process OR in a host, where the same `close_step` runs on
/// shutdown and the framework-installed `ArtifactStoreCursorDisposer` members then panic in `Drop`.
#[semio_framework_async_macros::async_test]
async fn fixture_app_reaches_its_terminal_empty_close_witness() {
    match close_witness(app().await) {
        Ok(true) => {}
        Ok(false) => panic!("close_step reported Complete without reaching terminal-empty ownership"),
        Err(fault) => panic!("every owned close lane must supply its bounded disposer, got {fault:?}"),
    }
}
//#endregion 🔖️Close

/// 🎟️ Wave W-P: `Puzzle3dPlayApp` used to be rebuilt from `default()` on every dispatch and every render,
/// so `geometry_cache` was structurally unable to observe two calls in a row and the whole fixture was
/// re-serialized every time (`📓️2026-09-08-performance-architecture-audit.md` §1, fix #1). With a session
/// slot keyed by `app_instance_id`, the second call for the same document must serialize NOTHING.
#[test]
fn a_second_call_on_one_instance_reuses_the_geometry_cache_instead_of_reserializing() {
    let config = Puzzle3dRuntime::default();
    let fixture = default_fixture();
    let fingerprint = main::fixture_geometry_fingerprint(&fixture);
    let session = Some((4_001_u32, Some("document-geometry".to_string())));
    let cold = PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get);
    let first = with_puzzle3d_app_for(session.clone(), &config, |app| app.geometry_jsons(&fixture));
    let after_first = PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get);
    assert_eq!(after_first - cold, 1, "the first call for a cold instance serializes exactly once");
    let second = with_puzzle3d_app_for(session, &config, |app| {
        let cached = app.mesh_cache.lock().expect("mesh cache");
        assert_eq!(cached.as_ref().map(|(cached, _)| *cached), Some(fingerprint), "the session slot handed the warm cache to a brand-new app object");
        drop(cached);
        assert_eq!(app.instance_residency.lock().expect("instance residency").as_ref().map(|residency| residency.revision()), Some(1), "the per-object instance residency came back with the slot too");
        app.geometry_jsons(&fixture)
    });
    assert_eq!(PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get), after_first, "the second call on the same instance must not re-serialize anything");
    assert_eq!(first, second, "a cache hit returns byte-identical instance and mesh json");
}

/// 🥽️ Wave W-P: a mesh registered by one dispatch used to be gone by the next, because the collision
/// engine died with its app object (audit bottleneck (h)). A worker hop is exactly "a new app object with
/// the same instance id", so the resumed call must still hold the registered geometry.
#[test]
fn a_worker_hop_resume_still_holds_the_registered_brush_mesh() {
    let config = Puzzle3dRuntime::default();
    let session = Some((4_002_u32, Some("document-mesh".to_string())));
    let url = "/test/session-hop.glb";
    let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
    with_puzzle3d_app_for(session.clone(), &config, |app| {
        app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
        assert!(app.precompute.borrow().has_mesh(url), "the registering dispatch sees its own mesh");
    });
    with_puzzle3d_app_for(session, &config, |app| {
        assert!(app.precompute.borrow().has_mesh(url), "the resumed dispatch adopted the same instance's registered mesh");
    });
    with_puzzle3d_app_for(Some((4_003_u32, Some("document-other".to_string()))), &config, |app| {
        assert!(app.precompute.borrow_mut().adopt_shared_mesh(url, None), "a different document reaches the same geometry by id alone");
    });
    with_puzzle3d_app_for(None, &config, |app| {
        assert!(!app.precompute.borrow().has_mesh(url), "a call with no instance identity stays session-less, exactly as before");
    });
}

/// 🎫 Wave W-P: instance ids are reused by the framework, so a slot re-keyed to a different parent
/// document must retire — and every lease taken against the previous document must fail to check in
/// rather than leak one document's cached geometry into another's render.
#[test]
fn a_stale_session_lease_is_rejected_and_a_rekeyed_instance_starts_cold() {
    let instance = 4_004_u32;
    let stale = {
        let mut registry = puzzle3d_session_registry().lock().expect("session registry");
        let (stale, held) = registry.check_out(instance, Some("document-a")).expect("first lease");
        assert!(held.instances.is_none() && held.meshes.is_none(), "a cold slot hands out no cached geometry");
        let (fresh, rekeyed) = registry.check_out(instance, Some("document-b")).expect("re-keyed lease");
        assert!(rekeyed.instances.is_none() && rekeyed.meshes.is_none(), "a re-keyed instance starts cold instead of adopting the previous document");
        assert_ne!(stale.generation, fresh.generation, "re-keying bumps the slot generation");
        registry.check_in(stale, Puzzle3dSessionState { meshes: Some((7, "stale-meshes".into())), ..Default::default() });
        stale
    };
    let mut registry = puzzle3d_session_registry().lock().expect("session registry");
    let (_, adopted) = registry.check_out(instance, Some("document-b")).expect("post-stale lease");
    assert!(adopted.instances.is_none() && adopted.meshes.is_none(), "the stale lease's state was refused, so document-b is still cold");
    assert_ne!(stale.generation, registry.generations[usize::try_from(instance).expect("slot base") % PUZZLE3D_SESSION_SLOTS], "the retired generation is never handed out again");
}

/// ⚖️ Wave W-P: the session census is a real bound, not a slot count — a check-in whose bytes would cross
/// `PUZZLE3D_SESSION_PROCESS_BYTES` is dropped, which costs one cold rebuild and never corrupts anything.
#[test]
fn a_session_check_in_over_the_process_byte_ceiling_is_dropped() {
    let mut registry = Puzzle3dSessionRegistry::default();
    let (lease, _) = registry.check_out(11, Some("document-census")).expect("lease");
    registry.check_in(lease, Puzzle3dSessionState { meshes: Some((1, "x".repeat(PUZZLE3D_SESSION_PROCESS_BYTES))), ..Default::default() });
    assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "a census exactly at the ceiling is still admissible");
    let (lease, held) = registry.check_out(12, Some("document-second")).expect("second lease");
    assert!(held.instances.is_none() && held.meshes.is_none(), "a different instance owns a different slot and starts cold");
    registry.check_in(lease, Puzzle3dSessionState { meshes: Some((2, "y".into())), ..Default::default() });
    assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "one byte past the ceiling is refused rather than admitted");
    let (_, refused) = registry.check_out(12, Some("document-second")).expect("third lease");
    assert!(refused.instances.is_none() && refused.meshes.is_none(), "the refused state is simply absent on the next call, so that instance rebuilds cold");
}

/// 📐️ Wave W-P: the session row is a fixed 64-slot array, so one slot's inline size is multiplied by 64
/// every time the registry is constructed. Anything multi-kilobyte by value (a `BuiltNode`, a fixture)
/// belongs behind a pointer, not inline — a fat slot is how a fixed row turns into a stack overflow.
#[test]
fn one_session_slot_stays_small_enough_for_a_fixed_row() {
    let slot = size_of::<Puzzle3dSessionSlot>();
    let state = size_of::<Puzzle3dSessionState>();
    let collision = size_of::<Puzzle3dCollisionSession>();
    let app = size_of::<Puzzle3dPlayApp>();
    println!("[wave-P sizes] slot={slot} state={state} collision={collision} app={app} row={}", slot * PUZZLE3D_SESSION_SLOTS);
    assert!(slot <= 64, "one session slot grew to {slot} bytes; keep the cached state behind a pointer");
    assert!(slot * PUZZLE3D_SESSION_SLOTS <= 8 * 1024, "the whole session row grew to {} bytes and is built by value", slot * PUZZLE3D_SESSION_SLOTS);
    assert!(state <= 2048, "one session state grew to {state} bytes");
    assert!(collision <= 2048, "the carried collision session grew to {collision} bytes");
    assert!(app <= 32 * 1024, "Puzzle3dPlayApp grew to {app} bytes; it is built on the stack on every dispatch and every render, and async dispatch futures hold several copies inline");
}

//#region ⏱️InteractiveStepBudget
/// 🚨️ Mirror of `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000 µs) — that crate is not a
/// direct dependency of this artifact. `semio_framework_job` faults any interactive step at or over it
/// with `interactive_step_contract_violated`, and `MountedTypedCommandFullOperation`'s worker pump
/// cancels the lease before the fault body is surfaced, so the user reads a real budget overrun as a
/// bare "typed-operation cancelled". Every retained step of every puzzle3d command must stay under it.
const PUZZLE3D_INTERACTIVE_STEP_CEILING: std::time::Duration = std::time::Duration::from_micros(8_000);

/// 🎯️ What this artifact holds itself to in the UNOPTIMIZED test profile — a quarter of the framework
/// ceiling, so an optimized guest keeps an order of magnitude of headroom over the same document.
const PUZZLE3D_MEASURED_STEP_BUDGET: std::time::Duration = std::time::Duration::from_micros(2_000);

/// 🔁️ Cold runs each measured law drives, keeping each TURN's best across them — the standard robust
/// estimator for "how much work does one turn do", which is the only thing this artifact controls. This
/// repository is worked on by several sessions at once and this box runs their cargo builds alongside
/// the suite: at the millisecond scale a single wall-clock sample measures the scheduler, not the
/// artifact. Measured on the SAME binary, same turn: `fillBuildTick` gave 1.55 / 3.68 / 2.69 / 3.72 /
/// 2.19 ms across five consecutive runs at load average 64, and 12.67 ms at load average 79, against a
/// best of 1.25 ms (ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3). Both bounds below are therefore read
/// off those per-turn minima — including the framework ceiling, whose per-turn contract the framework
/// itself enforces at runtime; what a test can prove is that the WORK inside a turn fits it.
const PUZZLE3D_MEASURED_STEP_RUNS: u32 = 5;

/// ⏱️ Drives one retained command work to `Complete` through its REAL `step()` and answers how long
/// every single turn took, in order. The bounds are asserted by the caller, over `measured_cold_runs`'
/// per-turn best of several cold runs.
fn measured_step_loop(work: &mut dyn crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>>, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig, guard: usize, label: &str) -> Vec<std::time::Duration> {
    use crate::retained_command::PuzzleCommandWorkStep;
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let mut turns = Vec::with_capacity(guard);
    loop {
        assert!(turns.len() <= guard, "{label} step() did not reach Complete within {guard} bounded turns");
        let started = std::time::Instant::now();
        let outcome = work.step(command, snapshot, config, &interaction, &hover).expect("bounded step");
        turns.push(started.elapsed());
        if matches!(outcome, PuzzleCommandWorkStep::Complete(_)) {
            break;
        }
    }
    turns
}

/// 🎛️ The exact work `build_tool_job` routes this tool id to, bound to a window context the way the
/// framework binds every admitted tool job. The routing itself is pinned by this file's own
/// source-text guards; this mirrors it so a measurement drives the REAL production work. Each run is
/// bound to its OWN instance id so it starts from a cold session slot — a warm slot skips the mesh
/// seeding that is exactly what the budget is being measured against.
fn measured_tool_work(tool_id: &'static str, run: u32) -> Box<dyn crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>>> {
    use crate::retained_command::PuzzleCommandWork;
    let mut work: Box<dyn PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>>> = match tool_id {
        "acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default()),
        "setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default()),
        "fillBuildTick" => Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id)),
        "openVortexSuggestions" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
        _ => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent)),
    };
    work.bind_view_state(Some(measured_view_state()));
    work.bind_window_owners(None, None);
    work.bind_instance(9_000 + run, &format!("measured-{tool_id}-{run}"));
    work
}

/// ⏱️ Drives one tool id's REAL work to `Complete` `PUZZLE3D_MEASURED_STEP_RUNS` times, each from its
/// own cold session slot, and answers `(turns, the worst PER-TURN BEST)`: every run of the same cold
/// document takes the same bounded turns in the same order (asserted), so turn `t`'s cost is the
/// minimum of that turn across the runs, and the law is on the worst of those minima. A scheduler
/// preemption lands on one turn of one run and is cancelled by the others — see
/// `PUZZLE3D_MEASURED_STEP_RUNS` for why that matters on this box.
fn measured_cold_runs(tool_id: &'static str, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig) -> (usize, std::time::Duration) {
    let mut best: Vec<std::time::Duration> = Vec::new();
    for run in 0..PUZZLE3D_MEASURED_STEP_RUNS {
        let mut work = measured_tool_work(tool_id, run);
        let turns = measured_step_loop(work.as_mut(), command, snapshot, config, crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, tool_id);
        if best.is_empty() {
            best = turns;
            continue;
        }
        assert_eq!(turns.len(), best.len(), "{tool_id} run {run} took a different number of bounded turns than its cold siblings");
        for (slot, measured) in best.iter_mut().zip(turns) {
            *slot = (*slot).min(measured);
        }
    }
    let (index, worst) = best.iter().enumerate().max_by_key(|(_, turn)| **turn).map_or((0, std::time::Duration::ZERO), |(index, turn)| (index + 1, *turn));
    eprintln!("[DEBUG] puzzle3d {tool_id}: {} turns, worst turn {index} at {worst:?}", best.len());
    (best.len(), worst)
}

fn measured_view_state() -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel {
        window_id: Some(main::WINDOW_KIND_ID.to_string()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }],
        ..Default::default()
    }
}

fn measured_nakagin_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into())
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END: `openVortexSuggestions` syncs the whole precompute session
/// and refreshes one vortex's brush candidates. Measured on Nakagin at opt-level 0: 82.5 ms (W-P2's
/// baseline), 17.6 ms once W-P2 removed the doubled session sync and the doubled projection decode, and
/// under this artifact's own 2 000 µs budget once W-P3 split `handle_action_impl`'s prologue into the
/// scene / session-sync / dispatch turns [`Puzzle3dActionPrologue`] declares and made each of them typed.
/// This drives the REAL `Puzzle3dWindowCommandWork` `build_tool_job` routes this tool id to.
#[test]
fn open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("openVortexSuggestions", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(main::WINDOW_KIND_ID.to_string())).expect("openVortexSuggestions command decodes");
    let (steps, worst) = measured_cold_runs("openVortexSuggestions", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "openVortexSuggestions worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "openVortexSuggestions worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END: `fillBuildTick` scans the document across its bounded census
/// turns and then runs the shared action prologue, which used to be ONE 17.6 ms publish turn and is now
/// the scene turn, one turn per owed collision-mesh fallback, the engine-scene build and push turns, and
/// the dispatch turn — every one of them inside this artifact's own 2 000 µs budget.
#[test]
fn fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("fillBuildTick", None, Some(main::WINDOW_KIND_ID.to_string())).expect("fillBuildTick command decodes");
    let (steps, worst) = measured_cold_runs("fillBuildTick", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "fillBuildTick worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "fillBuildTick worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P2: `acceptSuggestion` walks the document for its target
/// vortex and publishes the placement. Measured at 11 796 µs in ONE step before this wave.
#[test]
fn accept_suggestion_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(main::WINDOW_KIND_ID.to_string())).expect("acceptSuggestion command decodes");
    let (steps, worst) = measured_cold_runs("acceptSuggestion", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "acceptSuggestion worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "acceptSuggestion worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

fn measured_concrete_forest_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&CONCRETE_FOREST_EXAMPLE_FIXTURE.clone())).into())
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3: swapping the Concrete Forest document for the
/// 180-object Nakagin one is the single largest document gesture this artifact has — it deletes every
/// existing attraction and object and creates every Nakagin one. Its work is fully typed and cursorized
/// (`Puzzle3dSetActiveExampleWork`), so no turn of it may cross the interactive step ceiling either.
#[test]
fn set_active_example_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_concrete_forest_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(main::WINDOW_KIND_ID.to_string())).expect("setActiveExample command decodes");
    let (steps, worst) = measured_cold_runs("setActiveExample", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "setActiveExample worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "setActiveExample worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// 🌉️ Differential law for W-P3's typed `scene_from_snapshot`: the derived `ToValue`/`FromValue`
/// machinery is an independent implementation of the same structural-twin translation, so the typed
/// fixture must equal what the persisted-projection bridge produced for every shipped document. A
/// disagreement here is a real behaviour change, not a performance one — the semantic document delta
/// every editing action publishes is taken against exactly this fixture.
///
/// 🪪️ Each snapshot is canonicalized first (rebuilt from its OWN typed authority) because that is the
/// only projection production ever hands the app: a store-driven `Puzzle3dPlaySnapshot` carries the
/// typed document and materializes `value()` from it, so `value()` is by construction
/// `ToValue(typed())`. Feeding a raw editor-side fixture straight into `new()` can hand the two halves
/// different content — `empty_fixture()`'s `meta` serializes both members as `Null`, which the typed
/// decode refuses, and `new()` silently falls back to `Puzzle3dSnapshot::default()`; see this wave's
/// report §6.
///
/// 🗝️ The two untyped `meta` members are compared as the TYPED catalogs they stand for
/// (`Puzzle3dKindCatalogs` / `Vec<Puzzle3dKindCompatibility>` — lossless, and what every reader of
/// those members ultimately decodes them into), because raw `DslValue` equality would compare key
/// ORDER: the bridge inherits the persisted projection's own `serde_json` map order and the typed
/// construction emits declaration order, for byte-identical content. Every typed member of the fixture
/// is compared directly, unnormalized.
#[test]
fn puzzle3d_typed_fixture_matches_the_projection_bridge_for_every_example() {
    for (label, fixture) in [("empty", empty_fixture()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()), ("nakagin", NAKAGIN_EXAMPLE_FIXTURE.clone())] {
        let seed = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&fixture)).into());
        let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(seed.typed())).into());
        let bridged = scene_from_projection(&puzzle3d_projection_value(snapshot.value()), Puzzle3dRuntime::default(), "utility");
        let typed = scene_from_snapshot(snapshot.typed(), Puzzle3dRuntime::default(), "utility");
        assert_eq!(typed.fixture.schema, bridged.fixture.schema, "{label}: schema disagrees");
        assert_eq!(typed.fixture.domain, bridged.fixture.domain, "{label}: domain disagrees");
        assert_eq!(typed.fixture.objects, bridged.fixture.objects, "{label}: objects disagree");
        assert_eq!(typed.fixture.attractions, bridged.fixture.attractions, "{label}: attractions disagree");
        assert_eq!(typed.fixture.target_volumes, bridged.fixture.target_volumes, "{label}: target volumes disagree");
        assert_eq!(typed.fixture.references, bridged.fixture.references, "{label}: references disagree");
        let catalogs = |meta: &Puzzle3dFixtureMeta| -> crate::Puzzle3dKindCatalogs { meta.kind_catalogs.clone().map_or_else(crate::Puzzle3dKindCatalogs::default, |rows| dsl::FromValue::from_value(rows).expect("kind catalogs decode")) };
        let compatibility = |meta: &Puzzle3dFixtureMeta| -> Vec<crate::Puzzle3dKindCompatibility> { meta.kind_compatibility.clone().map_or_else(Vec::new, |rows| dsl::FromValue::from_value(rows).expect("kind compatibility decodes")) };
        assert_eq!(catalogs(&typed.fixture.meta), catalogs(&bridged.fixture.meta), "{label}: kind catalogs disagree");
        assert_eq!(compatibility(&typed.fixture.meta), compatibility(&bridged.fixture.meta), "{label}: kind compatibility disagrees");
        assert_eq!(typed.active_utility, bridged.active_utility, "{label}: active utility disagrees");
    }
}

/// 🌉️ Differential law for W-P3's typed `scene_config`: the same engine scene the all-`DslValue` bridge
/// (`scene_config_value` + the derived `FromValue`) produced, field for field, on every shipped document
/// and with real kind weights on the runtime. `SceneConfig: PartialEq` is the engine's OWN resync
/// verdict, so equality here is exactly the property the precompute session reads.
#[test]
fn puzzle3d_typed_scene_config_matches_the_value_bridge_for_every_example() {
    for (label, fixture) in [("empty", empty_fixture()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()), ("nakagin", NAKAGIN_EXAMPLE_FIXTURE.clone())] {
        let mut runtime = Puzzle3dRuntime::default();
        runtime.overlap_budget = 0.375;
        runtime.object_kind_weights.insert("capsule".into(), 0.25);
        runtime.vortex_kind_weights.insert("rim".into(), 0.75);
        let envelope = Puzzle3dScene { fixture, runtime, active_utility: "utility".into() };
        let bridged: crate::standards::v1::subsets::any::schema::SceneConfig = dsl::FromValue::from_value(scene_config_value(&envelope)).expect("value bridge decodes");
        let typed = scene_config(&envelope).expect("typed scene config builds");
        assert_eq!(typed, bridged, "{label}: typed engine scene disagrees with the value bridge");
    }
}

/// 🥽️ W-P3: the mesh-url index answers exactly what the per-object catalog scan answered, for every
/// object of every shipped document, and `collect_mesh_urls` still returns the same SET of identities.
#[test]
fn puzzle3d_kind_mesh_index_matches_a_per_object_catalog_scan() {
    for (label, fixture) in [("empty", empty_fixture()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()), ("nakagin", NAKAGIN_EXAMPLE_FIXTURE.clone())] {
        let index = Puzzle3dKindMeshIndex::of(&fixture.meta);
        for object in &fixture.objects {
            let scanned = fixture
                .meta
                .kind_catalogs
                .as_ref()
                .and_then(|catalogs| catalogs.get("objects"))
                .and_then(dsl::DslValue::as_array)
                .and_then(|rows| rows.iter().find(|row| row.get("id").and_then(dsl::DslValue::as_str) == object.object_kind.as_deref()))
                .and_then(|row| row.get("meshUrl"))
                .and_then(dsl::DslValue::as_str);
            let expected = object.mesh_url.as_deref().filter(|url| !url.is_empty()).or(scanned);
            assert_eq!(index.resolve(object), expected, "{label}: {} resolved to a different mesh identity", object.id);
        }
        let indexed: std::collections::BTreeSet<String> = collect_mesh_urls(&fixture).into_iter().collect();
        assert!(indexed.iter().all(|url| !url.is_empty()), "{label}: an empty mesh identity was collected");
    }
}

//#endregion ⏱️InteractiveStepBudget

//#region 🩹️CheckedDefects
/// 🎯️ Every action id one built context menu dispatches, groups and submenus included.
fn context_menu_action_ids(items: &[semio_framework_plugin::ContextMenuItemSpec], into: &mut Vec<(String, String)>) {
    for item in items {
        if let Some(action) = item.action.as_deref() {
            into.push((item.id.clone(), action.to_string()));
        }
        if let Some(children) = item.children.as_ref() {
            context_menu_action_ids(children, into);
        }
    }
}

/// 🎯️ Every action id `create_puzzle3d_app()` declares — the set `dispatch_action` can actually route.
fn declared_action_ids() -> std::collections::BTreeSet<String> {
    create_puzzle3d_app().window_kinds.iter().flat_map(|window| window.actions.iter()).map(|action| action.id.clone()).collect()
}

/// 🖱️ Context-menu rows bypass the registry-validated `Menu::action` builder (they carry
/// `Puzzle3dLabels` text the English-only `ActionDefinition` cannot resolve), so nothing catches a
/// typo'd action id at build time. `📓️2026-09-09-user-feature-checklist.md` §15/§22 and summary #7:
/// "Zoom to Selection" dispatched `zoomToSelection`, which this crate never declares — the registered
/// verb is `focusSelection`. This law walks EVERY selection kind's menu and holds every emitted id
/// inside the declared set, so the whole class cannot come back.
#[semio_framework_async_macros::async_test]
async fn every_context_menu_row_dispatches_a_declared_action() {
    let declared = declared_action_ids();
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let object_id = first_object_id(&app);
    let vortex_id = first_vortex_full_id(&app);
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let mut zoom_rows = 0usize;
    for (granularity, id) in [
        (PUZZLE3D_GRANULARITY_OBJECT, object_id.as_str()),
        (PUZZLE3D_GRANULARITY_VORTEX, vortex_id.as_str()),
        (PUZZLE3D_GRANULARITY_TARGET_VOLUME, volume_id.as_str()),
        (PUZZLE3D_GRANULARITY_REFERENCE, "reference-1"),
        (PUZZLE3D_GRANULARITY_ATTRACTION, "attraction-1"),
    ] {
        let menu = context_menu_for_selection(&mut app, granularity, id).await;
        let mut rows = Vec::new();
        context_menu_action_ids(&menu, &mut rows);
        assert!(!rows.is_empty(), "{granularity} selection built a menu with no actionable row");
        for (row, action) in &rows {
            assert!(declared.contains(action), "{granularity} row {row} dispatches undeclared action {action}; declared: {declared:?}");
            if row == "zoom" {
                zoom_rows += 1;
                assert_eq!(action, "focusSelection", "the zoom row must dispatch the registered camera verb");
            }
        }
        eprintln!("[DEBUG] context menu {granularity} rows={rows:?}");
    }
    assert_eq!(zoom_rows, 3, "object, vortex and reference selections each carry a Zoom to Selection row");
    // 🎯️ Reachability, not just declaration: the id the row carries must be one the typed command
    // channel actually admits (an unknown id panics inside `dispatch`).
    dispatch(&mut app, "focusSelection", None, None).await.expect("the context menu's zoom row must dispatch");
}

/// 🗂️ `📓️2026-09-09-user-feature-checklist.md` §23/summary #9: both the "Add Object" dialog and the
/// standalone `addObjectKind` arg form hardcoded ONE static option (`"Object"`), a kind neither
/// example's catalog declares — so the dialog could not add a single real object kind. Both forms must
/// now offer exactly the declared examples' own catalog rows, bounded by
/// [`PUZZLE3D_OBJECT_KIND_OPTIONS_MAX`], with a default that IS one of them.
#[semio_framework_async_macros::async_test]
async fn the_add_object_dialog_offers_every_object_kind_of_both_examples() {
    use semio_framework_plugin::ArgSchema;
    let definition = create_puzzle3d_app();
    let expected: Vec<String> = [&*CONCRETE_FOREST_EXAMPLE_FIXTURE, &*NAKAGIN_EXAMPLE_FIXTURE]
        .into_iter()
        .flat_map(|fixture| puzzle3d_kind_ids(fixture, "objects"))
        .fold(Vec::new(), |mut ids, id| {
            if !ids.contains(&id) {
                ids.push(id);
            }
            ids
        });
    assert!(expected.len() >= 2, "both shipped examples must declare object kinds, got {expected:?}");
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    let standalone = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "addObjectKind").expect("addObjectKind declared").args.clone();
    for (surface, args) in [("dialog", dialog.args.clone()), ("action", standalone)] {
        let arg = args.iter().find(|arg| arg.id == "objectKind").unwrap_or_else(|| panic!("{surface} declares an objectKind arg"));
        let ArgSchema::String { options, .. } = &arg.schema else { panic!("{surface}'s objectKind must stay a select") };
        let values: Vec<String> = options.iter().map(|option| option.value.clone()).collect();
        assert_eq!(values, expected, "{surface} must offer the live catalog's object kinds");
        assert!(values.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "{surface} exceeded its fixed option ceiling");
        let default = arg.default.as_ref().and_then(dsl::DslValue::as_str).unwrap_or_default().to_string();
        assert!(values.contains(&default), "{surface}'s default {default} is not one of its own options");
    }
    assert!(dialog.args.iter().all(|arg| arg.required), "the dialog's kind select stays required");
}

/// 🗨️ Wave B11 (checklist §23, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 7): the shell
/// fallback menu / command palette published TWO rows carrying the identical "Add Object…" label —
/// `addObjectKind` (arg-carrying, so the shell redirects it to the bare action pane) and
/// `openAddObjectDialog` (the declared dialog). The arg-carrying one came FIRST, so every user and
/// every probe clicking "Add Object…" got the action pane and the dialog was unreachable. Exactly one
/// add-object row may be palette/menu vocabulary, and it must be the dialog opener; it must also be
/// declared ahead of `addObjectKind`, because the shell menu keeps its first leaves at top level and
/// folds the rest into "More ›".
#[semio_framework_async_macros::async_test]
async fn exactly_one_add_object_row_is_menu_vocabulary_and_it_opens_the_dialog() {
    let definition = create_puzzle3d_app();
    let actions: Vec<&ActionDefinition> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).collect();
    let find = |id: &str| *actions.iter().find(|action| action.id == id).unwrap_or_else(|| panic!("{id} declared"));
    let opener = find("openAddObjectDialog");
    let parametrized = find("addObjectKind");
    assert_eq!(opener.kind, ActionKind::Shell, "the dialog opener emits an effect, never a document edit");
    assert!(opener.in_palette, "the dialog opener IS the user-facing add-object row");
    assert!(!parametrized.in_palette, "the dialog's parametrized verb must not publish a second identical row");
    fn label(action: &ActionDefinition) -> &str {
        action.label.resolve(semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::En)
    }
    assert!(label(opener).starts_with(label(parametrized)), "both rows describe the same verb, which is exactly why only one may be offered: {:?} vs {:?}", label(opener), label(parametrized));
    let visible: Vec<&str> = actions.iter().filter(|action| action.in_palette && label(action).starts_with("Add Object")).map(|action| action.id.as_str()).collect();
    assert_eq!(visible, vec!["openAddObjectDialog"], "exactly one add-object row may reach a menu");
    let order = |id: &str| actions.iter().position(|action| action.id == id).unwrap_or_else(|| panic!("{id} declared"));
    assert!(order("openAddObjectDialog") < order("addObjectKind"), "the dialog opener must be declared ahead of the verb it drives so the shell menu keeps it at top level");
    // 🎯️ Reachability, not just declaration: the row's id must be one the typed command channel admits.
    let mut app = app().await;
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("the add-object row must dispatch");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::OpenDialog { dialog_id, .. } if dialog_id == "addObject")),
        "clicking the add-object row must open the declared dialog: {:?}",
        result.requested_effects,
    );
}

/// 🗣️ `📓️2026-09-09-user-feature-checklist.md` §14/summary #10: the engagement input advertised
/// `clear`/`rectangle`/`lasso` while the parser silently dropped all three. The placeholder is now
/// DERIVED from the verb list, and every verb in it must do something observable: the three marquee
/// verbs move the viewport's `selection.method`, and `clear` empties the framework-owned domain.
#[semio_framework_async_macros::async_test]
async fn every_advertised_engagement_verb_is_implemented() {
    use crate::editor::puzzle3d::commands::engagement_submit::PUZZLE3D_ENGAGEMENT_VERBS;
    let mut app = app().await;
    let envelope = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let placeholder = main::engagement(&envelope, &Puzzle3dLabels::NATIVE_EN).input.and_then(|input| input.placeholder).unwrap_or_default();
    for verb in PUZZLE3D_ENGAGEMENT_VERBS {
        assert!(placeholder.contains(verb), "the engagement placeholder must advertise {verb}: {placeholder}");
    }
    let method_of = |node: &Value| selection_of(node).get("method").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(method_of(&render_composite(&mut app).await), PUZZLE3D_SELECTION_METHOD_PICK, "a fresh window sweeps with the pick default");
    for method in [PUZZLE3D_SELECTION_METHOD_RECTANGLE, PUZZLE3D_SELECTION_METHOD_LASSO, PUZZLE3D_SELECTION_METHOD_PICK] {
        dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": method })), None).await.unwrap_or_else(|error| panic!("engagementSubmit {method}: {error:?}"));
        let rendered = render_composite(&mut app).await;
        assert_eq!(method_of(&rendered), method, "typing {method} must move the viewport marquee method");
    }
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let selected = |node: &Value| selection_of(node).get("ids").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
    assert_eq!(selected(&render_composite(&mut app).await), 1, "the object must be selected before clear");
    dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "clear" })), None).await.expect("engagementSubmit clear");
    assert_eq!(selected(&render_composite(&mut app).await), 0, "typing clear must empty the framework-owned selection");
    eprintln!("[DEBUG] engagement verbs={PUZZLE3D_ENGAGEMENT_VERBS:?}");
}

/// 🪣️ Wave B13, `📓️2026-09-11-wave-B1-battery-extension.md` §14 `engagement-fill-verb`: typing `fill <n>`
/// has to ARM the Fill tool, not merely park a utility name in the scene. Fill is a mode-level TOOL, so the
/// only thing that can press its `#tool.fill` tab is an `Effect::SetActiveTool` leaving the turn; `brush` is
/// a window UTILITY and must take the `Effect::SetActiveUtility` lane instead. This law pins both lanes plus
/// the count hand-off, because the two are one `match` apart in `engagement_submit` and the browser
/// symptom of getting it wrong (`#tool.fill aria-pressed=null`) is indistinguishable from a dead input.
#[semio_framework_async_macros::async_test]
async fn the_engagement_fill_verb_arms_the_fill_tool_and_hands_over_its_count() {
    let mut app = app().await;
    let result = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "fill 5" })), None).await.expect("engagementSubmit fill 5");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == fill_tool::TOOL_ID)),
        "typing `fill 5` must arm the fill tool: {:?}",
        result.requested_effects,
    );
    assert!(
        !result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { .. })),
        "fill is a tool, so it must not also claim a window utility: {:?}",
        result.requested_effects,
    );
    let count = result
        .requested_effects
        .iter()
        .find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "setFillCount" => args.as_ref().and_then(|value| value.get("value")).and_then(dsl::DslValue::as_f64),
            _ => None,
        })
        .expect("typing `fill 5` must hand the count to the retained setFillCount command");
    assert_eq!(count, 5.0, "the typed count reaches setFillCount verbatim");
    let brush = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), None).await.expect("engagementSubmit brush");
    assert!(
        brush.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { utility_id, .. } if utility_id == "brush")),
        "typing `brush` must claim the brush window utility: {:?}",
        brush.requested_effects,
    );
    assert!(
        !brush.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { .. })),
        "leaving fill is the host's own `setActiveTool \"\"`, never a guest tool effect: {:?}",
        brush.requested_effects,
    );
}

/// 🧰️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B35, checklist §14 `engagement-brush-verb`: `fill`
/// arms and `brush` does not, because `fill` is a MODE-level tool whose effect carries no window and
/// `brush` is a window utility whose effect carries one. `active_utility_by_window_id` is keyed by
/// window INSTANCE (wave B9), and a pane reads only its own key — so an effect addressed at the bare
/// window KIND (`puzzle3d-main`) arms a key nothing ever reads and the world lane keeps publishing
/// `activeUtility=select` forever, which is exactly the browser verdict. The sibling law above pins
/// WHICH effect the verb emits; this one pins WHERE it lands, on the multi-pane roster the shell
/// actually runs (the engagement line is window chrome, and an app-level panel resolves through
/// `focused_window_id`, wave B15 — both must resolve to an instance, never to the kind).
#[semio_framework_async_macros::async_test]
async fn the_engagement_brush_verb_arms_the_utility_of_a_pane_instance_never_the_window_kind() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    let top = "puzzle3d-main-top";
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": true })), Some(top)).await.expect("grow the roster to two panes");
    let armed = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), Some(perspective)).await.expect("engagementSubmit brush at the pane");
    let addressed: Vec<&String> = armed
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == utilities::brush::UTILITY_ID => Some(window_id),
            _ => None,
        })
        .collect();
    assert_eq!(addressed, vec![&perspective.to_string()], "the pane-addressed brush verb must arm that pane's own key: {:?}", armed.requested_effects);
    let unaddressed = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), None).await.expect("engagementSubmit brush with no window of its own");
    let fallback: Vec<&String> = unaddressed
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == utilities::brush::UTILITY_ID => Some(window_id),
            _ => None,
        })
        .collect();
    eprintln!("[DEBUG] engagement brush addressed={addressed:?} fallback={fallback:?}");
    assert!(
        fallback.iter().all(|window_id| !crate::editor::puzzle3d::puzzle3d_window_id_is_kind(window_id)),
        "a verb that carries no window of its own must still resolve a pane INSTANCE, never the bare kind: {fallback:?}",
    );
}

/// 🛑 Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31, checklist §12/§14 `Abort`: Escape while the Fill
/// TOOL is armed has to LEAVE it — cancel whatever plan is in flight and disarm the tool — instead of
/// returning untouched, which is what `engagement_abort` did while `puzzle3d_fill_tool_active` and why
/// the browser read `engagement-abort ... activeUtility=fill` after every `fill <n>` (wave B29 §2.5).
/// The disarm is the app's own explicit effect, and the abort leaves NO tool and NO utility armed —
/// the same place the shell's own Escape leaves them (`ShellHost` dispatches `setActiveTool ""`), not
/// a remembered previous tool this app keeps no history of. The `brush`-claims-a-utility law above
/// still holds: a plain verb switch never emits a tool effect, only this abort does.
#[semio_framework_async_macros::async_test]
async fn escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    let mut armed = app().await;
    dispatch(&mut armed, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("arm fill");
    let aborted = dispatch(&mut armed, "engagementAbort", None, None).await.expect("engagementAbort");
    assert!(
        aborted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())),
        "Escape with fill armed must disarm the tool: {:?}",
        aborted.requested_effects,
    );
    assert!(
        !aborted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == fill_tool::TOOL_ID)),
        "and must never re-arm the tool it is leaving: {:?}",
        aborted.requested_effects,
    );
    let active_utility = |node: &Value| selection_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    let rendered = render_window(&mut armed, main::WINDOW_KIND_ID).await;
    assert_eq!(active_utility(&rendered), PUZZLE3D_DEFAULT_UTILITY, "the abort leaves no utility armed either: {}", selection_of(&rendered));

    // 🏛️ `active_tool_id` is host-owned (`ViewModel`, `🪟️window/🦀️.rs` `host_activation`), so the guest
    // keeps reading `fill` until the host answers the disarm. A second Escape in that window therefore
    // repeats the same idempotent disarm — and still cancels nothing, because there is no live plan.
    let repeated = dispatch(&mut armed, "engagementAbort", None, None).await.expect("second engagementAbort");
    assert!(
        repeated.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())),
        "while the host still reports fill armed the disarm repeats rather than giving up: {:?}",
        repeated.requested_effects,
    );
    assert!(
        !repeated.requested_effects.iter().any(|effect| matches!(effect, Effect::CancelJob { .. })),
        "and cancels no job, because none is in flight: {:?}",
        repeated.requested_effects,
    );

    let mut unarmed = app().await;
    let untouched = dispatch(&mut unarmed, "engagementAbort", None, None).await.expect("engagementAbort with nothing armed");
    assert!(
        !untouched.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { .. } | Effect::CancelJob { .. })),
        "an abort with no tool armed owes no tool or cancel effect: {:?}",
        untouched.requested_effects,
    );
}

/// 🧯️ `📓️2026-09-09-user-feature-checklist.md` §9/§13/summary #13: a placement that produces nothing
/// used to produce NOTHING — no fault, no notice, no visible change. Every refusal path of the two
/// placement verbs must now carry exactly one `Effect::Notify`, and a SUCCESSFUL placement must carry
/// none (a notice on the happy path would be noise, and would hide the real ones).
#[semio_framework_async_macros::async_test]
async fn a_refused_placement_surfaces_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    // 🎯️ Nothing selected, no popup open, no `fullId` — the accept has no target at all.
    let orphan = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion without a target still completes");
    assert_eq!(notices(&orphan).len(), 1, "an accept with no target must say so: {:?}", orphan.requested_effects);
    // 🎯️ A brush placement naming a vortex no object owns, on a kind the catalog does not declare.
    let unknown = dispatch(
        &mut app,
        "addBrushObject",
        Some(&json!({ "targetVortexFullId": "no-such-object:v0", "objectKindId": "no-such-kind", "sourceVortexIndex": 0, "origin": [0.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0] })),
        None,
    )
    .await
    .expect("addBrushObject on an unknown kind still completes");
    assert_eq!(notices(&unknown).len(), 1, "a brush placement onto an undeclared kind must say so: {:?}", unknown.requested_effects);
    for message in notices(&orphan).into_iter().chain(notices(&unknown)) {
        assert_ne!(message, PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored locale/terminology axis, so the notice must be real prose");
        assert!(!message.is_empty(), "a notice must carry text");
    }
    // 🎯️ The happy path stays quiet.
    let placed = dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    assert!(notices(&placed).is_empty(), "a successful action must not raise a notice: {:?}", placed.requested_effects);
    eprintln!("[DEBUG] refusal notices orphan={:?} unknown={:?}", notices(&orphan), notices(&unknown));
}

/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §16 (re-checked for W-D4's §17 law): the inspection
/// panel's `patchInspector` flag rows must carry the INVERSE of the row's current state, so an object
/// hidden from here can be un-hidden from here. Driven end to end — hide through the panel's own
/// dispatch shape, then re-read the panel and show again.
#[semio_framework_async_macros::async_test]
async fn inspection_flag_rows_toggle_back_off() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    for expected in [true, false] {
        let panel = to_json_string(&render_body(&mut app, inspection::BODY_KEY).await);
        assert!(panel.contains("puzzle3d-play-inspector.object.hidden"), "the inspector must render the object's hidden row: {panel}");
        dispatch(&mut app, "patchInspector", Some(&json!({ "entity": PUZZLE3D_GRANULARITY_OBJECT, "field": "hidden", "ids": [object_id.as_str()], "value": expected })), None).await.expect("patchInspector hidden");
        assert_eq!(hidden_of(&app, &object_id), expected, "the inspector's hidden row must reach {expected}");
    }
}

/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §17: the outliner row's own Hide/Show control, driven
/// through the args the panel itself declares — never a hand-written literal, since the reported defect
/// ("Hide does nothing, Show can never un-hide") was exactly a wrong `value` baked into those args. Hide,
/// re-read the panel, and use the row's NEW args to show again; the flag must round-trip both ways.
#[semio_framework_async_macros::async_test]
async fn outliner_row_hide_and_show_round_trip_through_their_own_declared_args() {
    fn hidden_flag_args(node: &Value, object_id: &str) -> Option<Value> {
        if node.get("flag").and_then(Value::as_str) == Some("hidden")
            && node.get("ids").and_then(Value::as_array).is_some_and(|ids| ids.iter().any(|id| id.as_str() == Some(object_id)))
        {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| hidden_flag_args(child, object_id)),
            Value::Array(items) => items.iter().find_map(|item| hidden_flag_args(item, object_id)),
            _ => None,
        }
    }
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    assert!(!hidden_of(&app, &object_id), "a freshly added object starts visible");
    for expected in [true, false] {
        let panel: Value = from_json_str(&to_json_string(&render_body(&mut app, document::BODY_KEY).await)).expect("the outliner renders parseable ui json");
        let args = hidden_flag_args(&panel, &object_id).unwrap_or_else(|| panic!("the outliner row must declare a hidden-flag action for {object_id}: {panel}"));
        assert_eq!(args.get("value").and_then(Value::as_bool), Some(expected), "the row's own action must ask for the INVERSE of the state it renders: {args}");
        dispatch(&mut app, "setSelectionFlag", Some(&args), None).await.expect("setSelectionFlag from the outliner row's own args");
        assert_eq!(hidden_of(&app, &object_id), expected, "the outliner row's hidden flag must reach {expected}");
    }
}

/// 🙈️ The hop AFTER the one above: hiding an object from the outliner row must also reach the WORLD
/// lane the viewport paints from, and the row itself must re-render with the inverse control. Wave B20
/// measured the browser publishing an unchanged row AND an unchanged `data-instances-json` while the
/// host's history had already recorded `change-object-hidden id=seed-left-001 new-hidden=true`, so
/// "the flag reached the projection" (the law above) is NOT the same claim as "the two surfaces the
/// user looks at moved". `world_instances_geometry_json` encodes hidden as a zero scale — the id stays
/// in the array so no other object's index shifts — and the row's own control flips Hide↔Show.
#[semio_framework_async_macros::async_test]
async fn outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control() {
    fn instance_scale(world: &Value, object_id: &str) -> Vec<f64> {
        let instances: Value = parse(world.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("the world body publishes an instances lane")).expect("instances lane is json");
        instances
            .as_array()
            .expect("instances lane is an array")
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id))
            .and_then(|instance| instance.get("scale").and_then(Value::as_array))
            .map(|scale| scale.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    }
    /// 🙈️ The visibility row action's rendered face: the smallest node that both carries an `icon` and
    /// declares this object's `hidden` flag args underneath it. Keyed on the args rather than on a row
    /// id so the law survives any reshaping of the tree above the control.
    fn visibility_control_icon(panel: &Value, object_id: &str) -> Option<String> {
        fn declares_hidden(node: &Value, object_id: &str) -> bool {
            if node.get("flag").and_then(Value::as_str) == Some("hidden") && node.get("ids").and_then(Value::as_array).is_some_and(|ids| ids.iter().any(|id| id.as_str() == Some(object_id))) {
                return true;
            }
            match node {
                Value::Object(fields) => fields.iter().any(|(_, child)| declares_hidden(child, object_id)),
                Value::Array(items) => items.iter().any(|item| declares_hidden(item, object_id)),
                _ => false,
            }
        }
        fn walk(node: &Value, object_id: &str) -> Option<String> {
            let deeper = match node {
                Value::Object(fields) => fields.iter().find_map(|(_, child)| walk(child, object_id)),
                Value::Array(items) => items.iter().find_map(|item| walk(item, object_id)),
                _ => None,
            };
            if deeper.is_some() {
                return deeper;
            }
            let icon = node.get("icon").and_then(Value::as_str)?;
            (declares_hidden(node, object_id) && matches!(icon, "eye" | "eye-off")).then(|| icon.to_string())
        }
        walk(panel, object_id)
    }
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let visible = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(instance_scale(&visible, &object_id), vec![1.0, 1.0, 1.0], "a visible object publishes its real scale");
    let before_icon = visibility_control_icon(&render_body(&mut app, document::BODY_KEY).await, &object_id);
    assert_eq!(before_icon.as_deref(), Some("eye"), "a visible object's row renders the Hide control");

    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "flag": "hidden", "ids": [object_id.clone()], "value": true })), None).await.expect("setSelectionFlag hidden");

    let hidden = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(instance_scale(&hidden, &object_id), vec![0.0, 0.0, 0.0], "a hidden object must publish a zero scale into the world instance lane");
    let after_icon = visibility_control_icon(&render_body(&mut app, document::BODY_KEY).await, &object_id);
    assert_eq!(after_icon.as_deref(), Some("eye-off"), "a hidden object's row must re-render as the Show control");
    eprintln!("[DEBUG] outliner hide world lane scale={:?} rowIcon={after_icon:?}", instance_scale(&hidden, &object_id));
}

/// 👁️ The RESTORE half of the law above, in the SAME settle: the row's Show control must put the object's
/// real scale back into the world instance lane, not merely flip the row's label back. The lane is served
/// from [`Puzzle3dInstanceResidency`], which re-serializes only the records whose own
/// `instance_record_fingerprint` moved and caches every other record verbatim — so a key that failed to
/// read `hidden` would leave an un-hidden object at `[0,0,0]` for the rest of the session while the row,
/// the projection and the history all read `hidden=false`, and nothing but the world would disagree.
/// Browser battery #64 reported `outliner-show-restores restored=false` while the `seed-left-001` row had
/// already come back to `Hide` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B57), and no row-text verdict
/// can tell those two apart.
#[semio_framework_async_macros::async_test]
async fn outliner_show_restores_the_world_instance_scale_in_the_same_settle() {
    fn instance_scale(world: &Value, object_id: &str) -> Vec<f64> {
        let instances: Value = parse(world.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("the world body publishes an instances lane")).expect("instances lane is json");
        instances
            .as_array()
            .expect("instances lane is an array")
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id))
            .and_then(|instance| instance.get("scale").and_then(Value::as_array))
            .map(|scale| scale.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    }
    let flag_args = |object_id: &str, value: bool| json!({ "entity": "object", "flag": "hidden", "ids": [object_id], "value": value });
    let mut app = app().await;
    let object_id = first_object_id(&app);
    assert_eq!(instance_scale(&render_body(&mut app, main::BODY_KEY).await, &object_id), vec![1.0, 1.0, 1.0], "a visible object publishes its real scale");
    dispatch(&mut app, "setSelectionFlag", Some(&flag_args(&object_id, true)), None).await.expect("setSelectionFlag hidden=true");
    assert_eq!(instance_scale(&render_body(&mut app, main::BODY_KEY).await, &object_id), vec![0.0, 0.0, 0.0], "the hide half must still reach the world lane");
    dispatch(&mut app, "setSelectionFlag", Some(&flag_args(&object_id, false)), None).await.expect("setSelectionFlag hidden=false");
    let shown = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(object_flag(&app, &object_id, "hidden"), Some(false), "the show write must reach the document");
    assert_eq!(instance_scale(&shown, &object_id), vec![1.0, 1.0, 1.0], "un-hiding must republish the object's real scale — a residency that cached the zero-scale record would leave it invisible for good");
}

/// 🎯️ An outliner row's flag write names its OWN entity, so the live selection must not decide what it
/// hits. `set_selection_flag` chooses between the explicit `{entity, ids}` the row declares and the whole
/// live selection, and the browser only ever exercised the branch with NOTHING selected: with a sibling
/// entity selected, `outliner-hide-applies` failed on the same row with the same args while the command
/// still settled `command-complete` with `historyUpserts: 0` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B47 §6 — PASS with an empty selection at 2 345 ms, FAIL with a sibling selected at 30 237 ms). Locked,
/// too: a lock guards an entity's GEOMETRY, and a row that says "Hide" must still hide.
#[semio_framework_async_macros::async_test]
async fn an_explicit_outliner_flag_write_ignores_whatever_is_selected() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select a vortex, i.e. NOT the object the row names");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "flag": "locked", "ids": [object_id.clone()], "value": true })), None).await.expect("lock the object by its own row");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "an explicit lock write lands while a vortex is the live selection");

    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "flag": "hidden", "ids": [object_id.clone()], "value": true })), None).await.expect("hide the object by its own row");

    assert_eq!(object_flag(&app, &object_id, "hidden"), Some(true), "the row's explicit id decides what is hidden — never the live selection, and never its lock");
    eprintln!("[DEBUG] explicit flag write hidden={:?} locked={:?} selection=vortex", object_flag(&app, &object_id, "hidden"), object_flag(&app, &object_id, "locked"));
}


fn first_target_volume_id(app: &Puzzle3dApp) -> String {
    let projection = projection_of(app);
    projection
        .get("targetVolumes")
        .or_else(|| projection.get("target_volumes"))
        .and_then(Value::as_array)
        .and_then(|volumes| volumes.first())
        .and_then(|volume| volume.get("id").and_then(Value::as_str))
        .expect("a target volume is present")
        .to_string()
}

fn volume_origin(app: &Puzzle3dApp, volume_id: &str) -> Vec<f64> {
    let projection = projection_of(app);
    let volumes = projection.get("targetVolumes").or_else(|| projection.get("target_volumes")).and_then(Value::as_array).cloned().unwrap_or_default();
    volumes
        .iter()
        .find(|volume| volume.get("id").and_then(Value::as_str) == Some(volume_id))
        .and_then(|volume| volume.get("origin").or_else(|| volume.get("position")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn relocate_target_volume_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = first_target_volume_id(&app);
    let start = volume_origin(&app, &volume_id);
    dispatch(&mut app, "relocateTargetVolume", Some(&json!({ "volumeId": volume_id, "after": { "position": [4.0, 5.0, 6.0] } })), None).await.expect("relocate");
    let moved = volume_origin(&app, &volume_id);
    assert!((moved[0] - 4.0).abs() < 1e-9 && (moved[1] - 5.0).abs() < 1e-9 && (moved[2] - 6.0).abs() < 1e-9, "relocateTargetVolume must write the after pose, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(volume_origin(&app, &volume_id), start, "undo restores the volume pose");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(volume_origin(&app, &volume_id), moved, "redo reapplies the volume pose");
}

#[semio_framework_async_macros::async_test]
async fn world_relocate_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [9.0, 8.0, 7.0] })), None).await.expect("worldRelocate");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 9.0).abs() < 1e-9 && (moved[1] - 8.0).abs() < 1e-9 && (moved[2] - 7.0).abs() < 1e-9, "worldRelocate must write the world position, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "undo restores the object origin");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_origin(&app, &object_id), moved, "redo reapplies the object origin");
}


/// 🚚️ Wave W-Y: Nakagin-scale `worldRelocate` of one object must admit and complete inside the
/// command work budget — the extent is the true per-item vortex walk, not `objects + attractions`.
#[semio_framework_async_macros::async_test]
async fn world_relocate_on_nakagin_admits_and_completes() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [4.0, 5.0, 6.0] })), None).await.expect("worldRelocate nakagin");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 4.0).abs() < 1e-6 && (moved[1] - 5.0).abs() < 1e-6 && (moved[2] - 6.0).abs() < 1e-6, "nakagin relocate must land, got {moved:?} from {start:?}");
}

/// 🚚️ Wave B5: the Relocate utility's single commit, both answers. An UNLOCKED object takes the absolute
/// world origin the host's pointer-up sends and publishes a real document edit; the SAME verb on a LOCKED
/// object refuses with exactly one `selection_locked` notice, no edit and no movement — the identical
/// refusal `translateSelection` already owes a locked selection, which this verb used to skip silently
/// (the host's relocate ghost just snapped back with nothing said). The declared `ActionKind` is pinned
/// here too: `worldRelocate` writes the document, so undo/redo and the history panel must see a
/// `Mutation`, never the `View` the 2026-09-10 checklist reverification claimed.
#[semio_framework_async_macros::async_test]
async fn world_relocate_moves_an_unlocked_object_and_refuses_a_locked_one_with_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let moved = dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id.as_str(), "position": [3.0, -2.0, 1.5] })), None).await.expect("worldRelocate unlocked");
    assert!(notices(&moved).is_empty(), "an unlocked relocate must not refuse: {:?}", moved.requested_effects);
    let landed = object_origin(&app, &object_id);
    assert!((landed[0] - 3.0).abs() < 1e-9 && (landed[1] + 2.0).abs() < 1e-9 && (landed[2] - 1.5).abs() < 1e-9, "worldRelocate must write the absolute origin, got {landed:?}");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let refused = dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id.as_str(), "position": [9.0, 9.0, 9.0] })), None).await.expect("worldRelocate locked");
    let raised = notices(&refused);
    assert_eq!(raised.len(), 1, "a locked relocate must raise exactly one notice: {:?}", refused.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so the refusal must be real prose");
    assert!(refused.mutations.is_empty(), "a locked relocate must emit no edit: {:?}", refused.mutations);
    assert_eq!(object_origin(&app, &object_id), landed, "a locked object must not move");
    let declared = create_puzzle3d_app().window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "worldRelocate").map(|action| action.kind).expect("worldRelocate is declared");
    assert_eq!(declared, ActionKind::Mutation, "worldRelocate edits the document, so history and undo must see a Mutation");
}

/// 📋️ Wave W-Y: copy then paste clones the selection with new ids as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn copy_then_paste_clones_selection_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let copied = dispatch(&mut app, "copy", None, None).await.expect("copy");
    let fragment = copied.requested_effects.iter().find_map(|effect| match effect {
        Effect::ClipboardWrite { fragment } => Some(fragment.clone()),
        _ => None,
    }).expect("copy must emit ClipboardWrite");
    let before = object_count(&app);
    let paste_args = json!({ "fragment": json::from_dsl_value(&dsl::ToValue::to_value(&fragment)) });
    let pasted = dispatch(&mut app, "paste", Some(&paste_args), None).await.expect("paste");
    assert!(!pasted.mutations.is_empty(), "paste must emit one mutation edit: {:?}", pasted.mutations);
    assert_eq!(object_count(&app), before + 1, "paste clones the selection");
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).unwrap().iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    assert!(ids.iter().any(|id| id != &object_id), "the clone must carry a new id: {ids:?}");
}

/// ✂️ Wave W-Y: cut is copy + delete as one undo step.
#[semio_framework_async_macros::async_test]
async fn cut_undoes_as_one_step() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let before = object_count(&app);
    dispatch(&mut app, "cut", None, None).await.expect("cut");
    assert_eq!(object_count(&app), before - 1, "cut removes the selection");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), before, "one undo restores the cut");
}

/// ⬇️ Wave W-Y: export carries the full round-trippable fixture JSON.
#[semio_framework_async_macros::async_test]
async fn export_fixture_downloads_round_trippable_json() {
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, "exportFixture", None, None).await.expect("export");
    let data = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, mime_type, data, .. } => {
            assert_eq!(filename, "concrete-forest.json", "a fresh session exports under the example its document was seeded from");
            assert_eq!(mime_type, "application/json");
            Some(data.clone())
        }
        _ => None,
    }).expect("export must emit DownloadMediaExport");
    let exported: Value = parse(&data).expect("export JSON parses");
    assert_eq!(exported.get("schema"), before.get("schema"), "export schema must match the live fixture");
    assert_eq!(exported.get("objects").and_then(Value::as_array).map(Vec::len), before.get("objects").and_then(Value::as_array).map(Vec::len));
}

/// 🧬️ Identity of imported objects without fixture/snapshot projection twins (anchor / null scale).
fn object_cores(value: &Value) -> Vec<(String, String, String, Value, Vec<String>)> {
    value
        .get("objects")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|object| {
            let id = object.get("id").and_then(Value::as_str)?.to_string();
            let kind = object.get("objectKind").and_then(Value::as_str).unwrap_or_default().to_string();
            let mesh = object.get("meshUrl").and_then(Value::as_str).unwrap_or_default().to_string();
            let origin = object.get("origin").cloned().unwrap_or(Value::Null);
            let vortices = object
                .get("vortices")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|vortex| vortex.get("id").and_then(Value::as_str).map(str::to_string))
                .collect();
            Some((id, kind, mesh, origin, vortices))
        })
        .collect()
}

/// 📥️ Wave W-Y: importing that JSON reproduces the document as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn import_fixture_reproduces_the_exported_document() {
    let mut app = app().await;
    let source = projection_of(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let (imported, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("import");
    assert_eq!(history_rows(&settled), 1, "import must be one mutation edit");
    assert_eq!(object_cores(&projection_of(&app)), object_cores(&source), "import reproduces the exported objects");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), 0, "one undo restores the empty document");
}

/// 📥️ Wave W-AB: workspace Import is `openImportFixture` (file picker). `importFixture` stays
/// dispatchable for the host re-dispatch after the pick, but is not a file-menu row.
#[test]
fn file_menu_import_row_opens_the_file_picker() {
    let definition = create_puzzle3d_app();
    let file: Vec<(&str, bool)> = definition
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter())
        .filter(|action| action.category.as_deref() == Some("file"))
        .map(|action| (action.id.as_str(), action.in_palette))
        .collect();
    assert!(file.iter().any(|(id, _)| *id == "exportFixture"), "file menu keeps Export: {file:?}");
    assert!(file.iter().any(|(id, in_palette)| *id == "openImportFixture" && *in_palette), "file menu Import is openImportFixture: {file:?}");
    assert!(!file.iter().any(|(id, _)| *id == "importFixture"), "importFixture is the picker completion, not a menu row: {file:?}");
    let import = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "importFixture").expect("importFixture stays dispatchable");
    assert!(!import.in_palette);
    assert_eq!(import.category.as_deref(), None);
}

/// 📥️ Wave W-AB: activating Import requests a file open; completing it with fixture JSON imports.
#[semio_framework_async_macros::async_test]
async fn open_import_fixture_requests_file_open_then_import_applies_payload() {
    let mut app = app().await;
    let source = projection_of(&app);
    let opened = dispatch(&mut app, "openImportFixture", None, None).await.expect("openImportFixture");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(read_as.as_deref(), Some("text"));
            assert_eq!(import_action, "importFixture");
            assert!(!*multiple);
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "Import must emit RequestFileOpen: {:?}", opened.requested_effects);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let (imported, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("import");
    assert_eq!(history_rows(&settled), 1, "import must be one mutation edit");
    assert_eq!(object_cores(&projection_of(&app)), object_cores(&source), "picked payload reproduces the exported objects");
}

/// 📥️ Wave W-AB: re-importing the live fixture is a store identity, not a guest payload dedupe.
/// `import_fixture` always assigns; a no-op history row means the document fold saw equal content.
#[semio_framework_async_macros::async_test]
async fn import_fixture_of_the_live_document_records_whether_identical_content_is_an_edit() {
    let mut app = app().await;
    let source = projection_of(&app);
    let (imported, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("reimport");
    let objects_after = object_cores(&projection_of(&app));
    assert_eq!(objects_after, object_cores(&source), "identical payload must not rewrite object cores");
    assert_eq!(history_rows(&settled), 0, "identical live fixture is a store no-op, not a guest dedupe: ingress still delivers payload+name");
}

/// 📥️ Wave W-AB #44: leftover `importFixture` against a one-object live fixture applies a distinct two-object JSON.
#[semio_framework_async_macros::async_test]
async fn import_fixture_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations() {
    let source = include_str!("../../🦀️.rs");
    assert!(
        source.contains(r#""importFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id))"#),
        "importFixture must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("seed");
    assert_eq!(object_count(&app), 1, "live fixture must start as one object");
    let live_id = first_object_id(&app);
    let one = projection_of(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("distinct");
    let two = projection_of(&app);
    let cores = object_cores(&two);
    assert_eq!(cores.len(), 2, "distinct payload must carry two objects");
    let distinct_id = cores.iter().map(|row| row.0.as_str()).find(|id| *id != live_id).expect("distinct object id").to_string();
    dispatch(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&one) })), None).await.expect("restore one-object live fixture");
    assert_eq!(object_count(&app), 1, "live fixture is one object before import");
    assert_eq!(first_object_id(&app), live_id);
    let (imported, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&two) })), None).await;
    imported.expect("import distinct");
    assert_eq!(history_rows(&settled), 1, "distinct two-object import must emit operations");
    let after = object_cores(&projection_of(&app));
    assert_eq!(after.len(), 2, "after-snapshot must contain both objects");
    assert!(after.iter().any(|row| row.0 == distinct_id), "after-snapshot must contain the distinct object id {distinct_id}");
}

/// 📥️ Wave B9 lane 3: the picker's REAL round trip — the bytes `exportFixture` hands the host's
/// download are the bytes the file chooser hands `importFixture` back. A distinct export must upsert
/// its objects and record one history row; re-importing the SAME file against the document it
/// describes must stay an identity no-op. The browser measured `effects:0 historyUpserts:0` on the
/// distinct half, which is also exactly what a silently-dropped refusal notice looks like — so this
/// law asserts the absence of a refusal as well as the presence of the edit.
#[semio_framework_async_macros::async_test]
async fn exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity() {
    let exported = |result: &semio_framework_plugin::InvocationResult| -> String {
        result
            .requested_effects
            .iter()
            .find_map(|effect| match effect {
                Effect::DownloadMediaExport { data, .. } => Some(data.clone()),
                _ => None,
            })
            .expect("exportFixture emits one DownloadMediaExport")
    };
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    let one_object = exported(&dispatch(&mut app, "exportFixture", None, None).await.expect("export the boot document"));
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("seed a second object");
    let two_objects = exported(&dispatch(&mut app, "exportFixture", None, None).await.expect("export the distinct document"));
    assert_ne!(one_object, two_objects, "the two exports must be distinct files");
    let restored = dispatch(&mut app, "importFixture", Some(&json!({ "payload": one_object.as_str(), "name": "puzzle-3d.json" })), None).await.expect("restore the one-object document");
    assert!(notices(&restored).is_empty(), "restoring an exported file must not refuse: {:?}", notices(&restored));
    assert_eq!(object_count(&app), 1, "the live document is one object before the distinct import");
    let (distinct, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": two_objects.as_str(), "name": "puzzle-3d-distinct.json" })), None).await;
    let distinct = distinct.expect("import the distinct file");
    assert!(notices(&distinct).is_empty(), "a distinct exported file must not refuse: {:?}", notices(&distinct));
    assert_eq!(history_rows(&settled), 1, "a distinct exported file must record one history row");
    assert_eq!(object_count(&app), 2, "a distinct exported file must upsert its objects");
    let after_distinct = object_cores(&projection_of(&app));
    let again = dispatch(&mut app, "importFixture", Some(&json!({ "payload": two_objects.as_str(), "name": "puzzle-3d-distinct.json" })), None).await.expect("re-import the same file");
    assert!(notices(&again).is_empty(), "re-importing the same file must not refuse: {:?}", notices(&again));
    assert_eq!(object_cores(&projection_of(&app)), after_distinct, "re-importing the file the document already IS is an identity on the document");
}

/// 🔢️ The payload an import actually receives was written by a BROWSER, not by this crate's own writer:
/// the file chooser hands back whatever `JSON.stringify` produced, which spells a whole float as `1` (no
/// `.0`), a tiny one in exponent form (`-5.551115123125783e-17`), an absent optional as `null`, and a
/// non-ASCII label as a `\u` escape. The law above only ever fed `import_fixture` this crate's own export
/// text, so none of those spellings was under test — and the browser's `import-distinct` read
/// `effects:0 historyUpserts:0` with NO refusal notice, which is exactly what `import_fixture`'s
/// `ctx.abort` path looks like when `parse` rejects the text it was handed (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B57 §2). One unreadable spelling is a silently dropped document.
#[semio_framework_async_macros::async_test]
async fn a_browser_serialized_fixture_payload_imports_every_json_number_spelling() {
    let mut app = app().await;
    let payload = r#"{"schema":"puzzle.3d.fixture","domain":"architecture","objects":[{"id":"browser-clone","label":"Distinct Capsule J · cs_sl1","objectKind":"Object","origin":[-16.75,-3.6499999999999986,0],"orientation":[-5.551115123125783e-17,5.551115123125783e-17,0.7071067811865475,0.7071067811865475],"scale":null,"vortices":[{"id":"browser-clone:v0","position":[1,0,2]}],"hidden":false,"locked":false}],"attractions":[],"targetVolumes":[],"references":[]}"#;
    let (imported, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": payload, "name": "browser-stringify.json" })), None).await;
    let imported = imported.expect("import a browser-serialized payload");
    let notices: Vec<String> = imported
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert!(notices.is_empty(), "a browser-serialized payload must not be refused: {notices:?}");
    assert_eq!(history_rows(&settled), 1, "a browser-serialized payload must record one history row");
    let objects = object_cores(&projection_of(&app));
    assert_eq!(objects.len(), 1, "the imported document is exactly the payload's objects: {objects:?}");
    assert_eq!(objects.first().map(|(id, ..)| id.as_str()), Some("browser-clone"), "the payload's own object id survives the import: {objects:?}");
}

/// 🧱️ Wave B59: the payload the browser hands `importFixture` for a REAL example is 145 714 B — the
/// Nakagin Capsule Tower export, 180 objects — and the live `import-distinct` verdict read
/// `paneObjects=180→180` with no notice at all (wave B57 §2.3). Every import law before this one fed a
/// payload of a few hundred bytes, so the size class the product actually imports was never under test.
///
/// 🧩️ The payload rides the chunk lane the host builds (`importPayloadChunks`, `🛠️ShellHelpers/🟦️.tsx`),
/// mirrored here by `puzzle3d_import_chunks`, so the law exercises the wire the renderer sends rather than
/// a shape only tests use. The witnesses are the browser's own three: the object census moves, exactly ONE
/// history row lands — on the SEALING chunk, never on a staged one — and nothing is refused.
#[semio_framework_async_macros::async_test]
async fn a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle() {
    use crate::editor::puzzle3d::commands::import_fixture::puzzle3d_import_chunks;
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = crate::editor::puzzle3d::commands::export_fixture::puzzle3d_export_json(&puzzle3d_fixture_from_projection(&projection_of(&app)));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    assert_eq!(object_count(&app), seeded, "the document is back at the example census before the import");
    assert!(distinct.len() > 140_000, "the payload under test must be the product's own size class; observed {} B", distinct.len());
    let chunks = puzzle3d_import_chunks(&distinct);
    assert!(chunks.len() > 1, "a product-sized payload must not fit one chunk; observed {} chunk(s)", chunks.len());
    eprintln!("[DEBUG] B59 import payload bytes={} chunks={} seeded={seeded}", distinct.len(), chunks.len());
    let mut settled_rows = 0usize;
    let mut notices: Vec<String> = Vec::new();
    let count = chunks.len();
    for (index, chunk) in chunks.iter().enumerate() {
        let args = json!({ "payload": chunk.as_str(), "name": "nakagin-capsule-tower-distinct.json", "chunk": index as f64, "chunkCount": count as f64 });
        let (result, settled) = dispatch_reporting(&mut app, "importFixture", Some(&args), None).await;
        let result = result.expect("every chunk of a product-sized import is admitted");
        notices.extend(result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }));
        let rows = history_rows(&settled);
        if index + 1 < count {
            assert_eq!(rows, 0, "a STAGED chunk is not a document edit; chunk {index} recorded {rows} history row(s)");
            assert_eq!(object_count(&app), seeded, "a staged chunk must not move the document; chunk {index}");
        }
        settled_rows += rows;
    }
    assert!(notices.is_empty(), "a payload inside the declared import budget must not be refused: {notices:?}");
    assert_eq!(settled_rows, 1, "the whole chunked import records exactly one history row");
    assert_eq!(object_count(&app), seeded + 1, "a product-sized distinct payload must replace the document it was imported over");
}

/// 🧯️ Wave B59: an import the host never chunked is REFUSED with a notice, never silently dropped. This is
/// the shape the live verdict actually saw — one command carrying 145 924 B — and the shape every hop from
/// the renderer's pack encode to the guest's `read_bounded_bytes` answers by asking the fixed guest heap for
/// one contiguous block 2.2× its own per-request ceiling.
#[semio_framework_async_macros::async_test]
async fn an_unchunked_over_ceiling_import_refuses_with_a_notice() {
    use crate::editor::puzzle3d::commands::import_fixture::PUZZLE3D_IMPORT_CHUNK_BYTES;
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    let whole = crate::editor::puzzle3d::commands::export_fixture::puzzle3d_export_json(&puzzle3d_fixture_from_projection(&projection_of(&app)));
    assert!(whole.len() > PUZZLE3D_IMPORT_CHUNK_BYTES, "the payload under test must be above one chunk");
    let (result, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": whole.as_str(), "name": "unchunked.json" })), None).await;
    let result = result.expect("an over-ceiling import is an ANSWER, never a fault");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an over-ceiling import publishes exactly one notice: {notices:?}");
    assert_eq!(history_rows(&settled), 0, "a refused import records no history row");
    assert_eq!(object_count(&app), seeded, "a refused import leaves the document alone");
    eprintln!("[DEBUG] B59 unchunked refusal notice={notices:?}");
}

/// 🧊️ Wave B59, the contiguous-request law: importing a 145 KB document must never ask the guest for a
/// single block above [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`]. Both halves are measured — the CHUNK the
/// host sends, and the largest contiguous value the guest's own paged reassembly captures while rebuilding
/// the root object element by element.
///
/// 🧨️ The bound is the framework's own, not a literal of this app's: `semio_framework_trace`'s ceiling is
/// the same number `🧮️memory/🟦️.ts` publishes to the host, so the two chunkers cannot drift apart.
#[test]
fn a_one_hundred_forty_five_kilobyte_import_never_asks_for_a_block_above_the_guest_contiguous_ceiling() {
    use crate::editor::puzzle3d::commands::import_fixture::{puzzle3d_import_chunks, puzzle3d_import_root_value, PUZZLE3D_IMPORT_CHUNK_BYTES, PUZZLE3D_IMPORT_MAXIMUM_CHUNKS, PUZZLE3D_IMPORT_TOTAL_BYTES};
    use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
    assert_eq!(PUZZLE3D_IMPORT_CHUNK_BYTES, GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2, "the chunk extent is DERIVED from the guest ceiling, never a literal");
    assert_eq!(PUZZLE3D_IMPORT_TOTAL_BYTES, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, "an import must carry exactly what an export may stream");
    assert_eq!(PUZZLE3D_IMPORT_MAXIMUM_CHUNKS, PUZZLE3D_IMPORT_TOTAL_BYTES.div_ceil(PUZZLE3D_IMPORT_CHUNK_BYTES));
    let payload = crate::editor::puzzle3d::commands::export_fixture::puzzle3d_export_json(&NAKAGIN_EXAMPLE_FIXTURE.clone());
    assert!(payload.len() > 140_000, "the payload under test is the product's own size class: {} B", payload.len());
    let chunks = puzzle3d_import_chunks(&payload);
    assert!(chunks.len() <= PUZZLE3D_IMPORT_MAXIMUM_CHUNKS, "{} chunks exceeds the declared run length", chunks.len());
    for (index, chunk) in chunks.iter().enumerate() {
        assert!(chunk.len() <= PUZZLE3D_IMPORT_CHUNK_BYTES, "chunk {index} is {} B, above the {PUZZLE3D_IMPORT_CHUNK_BYTES} B chunk extent", chunk.len());
    }
    assert_eq!(chunks.concat(), payload, "the chunk run must reassemble the document byte for byte");
    let root = puzzle3d_import_root_value(&chunks).expect("the paged reassembly rebuilds the root object");
    let members = root.as_object().expect("the reassembled document is one JSON object");
    let widest = members.iter().map(|(_, value)| dsl::os_pack::json::to_json_string(value).len()).max().unwrap_or_default();
    let widest_element = members
        .iter()
        .filter_map(|(_, value)| value.as_array())
        .flat_map(|items| items.iter().map(|item| dsl::os_pack::json::to_json_string(item).len()))
        .max()
        .unwrap_or_default();
    assert!(widest > GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, "the widest root MEMBER is {widest} B — if no member is over the ceiling this law proves nothing about paging");
    assert!(
        widest_element <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "the reassembly captures one ELEMENT at a time, so the widest contiguous request is {widest_element} B against a {GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES} B ceiling"
    );
    assert_eq!(
        crate::editor::puzzle3d::commands::export_fixture::puzzle3d_export_json(&Puzzle3dFixture::from_value(dsl::json::to_dsl_value(&root)).expect("the reassembled root IS a puzzle 3D document")),
        payload,
        "the paged reassembly is byte-identical to the document that was chunked"
    );
    eprintln!("[DEBUG] B59 ceiling law chunks={} widestMember={widest} widestElement={widest_element}", chunks.len());
}

/// 🕳️ Wave B59: a chunk that does not continue its run is REFUSED by name, and the broken run is dropped so
/// the client re-opens at chunk 0 rather than resuming into bytes nobody can account for — the same gap
/// discipline the `registerBrushMesh` page run already enforces.
#[test]
fn an_out_of_order_import_chunk_is_refused_and_a_retransmitted_one_is_acknowledged() {
    use crate::editor::puzzle3d::commands::import_fixture::{retire_abandoned_import_runs, stage_import_chunk, Puzzle3dImportEnvelope, Puzzle3dImportFault, Puzzle3dImportStep, PUZZLE3D_IMPORT_CHUNK_BYTES, PUZZLE3D_IMPORT_MAXIMUM_CHUNKS};
    retire_abandoned_import_runs();
    retire_abandoned_import_runs();
    let envelope = |chunk: usize, chunk_count: usize| Puzzle3dImportEnvelope { name: "b59-gap.json".into(), chunk, chunk_count };
    assert_eq!(stage_import_chunk(&envelope(1, 3), "b"), Err(Puzzle3dImportFault::Gap), "a run may only open at chunk 0");
    assert_eq!(stage_import_chunk(&envelope(0, 3), "a"), Ok(Puzzle3dImportStep::Staged { next_chunk: 1, chunk_count: 3 }));
    assert_eq!(stage_import_chunk(&envelope(2, 3), "c"), Err(Puzzle3dImportFault::Gap), "a skipped chunk drops the run");
    assert_eq!(stage_import_chunk(&envelope(0, 3), "a"), Ok(Puzzle3dImportStep::Staged { next_chunk: 1, chunk_count: 3 }));
    assert_eq!(stage_import_chunk(&envelope(1, 3), "b"), Ok(Puzzle3dImportStep::Staged { next_chunk: 2, chunk_count: 3 }));
    assert_eq!(stage_import_chunk(&envelope(1, 3), "b"), Ok(Puzzle3dImportStep::Staged { next_chunk: 2, chunk_count: 3 }), "a retransmitted chunk is acknowledged at the cursor, never charged the whole run again");
    assert_eq!(stage_import_chunk(&envelope(2, 3), "c"), Ok(Puzzle3dImportStep::Complete(vec!["a".into(), "b".into(), "c".into()])));
    assert_eq!(stage_import_chunk(&envelope(0, 0), "a"), Err(Puzzle3dImportFault::Envelope));
    assert_eq!(stage_import_chunk(&envelope(0, PUZZLE3D_IMPORT_MAXIMUM_CHUNKS + 1), "a"), Err(Puzzle3dImportFault::Envelope));
    assert_eq!(stage_import_chunk(&envelope(0, 2), &"x".repeat(PUZZLE3D_IMPORT_CHUNK_BYTES + 1)), Err(Puzzle3dImportFault::Chunk));
    retire_abandoned_import_runs();
    retire_abandoned_import_runs();
}

/// 📥️ Wave B16: leftover `exportFixture` must emit `DownloadMediaExport`.
/// 🏷️ Wave B30: a fresh session's document IS the Concrete Forest example
/// (`ArtifactApp::initial_snapshot`), and since the config lane says so from the first render the
/// boot download is named after it. The app-generic `puzzle-3d.json` belongs to a document that came
/// from no example at all — see `export_fixture_names_the_download_after_the_active_example`, which
/// reaches that state by clearing the picker.
#[semio_framework_async_macros::async_test]
async fn leftover_export_fixture_downloads_the_boot_example_json() {
    let source = include_str!("../../🦀️.rs");
    assert!(
        source.contains(r#""exportFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id))"#),
        "exportFixture must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    let result = dispatch(&mut app, "exportFixture", None, None).await.expect("leftover export");
    let filename = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, .. } => Some(filename.as_str()),
        _ => None,
    });
    assert_eq!(filename, Some("concrete-forest.json"), "leftover export must name the boot example: {:?}", result.requested_effects);
}

/// 🏷️ Wave B26: exporting Concrete Forest and exporting Nakagin must not both land as one constant
/// filename. The download is named after the ACTIVE EXAMPLE — the id `set_active_example` persists on
/// the shared config, restored into `Puzzle3dRuntime` by `window::runtime` — and a document that came
/// from no example at all keeps the app-generic `puzzle-3d.json`. Both picker aliases (`concrete`,
/// `nakagin`) resolve to the canonical id, so the filename never depends on how the row was spelled.
#[semio_framework_async_macros::async_test]
async fn export_fixture_names_the_download_after_the_active_example() {
    /// 🏷️ The name is asserted across BOTH publication lanes, because the lane is chosen by payload size
    /// and the naming contract is not: an over-budget example publishes a segmented handle rather than an
    /// inline `DownloadMediaExport`, and it must carry the same filename (wave B38).
    async fn exported_filename(app: &mut Puzzle3dApp) -> String {
        let (result, settled) = dispatch_reporting(app, "exportFixture", None, None).await;
        result.expect("export");
        let inline = settled.effects.iter().find_map(|effect| match effect {
            Effect::DownloadMediaExport { filename, .. } => Some(filename.clone()),
            _ => None,
        });
        let segmented = settled.downloads.first().map(|download| download.filename.clone());
        assert!(inline.is_none() || segmented.is_none(), "one export publishes exactly one lane, never both: inline={inline:?} segmented={segmented:?}");
        inline.or(segmented).expect("export must publish a download on one of the two lanes")
    }
    let mut app = app().await;
    assert_eq!(exported_filename(&mut app).await, "concrete-forest.json", "a fresh session already names the example its document was seeded from");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("clear to a blank document");
    assert_eq!(exported_filename(&mut app).await, "puzzle-3d.json", "a document that came from no example keeps the app-generic name");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "concrete-forest" })), None).await.expect("load concrete forest");
    assert_eq!(exported_filename(&mut app).await, "concrete-forest.json", "Concrete Forest must export under its own id");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "nakagin" })), None).await.expect("load nakagin by alias");
    assert_eq!(exported_filename(&mut app).await, "nakagin-capsule-tower.json", "the picker alias must still export the canonical example id");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("clear to a blank document");
    assert_eq!(exported_filename(&mut app).await, "puzzle-3d.json", "clearing the example clears the name it exported under");
    eprintln!("[DEBUG] export filename law reached the blank/concrete/nakagin/blank sequence");
}

/// ⬇️ Wave B43: an export above what ONE segmented download may carry is refused with a NOTICE, and the
/// budget it is measured against is the framework's own end-to-end cap — never a literal of this app's.
///
/// 🧯 A fault here would land as a dead job; before B43 the whole lane's over-cap answer was a producer
/// `Fault`. The refusal names the file, its size and the budget, so the answer is readable.
#[test]
fn export_refuses_a_payload_above_the_declared_segmented_budget_with_a_notice() {
    use crate::editor::puzzle3d::commands::export_fixture::{puzzle3d_export_refusal, puzzle3d_export_segmented, puzzle3d_export_segmented_budget_bytes};
    use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
    use semio_framework_plugin::app::ArtifactOutputChunks;
    let budget = puzzle3d_export_segmented_budget_bytes().expect("the declared output budget is within the framework cap");
    assert_eq!(budget, PUZZLE_COMMAND_OUTPUT_BYTES, "the segmented budget IS this command's declared contract output budget");
    assert!(budget <= ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, "no app may declare a download larger than the wire admits");
    let over = "x".repeat(budget + 1);
    assert!(puzzle3d_export_segmented("nakagin-capsule-tower.json".into(), &over).is_err(), "a payload above the budget must never open a download handle");
    let notice = puzzle3d_export_refusal("nakagin-capsule-tower.json", budget + 1, budget);
    assert!(notice.contains("nakagin-capsule-tower.json") && notice.contains(&(budget + 1).to_string()) && notice.contains(&budget.to_string()), "the refusal must name the file, its size and the budget: {notice}");
}

/// ⬇️ Wave B38: an export larger than one wire page must reach the user as ONE download carrying the WHOLE
/// fixture, through the framework's segmented lane — never as an inline effect field.
///
/// 🧊️ The lane is picked by size against the guest's own contiguous-request ceiling
/// (`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`, one wasm page), derived from the wire constant rather than
/// spelled as a literal: Concrete Forest (7 542 B) rides inline, Nakagin Capsule Tower (145 714 B) does not.
/// The browser measured exactly that split — the inline Nakagin effect left the guest (`"effects":1`) and
/// reached no file (`📓️2026-09-12-wave-B36-full-run-bisect-2.md` §5).
///
/// 🧾️ The assertion is the REASSEMBLED payload, drained one bounded chunk at a time exactly as
/// `drainSegmentedMediaExport` drains it, parsed back to JSON and compared object-for-object with the live
/// document — so a lane that publishes a handle but loses, reorders or truncates chunks fails here.
#[semio_framework_async_macros::async_test]
async fn export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture() {
    use crate::editor::puzzle3d::commands::export_fixture::{puzzle3d_export_inline_budget_bytes, puzzle3d_export_json};
    assert_eq!(
        puzzle3d_export_inline_budget_bytes(),
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "the inline budget is the guest's own contiguous-request ceiling, never an independent literal"
    );
    let mut app = app().await;

    // 🌲️ Below the budget: the inline lane, unchanged, and NO segmented handle at all.
    let (small, small_settled) = dispatch_reporting(&mut app, "exportFixture", None, None).await;
    small.expect("concrete forest export");
    let small_bytes = puzzle3d_export_json(&puzzle3d_fixture_from_snapshot(app.snapshot().expect("live snapshot").typed())).len();
    assert!(small_bytes <= puzzle3d_export_inline_budget_bytes(), "Concrete Forest must sit under the inline budget, got {small_bytes} B");
    assert!(small_settled.downloads.is_empty(), "a payload that fits one page must not open a segmented handle: {:?}", small_settled.downloads);
    let inline = small_settled.effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, data, encoding, .. } => Some((filename.clone(), data.len(), encoding.clone())),
        _ => None,
    });
    assert_eq!(inline, Some(("concrete-forest.json".to_string(), small_bytes, Some("utf-8".to_string()))), "the inline lane carries the whole payload and the utf-8 encoding");

    // 🏢️ Above the budget: one segmented handle, no inline payload, and the full JSON on drain.
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "nakagin" })), None).await.expect("load nakagin");
    let expected = puzzle3d_export_json(&puzzle3d_fixture_from_snapshot(app.snapshot().expect("live snapshot").typed()));
    assert!(expected.len() > puzzle3d_export_inline_budget_bytes(), "Nakagin must exceed the inline budget, got {} B", expected.len());
    let (large, large_settled) = dispatch_reporting(&mut app, "exportFixture", None, None).await;
    large.expect("nakagin export");
    assert!(
        !large_settled.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })),
        "an over-budget export must publish no inline download effect: {:?}",
        large_settled.effects
    );
    assert_eq!(large_settled.downloads.len(), 1, "exactly one segmented download handle: {:?}", large_settled.downloads);
    let handle = large_settled.downloads[0].clone();
    assert_eq!(handle.filename, "nakagin-capsule-tower.json", "the segmented lane keeps the per-example filename");
    assert_eq!(handle.mime_type, "application/json");
    assert_eq!(handle.encoding.as_deref(), Some("identity"), "UTF-8 JSON streams as identity, never base64");
    assert_eq!(handle.bytes, expected.len(), "the handle declares the whole payload");
    let drained = drain_segmented_download(&mut app, &handle).await;
    assert_eq!(drained.len(), expected.len(), "the drained bytes are the whole payload");
    assert_eq!(String::from_utf8(drained).expect("drained payload is UTF-8"), expected, "the drained payload IS the export, byte for byte");
    let reassembled: Value = parse(&expected).expect("the reassembled export parses");
    let live = projection_of(&app);
    assert_eq!(object_cores(&reassembled), object_cores(&live), "the streamed export reproduces every object of the live document");
    assert_eq!(reassembled.get("schema"), live.get("schema"));
}

/// 📥️ Wave B16: leftover `openImportFixture` must emit the file picker that completes as `importFixture`.
#[semio_framework_async_macros::async_test]
async fn leftover_open_import_fixture_requests_file_open() {
    let source = include_str!("../../🦀️.rs");
    assert!(
        source.contains(r#""openImportFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id))"#),
        "openImportFixture must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    let opened = dispatch(&mut app, "openImportFixture", None, None).await.expect("leftover openImportFixture");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, import_action, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(import_action, "importFixture");
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "leftover Import must emit RequestFileOpen: {:?}", opened.requested_effects);
}

/// 📥️ Wave B16: leftover `importFixture` of a distinct two-object JSON replaces the live one-object document.
#[semio_framework_async_macros::async_test]
async fn leftover_import_fixture_replaces_live_document_with_distinct_two_object_json() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("seed");
    assert_eq!(object_count(&app), 1, "live fixture must start as one object");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("distinct");
    let two = projection_of(&app);
    assert_eq!(object_cores(&two).len(), 2, "distinct payload must carry two objects");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty again");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("one-object live");
    assert_eq!(object_count(&app), 1, "live fixture is one object before import");
    let (imported, settled) = dispatch_reporting(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&two), "name": "puzzle-3d-distinct.json" })), None).await;
    imported.expect("leftover distinct import");
    assert_eq!(history_rows(&settled), 1, "distinct leftover import must upsert history");
    assert_eq!(object_count(&app), 2, "distinct leftover import must replace objects after_objects=2");
}

/// 🖱️ Wave W-Y: a selected-object context menu is puzzle-owned — never the shell fallback vocabulary.
#[semio_framework_async_macros::async_test]
async fn object_context_menu_owns_puzzle_rows_not_shell_fallback() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("duplicateSelection"), "object menu must own Duplicate: {menu_json}");
    assert!(menu_json.contains("selectSameKindSelection"), "object menu must own Select same kind: {menu_json}");
    assert!(!menu_json.contains("setActiveExample"), "object menu must not dress itself as the shell fallback: {menu_json}");
}

/// 📏️ Wave W-Y: gumball scale on a locked volume refuses with a notice and emits no edit.
#[semio_framework_async_macros::async_test]
async fn gumball_scale_on_locked_volume_refuses_without_edit() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("volume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id, "flag": "locked", "value": true })), None).await.expect("lock");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select volume");
    let before = projection_of(&app);
    let result = dispatch(&mut app, "scaleSelection", Some(&json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })), None).await.expect("scale locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked volume must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked volume must emit no edit: {:?}", result.mutations);
    assert_eq!(projection_of(&app).get("targetVolumes"), before.get("targetVolumes"));
}

/// 📋️ leftover vortex-granularity selection still captures the object, and paste clones it with a new id.
#[test]
fn leftover_copy_paste_clones_selected_object() {
    let mut fixture = empty_fixture();
    fixture.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["seed-left-001".into()],
        hovered: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &fixture);
    assert_eq!(objects.len(), 1, "leftover selected object id must copy even when granularity is vortex: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    assert_eq!(objects[0].origin, [1.0, 2.0, 3.0]);
    let fragment = puzzle3d_copy_fragment_from(&fixture, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations_on(&fixture, &fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "paste must commit one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001", "clone must mint a fresh id");
    assert!((created[0].object.origin[0] - 1.5).abs() < 1e-9, "clone origin must keep source fields plus paste offset: {:?}", created[0].object.origin);
    assert_eq!(created[0].object.object_kind.as_deref(), Some("Object"));
    assert_eq!(created[0].object.label.as_deref(), Some("seed"));
}

/// 📋️ leftover selected vortex uuid (not object.id) still captures the parent object for copy.
#[test]
fn leftover_copy_paste_clones_object_from_selected_vortex_uuid() {
    let mut fixture = empty_fixture();
    fixture.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: vec![Puzzle3dVortex {
            id: "5de35caa-0f02-43d7-ae74-aa730efd3386".into(),
            vortex_kind: None,
            position: [0.0, 0.0, 0.0],
            direction: None,
            radius: None,
            hidden: false,
            locked: false,
        }],
        hidden: false,
        locked: false,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["5de35caa-0f02-43d7-ae74-aa730efd3386".into()],
        hovered: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &fixture);
    assert_eq!(objects.len(), 1, "leftover selected vortex uuid must resolve to the parent object: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    let fragment = puzzle3d_copy_fragment_from(&fixture, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations_on(&fixture, &fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "vortex-uuid leftover copy must paste one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001");
}

/// 🕹️ leftover overlay → translateSelection on a locked object must refuse with a visible notice.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_on_locked_object_refuses_with_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let before = object_origin_x(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("translate locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked object must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked object must emit no edit: {:?}", result.mutations);
    assert!((object_origin_x(&app, &object_id) - before).abs() < 1e-9, "locked object origin must not move");
}

/// 🕹️ leftover overlay → translateSelection carries explicit ids; guest snapshot granularity may be empty.
#[semio_framework_async_macros::async_test]
async fn leftover_overlay_translate_selection_moves_unlocked_object() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover overlay translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "unlocked leftover overlay must not refuse: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 4.0).abs() < 1e-9, "leftover overlay ids must move the unlocked object by dx");
}

/// 🕹️ leftover `translateSelection` with explicit ids + mesh mode commits a pose delta.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_mesh_mode_commits_pose_delta() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "mode": "mesh", "dx": 3.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover mesh translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "mesh leftover translate must pre-admit: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 3.0).abs() < 1e-9, "leftover mesh-mode translateSelection must commit a pose delta");
}


//#endregion 🩹️CheckedDefects

/// 📏️ Wave W-M2: one whole `registerBrushMesh` page validates inside one command's own work budget.
/// The page scan is cursorized at [`PUZZLE3D_MESH_PAGE_SCAN_CHARS`] characters per step over both
/// payload streams, and the largest page the plugin admits is
/// `PUZZLE3D_MESH_PAGE_BASE64_CHARS` characters per stream — so the extent a page claims can never
/// approach `PUZZLE_COMMAND_WORK_ITEMS`, which is what keeps every per-page unit under the 8 ms law.
#[test]
fn one_brush_mesh_page_validates_inside_one_command_work_budget() {
    let page = crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
    let steps = page.div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).saturating_mul(2).saturating_add(1);
    assert!(steps <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "a whole page validates in {steps} bounded steps, inside {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
    assert!(page + 8 < crate::retained_command::PUZZLE_COMMAND_RAW_BYTES, "a page's two payload streams — one page's values, split at most once and padded per stream — leave the JSON envelope room inside the raw wire");
}

/// 🚚️ Wave W-H: a client whose "already uploaded" bookkeeping outlived the guest that justified it
/// announces identities this instantiation holds nothing for. The arm may not answer that with a notice
/// — nothing on the host reads a notification's text, so the brush utility would silently keep no
/// collision body until a full page reload. It publishes the identity on the world body instead, and
/// widens its own otherwise-`Quiet` scope to the viewport so that body is actually republished. The
/// client answers with the page run, and the request retires.
#[semio_framework_async_macros::async_test]
async fn an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes() {
    let mut app = app().await;
    // 🎲️ Geometry no other law in this binary derives — the brush-mesh store is content-addressed since
    // wave B22, so the shared cube's digest would be adoptable under a new id and the refusal this law is
    // about could never happen.
    let (positions, indices) = crate::standards::v1::subsets::any::schema::precompute_model_tests::context::seeded_cube_mesh_buffers(23.0);
    let digest = crate::editor::puzzle3d::precompute::brush_mesh_digest(&positions, &indices);
    let url = "/test/restarted-guest-reannounce.glb";
    let announcement = json!({ "surfaceId": "world-3d", "url": url, "digest": digest });
    let refused = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("id-only announcement");
    assert!(!refused.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "a refusal the client cannot act on is not an answer");
    assert!(
        matches!(&refused.ui_scope, UiDirtyScope::Partial { window_bodies, .. } if window_bodies.iter().any(|body| body == main::BODY_KEY)),
        "the request is worthless until the world body carrying it repaints; got {:?}",
        refused.ui_scope
    );
    let requested = interaction_of(&render_composite(&mut app).await);
    assert_eq!(requested.pointer("/meshReuploadUrls").and_then(Value::as_array).cloned().unwrap_or_default(), vec![json!(url)], "the refused identity is published as a request for its bytes");
    let residency = requested.pointer("/meshResidency").and_then(Value::as_u64).expect("the world body publishes the guest's mesh residency");
    // 🐢️ Wave B32: the SECOND refusal of the same id is where the storm lived. The standing request set
    // is already published, so this announcement has nothing new for any surface — and republishing the
    // world body for it re-drives the client's registrar, which re-announces, which refuses again. The
    // live `:6013` shell measured 23 of 33 typed-operation completions carrying this viewport scope in a
    // 75 s window with four user actions, 23 of 31 refresh passes answering every world body
    // `unchanged`, and `translateSelection` needing 8–15 s to reach `data-instances-json` behind them.
    let repeated = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the client re-announces the same dead identity");
    assert!(matches!(repeated.ui_scope, UiDirtyScope::None), "a re-announcement of an ALREADY standing request republishes nothing and must stay Quiet; got {:?}", repeated.ui_scope);
    let unchanged = interaction_of(&render_composite(&mut app).await);
    assert_eq!(unchanged.pointer("/meshReuploadUrls"), requested.pointer("/meshReuploadUrls"), "the second refusal leaves the published request set byte-identical, which is why it owes no repaint");

    let position_bytes: Vec<u8> = positions.iter().flat_map(|value| value.to_le_bytes()).collect();
    let index_bytes: Vec<u8> = indices.iter().flat_map(|value| value.to_le_bytes()).collect();
    let page = json!({
        "surfaceId": "world-3d",
        "url": url,
        "digest": digest,
        "page": 0,
        "pageCount": 1,
        "positionsB64": semio_framework_io_base64::base64_standard_encode(&position_bytes),
        "indicesB64": semio_framework_io_base64::base64_standard_encode(&index_bytes),
    });
    let accepted = dispatch(&mut app, "registerBrushMesh", Some(&page), None).await.expect("the client answers with the page run");
    assert!(!accepted.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "an accepted page run is silent");
    let settled = interaction_of(&render_composite(&mut app).await);
    assert_eq!(settled.pointer("/meshReuploadUrls").and_then(Value::as_array).map(Vec::len), Some(0), "an answered request retires, so a steady state carries no standing request");
    assert!(settled.pointer("/meshResidency").and_then(Value::as_u64).expect("residency") > residency, "the client's proof that this guest holds the geometry is the counter climbing");

    let readopted = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the identity is now resident");
    assert!(readopted.requested_effects.is_empty(), "an identity this guest holds is adopted by id alone, at no cost on the wire");
    assert!(matches!(readopted.ui_scope, UiDirtyScope::None), "the adopting fast path stays the quiet command it is declared to be");
    assert!(
        !puzzle3d_action_uses_precompute("registerBrushMesh"),
        "registerBrushMesh only WRITES geometry into the precompute session, so it may not owe the sync prologue: that prologue seeds a fallback body for every id the session has no geometry for — precisely the ids this command supplies — clearing the brush cache and rebuilding the queue each time, which the arm's own install then discards, and which is where the 0.65 s → 2.85 s per announcement measured on 2026-09-09 came from"
    );
}

//#region 📷️OpeningCamera
/// 📷️ Reads one pane's published pose straight off the world lane it renders — the exact string
/// `World3dHost` parses into `data-camera-json`.
async fn published_camera(app: &mut Puzzle3dApp, window_id: &str) -> Value {
    camera_of(&render_window(app, window_id).await)
}

fn camera_position(camera: &Value) -> Vec<f64> {
    camera.get("position").and_then(Value::as_array).map(|axes| axes.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
}

/// 📷️ Every pane opens on a real pose of its own before the user has touched anything.
///
/// `Puzzle3dCamera::default()` is all zeros, and `World3dHost` frames each pane LOCALLY and
/// deliberately never dispatches that framing back (`🌐️World3dHost/🟦️.tsx`'s `WorldAutoFit`), so
/// until this wave the world lane of both panes carried position == target == `[0,0,0]` until the
/// first `setCamera` landed: no view direction at boot, and two panes of one document publishing the
/// SAME empty pose (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §4.1 measured it live, wave B15
/// fixed it).
#[semio_framework_async_macros::async_test]
async fn every_pane_opens_on_its_own_framed_camera_before_any_gesture() {
    let mut app = app().await;
    let top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    let perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    for (window_id, camera) in [(main::WINDOW_INSTANCE_TOP, &top), (main::WINDOW_INSTANCE_PERSPECTIVE, &perspective)] {
        let position = camera_position(camera);
        assert_eq!(position.len(), 3, "{window_id} publishes a three-axis camera position: {camera}");
        assert!(position.iter().any(|axis| axis.abs() > 1e-6), "{window_id} opens on a real pose, never the all-zero default: {camera}");
        assert_ne!(camera.get("position"), camera.get("target"), "{window_id}'s opening camera must look at something other than where it stands: {camera}");
    }
    assert_ne!(top, perspective, "the Top and Perspective panes of ONE document open on DIFFERENT poses: top={top} perspective={perspective}");
    assert_eq!(top.pointer("/projection/mode/kind").and_then(Value::as_str), Some("orthographic"), "the Top pane opens under the orthographic plan its layout template declares: {top}");
    assert_eq!(top.pointer("/projection/orientation/view").and_then(Value::as_str), Some("top"), "the Top pane looks straight down: {top}");
    assert_eq!(perspective.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "the Perspective pane keeps the three-point default: {perspective}");
}

/// 📷️ Swapping the example re-frames the pane on what the NEW document holds, instead of leaving the
/// camera framed on bounds nothing occupies any more.
#[semio_framework_async_macros::async_test]
async fn switching_the_example_reframes_every_unposed_pane_on_the_new_document() {
    let mut app = app().await;
    let before_perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let before_top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;

    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("switch to the Nakagin example");

    let after_perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let after_top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    assert_ne!(before_perspective, after_perspective, "the pane the switch was dispatched from re-frames: before={before_perspective} after={after_perspective}");
    assert_ne!(before_top, after_top, "a sibling pane the user never posed re-frames on the new document too: before={before_top} after={after_top}");
    assert_ne!(after_top, after_perspective, "re-framing keeps the two panes distinct: top={after_top} perspective={after_perspective}");
}

/// 📷️ A framed pane is framed ONCE: an opening pose is not a gesture, so re-reading the same pane
/// twice, and dispatching an unrelated window option in between, must publish the same pose.
#[semio_framework_async_macros::async_test]
async fn an_opening_camera_is_stable_and_never_overrules_a_pose_the_user_set() {
    let mut app = app().await;
    let opening = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("an unrelated window option");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await, opening, "an unrelated option must not move the opening pose");

    let posed = json!({ "camera": { "position": [11.0, -7.0, 5.0], "target": [1.0, 2.0, 3.0], "zoom": 1.0 } });
    dispatch(&mut app, "setCamera", Some(&posed), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("the user poses the camera");
    let user = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(camera_position(&user), vec![11.0, -7.0, 5.0], "a real gesture owns the pane's pose from then on: {user}");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await, user, "and the framing never takes it back: {user}");
}

/// 📽️ Flipping ONE pane's projection repaints THAT pane's published camera and no sibling's.
///
/// A projection is not a label on the pose — it IS the pose: `setProjection` re-derives position and
/// up from the new orientation around the unchanged target (`🎮️commands/📽️set-projection`), and the
/// `WindowOption` scope it publishes under repaints the world body, so the world lane's `cameraJson`
/// — what `World3dHost` parses into `data-camera-json` — must differ in the same settle. The browser
/// probe read `projection-repaints-camera` as bit-identical (ticket 26/09/02 wave B38 §1.3); this law
/// is what says whether the guest owes that, and it does not — the probe sampled the camera ~100 ms
/// after the click, while a measures-rail control's own round trip is documented at 0.7 s and up
/// (`🛠️ShellHelpers/🎚️measure-controls`'s `useWindowMeasureDraft`), so the value it saw flip was that
/// control's optimistic DRAFT and the camera it read was the pre-dispatch one.
#[semio_framework_async_macros::async_test]
async fn flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone() {
    let mut app = app().await;
    let before = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let sibling_before = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    assert_eq!(before.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "the law starts from the perspective pane's own three-point default: {before}");

    dispatch(&mut app, "setProjection", Some(&json!({ "field": "orthographicView", "value": "plan" })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setProjection orthographicView=plan");

    let after = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(after.pointer("/projection/mode/kind").and_then(Value::as_str), Some("orthographic"), "the flipped pane publishes the projection it was given: {after}");
    assert_eq!(after.pointer("/projection/orientation/view").and_then(Value::as_str), Some("plan"), "and the orientation it was given: {after}");
    assert_ne!(camera_position(&after), camera_position(&before), "an orthographic plan looks straight down — the POSE must move, not only its label: before={before} after={after}");
    assert_ne!(after.get("up"), before.get("up"), "and the up vector is re-derived with it: before={before} after={after}");
    let eye = camera_position(&after);
    let focus = camera_position(&after.get("target").cloned().map(|target| json!({ "position": target })).expect("the camera lane carries its target"));
    assert!((eye[0] - focus[0]).abs() < 1e-9 && (eye[1] - focus[1]).abs() < 1e-9 && eye[2] > focus[2], "a plan view stands directly above what it looks at: eye={eye:?} target={focus:?}");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await, sibling_before, "the sibling pane's camera lane is untouched by another pane's projection: {sibling_before}");

    dispatch(&mut app, "setProjection", Some(&json!({ "field": "perspectiveKind", "value": "threePoint" })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setProjection perspectiveKind=threePoint");
    let restored = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(restored.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "flipping back is symmetric: {restored}");
    assert_ne!(camera_position(&restored), camera_position(&after), "and moves the pose back off the plan axis: after={after} restored={restored}");
    eprintln!("[DEBUG] projection repaint: before={} plan={} restored={}", camera_position(&before).len(), camera_position(&after).len(), camera_position(&restored).len());
}
//#endregion 📷️OpeningCamera

//#region ⚙️SettingsPanelScope
/// ⚙️ The Settings panel addresses the pane the user is LOOKING at.
///
/// 🎯️ An app-level panel body is rendered through `ViewModel::for_panel()`, which clears `window_id`
/// by design — so before this wave `puzzle3d_addressed_window_id` fell through to the roster's first
/// entry, the base window KIND, and every stepper baked THAT into its args: a Settings edit landed on
/// a pane nobody has open (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1 measured
/// `setGridSpacing window=Some("puzzle3d-main")` live). `focused_window_id` survives `for_panel` and
/// is what the panel resolves through now.
#[semio_framework_async_macros::async_test]
async fn the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind() {
    fn node_by_key(node: &Value, key: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(key) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| node_by_key(child, key)),
            Value::Array(items) => items.iter().find_map(|item| node_by_key(item, key)),
            _ => None,
        }
    }
    let mut app = app().await;
    for focused in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        let panel = render_panel_body(&mut app, settings_panel::BODY_KEY, Some(focused)).await;
        let rendered = panel.to_string();
        assert!(rendered.contains(focused), "the Settings section names the pane it is addressed at, so the user can see which one a bump will retune: {rendered}");
        for field in ["overlap-budget", "proximity-radius", "chunk-size", "grid-spacing"] {
            let control = node_by_key(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}.control")).unwrap_or_else(|| panic!("the {field} stepper: {panel}"));
            let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
            assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(focused), "{field} tags the focused pane, never the base window kind: {bindings:?}");
        }
    }
    let unfocused = render_panel_body(&mut app, settings_panel::BODY_KEY, None).await;
    let control = node_by_key(&unfocused, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.grid-spacing.control")).expect("the grid spacing stepper");
    let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
    assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(main::WINDOW_KIND_ID), "with nothing focused the panel falls back to the roster, which is the pre-existing behaviour: {bindings:?}");
}

/// ⚙️ A Settings bump reaches the focused pane's own rail and leaves the sibling pane alone — the
/// split-window law A1/A2 §19 asked for, played through the id the panel itself baked.
#[semio_framework_async_macros::async_test]
async fn a_settings_bump_retunes_only_the_focused_panes_rail() {
    let spacing_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing");
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let top_view = app.window_view(main::WINDOW_INSTANCE_TOP);
    let perspective_view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let top_before = find_measure_slider(app.window_measures(&top_view).await.get(main::WINDOW_INSTANCE_TOP).expect("top rail"), &spacing_id);
    let perspective_before = find_measure_slider(app.window_measures(&perspective_view).await.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id);
    assert_eq!(top_before, perspective_before, "both panes start on the same spacing, so a divergence below can only come from the addressing");

    let bumped = top_before.expect("the rail publishes a spacing") + 0.5;
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "windowId": main::WINDOW_INSTANCE_TOP, "value": bumped })), Some(main::WINDOW_INSTANCE_TOP)).await.expect("bump the focused pane's spacing");

    let top_view = app.window_view(main::WINDOW_INSTANCE_TOP);
    let measures = app.window_measures(&top_view).await;
    assert_eq!(find_measure_slider(measures.get(main::WINDOW_INSTANCE_TOP).expect("top rail"), &spacing_id), Some(bumped), "the focused pane's own rail carries what the Settings stepper published");
    assert_eq!(find_measure_slider(measures.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id), perspective_before, "the pane the user is NOT looking at keeps its own spacing");
}

/// ⚙️ The Settings panel must RENDER the pane it is ADDRESSED at. Every stepper tags the focused pane
/// as `windowId` (law above), but the value it shows came from `ConfigView::window`, which is `None` for
/// a panel projection (`ViewModel::for_panel` drops `window_id`) and therefore fell through to
/// `Puzzle3dWindowConfig::default()`. The panel then displayed the DEFAULT spacing while writing to the
/// focused pane: battery #53 read `settings-value-reaches-window-rail settings=10.5 windowRail=12.5`
/// after the rail had been nudged to 12.5 — the panel bumped a 10.0 it had invented
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B36).
#[semio_framework_async_macros::async_test]
async fn the_settings_panel_renders_the_focused_panes_own_value_not_a_default() {
    fn control(node: &Value, key: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(key) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| control(child, key)),
            Value::Array(items) => items.iter().find_map(|item| control(item, key)),
            _ => None,
        }
    }
    let spacing_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing");
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "windowId": main::WINDOW_INSTANCE_PERSPECTIVE, "value": 12.5 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("nudge the perspective pane's spacing");
    let perspective_view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let rail = app.window_measures(&perspective_view).await;
    assert_eq!(find_measure_slider(rail.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id), Some(12.5), "the rail carries the nudge, so the panel below has something to disagree with");
    let panel = render_panel_body(&mut app, settings_panel::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await;
    let stepper = control(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.grid-spacing.control")).unwrap_or_else(|| panic!("the grid spacing stepper: {panel}"));
    let rendered = stepper.get("component").and_then(|props| props.get("value")).and_then(Value::as_f64);
    assert_eq!(rendered, Some(12.5), "the Settings panel renders the focused pane's own spacing, never a default it invented: {stepper}");
}
//#endregion ⚙️SettingsPanelScope

//#region 🔬️B23RenderedVsMutatedDocument
/// 🔬️ Wave B23 — the browser's own sequence, on the two-pane session the browser boots: pick the
/// object, read Inspection through BOTH host render routes (the panel route, which carries no
/// `window_id` at all, and the window-instance route), lock the picked selection through the
/// context-menu path (no explicit ids — the arm reads the selection the render just painted), then
/// translate it and read the world lane the pane publishes.
///
/// 🕳️ Every failing verdict of the `#48` selection-scoped family is one row of this law
/// (`inspection-object-fields id=null`, `locked-flag-row lockChrome=false`,
/// `gumball-scene-delta sceneDelta=false`, `relocate-pose-delta`), and the law is GREEN — the guest
/// keeps the pick across both routes and both mutating dispatches, including the
/// `revalidate_interaction_state_after_document_change` pass every document-intent dispatch runs.
/// What the browser has and this law cannot reach is the surface-context route
/// (`plugin_mount_surface` → `SurfaceContexts` → `plugin_render_surface`): every test context render
/// helper calls `PluginApp::render` directly with a hand-built `ViewModel`, so a body that is never
/// re-rendered — or re-rendered against another surface's retained view — is invisible here.
#[semio_framework_async_macros::async_test]
async fn a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches() {
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("browser-shaped pick");
    let panel_route = render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string();
    assert!(!panel_route.contains("puzzle3d-play-inspector.empty"), "the panel route — the only one a real Inspection refresh uses — must see the pick: {panel_route}");
    assert!(panel_route.contains(object_id.as_str()), "the panel route must name the picked object: {panel_route}");
    assert!(panel_route.contains("puzzle3d-play-inspector.object.locked"), "the panel route must assemble the lock flag_row the context menu drives: {panel_route}");
    let window_route = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!window_route.contains("puzzle3d-play-inspector.empty"), "the window-instance route must see the same pick: {window_route}");

    let before = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("lock the picked object");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "the context-menu lock path must land on the object the pick named");
    dispatch(&mut app, "translateSelection", Some(&json!({ "dx": 3.0, "dy": 0.0, "dz": 0.0 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("translate the picked object");
    let after = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    assert_ne!(before, after, "a translate taken on the SECOND mutating dispatch after the pick must still change the world lane the pane renders");
    assert!(
        !render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string().contains("puzzle3d-play-inspector.empty"),
        "two document-intent dispatches run two `revalidate_interaction_state_after_document_change` passes; neither may eat the pick"
    );
}

fn object_flag(app: &Puzzle3dApp, object_id: &str, flag: &str) -> Option<bool> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get(flag).and_then(Value::as_bool))
}
//#endregion 🔬️B23RenderedVsMutatedDocument
