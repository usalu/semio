//! 📋️ Receipts, drop and clipboard: the retained input receipts, the action publication, the drop target actions and payload, the clipboard paste handling, the pointer button codes and the scene ui command. See [`super`].

use super::*;

//#region RetainedEngineCutover
/// 🚦️ One gate before the gesture becomes a queued action: a `seq` this surface has
/// already admitted is a duplicate or a reorder and is dropped, exactly as React's
/// runtime drops one. The stale-revision half already ran in the renderer's own
/// `events::EventRouter::build_intent`, against the tree that dispatched it.
/** 🧵️ W3 clipboard/drag-drop wiring (`report-w3-clipboard-dnd.md`): every `ui_wgpu::wgpu::UiCommand`
 * variant now has an explicit arm — no more silently-dropped-by-omission commands.
 *  - `App` → reserves and copies `action` into the bounded action authority every other
 *    action already flows through.
 *  - `ClipboardCopy`/`ClipboardCut` → `write_os_clipboard` (real OS clipboard via `ui_wgpu::wgpu::host`,
 *    mocked in this module's own tests — see `MOCK_CLIPBOARD_WRITES`).
 *  - `ClipboardPasteRequested` → `apply_clipboard_paste_requested`, below. Production native
 *    hosts submit the blocking read to the process worker pool; browsers use their Promise API.
 *  - `DropCommitted` → `apply_drop_committed`, below.
 *  - `DropCancelled` → intentional no-op: mirrors `framework/renderer/react/index.tsx`'s
 *    `UiStackHost.onDrop` (native HTML5 DnD), which has no "drag cancelled" callback on any
 *    declarative node either — there is nothing to dispatch.
 *  - `OverlayClosed` → intentional no-op: overlay open/close is fully internal
 *    `events::EventRouter` bookkeeping the retained engine already repaints correctly on its own; no
 *    `UiNode` variant carries an "on close"/`onOpenChange` callback today (confirmed by grep across
 *    every `Ui*Node` struct in `ui_wgpu::wgpu::component::ui`), so there is nothing for a host to fire.
 *  - `FocusChanged` → no-op HERE by design, and no longer DEFERRED: the shell's own
 *    `note_content_focus_commands` (`🐚️Shell/🎯️targets/🧊️wgpu`, region `ShellInput`) is the single
 *    consumer, and since ticket 26/09/17 packet W2k it records WHICH node took focus rather than a
 *    bare "content is focused" boolean — React's own `focusin` listener stores the active ROOT the
 *    same way (`🧱️elements/🌈️Surface/🟦️.tsx:186-197`). Consuming it here as well would double-track
 *    one piece of state from two places; the modality half React gets free from `:focus-visible` is
 *    handled at the source instead, by `EventRouter`'s `NodeFlags::FOCUS_VISIBLE`.
 *  - `Scene` → `apply_scene_ui_command`, below (`w4-scene-input`): a real per-event pointer/wheel hit
 *    on a `ComponentScene` leaf, routed into the matching per-`SurfaceKind` handler in `scenes`
 *    instead of that region's own once-per-render-frame `InputState` sample. */
pub fn apply_ui_commands(commands: &mut Vec<semio_framework_ui_wgpu_events::UiCommand>, pointer_id: Option<ui_render::PointerId>, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) {
    for command in commands.iter() {
        match command {
            semio_framework_ui_wgpu_events::UiCommand::App { window_id, intent } => {
                if input.admit_intent(intent) != semio_framework_ui_action::UiIntentAdmission::Accepted {
                    continue;
                }
                let descriptor = intent.descriptor();
                if let Err(fault) = publish_retained_action(input, window_id, &descriptor, intent) {
                    input.record_action_fault(fault);
                    return;
                }
            }
            semio_framework_ui_wgpu_events::UiCommand::ClipboardCopy { text, .. } | semio_framework_ui_wgpu_events::UiCommand::ClipboardCut { text, .. } => {
                #[cfg(any(any(test, feature = "testkit"),target_arch="wasm32"))]
                write_os_clipboard(text, input);
            }
            semio_framework_ui_wgpu_events::UiCommand::ClipboardPasteRequested { window_id } => {
                #[cfg(any(any(test, feature = "testkit"),target_arch="wasm32"))]
                apply_clipboard_paste_requested(window_id, input);
            }
            semio_framework_ui_wgpu_events::UiCommand::DropCommitted { window_id, target, payload, .. } => {
                if let Err(fault) = apply_drop_committed(window_id, *target, payload, input) {
                    input.record_action_fault(fault);
                    return;
                }
            }
            semio_framework_ui_wgpu_events::UiCommand::Scene { window_id, node, kind, rect, event, .. } => {
                apply_scene_ui_command(window_id, *node, *kind, *rect, event, pointer_id, input);
            }
            semio_framework_ui_wgpu_events::UiCommand::DropCancelled { .. } | semio_framework_ui_wgpu_events::UiCommand::OverlayClosed { .. } | semio_framework_ui_wgpu_events::UiCommand::FocusChanged { .. } => {}
        }
    }
    #[cfg(all(not(any(test, feature = "testkit")),not(target_arch="wasm32")))]
    {
        let mut index=0;
        while index<commands.len(){
            let clipboard=matches!(commands[index],semio_framework_ui_wgpu_events::UiCommand::ClipboardCopy{..}|semio_framework_ui_wgpu_events::UiCommand::ClipboardCut{..}|semio_framework_ui_wgpu_events::UiCommand::ClipboardPasteRequested{..});
            if !clipboard{index+=1;continue}
            if !input.original_clipboard_has_capacity(){break}
            let command=commands.remove(index);
            let(recipient,text)=match command{
                semio_framework_ui_wgpu_events::UiCommand::ClipboardCopy{window_id,text}|semio_framework_ui_wgpu_events::UiCommand::ClipboardCut{window_id,text}=>{(semio_framework_ui_wgpu_input::ClipboardIntentRecipient::Write(window_id),Some(text))},
                semio_framework_ui_wgpu_events::UiCommand::ClipboardPasteRequested{window_id}=>(semio_framework_ui_wgpu_input::ClipboardIntentRecipient::Window(window_id),None),
                _=>unreachable!(),
            };
            input.retain_original_clipboard_intent(recipient,text).unwrap_or_else(|_|panic!("original clipboard command lost its admitted source or fixed recipient"));
        }
    }
    drive_scene_interaction_step(input);
}

struct RetainedInputSubmission {
    pub(super) token: std::num::NonZeroU64,
    pub window: String,
    pub draft: semio_framework_ui_wgpu_engine::RetainedInputDraft,
}

struct RetainedInputReceipts {
    sequence: u64,
    pub slots: [Option<RetainedInputSubmission>; semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY],
}

impl Default for RetainedInputReceipts {
    fn default() -> Self { Self { sequence: 0, slots: std::array::from_fn(|_| None) } }
}

pub static RETAINED_INPUT_RECEIPTS: WorkerCell<RetainedInputReceipts> = WorkerCell::new(Default::default);

#[cfg(any(test, feature = "testkit"))]
pub struct RetainedInputReceiptReservation(#[doc(hidden)] pub Option<semio_framework_ui_action::ActionQueueReceipt>);

#[cfg(not(any(test, feature = "testkit")))]
struct RetainedInputReceiptReservation(Option<semio_framework_ui_action::ActionQueueReceipt>);

impl Drop for RetainedInputReceiptReservation {
    fn drop(&mut self) { if let Some(receipt) = self.0.take() { settle_retained_input_receipt(receipt, false); } }
}

pub fn reserve_retained_input_receipt(window: &str, intent: &semio_framework_ui_action::UiIntentCommand) -> Result<RetainedInputReceiptReservation, semio_framework_ui_action::BoundedActionFault> {
    let Some(draft) = UI_ENGINE.with(|cell| cell.borrow().retained_input_draft(window, intent)) else { return Ok(RetainedInputReceiptReservation(None)) };
    RETAINED_INPUT_RECEIPTS.with(|cell| {
        let mut receipts = cell.borrow_mut();
        let slot = receipts.slots.iter().position(Option::is_none).ok_or(semio_framework_ui_action::BoundedActionFault::ItemCredits)?;
        let sequence = receipts.sequence.checked_add(1).ok_or(semio_framework_ui_action::BoundedActionFault::ItemCredits)?;
        let token = sequence.checked_mul(semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY as u64).and_then(|prefix| prefix.checked_add(slot as u64)).and_then(std::num::NonZeroU64::new).ok_or(semio_framework_ui_action::BoundedActionFault::ItemCredits)?;
        receipts.sequence = sequence;
        receipts.slots[slot] = Some(RetainedInputSubmission { token, window: window.to_string(), draft });
        Ok(RetainedInputReceiptReservation(Some(semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::RetainedInput, token, member: 0, abort_correlation_on_error: false })))
    })
}

/// 🛟️ Consumes one exact native input receipt without disturbing newer drafts or retained node replacements.
pub fn settle_retained_input_receipt(receipt: semio_framework_ui_action::ActionQueueReceipt, discard: bool) {
    if receipt.source != semio_framework_ui_action::ActionQueueReceiptSource::RetainedInput { return; }
    let submission = RETAINED_INPUT_RECEIPTS.with(|cell| {
        let mut receipts = cell.borrow_mut();
        let slot = receipts.slots.get_mut((receipt.token.get() % semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY as u64) as usize)?;
        if slot.as_ref()?.token != receipt.token { return None; }
        slot.take()
    });
    if discard { if let Some(submission) = submission { UI_ENGINE.with(|cell| { cell.borrow_mut().discard_retained_input_draft(&submission.window, &submission.draft); }); } }
}

fn publish_retained_action(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, window_id: &str, action: &ActionDescriptor, intent: &semio_framework_ui_action::UiIntentCommand) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let mut reservation = input.reserve_action(&action.controller_id, &action.action, semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    if let Some(semio_framework::DslValue::Object(entries)) = action.args.as_ref() {
        for (key, value) in entries {
            if key != "windowId" {
                builder.value(Some(key), value)?;
            }
        }
    }
    builder.string(Some("windowId"), window_id)?;
    builder.end_container()?;
    let mut receipt = reserve_retained_input_receipt(window_id, intent)?;
    if let Some(receipt) = receipt.0 { builder.set_receipt(receipt)?; }
    reservation.publish()?;
    receipt.0 = None;
    Ok(())
}

/** 🫳️ Resolves `target`'s own `drop_action` (the only node kind `ui_wgpu`'s own
 * `sync_interactive_state` currently flags `NodeFlags::DROP_TARGET` for is `UiNode::Stack` — a
 * `UiTreeNode` also carries a `drop_action` field but nothing syncs `DROP_TARGET` for `Tree` nodes
 * yet, a `ui_wgpu`-side gap this fn doesn't paper over), decodes `payload`'s first
 * `application/x-semio-*` mime entry as JSON (mirrors `framework/renderer/react/index.tsx`'s
 * `UiStackHost.onDrop`: `[...dataTransfer.types].filter(k => k.startsWith("application/x-semio-"))`),
 * and queues the merged `ActionDescriptor` — `dispatchUiAction`'s own React-side merge order
 * (`{...descriptor.args, ...patch}`, patch wins) reproduced by `merge_action_args`. A no-op if the
 * target isn't (or no longer is, by the time this command is applied) a `Stack` with a `drop_action`,
 * or the payload carries no decodable semio-mime entry. */
pub fn apply_drop_committed(window_id: &str, target: NodeId, payload: &DragPayload, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let Some(action) = drop_target_action(window_id, target) else {
        return Ok(());
    };
    let Some(patch) = decode_drop_payload(payload) else {
        return Ok(());
    };
    let mut reservation = input.reserve_action(&action.controller_id, &action.action, semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    if let Some(semio_framework::DslValue::Object(entries)) = action.args.as_ref() {
        for (key, value) in entries {
            if key != "windowId" && !patch.contains_key(key) {
                builder.value(Some(key), value)?;
            }
        }
    }
    for (key, value) in &patch {
        if key != "windowId" {
            write_json_action_value(builder, Some(key), value)?;
        }
    }
    builder.string(Some("windowId"), window_id)?;
    builder.end_container()?;
    reservation.publish()
}

fn write_json_action_value(builder: &mut semio_framework_ui_action::BoundedActionBuilder, key: Option<&str>, value: &Value) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    match value {
        Value::Null => builder.null(key),
        Value::Bool(value) => builder.boolean(key, *value),
        Value::Number(value) => builder.number(key, value.as_f64().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?),
        Value::String(value) => builder.string(key, value),
        Value::Array(values) => {
            builder.begin_array(key)?;
            for value in values {
                write_json_action_value(builder, None, value)?;
            }
            builder.end_container()
        }
        Value::Object(entries) => {
            builder.begin_object(key)?;
            for (key, value) in entries {
                write_json_action_value(builder, Some(key), value)?;
            }
            builder.end_container()
        }
    }
}

pub fn drop_target_action(window_id: &str, target: NodeId) -> Option<ActionDescriptor> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let node = engine.tree(window_id)?.node(target)?;
        match &node.spec.0 {
            UiNode::Stack(stack) => stack.drop_action.clone(),
            _ => None,
        }
    })
}

/// 🎯️ The first `application/x-semio-*` payload entry with non-empty trimmed text, JSON-decoded as
/// an object — `None` if no such entry exists or it doesn't decode to a JSON object, matching
/// `UiStackHost.onDrop`'s own `try { JSON.parse(encoded) } catch { return; }` bail-out.
pub fn decode_drop_payload(payload: &DragPayload) -> Option<serde_json::Map<String, Value>> {
    let (_, encoded) = payload.iter().find(|(mime, value)| mime.starts_with("application/x-semio-") && !value.trim().is_empty())?;
    match serde_json::from_str::<Value>(encoded).ok()? {
        Value::Object(map) => Some(map),
        _ => None,
    }
}

/// 🔀️ `{...existing, ...patch}` (patch wins on key collision) — mirrors
/// `framework/renderer/react/index.tsx`'s `dispatchUiAction` merge order exactly.
#[cfg(any(test, feature = "testkit"))]
pub fn merge_action_args(existing: Option<&semio_framework::DslValue>, patch: serde_json::Map<String, Value>) -> Option<semio_framework::DslValue> {
    let mut base = match existing {
        Some(dsl) => match Value::from(dsl) {
            Value::Object(map) => map,
            _ => serde_json::Map::new(),
        },
        None => serde_json::Map::new(),
    };
    base.extend(patch);
    Some(semio_framework::DslValue::from(Value::Object(base)))
}

/** 🧪️ Test-only native clipboard handling. The in-memory mock is deliberately synchronous so
 * focused retained-engine tests can assert the resulting paste commands without an OS clipboard or
 * worker race. Production native handling below submits the blocking read to the I/O lane. */
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
pub fn apply_clipboard_paste_requested(window_id: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) {
    let Some(text) = read_os_clipboard() else { return };
    let mut commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window_id, semio_framework_ui_wgpu_events::UiEvent::Paste { text }));
    apply_ui_commands(&mut commands, None, input);
}

/** 🌐️ `ClipboardPasteRequested`'s wasm handling: the browser's Clipboard API is Promise-based with
 * no synchronous read (`ui_wgpu::wgpu::clipboard_read_text` is `async` there — see that fn's doc comment).
 * Spawns a
 * owned browser-local task that re-enters `UI_ENGINE` once the browser grants/denies
 * the read, on a later microtask (no `&mut InputState<ActionDescriptor>` borrow can survive that
 * boundary — it's tied to this frame's call stack). Whatever `UiCommand`s that later `dispatch_event`
 * call produces are queued once as asynchronous commands and consumed by the next interpreter
 * dispatch. Synchronous event commands are returned directly and are never cloned into that queue. */
#[cfg(target_arch = "wasm32")]
pub fn apply_clipboard_paste_requested(window_id: &str, _input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) {
    let window_id = window_id.to_string();
    semio_framework_async::browser::spawn_local(async move {
        if let Some(text) = semio_framework_ui_wgpu_hosting::host::clipboard_read_text().await {
            UI_ENGINE.with(|cell| {
                let mut engine = cell.borrow_mut();
                let commands = engine.dispatch_event(&window_id, semio_framework_ui_wgpu_events::UiEvent::Paste { text });
                engine.enqueue_async_commands(commands);
            });
        }
    });
}

/// 🖱️ Best-effort Left/Right/Middle mapping for `InputState::pointer_button`'s raw `i16` code —
/// the standard DOM `MouseEvent.button` convention (`0` primary/left, `1` middle, `2` secondary/
/// right) `scenes::handle_scene_pointer_button`'s own `button == 1`/`button == 2` checks (pan,
/// `✏️TextEditor`'s right-click context menu, …) already assume. 🐛️ W4 fix: this used to have `1`/`2`
/// swapped (`1 => Secondary, 2 => Middle`) — invisible until `apply_scene_ui_command` (below) started
/// round-tripping a synthesized `PointerButton` back through `pointer_button_code`'s inverse mapping
/// into a real `i16` for those same `scenes` handlers; nothing previously read `UiEvent::PointerDown/
/// Up`'s `button` field on this path (`events::EventRouter::dispatch` itself never branches on it), so
/// the swap had no observable effect before this ticket.
#[cfg(any(test, feature = "testkit"))]
pub fn pointer_button_from_code(code: i16) -> semio_framework_ui_wgpu_events::PointerButton {
    match code {
        1 => semio_framework_ui_wgpu_events::PointerButton::Middle,
        2 => semio_framework_ui_wgpu_events::PointerButton::Secondary,
        _ => semio_framework_ui_wgpu_events::PointerButton::Primary,
    }
}

/// 🖱️ Inverse of `pointer_button_from_code` — the DOM-standard `i16` code `scenes`' own handlers
/// expect, for `apply_scene_ui_command` to recover from a `UiCommand::Scene`'s `PointerButton`.
pub fn pointer_button_code(button: semio_framework_ui_wgpu_events::PointerButton) -> i16 {
    match button {
        semio_framework_ui_wgpu_events::PointerButton::Primary => 0,
        semio_framework_ui_wgpu_events::PointerButton::Middle => 1,
        semio_framework_ui_wgpu_events::PointerButton::Secondary => 2,
    }
}

/// 🎬️ `UiCommand::Scene` handling (`w4-scene-input`): routes a real per-event pointer/wheel hit
/// against a `ComponentScene` leaf — built by `ui_wgpu::wgpu::events::EventRouter::dispatch`'s own hit-
/// testing — into `scenes::handle_scene_pointer_button`/`handle_scene_pointer_move`/
/// `handle_scene_wheel`. This is now the ONLY caller of those three: they used to also be driven once
/// per render frame from `scenes::RenderEntry`'s own `apply_scene_wheel`/`apply_scene_pointer`,
/// sampling the aggregate `InputState` with manual "was it down last frame" edge detection — both
/// deleted (along with their tests) once every one of the 11 generic-fallback `SurfaceKind`s was
/// proven reachable through this real per-event path instead (see this ticket's report). Re-borrows
/// `UI_ENGINE` to look up `node`'s live `UiComponentSceneNode` (safe: every caller of
/// `apply_ui_commands`, this fn's only caller, already runs after its own `UI_ENGINE.with` borrow —
/// the one that produced `commands` — has dropped; see `apply_ui_commands`'s own doc comment). Skips
/// `scenes::scene_has_bespoke_pointer_dispatch` kinds (world-3d/node-graph/tiled-map/board-2d): those
/// already receive real OS-event-driven input through their own `dock`/`engine_canvas` host and must
/// not be double-dispatched here.
///
/// ⌨️ `UiEvent`'s four pointer variants carry `EventModifiers`, so shift-extend and ctrl-toggle reach
/// the surface handlers through this path exactly as React's DOM pointer event delivers them (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️audit-w14-scenes-residual.md` C1 — this used to be a
/// documented dead end, with `false` hardcoded at every merge-mode call downstream).
pub fn apply_scene_ui_command(window_id: &str, node: NodeId, kind: semio_framework_ui_declaration::ui::SurfaceKind, rect: Rect, event: &semio_framework_ui_wgpu_events::UiEvent, pointer_id: Option<ui_render::PointerId>, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) {
    if ui_document_close_pending_for(window_id) {
        return;
    }
    let event = match event {
        semio_framework_ui_wgpu_events::UiEvent::PointerDown { x, y, button, modifiers } => SceneIntentEvent::PointerDown { x: *x, y: *y, button: pointer_button_code(*button), modifiers: *modifiers },
        semio_framework_ui_wgpu_events::UiEvent::PointerUp { x, y, button, modifiers } => SceneIntentEvent::PointerUp { x: *x, y: *y, button: pointer_button_code(*button), modifiers: *modifiers },
        semio_framework_ui_wgpu_events::UiEvent::PointerMove { x, y, modifiers } => SceneIntentEvent::PointerMove { x: *x, y: *y, modifiers: *modifiers },
        semio_framework_ui_wgpu_events::UiEvent::Scroll { x, y, delta_y, modifiers, .. } => SceneIntentEvent::Scroll { x: *x, y: *y, delta_y: *delta_y, modifiers: *modifiers },
        _ => return,
    };
    let witness = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let revision = engine.tree_revision(window_id)?;
        let tree = engine.tree(window_id)?;
        let retained = tree.node(node)?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else {
            return None;
        };
        if scene.component_kind != kind || scene.host_id.len() > SCENE_INTENT_ID_BYTES {
            return None;
        }
        Some((revision, engine.surface_generation(window_id)?, scene.host_id.clone(), engine.driver_drag()))
    });
    let Some((tree_revision, surface_generation, host_id, driver_drag)) = witness else {
        input.record_action_fault(semio_framework_ui_action::BoundedActionFault::Structure);
        return;
    };
    if matches!(event, SceneIntentEvent::PointerDown { .. }) {
        if let Some(pointer_id) = pointer_id {
            if !retained_scene_target(window_id, node).is_some_and(|target| claim_scene_pointer_owner(target, pointer_id)) {
                input.record_action_fault(semio_framework_ui_action::BoundedActionFault::ItemCredits);
                return;
            }
        }
    }
    if let Err(fault) = SCENE_INTENTS.with(|cell| cell.borrow_mut().push(tree_revision, surface_generation, pointer_id, window_id, &host_id, node, kind, rect, event, driver_drag)) {
        input.record_action_fault(fault);
    }
    if matches!(event, SceneIntentEvent::PointerUp { .. }) {
        if let Some(pointer_id) = pointer_id {
            release_scene_pointer(pointer_id);
        }
    }
}
//#endregion RetainedEngineCutover
