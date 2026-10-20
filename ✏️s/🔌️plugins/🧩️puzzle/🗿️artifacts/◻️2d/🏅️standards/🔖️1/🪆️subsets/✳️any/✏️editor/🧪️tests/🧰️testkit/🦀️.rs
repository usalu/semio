//! 🧰️ Puzzle 2d editor test support: the registry-backed test app, the settle and dispatch drivers, the genesis builders and the render and measure probes every editor test drives. Compiled into the owner under `cfg(any(test, feature = "testkit"))`.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_pack_json::{json, Value};
use semio_framework_plugin::{Fault, InteractionTarget};
use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::recorder::{self, Puzzle2dRecorder};
use crate::editor::puzzle2d::snapshot::{self, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, Puzzle2dCommand, Puzzle2dInteractionSnapshot, Puzzle2dScene};
use crate::editor::puzzle2d::window;
use crate::editor::puzzle2d::{create_puzzle2d_app, Puzzle2dPlayApp};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use std::collections::BTreeSet;


/// 🧮️ Weighs what this test binary allocates, per thread (`semio_framework_trace::HeapWitness`), so a cost law
/// reads its own work and nothing a concurrent law allocates.
#[global_allocator]
static PUZZLE2D_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot::board_snapshot_nodes;
use semio_framework::Effect;

/// 🧩️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
/// scope pays the caller's terminal retirement after the app's paid close.
pub struct Puzzle2dApp(pub VcsArtifactApp<EditorApp<Puzzle2dPlayApp>>, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

impl std::ops::Deref for Puzzle2dApp {
    type Target = VcsArtifactApp<EditorApp<Puzzle2dPlayApp>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Puzzle2dApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 🧯️ Projects a fixture dispatch failure onto the diagnostic `Fault` the drivers' callers assert on: the original fault when that is the primary, else the complete debug form.
fn fault_of(failure: semio_framework_job::OriginalTurnFailure) -> Fault {
    failure.primary_fault().cloned().unwrap_or_else(|| Fault::from(format!("{failure:?}")))
}

pub fn meta(actor: &str) -> ActionMeta {
    semio_framework_plugin::artifact_app_laws::meta(actor)
}

/// ⏳️ The harness's own executor: polls one app future to completion, yielding between turns.
///
/// 🐛️ This file used to bridge every app call through `semio_framework_os_kernel::io::resolve_ready`, whose
/// contract is "an ARTIFACT-IO body must complete without a real suspension" — it PANICS on the
/// first `Pending`. That contract belongs to the synchronous `IoEntry`/compose thunks it was
/// written for, not to a harness driving a live app: the moment a dispatch gained a real await
/// point (the clipboard route's store admission) every law that went through it aborted with
/// "future was not ready on first poll". A live app's futures make progress on every poll, so the
/// harness polls — the same noop-waker spin the block3d window-transient law uses.
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

/// 🧰️ The registry-backed, instance-bound snapshot app: `bounded_first_step_tool_proofs!` joins the manifest's
/// migrated declarations to the live factories, which a registry-less app cannot satisfy.
pub fn app() -> Puzzle2dApp {
    std::sync::LazyLock::force(&crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE);
    std::sync::LazyLock::force(&crate::examples::puzzle2d::concrete_forest::SOURCE);
    app_with_registry()
}

/// 🧾️ `assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still take a `fn() ->
/// App` manifest (framework test context gap, not this packet's to fix — see the sibling `w2-cad-report`
/// "SDK gaps" §3); `create_puzzle2d_app` now returns `AppDefinition`, so this wraps it.
fn puzzle2d_manifest_for_tests() -> App {
    App { definition: create_puzzle2d_app(), examples: Vec::new() }
}


/// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the
/// utility contract are enforced exactly as in production.
pub fn app_with_registry() -> Puzzle2dApp {
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = block_on(semio_framework_plugin::artifact_app_laws::new_app_with_registry::<EditorApp<Puzzle2dPlayApp>>(puzzle2d_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()));
    block_on(app.bind_instance_id(1));
    Puzzle2dApp(app, scope)
}

pub fn window_view(kind: &str, id: &str) -> ViewModel {
    let mut window_instances = [crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID]
        .into_iter()
        .map(|kind| ViewWindowInstance { id: kind.into(), window_kind_id: kind.into() })
        .collect::<Vec<_>>();
    if !window_instances.iter().any(|window| window.id == id) {
        window_instances.push(ViewWindowInstance { id: id.into(), window_kind_id: kind.into() });
    }
    ViewModel { window_instances, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }.for_window_instance(id).expect("puzzle2d test window roster")
}

fn action_meta(args: Option<&Value>, window_id: Option<&str>) -> ActionMeta {
    let requested = args
        .and_then(|value| value.get("windowId").or_else(|| value.get("window_id")).or_else(|| value.get("pane")).or_else(|| value.get("window")))
        .and_then(Value::as_str);
    let kind = requested
        .filter(|kind| [crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID].contains(kind))
        .or_else(|| window_id.filter(|kind| [crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID].contains(kind)))
        .unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
    let id = window_id.or(requested).unwrap_or(kind);
    ActionMeta { view_state: Some(window_view(kind, id)), ..meta("local") }
}

/// 📄️ Drains everything one settle turn can have left behind — presented pages, effects, events,
/// completions (and the HISTORY PATCH they carry), the UI scope and local-interaction replies.
///
/// 🐛️ Called both inside the loop AND once more before the loop returns. `has_pending_typed_operations`
/// counts the outboxes and the mounted operations, but an operation that RETIRED during the previous
/// turn's `advance_typed_operation_publication` leaves its terminal completion queued while the
/// predicate already reads false — so the exit branch returned a result whose `history_patch` was
/// still sitting in the outbox, and every `committed_edits(&result)` read 0 for an edit that really
/// landed in the document.
///
/// 🧯️ A `Fault` page is ACKed like every other page — the host ACKs it too — and reported once the
/// operation has retired (`settle_registered_typed_operation`'s shape). Returning on it un-ACKed left the
/// faulted operation awaiting its ACK for ever, so the NEXT dispatch never settled either.
fn drain_settled(app: &mut Puzzle2dApp, result: &mut InvocationResult, fault: &mut Option<Fault>) -> Result<(), Fault> {
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
    while let Some(progress) = app.take_typed_operation_ui_progress() {
        result.ui_scope = progress.ui_scope;
    }
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            app.acknowledge_local_interaction_query(&token);
        }
    }
    Ok(())
}

fn settle(app: &mut Puzzle2dApp, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
    let mut result = result?;
    let mut fault = None;
    for _ in 0..1_048_576 {
        if !app.has_pending_typed_operations() {
            drain_settled(app, &mut result, &mut fault)?;
            return fault.map_or(Ok(result), Err);
        }
        app.maintain(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|failure| Fault::from(format!("{failure:?}")))?;
        block_on(app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).map_err(|failure| Fault::from(format!("{failure:?}")))?;
        drain_settled(app, &mut result, &mut fault)?;
    }
    Err(Fault::from("puzzle2d test operation did not settle"))
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle2dCommand` from the same
/// `(action, args, window_id)` triple every pre-B1 test already passed.
pub fn dispatch(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let action_meta = action_meta(args, window_id);
    if matches!(
        action,
        "undo"
            | "redo"
            | "commitCheckpoint"
            | "createAlternative"
            | "switchAlternative"
            | "checkoutCheckpoint"
            | "copy"
            | "cut"
            | "paste"
            | "revertToCommand"
            | "historyFilter"
            | "noteShellCommand"
            | "interactionSelect"
            | "interactionHover"
            | "clearSelection"
            | "selectAll"
            | "setSelectionMode"
            | "setInteractionGranularity"
            | "hostEvent"
    ) {
        let dsl_args = args.map(semio_framework_value::ToValue::to_value);
        let result = block_on(semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, dsl_args.clone(), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy().close)).and_then(|admitted| block_on(semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted))).map_err(fault_of);
        return settle(app, result);
    }
    let result = block_on(app.0.dispatch_typed(Puzzle2dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()));
    settle(app, result)
}

/// 🧰️ [`dispatch`] of an app verb from an explicit window view — for a law that needs what the view carries,
/// such as the window's active utility.
pub fn dispatch_in_view(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>, view_state: ViewModel) -> Result<InvocationResult, Fault> {
    let action_meta = ActionMeta { view_state: Some(view_state), ..meta("local") };
    let result = block_on(app.0.dispatch_typed(Puzzle2dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()));
    settle(app, result)
}

/// 🧾 How many DOCUMENT edits one dispatch actually committed. `InvocationResult.mutations` is
/// the INLINE carrier and the typed/retained ladder never uses it: a migrated verb commits its edits
/// inside the operation and reports them as command-log upserts, which `settle` adopts above.
///
/// 🐛️ Since 2026-09-20 (`record_typed_operation_lane`, `🧰️framework/…/🔌️plugin/🦀️.rs`) a typed
/// operation's document edit is logged under its own VERB (`addNode`, `paste`, …) and its declared
/// kind; `action_id: "apply"` is only the backfill of an edit no command claimed. Filtering on
/// `"apply"` therefore read 0 for every edit that really landed. The row that IS a document edit is
/// the one carrying printed document ops — a config-lane or no-lane row prints none — and it is
/// applied; a coalesced gesture re-upserts its ONE row, so rows are counted by `seq`.
/// Every law that means "this verb edited the document" counts these, never `mutations`.
pub fn committed_edits(result: &InvocationResult) -> usize {
    result.history_patch.as_ref().map_or(0, |patch| patch.upserts.iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).map(|entry| entry.seq).collect::<std::collections::BTreeSet<_>>().len())
}

/// 🛍️ One dispatch is the whole switch: `setActiveExample` answers one `Effect::LoadDocument` (no mutation row, no history
/// row), which this helper loads into the app as the host does. Returns the number of document edits the switch
/// committed — always zero.
pub fn load_example(app: &mut Puzzle2dApp, example_id: &str) -> usize {
    let result = dispatch(app, "setActiveExample", Some(&json!({ "exampleId": example_id })), None).expect("load example");
    let mut loaded = 0;
    for effect in &result.requested_effects {
        if let Effect::LoadDocument { pack, spr } = effect {
            block_on(semio_framework_plugin::app::artifact_app_laws::load_document(&mut app.0, &store::os_store::serialization::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("the example loads");
            loaded += 1;
        }
    }
    assert_eq!(loaded, 1, "an example switch is exactly one whole-document load");
    committed_edits(&result)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
/// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
/// deleted `setSelection` action.
pub fn select_id(app: &mut Puzzle2dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = semio_framework_pack_json::to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
}

pub fn concrete_forest_app() -> Puzzle2dApp {
    let mut app = app();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    app
}

/// 🪟️ The window kind a canvas body paints, `None` for a panel body.
fn body_window_kind(body_key: &str) -> Option<&'static str> {
    match body_key {
        overview::BODY_KEY => Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID),
        detail::BODY_KEY => Some(crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID),
        selection::BODY_KEY => Some(crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID),
        _ => None,
    }
}

/// 🖼️ The rendered body, serialized — every panel/window assertion greps this string. A canvas body
/// renders its kind's default instance (`id == kind`), the very window `action_meta` addresses when a
/// law names none: per-window state (camera, grid, gumball flags) is captured from the RENDERED
/// window's own partition (`window_config_store.capture(Some(view_state))`), so a render with no
/// window reads the empty default and never the window the law just published into.
pub fn render_body(app: &mut Puzzle2dApp, body_key: &str) -> String {
    let view = body_window_kind(body_key).map_or_else(|| ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), |kind| window_view(kind, kind));
    render_body_with_view(app, body_key, &view)
}

pub fn render_body_with_view(app: &mut Puzzle2dApp, body_key: &str, view_state: &ViewModel) -> String {
    let scene = block_on(semio_framework_plugin::artifact_app_laws::observe_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx(), |root| {
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                if surface.doc_schema.as_str() == <semio_framework_ui_scene::Board2dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                    let scene: semio_framework_ui_scene::Board2dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(node).expect("decode board scene with lanes");
                    return Some(semio_framework_pack_json::to_json_string(&json!({ "schema": surface.doc_schema, "board2d": scene })));
                }
            }
            stack.extend(node.children.iter());
        }
        None
    }))
    .expect("render");
    scene.unwrap_or_else(|| block_on(semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx())).expect("render projection"))
}

/// 🪟️ `body_key` rendered as the window instance `window_id`. The instance travels in the view state
/// the framework captures that window's partitions from; a `"{body}:{window}"` key is no body at all
/// and rendered `Unknown body`.
pub fn render_window(app: &mut Puzzle2dApp, body_key: &str, window_id: &str) -> String {
    let kind = body_window_kind(body_key).expect("render_window renders a canvas body");
    render_body_with_view(app, body_key, &window_view(kind, window_id))
}

/// 🧹️ Closes `app` to terminal-empty ownership; `#[track_caller]` so a refusal names WHICH app of a
/// two-app law did not get there.
#[track_caller]
pub fn close_app(app: &mut Puzzle2dApp) {
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), app.1.cx()).expect("Puzzle 2D registered app close");
    assert!(app.close_terminal_is_empty(), "Puzzle 2D registered app close did not reach terminal-empty ownership");
}

/// 🧾️ A standalone `Puzzle2dScene` for the measure/engagement builders that take one directly.
pub fn scene(snapshot: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str) -> Puzzle2dScene {
    Puzzle2dScene { board_snapshot: snapshot, runtime, active_utility: active_utility.into(), interaction: Puzzle2dInteractionSnapshot::default() }
}

/// 📼️ A recorder over the typed document a host fixture states.
pub fn recorder_of(snapshot: Value) -> Puzzle2dRecorder {
    let typed: crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&snapshot)).expect("typed fixture admits");
    Puzzle2dRecorder::from_typed(std::sync::Arc::new(typed))
}

/// 🌱️ The concrete kinds that build `board` from an empty document — manifest, kind relations, catalogs, a `create-node` per
/// node, a `connect-handles` per edge (with its flags) and a `create-target-region` per region. A fixture's
/// authoring script, recorded through the same recorder the editor uses; nothing here differences two boards.
pub fn board_genesis_kinds(board: &crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::{change_manifest_id, connect_kind_compatibility, create_node, create_target_region, replace_kind_catalogs};
    let mut recorder = Puzzle2dRecorder::from_typed(std::sync::Arc::new(crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot::default()));
    if board.meta.manifest_id.is_some() {
        recorder.record(change_manifest_id(board.meta.manifest_id.clone()));
    }
    for row in board.meta.kind_compatibility.iter() {
        recorder.record(connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, None));
    }
    if board.meta.kind_catalogs.is_some() {
        recorder.record(replace_kind_catalogs(board.meta.kind_catalogs.clone()));
    }
    for node in board.nodes.iter() {
        recorder.record(create_node(node.clone(), None));
    }
    for edge in board.edges.iter() {
        recorder.connect_edge(edge.clone());
    }
    for region in board.target_regions.iter() {
        recorder.record(create_target_region(region.clone(), None));
    }
    recorder.into_rows()
}

/// 🧱️ Seeds `board` into a fresh app as the concrete kinds that build it, ingested as one authored operation log.
pub fn seed_board(app: &mut Puzzle2dApp, board: &Value) {
    use protocol::OpText as _;
    let typed: crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(board)).expect("the seed board admits");
    let text = board_genesis_kinds(&typed).iter().map(|kind| kind.print_op()).collect::<Vec<_>>().join("\n");
    block_on(app.ingest_operations_text(&text, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).expect("seed the board");
}

/// 📥️ Dispatches `importSnapshot` for `board` and loads the whole document it requests, as the host does with an
/// `Effect::LoadDocument`.
pub fn import_board(app: &mut Puzzle2dApp, board: &Value) {
    let result = dispatch(app, "importSnapshot", Some(&json!({ "payload": board })), None).expect("import the board");
    assert!(result.mutations.is_empty(), "an import states no mutation: {:?}", result.mutations);
    let mut loaded = 0;
    for effect in result.requested_effects {
        if let Effect::LoadDocument { pack, spr } = effect {
            block_on(semio_framework_plugin::app::artifact_app_laws::load_document(&mut app.0, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("the import loads");
            loaded += 1;
        }
    }
    assert_eq!(loaded, 1, "an import is exactly one whole-document load");
}

pub fn fixture_of(app: &Puzzle2dApp) -> Value {
    app.snapshot().expect("projection").value().clone()
}

pub fn first_node_id(app: &Puzzle2dApp) -> String {
    board_snapshot_nodes(&fixture_of(app))[0].get("id").and_then(|value| value.as_str()).expect("node id").to_string()
}


impl Puzzle2dApp {
    /// 🧼️ One maintenance opportunity keeps its original caller and exact five-currency quote.
    pub fn maintain(&mut self, body: usize) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        let demand = self.0.maintenance_retirement_demands(body)?;
        let grant = semio_s_plugin_puzzle_jobs::testing::funded_turn(demand, 1);
        let app = &mut self.0;
        self.1.cx().with_retained_loan(grant, |cx| {
            let loan = cx.retained_grant();
            app.maintenance_step(cx, loan).map_err(semio_framework_job::OriginalTurnFailure::from)
        }).map_err(semio_framework_plugin::PluginLifecycleFailure::from)
    }
}

impl Drop for Puzzle2dApp {
    fn drop(&mut self) { if !std::thread::panicking() { close_app(self); } }
}

pub fn set_at(root: &mut semio_framework_pack_json::Value, pointer: &str, replacement: semio_framework_pack_json::Value) {
        let mut segments = pointer.split('/').filter(|segment| !segment.is_empty()).peekable();
        let mut current = root;
        while let Some(segment) = segments.next() {
            let last = segments.peek().is_none();
            current = match current {
                semio_framework_pack_json::Value::Array(items) => {
                    let index: usize = segment.parse().expect("an array position");
                    if last {
                        items[index] = replacement;
                        return;
                    }
                    &mut items[index]
                }
                semio_framework_pack_json::Value::Object(entries) => {
                    if last {
                        entries.insert(segment, replacement);
                        return;
                    }
                    entries.get_mut(segment).expect("an object member")
                }
                other => panic!("cannot descend into {other:?} at {segment}"),
            };
        }
        *current = replacement;
    }
