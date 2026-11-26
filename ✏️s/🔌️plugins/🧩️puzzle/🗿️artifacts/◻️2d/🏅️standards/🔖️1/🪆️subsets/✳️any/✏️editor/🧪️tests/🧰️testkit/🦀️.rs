//! 🧰️ Puzzle 2d editor test support: the registry-backed test app, the settle and dispatch drivers, the genesis builders and the render and measure probes every editor test drives. Compiled into the owner under `cfg(any(test, feature = "testkit"))`.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use semio_framework_pack_json::{json, Value};
use semio_plugin::{Fault, InteractionTarget};
use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::recorder::{self, Puzzle2dRecorder};
use crate::editor::puzzle2d::snapshot::{self, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, Puzzle2dInteractionSnapshot, Puzzle2dScene};
use crate::editor::puzzle2d::window;
use crate::editor::puzzle2d::{create_puzzle2d_app, Puzzle2dPlayApp};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use std::collections::BTreeSet;


use semio_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot::board_snapshot_nodes;
use semio_framework::Effect;

/// 🧩️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
/// scope pays the caller's terminal retirement after the app's paid close.
pub struct Puzzle2dApp(pub VcsArtifactApp<EditorApp<Puzzle2dPlayApp>>, pub semio_plugin::artifact_app_laws::OriginalFixtureScope);

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

/// 🔌️ Attaches the real actor port and keeps the host's endpoint for fixture delivery.
pub async fn attach_fixture_actor_backbone(
    app: &mut Puzzle2dApp,
    uri: &str,
) -> store::os_store::backbone::ActorBackboneChannelOwner {
    use store::os_store::backbone::{ActorBackboneChannelOwner, Backbones, PortBackbone};
    let (channel, owner) = ActorBackboneChannelOwner::pair(uri);
    app.attach_hot_backbone(Backbones::Port(PortBackbone::with_channel(uri, channel).await))
        .await.expect("attach the original actor backbone");
    owner
}

/// 📤️ Drives the mounted author and transfers every paid original actor frame to its host.
pub fn drain_fixture_actor_outbound(
    app: &mut Puzzle2dApp,
    owner: &store::os_store::backbone::ActorBackboneChannelOwner,
) -> Vec<Vec<u8>> {
    let mut delivery = None;
    let mut frames = Vec::new();
    for _ in 0..4096 {
        while let Some(frame) = owner.take_outbound().expect("take the paid actor frame") {
            frames.push(frame);
        }
        if delivery.is_none() && !app.0.original_atomic_outbound_pending() {
            return frames;
        }
        let cx = app.1.cx();
        let grant = cx.original_turn_ceiling().0;
        semio_plugin::artifact_app_laws::fixture_renew_turn(cx, semio_plugin::artifact_app_laws::OriginalFixtureCaller::turn_fuel(), grant)
            .expect("renew the actor output turn");
        app.0.advance_original_atomic_outbound(&mut delivery, [1; 32], cx).expect("drive the original actor outbox");
    }
    panic!("the original actor outbox never settled")
}

/// 👥️ Verifies the mounted board, camera publication and peer departure through their original transports.
pub async fn assert_camera_presence_round_trip() {
    use semio_framework_value::FromValue;
    use semio_plugin::artifact_app_laws::{fixture_mounted_policy, publish_fixture_presence_roster};
    use semio_plugin::PluginApp;
    let fixture = semio_framework_pack_json::parse(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/📷️camera/🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("portable shared presence fixture");
    let expected = protocol::PresenceWindowView::from_value(semio_framework_pack_json::to_dsl_value(&fixture["admission"][0]["value"])).expect("admitted shared camera");
    let protocol::PresenceViewKind::Canvas { x, y, zoom } = expected.kind else { panic!("portable canvas pose") };
    let actor = fixture["isolation"]["actorA"].as_str().expect("source actor");
    let mut source = app_for_actor(actor);
    let mut peer = app_for_actor(fixture["isolation"]["actorB"].as_str().expect("peer actor"));
    let channel = attach_fixture_actor_backbone(&mut source, "actor://puzzle2d-presence").await;
    dispatch(&mut source, "addNode", Some(&json!({"kind": "node"})), Some(&expected.window_id)).expect("source publishes a node");
    let before = source.document_pack().await.expect("source document before presence");
    dispatch(&mut source, "setCamera", Some(&json!({"camera": {"x": x, "y": y, "zoom": zoom}})), Some(&expected.window_id)).expect("shared camera publication");
    assert_eq!(source.document_pack().await.expect("source document after presence"), before);
    let camera = rendered_camera(&render_window(&mut source, overview::BODY_KEY, &expected.window_id));
    assert_eq!(camera, (x, y, zoom));
    let selected = first_node_id(&source);
    select_id(&mut source, snapshot::PUZZLE2D_GRANULARITY_NODE, &selected).expect("select the shared node");
    let lane = source.ephemeral_snapshot().await;
    let interaction = protocol::decode_presence_interaction(&lane.interaction, &mut 0).await.expect("decode the editor's selected node");
    assert!(interaction.domains.iter().any(|row| row.domain == PUZZLE2D_INTERACTION_DOMAIN && row.selected.contains(&selected)));
    let remote = protocol::PresencePeer {
        actor: actor.into(), connected_at_ms: 1, label: None, user_id: None, role: None, drag_ghost_json: None,
        presence_pack: Some(lane.presence), interaction: Some(interaction), color: Some(1), surface: None,
        views: vec![protocol::PresenceWindowView {
            kind: protocol::PresenceViewKind::Canvas { x: camera.0, y: camera.1, zoom: camera.2 },
            ..expected.clone()
        }], ui: None, tool_run: lane.tool_run, principal_kind: None, active_tool: None, history_edit: lane.history_edit, typing: Vec::new(),
    };
    let bytes = protocol::encode_presence_peer(&remote).await;
    let arrived = protocol::decode_presence_peer(&bytes).await.expect("decode the published peer");
    assert_eq!(arrived.views, vec![expected]);
    let peer_before = peer.document_pack().await.expect("peer document before presence");
    let grant = fixture_mounted_policy().maintenance;
    assert!(publish_fixture_presence_roster(&mut peer.0, 1, Some(9), std::slice::from_ref(&arrived), 1000, peer.1.cx(), grant).await.is_none());
    {
        let roster = peer.roster.peer_presence_root();
        let (remote_actor, presence) = roster.iter().next().expect("the source reached the peer");
        assert_eq!(remote_actor, actor);
        assert_eq!(presence.interaction, remote.interaction);
    }
    assert!(publish_fixture_presence_roster(&mut peer.0, 2, Some(9), &[], 2000, peer.1.cx(), grant).await.is_none());
    assert_eq!(peer.roster.peer_presence_root().len, 0);
    assert_eq!(peer.document_pack().await.expect("peer document after departure"), peer_before);
    assert_eq!(source.document_pack().await.expect("source document after selection"), before);
    let frames = drain_fixture_actor_outbound(&mut source, &channel);
    assert_eq!(ingest_fixture_actor_frames(&mut peer, &frames).await, 1);
    assert_eq!(fixture_of(&source), fixture_of(&peer), "original source events converge after ephemeral interaction");
    channel.begin_retire().expect("retire the original actor port");
    source.detach_backbone().await.expect("detach the actor port");
    close_app(&mut peer);
    close_app(&mut source);
}

/// 📥️ Delivers the exact ordinary actor messages through the mounted causal operation ingress.
pub async fn ingest_fixture_actor_frames(app: &mut Puzzle2dApp, frames: &[Vec<u8>]) -> usize {
    use store::os_store::backbone::{BackboneMessage, decode_hot_backbone_message_exact};
    let mut reports = 0;
    for frame in frames {
        let BackboneMessage::Mutations { envelopes } = decode_hot_backbone_message_exact(frame).expect("decode the original actor message") else {
            panic!("the actor carries a mutation batch");
        };
        let cx = app.1.cx();
        let grant = cx.original_turn_ceiling().0;
        semio_plugin::artifact_app_laws::fixture_renew_turn(cx, semio_plugin::artifact_app_laws::OriginalFixtureCaller::turn_fuel(), grant)
            .expect("renew the actor input turn");
        reports += app.0.ingest_operations(&envelopes.to_vec_owner(), &mut semio_plugin::artifact_app_laws::fixture_identity(), cx)
            .await.expect("ingest the authored actor mutations").len();
    }
    reports
}

/// 🧯️ Projects a fixture dispatch failure onto the diagnostic `Fault` the drivers' callers assert on: the original fault when that is the primary, else the complete debug form.
fn fault_of(failure: semio_framework_job::OriginalTurnFailure) -> Fault {
    failure.primary_fault().cloned().unwrap_or_else(|| Fault::from(format!("{failure:?}")))
}

pub fn meta(actor: &str) -> ActionMeta {
    semio_plugin::artifact_app_laws::meta(actor)
}

/// ⏳️ The harness's own executor: polls one app future to completion, yielding between turns.
///
/// 🐛️ This file used to bridge every app call through `semio_os_kernel::io::resolve_ready`, whose
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
pub fn puzzle2d_manifest_for_tests() -> App {
    App { definition: create_puzzle2d_app(), examples: Vec::new() }
}


/// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the
/// utility contract are enforced exactly as in production.
pub fn app_with_registry() -> Puzzle2dApp {
    app_for_actor(semio_os_kernel::LOCAL_ACTOR_ID)
}

/// 👤️ Opens the registered overview for the specified document author.
pub fn app_for_actor(actor: &str) -> Puzzle2dApp {
    let scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = block_on(semio_plugin::artifact_app_laws::new_app_with_registry::<EditorApp<Puzzle2dPlayApp>>(puzzle2d_manifest_for_tests, semio_os_kernel::ActorId(actor.into()), semio_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin::artifact_app_laws::fixture_identity()));
    block_on(app.bind_instance_id(1));
    let overview = crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID;
    app.window_transient_generation(&window_view(overview, overview))
        .expect("open the registered overview window")
        .expect("the overview owns window transients");
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
    ViewModel { window_instances, ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) }.for_window_instance(id).expect("puzzle2d test window roster")
}

fn action_meta(app: &mut Puzzle2dApp, args: Option<&Value>, window_id: Option<&str>) -> Result<ActionMeta, Fault> {
    let requested = args
        .and_then(|value| value.get("windowId").or_else(|| value.get("window_id")).or_else(|| value.get("pane")).or_else(|| value.get("window")))
        .and_then(Value::as_str);
    let kind = requested
        .filter(|kind| [crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID].contains(kind))
        .or_else(|| window_id.filter(|kind| [crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID].contains(kind)))
        .unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
    let id = window_id.or(requested).unwrap_or(kind);
    let view = window_view(kind, id);
    let _ = app.window_transient_generation(&view)?;
    Ok(ActionMeta { view_state: Some(std::sync::Arc::new(view)), ..meta(app.stores.store.local_actor_id().0.as_str()) })
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
    use semio_plugin::artifact_app_laws;

    let policy = artifact_app_laws::fixture_mounted_policy();
    while let Some(mut page) = artifact_app_laws::take_fixture_result_page(&mut app.0, 1, policy.preparation)? {
        if page.lane == semio_plugin::app::TypedOperationResultLane::Fault {
            fault.get_or_insert_with(|| Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
        }
        let token = artifact_app_laws::close_fixture_result_page(&mut page, policy.close);
        if !app.acknowledge_typed_operation_result(token)? {
            return Err(Fault::from("registered Puzzle 2D fixture rejected its exact result acknowledgment"));
        }
    }
    result.requested_effects.extend(app.take_typed_operation_effect());
    result.events.extend(app.take_typed_operation_event());
    while let Some(completion) = block_on(app.take_typed_operation_completion())? {
        result.ui_scope = completion.ui_scope;
        if let Some(mut patch) = completion.history_patch {
            if let Some(previous) = result.history_patch.take() {
                let replaced: std::collections::HashSet<_> = patch.upserts.iter().map(|row| row.seq).collect();
                patch.upserts.extend(previous.upserts.into_iter().filter(|row| !replaced.contains(&row.seq)));
                patch.upserts.sort_by_key(|row| std::cmp::Reverse(row.seq));
            }
            result.history_patch = Some(patch);
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

/// 🧾️ Pays one mounted driver turn and returns its ownership receipts.
pub fn drive_history_turn(app: &mut Puzzle2dApp, what: &str) {
    use semio_plugin::artifact_app_laws;

    let policy = artifact_app_laws::fixture_mounted_policy();
    let fuel = artifact_app_laws::OriginalFixtureCaller::turn_fuel();
    let grant = app.1.cx().original_turn_ceiling().0;
    artifact_app_laws::fixture_renew_turn(app.1.cx(), fuel, grant).unwrap_or_else(|fault| panic!("{what}: {fault:?}"));
    app.maintain(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).unwrap_or_else(|fault| panic!("{what}: {fault:?}"));
    artifact_app_laws::fixture_renew_turn(app.1.cx(), fuel, grant).unwrap_or_else(|fault| panic!("{what}: {fault:?}"));
    block_on(app.0.advance_typed_operation_publication(&mut artifact_app_laws::fixture_identity(), app.1.cx())).unwrap_or_else(|fault| panic!("{what}: {fault:?}"));
    artifact_app_laws::retain_original_ownership_receipts(&mut app.0, policy, &mut Vec::new()).unwrap_or_else(|fault| panic!("{what}: {fault:?}"));
}

fn settle(app: &mut Puzzle2dApp, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
    use semio_plugin::artifact_app_laws;

    let mut result = result?;
    let mut fault = None;
    let policy = artifact_app_laws::fixture_mounted_policy();
    let fuel = artifact_app_laws::OriginalFixtureCaller::turn_fuel();
    let (grant, _) = app.1.cx().original_turn_ceiling();
    let mut receipts = Vec::new();
    for _ in 0..1_048_576 {
        artifact_app_laws::retain_original_ownership_receipts(&mut app.0, policy, &mut receipts)?;
        receipts.clear();
        if !app.has_pending_typed_operations() && !app.0.render_ref().tool_overlay_retirement_pending() {
            drain_settled(app, &mut result, &mut fault)?;
            return fault.map_or(Ok(result), Err);
        }
        artifact_app_laws::fixture_renew_turn(app.1.cx(), fuel, grant).map_err(|failure| Fault::from(format!("{failure:?}")))?;
        artifact_app_laws::ensure_fixture_cancellation_issuer(&mut app.0, app.1.cx()).map_err(|failure| Fault::from(format!("{failure:?}")))?;
        let loan = app.1.cx().retained_grant();
        app.0.maintenance_step(app.1.cx(), loan).map_err(|failure| Fault::from(format!("{failure:?}")))?;
        artifact_app_laws::fixture_renew_turn(app.1.cx(), fuel, grant).map_err(|failure| Fault::from(format!("{failure:?}")))?;
        block_on(app.0.advance_typed_operation_publication(&mut artifact_app_laws::fixture_identity(), app.1.cx())).map_err(|failure| Fault::from(format!("{failure:?}")))?;
        artifact_app_laws::retain_original_ownership_receipts(&mut app.0, policy, &mut receipts)?;
        drain_settled(app, &mut result, &mut fault)?;
    }
    Err(Fault::from(format!("Puzzle 2D operation did not settle: {}; reserved phase {:?}", app.0.typed_operation_diagnostic(), app.0.media.reserved_mount.as_ref().map(|owner| owner.phase))))
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle2dCommand` from the same
/// `(action, args, window_id)` triple every pre-B1 test already passed.
pub fn dispatch(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let action_meta = action_meta(app, args, window_id)?;
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
        let result = block_on(semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, dsl_args.clone(), &action_meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::artifact_app_laws::fixture_mounted_policy().close)).and_then(|admitted| block_on(semio_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted, app.1.cx()))).map_err(fault_of);
        return settle(app, result);
    }
    let result = block_on(semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, action_args(args, window_id), &action_meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::artifact_app_laws::fixture_mounted_policy().close)).map_err(fault_of);
    settle(app, result)
}

/// 🧾️ The addressed window rides in the action arguments, exactly as the shell sends it.
fn action_args(args: Option<&Value>, window_id: Option<&str>) -> Option<semio_framework_value::DslValue> {
    let mut value = args.cloned();
    if let Some(window_id) = window_id {
        if let Value::Object(object) = value.get_or_insert_with(|| Value::Object(Default::default())) {
            object.insert("windowId", Value::String(window_id.into()));
        }
    }
    value.as_ref().map(semio_framework_value::ToValue::to_value)
}

/// 🧰️ [`dispatch`] of an app verb from an explicit window view — for a law that needs what the view carries,
/// such as the window's active utility.
pub fn dispatch_in_view(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>, view_state: ViewModel) -> Result<InvocationResult, Fault> {
    let _ = app.window_transient_generation(&view_state)?;
    let action_meta = ActionMeta { view_state: Some(std::sync::Arc::new(view_state)), ..meta(app.stores.store.local_actor_id().0.as_str()) };
    let result = block_on(semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, action_args(args, window_id), &action_meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::artifact_app_laws::fixture_mounted_policy().close)).map_err(fault_of);
    settle(app, result)
}

/// 🧾️ Returns the rows published for document edits appended by this command.
pub fn dispatch_new_edit_rows(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let before = app.stores.store.envelope().vcs.edits.len();
    let mut result = dispatch(app, action, args, window_id)?;
    let edits = &app.stores.store.envelope().vcs.edits;
    let added: std::collections::HashSet<_> = edits.iter().skip(before).map(|edit| edit.id.as_str()).collect();
    if let Some(patch) = result.history_patch.as_mut() {
        patch.upserts.retain(|row| row.edit_id.as_deref().is_some_and(|id| added.contains(id)));
    }
    assert_eq!(committed_edits(&result), added.len(), "every appended edit publishes its history row");
    Ok(result)
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
            block_on(semio_plugin::app::artifact_app_laws::load_document(&mut app.0, &store::os_store::serialization::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() }, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("the example loads");
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
    let view = body_window_kind(body_key).map_or_else(|| ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native), |kind| window_view(kind, kind));
    render_body_with_view(app, body_key, &view)
}

pub fn render_body_with_view(app: &mut Puzzle2dApp, body_key: &str, view_state: &ViewModel) -> String {
    let scene = block_on(semio_plugin::artifact_app_laws::observe_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx(), |root| {
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let semio_ui_contract::Component::Surface(surface) = &node.component {
                if surface.doc_schema.as_str() == <semio_ui_scene::Board2dScene as semio_ui_scene::SceneDoc>::SCHEMA {
                    let scene: semio_ui_scene::Board2dScene = semio_plugin::artifact_app_laws::built_surface_scene(node).expect("decode board scene with lanes");
                    return Some(semio_framework_pack_json::to_json_string(&json!({ "schema": surface.doc_schema, "board2d": scene })));
                }
            }
            stack.extend(node.children.iter());
        }
        None
    }))
    .expect("render");
    scene.unwrap_or_else(|| block_on(semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx())).expect("render projection"))
}

/// 🪟️ `body_key` rendered as the window instance `window_id`. The instance travels in the view state
/// the framework captures that window's partitions from; a `"{body}:{window}"` key is no body at all
/// 🎥️ Recovers the rendered pane camera `(x, y, zoom)` from a rendered `UiNode`'s embedded
/// `Board2dScene.cameraJson` — the only externally observable surface for the runtime camera
/// (the camera is never a document field, so it cannot be read back off `app.snapshot()`).
pub fn rendered_camera(rendered: &str) -> (f64, f64, f64) {
    fn find_camera_json(value: &Value) -> Option<String> {
        if let Some(json) = value.get("cameraJson").and_then(Value::as_str) {
            return Some(json.to_string());
        }
        match value {
            Value::Object(map) => map.iter().find_map(|(_, value)| find_camera_json(value)),
            Value::Array(items) => items.iter().find_map(find_camera_json),
            _ => None,
        }
    }
    let value: Value = semio_framework_pack_json::from_json_str(rendered, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rendered node parses");
    let camera_json = find_camera_json(&value).expect("rendered scene must carry cameraJson");
    let camera: Value = semio_framework_pack_json::from_json_str(&camera_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("cameraJson parses");
    (camera.get("x").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("y").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("zoom").and_then(Value::as_f64).unwrap_or(f64::NAN))
}

/// and rendered `Unknown body`.
pub fn render_window(app: &mut Puzzle2dApp, body_key: &str, window_id: &str) -> String {
    let kind = body_window_kind(body_key).expect("render_window renders a canvas body");
    render_body_with_view(app, body_key, &window_view(kind, window_id))
}

/// 🧹️ Closes `app` to terminal-empty ownership; `#[track_caller]` so a refusal names WHICH app of a
/// two-app law did not get there.
#[track_caller]
pub fn close_app(app: &mut Puzzle2dApp) {
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::artifact_app_laws::fixture_mounted_policy(), app.1.cx()).expect("Puzzle 2D registered app close");
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
    block_on(app.ingest_operations_text(&text, &mut semio_plugin::artifact_app_laws::fixture_identity())).expect("seed the board");
}

/// 📥️ Dispatches `importSnapshot` for `board` and loads the whole document it requests, as the host does with an
/// `Effect::LoadDocument`.
pub fn import_board(app: &mut Puzzle2dApp, board: &Value) {
    let result = dispatch(app, "importSnapshot", Some(&json!({ "payload": board })), None).expect("import the board");
    assert!(result.mutations.is_empty(), "an import states no mutation: {:?}", result.mutations);
    let mut loaded = 0;
    for effect in result.requested_effects {
        if let Effect::LoadDocument { pack, spr } = effect {
            block_on(semio_plugin::app::artifact_app_laws::load_document(&mut app.0, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("the import loads");
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
    pub fn maintain(&mut self, body: usize) -> Result<semio_plugin::PluginLifecycleStep, semio_plugin::PluginLifecycleFailure> {
        let demand = self.0.maintenance_retirement_demands(body, semio_plugin::artifact_app_laws::FIXTURE_EFFECT_WORK)?;
        let grant = semio_s_plugin_puzzle_jobs::testing::funded_turn(demand, 1);
        let app = &mut self.0;
        self.1.cx().with_retained_loan(grant, |cx| {
            let loan = cx.retained_grant();
            app.maintenance_step(cx, loan).map_err(semio_framework_job::OriginalTurnFailure::from)
        }).map_err(semio_plugin::PluginLifecycleFailure::from)
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
