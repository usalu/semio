/// 🏗️ Raw app fixture: the raw app, the live-cleanup grant, the maintenance stage grants, `Puzzle3dApp` and its close witness, the manifest for tests and the host turn drivers.

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

    fn record(&mut self, stage: u8, grant: semio_framework_value::retained_clone::RetainedCloneGrant, step: &Result<PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure>) {
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
/// inside a destructor. The fixture retains its app and original caller through a refused close;
/// an ordinary drop drains the same paid state machine and asserts its terminal witness.
/// 📦️ The raw app is boxed on purpose: `Puzzle3dRawApp` is 42 KiB by value, and a law's `#[async_test]` future — which
/// `#[async_test]` pins ON THE STACK — moves the scene_snapshot through every frame of the harness. The default test thread
/// has 2 MiB, and the app-scene_snapshot laws in this crate already spend most of it inside the framework's own construction
/// chain.
pub struct Puzzle3dApp {
    raw: Option<Box<Puzzle3dRawApp>>,
    /// 🎟️ The fixture's ONE original caller scope: every call that needs a `cx` threads it through [`Puzzle3dApp::parts`], and the scope pays the caller's terminal retirement after the paid app close.
    scope: semio_framework_plugin::artifact_app_laws::OriginalFixtureScope,
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
    /// 🎟️ The raw app together with the fixture's original caller context — disjoint borrows, so a call reads `let (raw, cx) = app.parts();`.
    pub fn parts(&mut self) -> (&mut Puzzle3dRawApp, &mut semio_framework_plugin::StepContext<'static>) {
        (self.raw.as_deref_mut().expect("scene_snapshot app was already consumed by close_witness"), self.scope.cx())
    }

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
    pub fn measure_maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        let stage = self.next_maintenance_stage();
        let (raw, cx) = self.parts();
        let demand = PluginApp::maintenance_retirement_demands(raw, maximum_bytes)?;
        let grant = semio_s_plugin_puzzle_jobs::testing::funded_turn(demand, maximum_items);
        let step = cx.with_retained_loan(grant, |cx| {
            let loan = cx.retained_grant();
            PluginApp::maintenance_step(raw, cx, loan).map_err(semio_framework_job::OriginalTurnFailure::from)
        }).map_err(semio_framework_plugin::PluginLifecycleFailure::from);
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

/// 🧯️ Projects a fixture dispatch failure onto the diagnostic `Fault` the drivers' callers assert on: the original fault when that is the primary, else the complete debug form.
pub fn fault_of(failure: semio_framework_job::OriginalTurnFailure) -> Fault {
    failure.primary_fault().cloned().unwrap_or_else(|| Fault::from(format!("{failure:?}")))
}

/// 🧹️ Drives `app` through the real `PluginApp` close state machine with the fixture's original caller, one grant
/// quoted from the owner per turn exactly as a host actor tick does, and returns the terminal-empty witness or the
/// fault that stopped it. Bounded by the caller's own opportunity (`maximum_turns`), never by a local spin cap.
fn drain_close(app: &mut Puzzle3dRawApp, cx: &mut semio_framework_plugin::StepContext<'_>) -> Result<bool, semio_framework_plugin::PluginLifecycleFailure> {
    artifact_app_laws::close_registered_fixture_app(app, artifact_app_laws::fixture_mounted_policy(), cx)?;
    Ok(app.close_terminal_is_empty())
}

/// 🧹️ Consumes one scene_snapshot app through its real close state machine and reports the outcome, so the
/// close contract is asserted by a test rather than by a destructor. `Err` carries the framework's own
/// fault; `Ok(false)` means the machine reported `Complete` without reaching terminal-empty ownership.
pub fn close_witness(app: &mut Puzzle3dApp) -> Result<bool, semio_framework_plugin::PluginLifecycleFailure> {
    let Some(raw) = app.raw.as_deref_mut() else { return Ok(true) };
    let complete = drain_close(raw, app.scope.cx())?;
    if complete {
        app.raw.take();
    }
    Ok(complete)
}

impl Drop for Puzzle3dApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(close_witness(self).expect("original Puzzle3D app retirement"));
        }
        puzzle3d_session_release(self.instance_id);
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
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let registry = puzzle3d_action_registry();
    let mut app = semio_framework_plugin::with_authoring_identity!(|identity| VcsArtifactApp::with_registry(EditorApp::<Puzzle3dPlayApp>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), artifact_app_laws::fixture_mounted_policy(), &mut identity).await);
    let instance_id = next_fixture_instance_id();
    app.bind_instance_id(instance_id).await;
    artifact_app_laws::settle_store_constructor(&mut app, artifact_app_laws::fixture_mounted_policy());
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: main::WINDOW_KIND_ID.into(), window_kind_id: main::WINDOW_KIND_ID.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    Puzzle3dApp { raw: Some(Box::new(app)), scope, instance_id, view, maintenance: Box::new(MaintenanceStageGrants::default()) }
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
    let (raw, cx) = app.parts();
    raw.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("advance one typed operation publication unit");
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
