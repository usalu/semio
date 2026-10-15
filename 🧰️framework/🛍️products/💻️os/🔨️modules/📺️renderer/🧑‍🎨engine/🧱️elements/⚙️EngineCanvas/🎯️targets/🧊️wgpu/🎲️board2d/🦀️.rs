//! 🎲️ Board2d host, event queue and pointer settle.
//!
//! Board2d host accessors, the coalesced event queue, pointer plans and settle, and the puzzle board entry points.

use super::engine_surface_attach::engine_camera_from_json;
#[cfg(test)]
use super::engine_surface_attach::sync_board_engine;
#[cfg(test)]
use super::engine_surface_registry::{ensure_engine_surface, BoardSyncCache};
use super::engine_surface_registry::{EngineSurface, ENGINE_SURFACES};
use super::paint2d::write_interaction_hover;
use super::tiled_map::board_local_pointer;
#[cfg(test)]
use super::with_engine_close_context;
#[cfg(test)]
use semio_framework_canvas as canvas;
use serde_json::{json, Value};
use std::collections::HashMap;
#[cfg(test)]
use std::mem::ManuallyDrop;
use ui_wgpu::wgpu::{ActionDescriptor, KeyAction, PointerModifiers, Rect};

//#region Board2d
/// 📬️ One coalesced board batch: the board rows for `applyBoardEvents`, the latest camera for the view lane (`setCamera`)
/// and whether the buffer flushes now.
pub struct CoalescedBoardEvents {
    pub flush_now: bool,
    pub events_json: String,
    pub camera: Option<[f64; 3]>,
}

impl CoalescedBoardEvents {
    /// 📬️ Whether any board row remains for `applyBoardEvents`.
    pub fn has_rows(&self) -> bool {
        self.events_json != "[]"
    }
}

pub fn with_board_host_mut<R>(surface_id: &str, f: impl FnOnce(&mut infinite_canvas::BoardHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        let host = entry.board_host.as_mut()?;
        Some(f(host))
    })
}

/// 👕️ One board surface's presence state: the scene camera its owning guest last published and the
/// board's active utility — exactly what React's `Board2dHost` stamps on its presence view
/// (`parseBoardCamera(sceneRef.current.cameraJson) ?? { x: 0, y: 0, zoom: 1 }`, `activeUtility`): a scene
/// that published no readable camera is the identity camera. `None` only for a surface that is not attached.
pub fn board2d_presence_state(host_id: &str) -> Option<((f64, f64, f64), Option<String>)> {
    ENGINE_SURFACES.with(|cell| {
        let registry = cell.borrow();
        let cache = &registry.get(host_id)?.board_sync_cache;
        Some((cache.camera_json.as_deref().and_then(engine_camera_from_json).unwrap_or((0.0, 0.0, 1.0)), cache.active_utility.clone()))
    })
}

pub fn with_board_host<R>(surface_id: &str, f: impl FnOnce(&infinite_canvas::BoardHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        let host = entry.board_host.as_ref()?;
        Some(f(host))
    })
}

/// 🤏️ Reads one board camera for shared touch math.
pub fn puzzle_board_camera(surface_id: &str) -> Option<[f64; 3]> {
    with_board_host(surface_id, infinite_canvas::BoardHost::camera)
}

/// 🤏️ Applies an ephemeral board camera during a pinch without enqueueing a camera event.
pub fn puzzle_board_set_camera_silent(surface_id: &str, camera: [f64; 3]) -> bool {
    with_board_host_mut(surface_id, |host| host.set_camera_silent(camera[0], camera[1], camera[2])).is_some()
}

/// 🏁️ Publishes the settled board camera once when the last pinch contact lifts.
pub fn puzzle_board_publish_camera_into(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(camera) = puzzle_board_camera(surface_id) else { return Ok(false) };
    let mut reservation = input.reserve_actions(1, ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    write_board_camera_flat(&mut reservation, controller_id, camera)?;
    reservation.publish()?;
    Ok(true)
}

/// 🔀️ Ends the board's single-pointer lane exactly once when a second touch acquires the surface — after the lane's
/// pending commit reached its terminal ([`settle_board_pointer_authority_into`]).
pub fn puzzle_board_yield_to_pinch_into(surface_id: &str, sx: f64, sy: f64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    settle_board_pointer_authority_into(surface_id, input)?;
    Ok(with_board_host_mut(surface_id, |host| {
        host.cancel_area_select();
        host.pointer_up_screen(sx, sy, false, false, false);
    })
    .is_some())
}

/// 🎯️ Most-specific pick target at a screen point, mirroring `pickMostSpecificCanvasTarget`.
pub fn board_pick_best_target_id(surface_id: &str, sx: f64, sy: f64) -> Option<String> {
    with_board_host(surface_id, |host| {
        let json = host.pick_targets_at_screen_json(sx, sy);
        let targets: Vec<Value> = serde_json::from_str(&json).unwrap_or_default();
        targets.into_iter().max_by_key(|t| t.get("generality").and_then(Value::as_u64).unwrap_or(0)).and_then(|t| t.get("id").and_then(|v| v.as_str()).map(str::to_string))
    })
    .flatten()
}

/// 📬️ The rows that never reach `applyBoardEvents` — the twin of React's `PUZZLE2D_TRANSIENT_EVENT_NAMES`
/// (`🖥️Board2dHost/🟦️.tsx`): live-preview frames (`nodeMove`, `transformPreview`) a gesture's ONE record
/// supersedes, link/brush/marquee chrome, and `hover`, which rides its own `interactionHover` lane
/// (see [`puzzle_board_hover_into`]).
fn board_event_transient(kind: infinite_canvas::BoardEventKind) -> bool {
    use infinite_canvas::BoardEventKind;
    matches!(kind, BoardEventKind::NodeMove | BoardEventKind::TransformPreview | BoardEventKind::Preselect | BoardEventKind::BrushPreview | BoardEventKind::LinkCompatibleNodes | BoardEventKind::LinkTargetRing | BoardEventKind::Hover)
}

/// 📬️ The terminal rows that flush the buffer at once — the twin of React's `PUZZLE2D_FLUSH_NOW_EVENT_NAMES`.
fn board_event_flush_now(kind: infinite_canvas::BoardEventKind) -> bool {
    use infinite_canvas::BoardEventKind;
    matches!(
        kind,
        BoardEventKind::Gesture
            | BoardEventKind::Select
            | BoardEventKind::PreselectCancel
            | BoardEventKind::BrushCandidates
            | BoardEventKind::BrushPlace
            | BoardEventKind::EdgeCreate
            | BoardEventKind::EdgeDelete
            | BoardEventKind::NodeDelete
            | BoardEventKind::RegionCreate
            | BoardEventKind::RegionResize
    )
}

/// 🪪️ The `gestureId` a `select` or `gesture` row carries, `None` for an untagged row.
fn board_event_gesture_id(event: &infinite_canvas::BoardOwnedEvent) -> Option<String> {
    serde_json::from_str::<Value>(event.payload_json()).ok()?.get("gestureId")?.as_str().map(str::to_string)
}

/// 🎯️ Classifies every entity id a board snapshot carries into the granularity a pick or hover
/// reports it under — the Rust twin of React's `board2dGranularityById`
/// (`🖥️Board2dHost/🟦️.tsx:190`), including its rule that an id the document does not carry reads as
/// a `node` rather than being dropped. A refused snapshot classifies nothing.
pub(super) fn board2d_granularity_by_id(snapshot_json: &str) -> HashMap<String, String> {
    let mut by_id = HashMap::new();
    let Ok(snapshot) = serde_json::from_str::<Value>(snapshot_json) else { return by_id };
    for node in snapshot.get("nodes").and_then(Value::as_array).into_iter().flatten() {
        if let Some(id) = node.get("id").and_then(Value::as_str) {
            by_id.insert(id.to_string(), "node".to_string());
        }
        for handle in node.get("handles").and_then(Value::as_array).into_iter().flatten() {
            if let Some(id) = handle.get("id").and_then(Value::as_str) {
                by_id.insert(id.to_string(), "handle".to_string());
            }
        }
    }
    for edge in snapshot.get("edges").and_then(Value::as_array).into_iter().flatten() {
        if let Some(id) = edge.get("id").and_then(Value::as_str) {
            by_id.insert(id.to_string(), "edge".to_string());
        }
    }
    by_id
}

fn append_board_owned_event(output: &mut String, first: &mut bool, event: &infinite_canvas::BoardOwnedEvent) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if !*first {
        output.push(',');
    }
    *first = false;
    event.write_json(output);
    if output.len() > ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    }
    Ok(())
}

/// 📬️ The Rust twin of React's `coalesceBoard2dEvents` (`🖥️Board2dHost/🟦️.tsx`); the shared corpus
/// `🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins both. Drops every transient row, takes the
/// latest `camera` out for the view lane, keeps every other row in order, and flushes now for a terminal row —
/// except a `select` tagged with a `gestureId` whose record the batch does not yet carry: an open gesture's
/// selection leaves together with its record, never mid-gesture.
pub(super) fn coalesce_owned_board_events(queue: &infinite_canvas::BoardEventQueue) -> Result<CoalescedBoardEvents, ui_wgpu::wgpu::BoundedActionFault> {
    use infinite_canvas::BoardEventKind;
    let recorded: Vec<String> = queue.iter().filter(|event| event.kind() == BoardEventKind::Gesture).filter_map(board_event_gesture_id).collect();
    let mut output = String::from("[");
    let mut first = true;
    let mut flush_now = false;
    let camera = queue.iter().filter(|event| event.kind() == BoardEventKind::Camera).filter_map(|event| engine_camera_from_json(event.payload_json())).last().map(|(x, y, zoom)| [x, y, zoom]);
    for event in queue.iter() {
        let kind = event.kind();
        if kind == BoardEventKind::Camera || board_event_transient(kind) {
            continue;
        }
        if board_event_flush_now(kind) && (kind != BoardEventKind::Select || board_event_gesture_id(event).is_none_or(|id| recorded.contains(&id))) {
            flush_now = true;
        }
        append_board_owned_event(&mut output, &mut first, event)?;
    }
    output.push(']');
    Ok(CoalescedBoardEvents { flush_now, events_json: output, camera })
}

/// 📬️ The rows of one sealed pointer page that reach `applyBoardEvents` — in order and byte for byte, every transient row
/// and every `camera` row left out by the table the buffered lanes coalesce by ([`board_event_transient`]); `None` when
/// nothing is left to dispatch.
///
/// ⚖️ A retained pointer plan seals its events as one page and the host published that page as it stood. Every idle move
/// is such a plan and its page is one `hover` row, so every hover was its own `applyBoardEvents` — a guest round trip that
/// changes nothing and one "Apply Board Events" row in History each; an area select's `preselect` frames, the brush
/// preview and the link rings went the same way (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F24).
/// React's host never dispatches those rows (`coalesceBoard2dEvents`); the shared corpus
/// `🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` names what each batch leaves.
fn board_page_dispatch_rows(events_json: &str) -> Result<Option<String>, ui_wgpu::wgpu::BoundedActionFault> {
    let page = events_json.trim();
    if page.is_empty() {
        return Ok(None);
    }
    let body = page.strip_prefix('[').and_then(|rest| rest.strip_suffix(']')).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let mut output = String::from("[");
    let (mut depth, mut in_string, mut escaped, mut start) = (0usize, false, false, 0usize);
    for (index, byte) in body.bytes().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                if depth == 0 {
                    start = index;
                }
                depth += 1;
            }
            b'}' | b']' => {
                depth = depth.checked_sub(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                if depth == 0 {
                    let row = &body[start..=index];
                    let name = row.strip_prefix("{\"name\":\"").and_then(|rest| rest.split('"').next()).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                    let kind = infinite_canvas::BoardEventKind::parse(name).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                    if kind != infinite_canvas::BoardEventKind::Camera && !board_event_transient(kind) {
                        if output.len() > 1 {
                            output.push(',');
                        }
                        output.push_str(row);
                    }
                }
            }
            _ => {}
        }
    }
    if depth != 0 || in_string {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    output.push(']');
    Ok((output.len() > 2).then_some(output))
}

/// 🧮️ Reserves exactly the actions one coalesced board batch publishes: `setCamera` for its camera, `applyBoardEvents`
/// for its rows.
fn reserve_board_coalesced<'input>(input: &'input mut ui_wgpu::wgpu::InputState<ActionDescriptor>, controller_id: &str, coalesced: &CoalescedBoardEvents) -> Result<ui_wgpu::wgpu::BoundedActionBatchReservation<'input>, ui_wgpu::wgpu::BoundedActionFault> {
    let camera_bytes = if coalesced.camera.is_some() { ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "setCamera", "camera", "x", "y", "zoom"])? } else { 0 };
    let rows_bytes = if coalesced.has_rows() { ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", &coalesced.events_json])? } else { 0 };
    input.reserve_actions(usize::from(coalesced.camera.is_some()) + usize::from(coalesced.has_rows()), camera_bytes + rows_bytes)
}

/// 📤️ Writes one coalesced board batch: its camera on the view lane first, then its board rows.
fn write_board_coalesced_flat(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, coalesced: &CoalescedBoardEvents) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if let Some(camera) = coalesced.camera {
        write_board_camera_flat(batch, controller_id, camera)?;
    }
    if coalesced.has_rows() {
        write_board_events_flat(batch, controller_id, &coalesced.events_json)?;
    }
    Ok(())
}

fn board_retirement_step(entry: &mut EngineSurface) -> bool {
    if let Some(retiring) = entry.board_retiring_events.as_mut() {
        if retiring.close_step() {
            entry.board_retiring_events = None;
        }
        return true;
    }
    false
}

pub(super) fn board_drain_into_buffer(surface_id: &str) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return false;
        };
        let retired = board_retirement_step(entry);
        if retired {
            return true;
        }
        let Some(host) = entry.board_host.as_mut() else {
            return false;
        };
        let Some(bytes) = host.peek_owned_event().map(|event| event.owned_bytes()) else {
            return false;
        };
        if entry.board_pending_events.reserve(1, bytes).is_err() {
            return false;
        }
        let event = host.pop_owned_event().expect("peeked board event remains owned by host");
        entry.board_pending_events.push(event).is_ok()
    })
}

fn board_peek_buffer_coalesced(surface_id: &str) -> Option<CoalescedBoardEvents> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        if entry.board_retiring_events.is_some() {
            return None;
        }
        let queue = &entry.board_pending_events;
        if queue.is_empty() {
            return None;
        }
        let coalesced = coalesce_owned_board_events(queue).ok()?;
        (coalesced.has_rows() || coalesced.camera.is_some()).then_some(coalesced)
    })
}

/// 📤️ Unconditional drain + coalesce + dispatch, mirroring `flushBoardEvents` (used after pointer-up, pointer-leave, and wheel).

/// 📤️ Drains into the buffer and only dispatches if a flush-now event (select, brushPlace, edgeCreate, ...) is pending, mirroring `drainAndMaybeFlush` (used on pointer-move).

fn write_board_events_flat(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, events_json: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let action = "applyBoardEvents";
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "eventsJson", events_json])?;
    batch.action(controller_id, action, bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("eventsJson"), events_json)?;
        builder.end_container()
    })
}

fn write_board_camera_flat(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, camera: [f64; 3]) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let action = "setCamera";
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "camera", "x", "y", "zoom"])?;
    batch.action(controller_id, action, bytes, |builder| {
        builder.begin_object(None)?;
        builder.begin_object(Some("camera"))?;
        builder.number(Some("x"), camera[0])?;
        builder.number(Some("y"), camera[1])?;
        builder.number(Some("zoom"), camera[2])?;
        builder.end_container()?;
        builder.end_container()
    })
}

pub(super) fn board_set_pointer_inside(surface_id: &str, inside: bool) {
    ENGINE_SURFACES.with(|cell| {
        if let Some(entry) = cell.borrow_mut().get_mut(surface_id) {
            entry.board_pointer_inside = inside;
        }
    });
}

fn board_pointer_plan_fault(fault: infinite_canvas::BoardPointerPlanFault) -> ui_wgpu::wgpu::BoundedActionFault {
    match fault {
        infinite_canvas::BoardPointerPlanFault::ItemCredits => ui_wgpu::wgpu::BoundedActionFault::ItemCredits,
        infinite_canvas::BoardPointerPlanFault::ByteCredits => ui_wgpu::wgpu::BoundedActionFault::ByteCredits,
        infinite_canvas::BoardPointerPlanFault::Unsupported => ui_wgpu::wgpu::BoundedActionFault::Structure,
    }
}

fn plan_board_pointer(surface_id: &str, intent: infinite_canvas::BoardPointerIntent) -> Result<Option<infinite_canvas::BoardPointerPlan>, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(host) = map.get(surface_id).and_then(|entry| entry.board_host.as_ref()) else {
            return Ok(None);
        };
        host.plan_pointer(intent).map(Some).map_err(board_pointer_plan_fault)
    })
}

fn commit_board_pointer(surface_id: &str, plan: &infinite_canvas::BoardPointerPlan, pointer_inside: Option<bool>) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return false;
        };
        let Some(host) = entry.board_host.as_mut() else {
            return false;
        };
        if !host.commit_pointer(plan) {
            return false;
        }
        if let Some(pointer_inside) = pointer_inside {
            entry.board_pointer_inside = pointer_inside;
        }
        true
    })
}

/// 🚦️ Begins one retained pointer commit and answers whether it dispatches anything: a view camera (`setCamera`, at once)
/// or the board rows its page leaves after the delivery table ([`board_page_dispatch_rows`]) — claimed here, published
/// when the commit reached its terminal ([`publish_board_pointer_step`]).
fn begin_board_pointer_commit(surface_id: &str, controller_id: &str, plan: infinite_canvas::BoardPointerPlan, pointer_inside: Option<bool>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if let Some(camera) = plan.view_camera() {
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "setCamera", "camera", "x", "y", "zoom"])?;
        let mut reservation = input.reserve_actions(1, bytes)?;
        write_board_camera_flat(&mut reservation, controller_id, camera)?;
        let mut admitted = None;
        let published = reservation.publish_with_checked(|| {
            let result = ENGINE_SURFACES.with(|cell| {
                let mut map = cell.borrow_mut();
                let Some(entry) = map.get_mut(surface_id) else {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
                };
                if entry.board_pointer_claim.is_some() || entry.board_pointer_controller_id.is_some() {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                let Some(host) = entry.board_host.as_mut() else {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
                };
                host.begin_pointer_commit(plan).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
                if let Some(pointer_inside) = pointer_inside {
                    entry.board_pointer_inside = pointer_inside;
                }
                Ok(())
            });
            let ok = result.is_ok();
            admitted = Some(result);
            ok
        });
        return admitted.unwrap_or(Ok(())).and(published).map(|()| true);
    }
    let dispatch = board_page_dispatch_rows(plan.events_json())?;
    let emits = dispatch.is_some();
    let claim = match dispatch.as_deref() {
        Some(rows) => Some(input.claim_action(ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", rows])?)?),
        None => None,
    };
    let admitted = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        };
        if entry.board_pointer_claim.is_some() || entry.board_pointer_controller_id.is_some() {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let Some(host) = entry.board_host.as_mut() else {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        };
        host.begin_pointer_commit(plan).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
        entry.board_pointer_claim = claim;
        entry.board_pointer_controller_id = emits.then(|| controller_id.to_owned());
        if let Some(pointer_inside) = pointer_inside {
            entry.board_pointer_inside = pointer_inside;
        }
        Ok(())
    });
    if admitted.is_err() {
        if let Some(claim) = claim {
            input.release_action_claim(claim)?;
        }
    }
    admitted.map(|()| emits)
}

/// 📏️ Steps one settle may spend. A pending commit and the drag frame queued behind it each walk one step per delta, per
/// world, screen and overlay point and a few phases, over at most [`infinite_canvas::BOARD_POINTER_ITEM_CAPACITY`] items;
/// an authority still busy past this is stuck, and that is a fault.
const BOARD_POINTER_SETTLE_STEPS: u64 = 8 * infinite_canvas::BOARD_POINTER_ITEM_CAPACITY as u64 + 64;

/// 🚦️ Brings one board's retained pointer authority to its terminal BEFORE the next input touches the board.
///
/// ⚖️ A retained pointer commit owns the board from `begin_pointer_commit` to its terminal: the engine pins it to the
/// `interaction_revision` it was planned at, and every direct handler — `pointer_down_screen`, `commit_wheel`,
/// `commit_pointer`, a key chord — bumps that revision. The frame steps the authority only in its `BoardAuthority`
/// phase, AFTER the whole input drain was dispatched, so a second input of the same drain met a commit that had not
/// taken one step, bumped the revision under it, and the phase answered `BoardAuthorityStep::Fault`: "board retained
/// authority faulted", the surface quarantined. Every idle move is such a commit (`Hover`), so the first ordinary click
/// — move, press, release in one drain — killed the renderer (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live
/// fault F14); a wheel after a move did the same, and the release of an area select or a pan that shared a drain with
/// its last move was refused (`begin_pointer_commit` chains drag frames only) and lost.
///
/// This is the frame phase's own ladder, stepped in place: the commit runs to `Complete` and its events are published
/// through the claim it reserved when it began — so they leave BEFORE the new input's — and a cancelled commit returns
/// its claim. Nothing is dropped and nothing overtakes; an idle authority costs one borrow.
pub fn settle_board_pointer_authority_into(surface_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if with_board_host(surface_id, infinite_canvas::BoardHost::pointer_authority_terminal_is_empty).unwrap_or(true) {
        return Ok(());
    }
    let mut sequence = 0;
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(BOARD_POINTER_SETTLE_STEPS, u64::MAX),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
    );
    for _ in 0..BOARD_POINTER_SETTLE_STEPS {
        match drive_board_authority_step(surface_id, &mut context) {
            infinite_canvas::BoardAuthorityStep::Pending => {}
            infinite_canvas::BoardAuthorityStep::Cancelled => release_board_pointer_claim(surface_id, input)?,
            infinite_canvas::BoardAuthorityStep::Fault => {
                release_board_pointer_claim(surface_id, input)?;
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            }
            infinite_canvas::BoardAuthorityStep::Complete => {
                publish_board_pointer_step(surface_id, input)?;
            }
        }
        if with_board_host(surface_id, infinite_canvas::BoardHost::pointer_authority_terminal_is_empty).unwrap_or(true) {
            return Ok(());
        }
    }
    Err(ui_wgpu::wgpu::BoundedActionFault::Structure)
}

pub fn drive_board_authority_step(surface_id: &str, context: &mut semio_framework_job::StepContext<'_>) -> infinite_canvas::BoardAuthorityStep {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(host) = map.get_mut(surface_id).and_then(|entry| entry.board_host.as_mut()) else {
            return infinite_canvas::BoardAuthorityStep::Complete;
        };
        if !host.pointer_authority_terminal_is_empty() {
            if host.pointer_cancel_pending() {
                return if host.step_pointer_cancel(context) { infinite_canvas::BoardAuthorityStep::Complete } else { infinite_canvas::BoardAuthorityStep::Pending };
            }
            return host.step_pointer_commit(context);
        }
        host.step_event_authority(context)
    })
}

pub fn puzzle_board_pointer_cancel_into(surface_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    let (cancelled, claim) = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else { return (false, None) };
        let Some(host) = entry.board_host.as_mut() else { return (false, None) };
        let cancelled = host.pointer_cancel_screen();
        entry.board_pointer_controller_id = None;
        entry.board_pointer_inside = false;
        (cancelled, entry.board_pointer_claim.take())
    });
    if let Some(claim) = claim {
        if let Err(fault) = input.release_action_claim(claim) {
            input.record_action_fault(fault);
        }
    }
    cancelled
}

pub fn publish_board_pointer_step(surface_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Ok(false);
        };
        let Some(host) = entry.board_host.as_mut() else { return Ok(false) };
        let Some(publication) = host.pointer_publication() else {
            return Ok(false);
        };
        let Some(events_json) = board_page_dispatch_rows(publication.events_json())? else {
            let retired = host.take_pointer_publication().is_some_and(|mut publication| publication.close_step() && publication.terminal_is_empty());
            return if retired { Ok(false) } else { Err(ui_wgpu::wgpu::BoundedActionFault::Structure) };
        };
        let claim = entry.board_pointer_claim.ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let controller_id = entry.board_pointer_controller_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let mut reservation = input.reserve_claimed_action(claim, controller_id, "applyBoardEvents")?;
        reservation.builder().begin_object(None)?;
        reservation.builder().string(Some("eventsJson"), &events_json)?;
        reservation.builder().end_container()?;
        reservation.publish_with_checked(|| {
            let Some(mut publication) = host.take_pointer_publication() else {
                return false;
            };
            publication.close_step() && publication.terminal_is_empty()
        })?;
        entry.board_pointer_claim = None;
        entry.board_pointer_controller_id = None;
        Ok(true)
    })
}

pub fn release_board_pointer_claim(surface_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let claim = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        entry.board_pointer_controller_id = None;
        entry.board_pointer_claim.take()
    });
    match claim {
        Some(claim) => input.release_action_claim(claim),
        None => Ok(()),
    }
}

/// 📤️ Publishes what the board's engine queued, the way React's `flushBoardEvents` does: EVERY queued event into the buffer,
/// coalesced by the shared corpus rules into at most ONE `applyBoardEvents` — the transient rows dropped, a gesture's
/// `select` riding with its record — and the latest `camera` on the view lane (`setCamera`); the buffer is retired either
/// way. The frame's `BoardAuthority` phase calls this for every board, and the direct lane after its handler.
///
/// ⚖️ The frame phase used to publish each queued event by itself and as it stood — one raw `applyBoardEvents` per step, a
/// `hover` the press queued included (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F24). One flush, one
/// table, for every lane.
pub fn publish_board_event_step(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    for _ in 0..2 * infinite_canvas::BOARD_EVENT_ITEM_CAPACITY + 2 {
        if !board_drain_into_buffer(surface_id) {
            break;
        }
    }
    if let Some(coalesced) = board_peek_buffer_coalesced(surface_id) {
        let mut reservation = reserve_board_coalesced(input, controller_id, &coalesced)?;
        write_board_coalesced_flat(&mut reservation, controller_id, &coalesced)?;
        reservation.publish_with_checked(|| board_retire_pending_events(surface_id))?;
        return Ok(true);
    }
    board_retire_pending_events(surface_id);
    Ok(false)
}

/// 📤️ Unconditional drain + coalesce + dispatch of the board's own event buffer — the wgpu twin of
/// React's `flushBoardEvents`. `puzzle_board_wheel_into` inlines the same three steps because it
/// must ride the camera action's reservation; this is the standalone form the keyboard chords use.
fn puzzle_board_flush_events_into(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    board_drain_into_buffer(surface_id);
    let Some(coalesced) = board_peek_buffer_coalesced(surface_id) else {
        return Ok(false);
    };
    let mut reservation = reserve_board_coalesced(input, controller_id, &coalesced)?;
    write_board_coalesced_flat(&mut reservation, controller_id, &coalesced)?;
    reservation.publish_with_checked(|| {
        ENGINE_SURFACES.with(|cell| {
            let mut map = cell.borrow_mut();
            let Some(entry) = map.get_mut(surface_id) else { return false };
            let pending = std::mem::take(&mut entry.board_pending_events);
            entry.board_retiring_events = Some(pending);
            true
        })
    })?;
    Ok(true)
}

/// 🎛️ Drives one move or release of a lane the retained plan does not model (`BoardHost::pointer_lane_is_direct`: the rotate
/// ring, a target-region drag, the area brush) the way React's Board2dHost drives every pointer: the engine's direct handler,
/// then the one coalesced flush ([`publish_board_event_step`]).
fn puzzle_board_direct_pointer_into(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, drive: impl FnOnce(&mut infinite_canvas::BoardHost)) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    with_board_host_mut(surface_id, drive).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    publish_board_event_step(surface_id, controller_id, input)
}

/// 🧹️ Hands the surface's pending board events to the bounded retirement; false while a previous batch still retires.
fn board_retire_pending_events(surface_id: &str) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else { return false };
        if entry.board_retiring_events.is_some() || entry.board_pending_events.is_empty() {
            return entry.board_retiring_events.is_none();
        }
        let pending = std::mem::take(&mut entry.board_pending_events);
        entry.board_retiring_events = Some(pending);
        true
    })
}

/// 🐁️ Publishes the board's live hover on the framework's own `interactionHover` lane, scoped to the
/// `Board2dScene.domain_id` the app declared — the wgpu twin of React's `dispatchBoardHover`
/// (`🖥️Board2dHost/🟦️.tsx:774-781`), including its two refusals: an app that declares NO domain
/// publishes nothing at all, and an unchanged target costs no round trip. The granularity comes from
/// the snapshot's own classification, defaulting to `"node"` exactly as React's
/// `granularityByIdRef.current.get(id) || "node"` does.
///
/// 🎯️ `domain_id` used to be carried on `Board2dScene` and read NOWHERE in this target, so every
/// board hover was invisible to the framework's interaction domain (outliner rows, sibling panes).
pub fn puzzle_board_hover_into(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let planned = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        let domain_id = entry.board_sync_cache.domain_id.clone()?;
        let hovered = entry.board_host.as_ref()?.hovered_id.clone();
        if entry.board_hover_published.as_ref() == Some(&hovered) {
            return None;
        }
        let granularity = hovered.as_ref().and_then(|id| entry.board_sync_cache.granularity_by_id.get(id).cloned()).unwrap_or_else(|| "node".to_string());
        Some((domain_id, hovered, granularity))
    });
    let Some((domain_id, hovered, granularity)) = planned else {
        return Ok(false);
    };
    let targets = match hovered.as_deref() {
        Some(id) => json!([{ "granularity": granularity, "id": id }]).to_string(),
        None => "[]".to_string(),
    };
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "interactionHover", "domainId", &domain_id, "channel", "pointer", "targets", &targets])?;
    let mut reservation = input.reserve_actions(1, bytes)?;
    write_interaction_hover(&mut reservation, controller_id, &domain_id, &targets)?;
    reservation.publish_with_checked(|| {
        ENGINE_SURFACES.with(|cell| {
            let mut map = cell.borrow_mut();
            let Some(entry) = map.get_mut(surface_id) else { return false };
            entry.board_hover_published = Some(hovered.clone());
            true
        })
    })?;
    Ok(true)
}

/// ⌨️ Board2d's keyboard chords, the wgpu twin of React's two `Board2dHost` keydown listeners:
/// `Escape` cancels an in-flight area-select (`🟦️.tsx:1292-1313`, `session.cancelAreaSelect`), and
/// `Tab`/`shift+Tab` walk the open brush slot's candidates forward/back (`🟦️.tsx:1338-1339`, a
/// CAPTURE-phase listener so it beats the shell's own chord table). Both were absent: the Board2d
/// wgpu region had zero keyboard handling of any kind, so an area-select could only be released by
/// pointer-up and the candidate picker was unreachable without the pointer.
///
/// Returns `true` when the chord was consumed and must NOT fall through to the shell's keybinding
/// dispatch — the Rust equivalent of React's `event.preventDefault()`.
///
/// 🪟️ React gates both listeners on `hoverActiveRef.current` (the pointer is over THIS pane) and
/// on `scene.interactive`; the pointer-inside witness is this target's equivalent.
pub fn puzzle_board_key_into(surface_id: &str, controller_id: &str, key: &KeyAction, modifiers: &PointerModifiers, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let armed = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        entry.board_pointer_inside.then_some(entry.board_sync_cache.active_utility.clone().unwrap_or_else(|| "select".into()))
    });
    let Some(active_utility) = armed else {
        return Ok(false);
    };
    settle_board_pointer_authority_into(surface_id, input)?;
    let acted = match key {
        KeyAction::Escape => with_board_host_mut(surface_id, |host| host.cancel_area_select()).unwrap_or(false),
        KeyAction::Tab => {
            if active_utility != "brush" {
                return Ok(false);
            }
            with_board_host_mut(surface_id, |host| {
                host.brush_cycle_candidate(!modifiers.shift);
                true
            })
            .unwrap_or(false)
        }
        _ => return Ok(false),
    };
    if !acted {
        return Ok(false);
    }
    puzzle_board_flush_events_into(surface_id, controller_id, input)?;
    Ok(true)
}

/// 🖱️ Presses the board — the engine's direct handler, so the authority is settled first
/// ([`settle_board_pointer_authority_into`]): a press never lands under the pending commit of the move before it.
pub fn puzzle_board_pointer_down_into(surface_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    settle_board_pointer_authority_into(surface_id, input)?;
    let (sx, sy) = board_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_down_screen(sx, sy, button.max(0) as u8, shift, ctrl_or_meta));
    board_set_pointer_inside(surface_id, true);
    Ok(())
}

pub fn puzzle_board_pointer_move_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl_or_meta: bool,
    alt: bool,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    settle_board_pointer_authority_into(surface_id, input)?;
    let (sx, sy) = board_local_pointer(inner, x, y);
    if with_board_host(surface_id, infinite_canvas::BoardHost::pointer_lane_is_direct).unwrap_or(false) {
        board_set_pointer_inside(surface_id, true);
        return puzzle_board_direct_pointer_into(surface_id, controller_id, input, |host| host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt));
    }
    let plan = plan_board_pointer(surface_id, infinite_canvas::BoardPointerIntent { phase: infinite_canvas::BoardPointerPhase::Move, x: sx, y: sy, shift, ctrl_or_meta, alt })?;
    let Some(plan) = plan else {
        return Ok(false);
    };
    if plan.requires_retained_commit() {
        return begin_board_pointer_commit(surface_id, controller_id, plan, Some(true), input);
    }
    if !commit_board_pointer(surface_id, &plan, Some(true)) {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    Ok(false)
}

pub fn puzzle_board_pointer_up_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl_or_meta: bool,
    alt: bool,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    settle_board_pointer_authority_into(surface_id, input)?;
    let (sx, sy) = board_local_pointer(inner, x, y);
    if with_board_host(surface_id, infinite_canvas::BoardHost::pointer_lane_is_direct).unwrap_or(false) {
        return puzzle_board_direct_pointer_into(surface_id, controller_id, input, |host| host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt));
    }
    let plan = plan_board_pointer(surface_id, infinite_canvas::BoardPointerIntent { phase: infinite_canvas::BoardPointerPhase::Up, x: sx, y: sy, shift, ctrl_or_meta, alt })?;
    let Some(plan) = plan else {
        return Ok(false);
    };
    if plan.requires_retained_commit() {
        return begin_board_pointer_commit(surface_id, controller_id, plan, None, input);
    }
    if !commit_board_pointer(surface_id, &plan, None) {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    Ok(false)
}

/// 🐁️ The leave clears the domain's hover before the board events go out — React's
/// `dispatchBoardHover(null)` on the same transition. The engine's own `hovered_id` is already
/// `None` by the time a leave is planned, so the witness comparison publishes exactly one clear.
pub fn puzzle_board_pointer_leave_into(surface_id: &str, controller_id: &str, alt: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let was_inside = ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).is_some_and(|entry| entry.board_pointer_inside));
    if !was_inside {
        return Ok(false);
    }
    settle_board_pointer_authority_into(surface_id, input)?;
    let cleared = puzzle_board_hover_into(surface_id, controller_id, input)?;
    let _ = cleared;
    let plan = plan_board_pointer(surface_id, infinite_canvas::BoardPointerIntent { phase: infinite_canvas::BoardPointerPhase::Leave, x: 0.0, y: 0.0, shift: false, ctrl_or_meta: false, alt })?;
    let Some(plan) = plan else {
        return Ok(false);
    };
    if plan.requires_retained_commit() {
        return begin_board_pointer_commit(surface_id, controller_id, plan, Some(false), input);
    }
    if !commit_board_pointer(surface_id, &plan, Some(false)) {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    Ok(false)
}

/// 🖐️ True from the press that opens a board pointer lane (a pending area select included) until its release, so pointer-up
/// outside the surface bounds still reaches the host (mirrors `tiled_map_drag_active`, which is live from the map's press).
pub fn board_drag_active(surface_id: &str) -> bool {
    with_board_host(surface_id, infinite_canvas::BoardHost::pointer_lane_in_flight).unwrap_or(false)
}

pub fn puzzle_board_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    settle_board_pointer_authority_into(surface_id, input)?;
    let mut reservation = input.reserve_actions(2, 2 * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    let (sx, sy) = board_local_pointer(inner, x, y);
    let plan = with_board_host(surface_id, |host| host.plan_wheel(sx, sy, delta as f64)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let camera = plan.camera();
    board_drain_into_buffer(surface_id);
    let buffered = board_peek_buffer_coalesced(surface_id);
    let retire_events = buffered.is_some();
    let events_json = buffered.filter(CoalescedBoardEvents::has_rows).map(|coalesced| coalesced.events_json);
    write_board_camera_flat(&mut reservation, controller_id, camera)?;
    if let Some(events_json) = events_json.as_deref() {
        write_board_events_flat(&mut reservation, controller_id, events_json)?;
    }
    reservation.publish_partial_with_checked(|| {
        let committed = with_board_host_mut(surface_id, |host| host.commit_wheel(&plan)).unwrap_or(false);
        if committed && retire_events {
            ENGINE_SURFACES.with(|cell| {
                if let Some(entry) = cell.borrow_mut().get_mut(surface_id) {
                    let pending = std::mem::take(&mut entry.board_pending_events);
                    entry.board_retiring_events = Some(pending);
                }
            });
        }
        committed
    })?;
    Ok(true)
}

//#endregion Board2d

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs"]
mod board2d_engine_tests;
