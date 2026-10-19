//! 🧰️ Puzzle 5d editor test support: the registry-backed test app, the settle and dispatch drivers, the render probes and the measure probes every editor test drives. Compiled under `cfg(test)` or the `testkit` feature.


use semio_framework_pack_json::Value;
use semio_framework_plugin::{ActionMeta, EditorApp, Fault, InteractionTarget, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowMeasure, artifact_app_laws};
use crate::editor::puzzle5d::entities::{PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_PLAY_WINDOWS};
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::editor::puzzle5d::{create_puzzle5d_app, Puzzle5dPlayApp};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand};

/// ✏️ `Puzzle5dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<Puzzle5dPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<Puzzle5dPlayApp>` builds it.
pub type Puzzle5dApp = VcsArtifactApp<EditorApp<Puzzle5dPlayApp>>;

/// 🧯️ Projects a fixture dispatch failure onto the diagnostic `Fault` the drivers' callers assert on: the original fault when that is the primary, else the complete debug form.
fn fault_of(failure: semio_framework_job::OriginalTurnFailure) -> Fault {
    failure.primary_fault().cloned().unwrap_or_else(|| Fault::from(format!("{failure:?}")))
}

pub fn meta(actor: &str) -> ActionMeta {
    semio_framework_plugin::artifact_app_laws::meta(actor)
}

/// ✏️ Adapts `create_puzzle5d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `context::new_app_with_registry` still expects — framework test context gap, not
/// modifiable here (`🧰️framework/**` is outside this packet's lease).
pub fn puzzle5d_app_manifest_for_tests() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_puzzle5d_app(), examples: Vec::new() }
}


/// 🧰️ The registry-backed app every test drives: the tool proof catalog joins the migrated declarations to live
/// factories only with a manifest, so kind discipline and the utility contract are enforced exactly as in
/// production.
/// ⏳️ The harness's own executor: polls one app future to completion, yielding between turns.
///
/// 🐛️ This file bridged every app call through `semio_framework_os_kernel::io::resolve_ready`, whose contract
/// is "an ARTIFACT-IO body must complete without a real suspension" — it PANICS on the first
/// `Pending`. That contract belongs to the synchronous `IoEntry`/compose thunks it was written for,
/// not to a harness driving a live app: the clipboard route's store admission is a real await point,
/// so `copy`/`cut`/`paste` through `dispatch` aborted with "future was not ready on first poll". A
/// live app's futures make progress on every poll, so the harness polls — the same noop-waker spin
/// `◻️2d`'s harness and block3d's window-transient law use.
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

pub fn app() -> Puzzle5dTestApp {
    app_with_registry()
}

/// 🧹️ A registered app that closes itself to terminal-empty ownership when the test drops it, so a test that
/// never reaches `close_app` still retires its stores exactly.
/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct Puzzle5dTestApp(pub Puzzle5dApp, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

impl std::ops::Deref for Puzzle5dTestApp {
    type Target = Puzzle5dApp;

    fn deref(&self) -> &Puzzle5dApp {
        &self.0
    }
}

impl std::ops::DerefMut for Puzzle5dTestApp {
    fn deref_mut(&mut self) -> &mut Puzzle5dApp {
        &mut self.0
    }
}

impl Drop for Puzzle5dTestApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            close_app(self);
        }
    }
}

/// 🧰️ A registry-backed app so kind discipline (View actions must emit no operations) and the
/// utility contract are enforced exactly as in production.
pub fn app_with_registry() -> Puzzle5dTestApp {
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_framework_plugin::with_authoring_identity!(|identity| ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::new_app_with_registry::<EditorApp<Puzzle5dPlayApp>>(puzzle5d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut identity)));
    ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(1));
    Puzzle5dTestApp(app, scope)
}

fn action_window_kind(action: &str) -> &'static str {
    if matches!(action, "setCamera2d" | "setLodMode" | "setGridSnapEnabled" | "setGridFactor" | "setSuggestionOffset" | "setFillCount" | "canvasPointerDown") {
        board2d::WINDOW_KIND_ID
    } else {
        world3d::WINDOW_KIND_ID
    }
}

pub fn window_view(kind: &str, id: &str) -> ViewModel {
    let mut window_instances = vec![
        ViewWindowInstance { id: board2d::WINDOW_KIND_ID.into(), window_kind_id: board2d::WINDOW_KIND_ID.into() },
        ViewWindowInstance { id: world3d::WINDOW_KIND_ID.into(), window_kind_id: world3d::WINDOW_KIND_ID.into() },
    ];
    if !window_instances.iter().any(|window| window.id == id) {
        window_instances.push(ViewWindowInstance { id: id.into(), window_kind_id: kind.into() });
    }
    ViewModel { window_instances, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }.for_window_instance(id).expect("puzzle5d test window roster")
}

fn action_meta(action: &str, args: Option<&Value>, window_id: Option<&str>) -> ActionMeta {
    let requested_window = args.and_then(|value| value.get("windowId").or_else(|| value.get("window"))).and_then(Value::as_str);
    let kind = requested_window.filter(|window| PUZZLE5D_PLAY_WINDOWS.contains(window)).unwrap_or_else(|| action_window_kind(action));
    let id = window_id.or(requested_window).unwrap_or(kind);
    ActionMeta { view_state: Some(window_view(kind, id)), ..meta("local") }
}

/// 📄️ Drains what one settle turn can have left behind — presented pages, effects, events, and the
/// completions that carry the HISTORY PATCH — plus the UI scope.
///
/// 🐛️ Called inside the loop AND once more before it returns: an operation that RETIRED during the
/// previous turn's `advance_typed_operation_publication` leaves its terminal completion queued while
/// `has_pending_typed_operations` already reads false, so the exit branch handed back a result whose
/// history patch was still in the outbox. Same correction `◻️2d`'s harness carries.
///
/// 🧯️ A `Fault` page is ACKed like every other page — the host ACKs it too — and reported once the
/// operation has retired (`settle_registered_typed_operation`'s shape). Returning on it un-ACKed left the
/// faulted operation awaiting its ACK for ever, so the NEXT dispatch never settled either.
fn drain_settled(app: &mut Puzzle5dTestApp, result: &mut InvocationResult, fault: &mut Option<Fault>) -> Result<(), Fault> {
    while let Some(page) = app.take_typed_operation_result_page(1) {
        if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
            fault.get_or_insert_with(|| Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
        }
        app.acknowledge_typed_operation_result(page.token)?;
    }
    result.requested_effects.extend(app.take_typed_operation_effect());
    result.events.extend(app.take_typed_operation_event());
    while let Some(completion) = block_on(app.take_typed_operation_completion())? {
        result.ui_scope = completion.ui_scope;
        if let Some(patch) = completion.history_patch {
            result.history_patch = Some(match result.history_patch.take() {
                Some(mut previous) => {
                    previous.upserts.extend(patch.upserts);
                    previous.cursor = patch.cursor;
                    previous.can_undo = patch.can_undo;
                    previous.can_redo = patch.can_redo;
                    previous
                }
                None => patch,
            });
        }
    }
    if let Some(scope) = app.take_typed_operation_ui_scope() {
        result.ui_scope = scope;
    }
    Ok(())
}

type Identity<'a, 'b> = &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>;

fn settle(app: &mut Puzzle5dTestApp, result: Result<InvocationResult, Fault>, identity: Identity<'_, '_>) -> Result<InvocationResult, Fault> {
    let mut result = result?;
    let mut fault = None;
    for _ in 0..1_048_576 {
        if !app.has_pending_typed_operations() {
            drain_settled(app, &mut result, &mut fault)?;
            return fault.map_or(Ok(result), Err);
        }
        semio_s_plugin_puzzle_jobs::testing::maintain(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance.maximum_copy_bytes, app.1.cx())?;
        block_on(app.0.advance_typed_operation_publication(identity, app.1.cx())).map_err(|failure| Fault::from(format!("{failure:?}")))?;
        semio_s_plugin_puzzle_jobs::testing::drain_outboxes(&mut app.0);
        // 📄️ EVERY presented page and EVERY completion per turn, never one:
        // `has_pending_typed_operations` counts the outboxes and the mounted operations but NOT a
        // page already presented and waiting for its ACK, so a turn that leaves a second page — or
        // the terminal completion that CARRIES THE HISTORY PATCH — queued can be the very turn this
        // loop exits on, and the caller then reads a result with no committed edits at all. The
        // framework's own `settle_registered_typed_operation` drains both in inner loops for exactly
        // this reason, and `◻️2d`'s harness was corrected the same way.
        drain_settled(app, &mut result, &mut fault)?;
    }
    Err(Fault::from("puzzle5d test operation did not settle"))
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle5dCommand` from the same
/// `(action, args, window_id)` triple every pre-migration test already passed.
pub fn dispatch(app: &mut Puzzle5dTestApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let action_meta = action_meta(action, args, window_id);
    // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
    // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
    // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
    // setInteractionGranularity to this reserved set.
    if matches!(
        action,
        "undo"
            | "redo"
            | "checkpoint"
            | "alternative"
            | "revertToCommand"
            | "historyFilter"
            | "noteShellCommand"
            | "copy"
            | "cut"
            | "paste"
            | "setActiveUtility"
            | "interactionSelect"
            | "interactionHover"
            | "clearSelection"
            | "selectAll"
            | "setSelectionMode"
            | "setInteractionGranularity"
    ) {
        let dsl_args = args.map(semio_framework_pack_json::to_dsl_value);
        return semio_framework_plugin::with_authoring_identity!(|identity| {
            let result = block_on(semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, dsl_args.clone(), &action_meta, &mut identity, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close)).and_then(|admitted| block_on(semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted))).map_err(fault_of);
            settle(app, result, &mut identity)
        });
    }
    semio_framework_plugin::with_authoring_identity!(|identity| {
        let result = block_on(app.0.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &action_meta, &mut identity, app.1.cx()));
        settle(app, result, &mut identity)
    })
}

/// 📥️ [`dispatch`] through the SDK's own action dispatch (`handle_action`) instead of the typed channel — the lane a
/// shell's picked-file chunks take, where the framework's import staging admits them first.
pub fn dispatch_through_action(app: &mut Puzzle5dTestApp, action: &str, args: &semio_framework_value::DslValue) -> Result<InvocationResult, Fault> {
    let action_meta = action_meta(action, None, None);
    semio_framework_plugin::with_authoring_identity!(|identity| {
        let result = block_on(semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, Some(args.clone()), &action_meta, &mut identity, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close)).map_err(fault_of);
        settle(app, result, &mut identity)
    })
}

/// 🩺️ `dispatch` for an app command, with the publication ladder's own census traced on stderr: every
/// power-of-two turn and every 30 s it prints the turn, wall time, the framework's per-ladder unit census
/// since dispatch, and the lanes of every result page presented so far. Temporary `[TRACE]` capture for
/// the `capsule-dream` switch (`📓️block-puzzle.md` §11), which names WHICH lane spends its units.
pub fn dispatch_traced(app: &mut Puzzle5dTestApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let action_meta = action_meta(action, args, window_id);
    let base = semio_framework_plugin::app::typed_operation_unit_census();
    let started = std::time::Instant::now();
    semio_framework_plugin::with_authoring_identity!(|identity| dispatch_traced_with(app, action, args, window_id, &action_meta, base, started, &mut identity))
}

fn dispatch_traced_with(app: &mut Puzzle5dTestApp, action: &str, args: Option<&Value>, window_id: Option<&str>, action_meta: &ActionMeta, base: semio_framework_plugin::app::TypedOperationUnitCensus, started: std::time::Instant, identity: Identity<'_, '_>) -> Result<InvocationResult, Fault> {
    let mut result = block_on(app.0.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), action_meta, identity, app.1.cx()))?;
    let mut fault = None;
    let mut lanes: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    let mut next_turn = 1_u64;
    let mut last_print = std::time::Instant::now();
    let mut last_census = base;
    for turn in 1..=1_048_576_u64 {
        if !app.has_pending_typed_operations() {
            drain_settled(app, &mut result, &mut fault)?;
            eprintln!("{action} settled turn={turn} ms={} census={} pages={lanes:?} fault={fault:?}", started.elapsed().as_millis(), semio_framework_plugin::app::typed_operation_unit_census() - base);
            return fault.map_or(Ok(result), Err);
        }
        semio_s_plugin_puzzle_jobs::testing::maintain(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance.maximum_copy_bytes, app.1.cx())?;
        let unit = block_on(app.0.advance_typed_operation_publication(identity, app.1.cx())).map_err(|failure| Fault::from(format!("{failure:?}")));
        semio_s_plugin_puzzle_jobs::testing::drain_outboxes(&mut app.0);
        while let Some(page) = app.take_typed_operation_result_page(1) {
            *lanes.entry(format!("{:?}", page.lane)).or_default() += 1;
            if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                eprintln!("{action} FAULT page operation={} turn={turn} ms={} census={} pages={lanes:?} fault={}", page.token.operation, started.elapsed().as_millis(), semio_framework_plugin::app::typed_operation_unit_census() - base, String::from_utf8_lossy(page.bytes()));
                fault.get_or_insert_with(|| Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
            }
            app.acknowledge_typed_operation_result(page.token)?;
        }
        unit?;
        drain_settled(app, &mut result, &mut fault)?;
        if turn == next_turn || last_print.elapsed().as_secs() >= 30 {
            let now = semio_framework_plugin::app::typed_operation_unit_census();
            eprintln!("{action} turn={turn} ms={} total={} window={} pages={lanes:?} pending={}", started.elapsed().as_millis(), now - base, now - last_census, app.has_pending_typed_operations());
            last_census = now;
            last_print = std::time::Instant::now();
            if turn == next_turn {
                next_turn = next_turn.saturating_mul(2);
            }
        }
    }
    Err(Fault::from("puzzle5d traced operation did not settle"))
}

/// 🎛️ Dispatches an app command from `window_id` with `utility_id` armed in that window, as a host whose
/// window instance view state carries the armed utility does.
pub fn dispatch_armed(app: &mut Puzzle5dTestApp, action: &str, args: Option<&Value>, window_id: &str, utility_id: &str) -> Result<InvocationResult, Fault> {
    let mut view = window_view(window_id, window_id);
    view.active_utility_by_window_id.insert(window_id.to_string(), utility_id.to_string());
    let action_meta = ActionMeta { view_state: Some(view), ..meta("local") };
    semio_framework_plugin::with_authoring_identity!(|identity| {
        let result = ::semio_framework_async::poll::resolve_ready(app.0.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), Some(window_id.to_string())), &action_meta, &mut identity, app.1.cx()));
        settle(app, result, &mut identity)
    })
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
/// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
/// deleted `setSelection` action.
pub fn select_id(app: &mut Puzzle5dTestApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    dispatch(app, "interactionSelect", Some(&semio_framework_pack_json::json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
}

/// 🖼️ The rendered body, as a JSON string — every panel/window assertion greps this value.
pub fn render_body(app: &mut Puzzle5dTestApp, body_key: &str) -> String {
    render_body_with_view(app, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))
}

pub fn render_body_with_view(app: &mut Puzzle5dTestApp, body_key: &str, view_state: &ViewModel) -> String {
    let scene_json = block_on(artifact_app_laws::observe_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx(), |root| {
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                let scene = match surface.doc_schema.as_str() {
                    schema if schema == <semio_framework_ui_scene::Board2dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA => {
                        serde_json::to_value(semio_framework_ui_scene::decode::<semio_framework_ui_scene::Board2dScene>(surface).expect("decode board scene"))
                    }
                    schema if schema == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA => {
                        serde_json::to_value(semio_framework_plugin::artifact_app_laws::built_surface_scene::<semio_framework_ui_scene::World3dScene>(node).expect("assemble world scene"))
                    }
                    _ => continue,
                }
                .expect("serialize scene");
                return Some(serde_json::json!({ "schema": surface.doc_schema, "scene": scene }).to_string());
            }
            stack.extend(node.children.iter());
        }
        None
    }))
    .expect("render");
    scene_json.unwrap_or_else(|| block_on(artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx())).expect("render projection"))
}

/// 🪟️ `body_key` rendered as the window instance `window_id`. The instance travels in the view state
/// the framework captures that window's config/transient partitions from; a `"{body}:{window}"` key
/// is no body at all.
pub fn render_window(app: &mut Puzzle5dTestApp, body_key: &str, window_id: &str) -> String {
    let kind = if body_key == board2d::BODY_KEY { board2d::WINDOW_KIND_ID } else { world3d::WINDOW_KIND_ID };
    render_body_with_view(app, body_key, &window_view(kind, window_id))
}

/// 🧹️ Closes `app` to terminal-empty ownership; `#[track_caller]` so a refusal names WHICH app of a
/// two-app law did not get there.
#[track_caller]
pub fn close_app(app: &mut Puzzle5dTestApp) {
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
    assert!(app.close_terminal_is_empty(), "Puzzle 5D registered app close did not reach terminal-empty ownership");
}

//#region 🎚️Measures
/// 🎚️ Every measure id in a window's chrome, groups included — the shared reader both window
/// nodes' option laws assert their group/row ids against.
pub fn measure_ids(measures: &[WindowMeasure]) -> Vec<String> {
    let mut ids = Vec::new();
    let mut stack: Vec<&WindowMeasure> = measures.iter().collect();
    while let Some(measure) = stack.pop() {
        match measure {
            WindowMeasure::Group { id, children, .. } => {
                ids.push(id.clone());
                stack.extend(children.iter());
            }
            WindowMeasure::Toggle { id, .. } | WindowMeasure::Slider { id, .. } | WindowMeasure::Select { id, .. } => ids.push(id.clone()),
            _ => {}
        }
    }
    ids
}

/// 🎚️ The pressed state one toggle row currently renders, at any depth.
pub fn toggle_pressed(measures: &[WindowMeasure], wanted: &str) -> Option<bool> {
    let mut stack: Vec<&WindowMeasure> = measures.iter().collect();
    while let Some(measure) = stack.pop() {
        match measure {
            WindowMeasure::Group { children, .. } => stack.extend(children.iter()),
            WindowMeasure::Toggle { id, pressed, .. } if id == wanted => return Some(*pressed),
            _ => {}
        }
    }
    None
}

/// 🎚️ The value one slider row currently renders, at any depth.
pub fn slider_value(measures: &[WindowMeasure], wanted: &str) -> Option<f64> {
    let mut stack: Vec<&WindowMeasure> = measures.iter().collect();
    while let Some(measure) = stack.pop() {
        match measure {
            WindowMeasure::Group { children, .. } => stack.extend(children.iter()),
            WindowMeasure::Slider { id, value, .. } if id == wanted => return Some(*value),
            _ => {}
        }
    }
    None
}

/// 🎚️ The value one select row currently renders, at any depth.
pub fn select_value(measures: &[WindowMeasure], wanted: &str) -> Option<String> {
    let mut stack: Vec<&WindowMeasure> = measures.iter().collect();
    while let Some(measure) = stack.pop() {
        match measure {
            WindowMeasure::Group { children, .. } => stack.extend(children.iter()),
            WindowMeasure::Select { id, value, .. } if id == wanted => return Some(value.clone()),
            _ => {}
        }
    }
    None
}

/// 🎚️ The live chrome of ONE window instance, exactly as `ArtifactApp::window_measures` builds it
/// — the read half of every "dispatch → config changes → measure reflects it" law.
pub fn window_measures_of(app: &mut Puzzle5dTestApp, window_kind: &str) -> Vec<WindowMeasure> {
    let view = window_view(window_kind, window_kind);
    ::semio_framework_async::poll::resolve_ready(PluginApp::window_measures(&mut app.0, &view)).remove(window_kind).unwrap_or_default()
}
//#endregion 🎚️Measures

pub fn projection_of(app: &Puzzle5dApp) -> Value {
    semio_framework_pack_json::from_dsl_value(app.snapshot().expect("projection").value())
}

pub fn part_count(app: &Puzzle5dApp) -> usize {
    projection_of(app).get("parts").and_then(|value| value.as_array()).map_or(0, Vec::len)
}

pub fn first_part_id(app: &Puzzle5dApp) -> String {
    projection_of(app).get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("id")).and_then(Value::as_str).expect("first part id").to_string()
}

/// 🎯️ Top-level utility tag of a `WindowMeasure::Group` by id, or `None` when the group is absent.
pub fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Group { id, active_utility_id, .. } if id == group_id => Some(active_utility_id.clone()),
        _ => None,
    })
}

/// 🔢️ Finds one `WindowMeasure::Number` leaf anywhere in a measure tree, descending into groups.
pub fn find_measure_number<'a>(measures: &'a [WindowMeasure], number_id: &str) -> Option<&'a WindowMeasure> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Number { id, .. } if id == number_id => Some(measure),
        WindowMeasure::Group { children, .. } => find_measure_number(children, number_id),
        _ => None,
    })
}
