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

    fn record(&mut self, stage: u8, grant: semio_framework_value::retained_clone::RetainedCloneGrant, step: &Result<PluginLifecycleStep, semio_plugin::PluginLifecycleFailure>) {
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
    scope: semio_plugin::artifact_app_laws::OriginalFixtureScope,
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
    pub fn parts(&mut self) -> (&mut Puzzle3dRawApp, &mut semio_plugin::StepContext<'static>) {
        (self.raw.as_deref_mut().expect("scene_snapshot app was already consumed by close_witness"), self.scope.cx())
    }

    /// 🪟️ Registers one window instance in the scene_snapshot's live roster, exactly as the shell's own
    /// `sessionWindowInstances` grows the moment a pane is opened or split. Idempotent.
    fn ensure_window(&mut self, window_id: &str) {
        if !self.view.window_instances.iter().any(|instance| instance.id == window_id) {
            self.view.window_instances.push(ViewWindowInstance { id: window_id.into(), window_kind_id: semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID.into() });
            let view = self.window_view(window_id);
            self.window_transient_generation(&view).expect("open the fixture window's transient partition").expect("the Puzzle window has a transient owner");
            self.birth_window_config(&view).expect("open the fixture window's config partition");
        }
    }

    /// 🎯️ The exact per-call projection a host sends: the whole live session — roster, mode-wide tool,
    /// per-window utility map — addressed at ONE concrete window instance through the framework's own
    /// `ViewModel::for_window_instance`, which is what stamps `active_utility_id` for that pane.
    /// 🗣️ Names the host's live label axes for every later render/measures call on this scene_snapshot. The
    /// app resolves its label set from `ViewModel.locale`/`.terminology` and fails closed on an axis it
    /// never authored (`puzzle3d_labels`), so a test that asserts German or reuse text has to say so —
    /// there is no default language to fall back to.
    pub fn set_label_axes(&mut self, locale: semio_ui_locale::Locale, terminology: semio_ui_locale::Terminology) {
        self.view.locale = locale;
        self.view.terminology = terminology;
    }

    /// 📼️ What a renderer echoes after it applied a tool run trace delta for `window_id`.
    pub fn echo_tool_run_trace_cursor(&mut self, window_id: &str, cursor: semio_framework_tool_run::ToolRunTraceCursor) {
        self.view.tool_run_trace_cursor_by_window_id.insert(window_id.to_string(), cursor);
    }

    /// 🔢️ The viewed window's config generation after the native partition birth the host pumps before every capture.
    pub async fn window_config_generation(&mut self, view: &ViewModel) -> Result<Option<u64>, semio_plugin::Fault> {
        self.birth_window_config(view)?;
        let (raw, _) = self.parts();
        raw.window_config_generation(view).await
    }

    /// 🌱️ Pumps the viewed window's native config partition birth under the caller's full grant, as the host does before every capture, so a keyed command never has to birth it inside the narrower publication grant.
    fn birth_window_config(&mut self, view: &ViewModel) -> Result<(), semio_plugin::Fault> {
        let fuel = artifact_app_laws::OriginalFixtureCaller::turn_fuel();
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
            let (raw, cx) = self.parts();
            let grant = cx.retained_grant();
            artifact_app_laws::fixture_renew_turn(cx, fuel, grant).map_err(|error| semio_plugin::Fault::from(format!("{error:?}")))?;
            let before = cx.retained_progress();
            match raw.stores.window_config_store.ensure_for_view(Some(view), cx) {
                Ok(ready) if ready && cx.retained_progress() == before => break,
                Ok(_) => {}
                Err(failure) => {
                    let fault = fault_of(failure);
                    return Err(fault);
                }
            }
        }
        Ok(())
    }

    pub fn window_view(&self, window_id: &str) -> WindowView {
        let mut view = self.view.clone();
        if !view.window_instances.iter().any(|instance| instance.id == window_id) {
            view.window_instances.push(ViewWindowInstance { id: window_id.into(), window_kind_id: semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID.into() });
        }
        let projected = view.for_window_instance(window_id).expect("puzzle3d test window roster");
        retire_view(view);
        WindowView(Some(projected))
    }

    /// 🪪️ The addressed window's host projection already bound to a local-actor `ActionMeta`, retired together with its view.
    pub fn window_meta(&self, window_id: &str) -> WindowMeta {
        WindowMeta(Some(ActionMeta { view_state: Some(std::sync::Arc::new(self.window_view(window_id).into_inner())), ..self.meta(self.stores.store.local_actor_id().0.as_str()) }))
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
                std::mem::take(&mut self.view.active_utility_by_window_id).retire_cold();
            }
            return;
        }
        let window = args.and_then(|value| value.get("windowId")).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or(window_id).to_string();
        match resolve_activation(self.view.active_utility_by_window_id.get(&window).and_then(Option::as_deref), requested("utilityId")) {
            Some(utility) => {
                self.view.active_utility_by_window_id.insert(window, Some(utility));
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
    pub fn measure_maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginLifecycleStep, semio_plugin::PluginLifecycleFailure> {
        use semio_plugin::{MountedOwnerIdentityV1, MountedOwnerPhaseV1, MountedOwnerTurnV1};

        let stage = self.next_maintenance_stage();
        let instance_id = self.instance_id;
        let (raw, cx) = self.parts();
        let identity = MountedOwnerIdentityV1 { instance_id, operation: cx.operation(), generation: cx.generation() };
        let phase = raw.mounted_owner_maintenance_phase()?;
        let mut policy = artifact_app_laws::fixture_mounted_policy();
        let loan = match phase {
            MountedOwnerPhaseV1::Preparation => &mut policy.preparation,
            MountedOwnerPhaseV1::Maintenance => &mut policy.maintenance,
            MountedOwnerPhaseV1::Close => &mut policy.close,
        };
        loan.maximum_items = loan.maximum_items.min(maximum_items);
        loan.maximum_copy_bytes = loan.maximum_copy_bytes.min(maximum_bytes);
        let Some(mut turn) = MountedOwnerTurnV1::admit(instance_id, identity, phase, policy, cx)? else {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        };
        let grant = turn.grant();
        let step = raw.mounted_owner_maintenance_step(&mut turn);
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

/// ♻️ Retires the two ordered roots of one host view synchronously, so dropping the view never aborts an unretired ownership.
pub fn retire_view(mut view: ViewModel) {
    std::mem::take(&mut view.active_utility_by_window_id).retire_cold();
    std::mem::take(&mut view.tool_run_trace_cursor_by_window_id).retire_cold();
}

/// 🪟️ One window projection of the host session; its ordered roots are retired on drop instead of aborting.
pub struct WindowView(Option<ViewModel>);

impl WindowView {
    pub fn into_inner(mut self) -> ViewModel {
        self.0.take().expect("the window view is live until consumed")
    }
}

impl std::ops::Deref for WindowView {
    type Target = ViewModel;
    fn deref(&self) -> &ViewModel {
        self.0.as_ref().expect("the window view is live until consumed")
    }
}

impl Drop for WindowView {
    fn drop(&mut self) {
        if let Some(view) = self.0.take().filter(|_| !std::thread::panicking()) {
            retire_view(view);
        }
    }
}

/// 🪪️ One `ActionMeta` whose shared window view is retired when the meta goes out of scope.
pub struct WindowMeta(Option<ActionMeta>);

impl std::ops::Deref for WindowMeta {
    type Target = ActionMeta;
    fn deref(&self) -> &ActionMeta {
        self.0.as_ref().expect("the window meta is live")
    }
}

impl std::ops::DerefMut for WindowMeta {
    fn deref_mut(&mut self) -> &mut ActionMeta {
        self.0.as_mut().expect("the window meta is live")
    }
}

impl Drop for WindowMeta {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        if let Some(view) = self.0.take().and_then(|meta| meta.view_state) {
            if let Ok(view) = std::sync::Arc::try_unwrap(view) {
                retire_view(view);
            }
        }
    }
}

/// 🧯️ Projects a fixture dispatch failure onto the diagnostic `Fault` the drivers' callers assert on: the original fault when that is the primary, else the complete debug form.
pub fn fault_of(failure: semio_framework_job::OriginalTurnFailure) -> Fault {
    failure.primary_fault().cloned().unwrap_or_else(|| Fault::from(format!("{failure:?}")))
}

/// 🧹️ Drives `app` through the real `PluginApp` close state machine with the fixture's original caller, one grant
/// quoted from the owner per turn exactly as a host actor tick does, and returns the terminal-empty witness or the
/// fault that stopped it. The fixture's `maximum_close_turns` bounds the number of grants.
fn drain_close(app: &mut Puzzle3dRawApp, cx: &mut semio_plugin::StepContext<'_>) -> Result<bool, semio_plugin::PluginLifecycleFailure> {
    artifact_app_laws::close_registered_fixture_app(app, artifact_app_laws::fixture_mounted_policy(), cx)?;
    Ok(app.close_terminal_is_empty())
}

/// 🧹️ Consumes one scene_snapshot app through its real close state machine and reports the outcome, so the
/// close contract is asserted by a test rather than by a destructor. `Err` carries the framework's own
/// fault; `Ok(false)` means the machine reported `Complete` without reaching terminal-empty ownership.
pub fn close_witness(app: &mut Puzzle3dApp) -> Result<bool, semio_plugin::PluginLifecycleFailure> {
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
            retire_view(std::mem::replace(&mut self.view, ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)));
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

semio_plugin::history_edit_acceptance_law!("puzzle", Puzzle3dPlayApp, puzzle3d_manifest_for_tests, "../../..");
semio_plugin::history_edit_acceptance_law!(@close Puzzle3dPlayApp, puzzle3d_manifest_for_tests, "../../..",
    "🧫️fixtures/🧬️mutations/✋️selection/✋️drag-selection/⏸️keeps-a-zero-offset");

/// 🧰️ The registry-backed, instance-bound scene_snapshot app — the ONLY constructor this plugin can use.
/// puzzle3d declares `bounded_first_step_tool_proofs!`, so `VcsArtifactApp::with_registry_on_bus`'s
/// `registry.tool_job_registration::<A>(…)` join needs the manifest's migrated generated
/// declarations; the registry-less `artifact_app_laws::new_app` carries an `AppActionRegistry::default()` and
/// fails closed with `interactive-job.catalog-authority` before any dispatch is attempted. The bound
/// instance id is what `advance_typed_operation_publication`/`maintenance_step` and the local
/// interaction query authority key their live-runtime bookkeeping on, exactly as the host binds it.
/// 🧱 Builds the action registry on its own frame and drops the definition before `with_registry`.
#[inline(never)]
fn puzzle3d_action_registry() -> semio_plugin::AppActionRegistry {
    let definition = create_puzzle3d_app();
    let registry = semio_plugin::AppActionRegistry::from_definition(&definition);
    drop(definition);
    registry
}

pub async fn app() -> Puzzle3dApp {
    app_for_actor(semio_os_kernel::LOCAL_ACTOR_ID).await
}

/// 👥️ Mounts a scene with an independent document author.
pub async fn app_for_actor(actor: &str) -> Puzzle3dApp {
    let scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let registry = puzzle3d_action_registry();
    let mut app = semio_plugin::with_authoring_identity!(|identity| VcsArtifactApp::with_registry(EditorApp::<Puzzle3dPlayApp>::default(), registry, semio_os_kernel::ActorId(actor.into()), artifact_app_laws::fixture_mounted_policy(), &mut identity).await.expect("the reference snapshot births"));
    let instance_id = next_fixture_instance_id();
    app.bind_instance_id(instance_id).await;
    artifact_app_laws::settle_store_constructor(&mut app, artifact_app_laws::fixture_mounted_policy());
    let view = ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    let mut fixture = Puzzle3dApp { raw: Some(Box::new(app)), scope, instance_id, view, maintenance: Box::new(MaintenanceStageGrants::default()) };
    fixture.ensure_window(WINDOW_KIND_ID);
    fixture
}

/// 🖌️ Host-session utility activation without minting a typed operation or running `settle`.
#[inline(never)]
pub fn activate_window_utility(app: &mut Puzzle3dApp, utility_id: &str) {
    app.ensure_window(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    app.activate(semio_plugin::SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utility_id })), semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
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
    let census_before = semio_plugin::app::typed_operation_unit_census();
    let fresh_fuel = artifact_app_laws::OriginalFixtureCaller::turn_fuel();
    let fresh_grant = app.parts().1.retained_grant();
    for _ in 0..1_048_576 {
        if !app.has_pending_typed_operations() {
            settled.census = semio_plugin::app::typed_operation_unit_census() - census_before;
            return settled;
        }
        {
            let (_, cx) = app.parts();
            artifact_app_laws::fixture_renew_turn(cx, fresh_fuel, fresh_grant).expect("renew the fixture turn like the host's fresh context");
        }
        host_turn(app, maximum_items, &mut settled).await;
    }
    let (raw, cx) = app.parts();
    panic!(
        "puzzle3d app never quiesced: pending typed operations outlived the settle budget: {}. History work {}, patch {}, host routes {}, host command {}, resumes {}, fuel {}, grant {:?}, should yield {}, pinned {:?}, return budget {:?}",
        raw.typed_operation_diagnostic(), raw.time_travel.has_pending_work(), raw.time_travel.has_patch(),
        raw.host_events.routes.pending(), raw.original_host_event_command.pending(), raw.host_events.resumes.pending(),
        cx.fuel_remaining(), cx.retained_grant(), cx.should_yield(),
        semio_framework_job::testkit::return_source_pinned(cx), semio_framework_job::testkit::return_budget_snapshot(cx),
    );
}

/// 🔁️ ONE host continuation turn — the unit [`settle_with_items`] repeats — folding what it forwards
/// into `settled`.
pub async fn host_turn(app: &mut Puzzle3dApp, maximum_items: usize, settled: &mut Puzzle3dSettled) {
    settled.turns += 1;
    let debug_before = semio_plugin::app::typed_operation_unit_census();
    let policy = artifact_app_laws::fixture_mounted_policy();
    let maintenance_items = maximum_items.max(policy.maintenance.maximum_items);
    let (publication_grant, caller_grant) = (policy.maintenance, app.parts().1.retained_grant());
    app.measure_maintenance_step(maintenance_items, policy.maintenance.maximum_copy_bytes).expect("maintenance step drives the mounted operation's worker and retirement stages");
    let instance_id = app.instance_id;
    let (raw, cx) = app.parts();
    artifact_app_laws::fixture_renew_turn(cx, artifact_app_laws::OriginalFixtureCaller::turn_fuel(), publication_grant).expect("renew the fixture turn for publication after maintenance");
    if let Err(failure) = raw.advance_typed_operation_publication(&mut semio_plugin::artifact_app_laws::fixture_identity(), cx).await {
        let runs = raw.tool_runs.entries.iter().map(|entry| (&entry.tool_id, entry.slot.state, entry.operations.len(), entry.job.as_ref().and_then(|job| job.origin.outcome()).map(|outcome| outcome.kind()))).collect::<Vec<_>>();
        panic!("publication failed on host turn {} at {}, tool runs {runs:?}: {failure:?}", settled.turns, raw.typed_operation_diagnostic());
    }
    semio_plugin::artifact_app_laws::retain_original_ownership_receipts(raw, semio_plugin::artifact_app_laws::fixture_mounted_policy(), &mut Vec::new()).expect("post the ownership receipts to the fixture caller like the host's exchange output");
    artifact_app_laws::fixture_renew_turn(cx, artifact_app_laws::OriginalFixtureCaller::turn_fuel(), caller_grant).expect("restore the fixture caller grant after publication");
    while let Some(mut page) = semio_plugin::artifact_app_laws::take_fixture_result_page(raw, instance_id, policy.preparation).expect("take one typed operation result page") {
        assert_ne!(page.lane, semio_plugin::app::TypedOperationResultLane::Fault, "retained operation faulted: {}", String::from_utf8_lossy(page.bytes()));
        if page.lane == semio_plugin::app::TypedOperationResultLane::Download {
            settled.downloads.push(Puzzle3dDownload::from_page(page.token.operation, page.bytes()));
        }
        let token = semio_plugin::artifact_app_laws::close_fixture_result_page(&mut page, policy.close);
        assert!(raw.acknowledge_typed_operation_result(token).expect("acknowledge one presented result page"), "the app's own presented result page must accept its exact token");
    }
    settled.effects.extend(app.take_typed_operation_effect());
    settled.events.extend(app.take_typed_operation_event());
    if let Some(completion) = app.take_typed_operation_completion().await.expect("take one typed operation completion witness") {
        settled.completions.push(Puzzle3dCompletion { operation: completion.operation, ui_scope: completion.ui_scope, history_patch: completion.history_patch });
    }
    if let Some(progress) = app.take_typed_operation_ui_progress() {
        if progress.ui_scope != UiDirtyScope::None || settled.scope.is_none() {
            settled.scope = Some(progress.ui_scope);
        }
    }
    acknowledge_local_interaction_pages(app);
    if std::env::var_os("L3D_TRACE").is_some() { eprintln!("[DEBUG] turn {} census [{}] diag {}", settled.turns, semio_plugin::app::typed_operation_unit_census() - debug_before, app.parts().0.typed_operation_diagnostic()); }
}
