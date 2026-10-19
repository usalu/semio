//! 🧮️ Control commit rules: the fired actions, the edit commit and refusal rules for text, number, stepper and slider controls, the press bindings and the bubbling. See [`super`].

use super::*;

// #region events
//#region 🔖️Commit
// 🎬️ THE ONE commit authority for a retained document's form controls. Every `UiNode` variant that
// carries a value — `Input`, `Toggle`, `Slider`, `NumberStepper`, `Ring`, `IconSelect` — turns its
// own gesture into a `UiIntentCommand` addressed at the node that fired it (`EventRouter::
// build_intent`), and hands it to the host as `UiCommand::App`. The host admits it by `seq` and then
// collapses it to an address through `UiIntentCommand::descriptor`, which is where its authored args
// and its trigger payload finally merge — `publish_retained_action` stamps `windowId` and reserves it
// from there, exactly as it already did.
//
// 🌉️ The spec-carried `ActionDescriptor` never leaves this module on its own any more: it is the
// CARRIER for `controller_id`/name/authored args, and the intent is the DISPATCH. React draws the
// same line — `UiInterpreterContext.onAction` is reserved for its unowned scene hosts while every
// semantic control goes through `onIntent` (`🗣️Interpreter/🟦️.tsx`).
//
// 🩸️ Before this region the retained router documented "committing the edited value via `on_change`
// is not implemented", and the shell's own immediate-mode `commit_focused_input`/`stepper_metas`
// system (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) only ever saw ids minted by the CHROME's immediate-mode
// widget walk — never a retained document's. Editing a generation's parameters on wgpu was therefore
// fully cosmetic: a live caret, a live knob, and nothing reaching the guest (ticket
// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-wgpu-parity-2026-09-13.md` gaps #1/#6).
//
// Parity reference — React's own retained interpreter, NOT its older declarative-control path:
// `🗣️Interpreter/🟦️.tsx`'s `dispatchTrigger` → `UiDocumentStore::emitIntent` → `🛠️ShellHelpers/🟦️.tsx`'s
// `uiIntentPayload`/`uiInputField`. Two rules come from there and are reproduced verbatim below:
// a scalar payload is NAMED by its trigger (`delta` for `Trigger::Delta`, `value` for every other),
// and it is MERGED OVER the node's authored args rather than replacing them.

/// 🎬️ One gesture, before it is addressed: which `Trigger` fired, the spec-carried descriptor whose
/// `controller_id`/`action`/AUTHORED args it inherits, and the trigger's own payload — kept apart
/// from the args exactly as `ui_contract::UiIntent` keeps `args` and `input` apart.
/// [`UiIntentCommand::payload`] is the single place they merge, on the way to the host.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct FiredAction {
    pub(super) trigger: Trigger,
    pub(super) action: ActionDescriptor,
    pub input: Option<DslValue>,
}

/// 🎬️ One gesture against `action`'s binding. `None` when the node declares no binding for this
/// trigger — `reconcile::record_action_or_inert` marks that by an EMPTY action name, which is exactly
/// the condition React's `emitIntent` answers `undefined` for.
pub(super) fn fired_action(action: &ActionDescriptor, trigger: Trigger, input: DslValue) -> Option<FiredAction> {
    if action.action.is_empty() {
        return None;
    }
    Some(FiredAction { trigger, action: action.clone(), input: Some(input) })
}

/// 🎬️ A gesture that carries no payload of its own (`Activate`, a focus commit's own re-dispatch).
pub(super) fn bare_action(action: &ActionDescriptor, trigger: Trigger) -> Option<FiredAction> {
    if action.action.is_empty() {
        return None;
    }
    Some(FiredAction { trigger, action: action.clone(), input: None })
}

/// 🆔️ The versioned [`ui_contract::ActionId`] a bare descriptor stands for, for the nodes no document
/// published (chrome widgets, testkit trees) and for a binding the record no longer carries. The
/// descriptor's `controller_id` IS the id's scope — the same identity React's
/// `uiIntentToActionDescriptor` maps in the other direction.
pub(super) fn descriptor_action_id(action: &ActionDescriptor) -> Option<ui_contract::ActionId> {
    ui_contract::ActionId::try_v1(&action.controller_id, &action.action)
}

/// ✍️ Which `UiNode` variants own a live `EditState` text buffer. `Input` is the obvious one;
/// `IconSelect`'s value IS its icon string, which React edits through the `IconSelector`'s own
/// textarea (`🎴️IconSelector/🟦️.tsx`'s `onEditorChange`), so the retained target edits it the same
/// way rather than inventing a second gesture for it.
pub fn editable_value(node: &UiNode) -> Option<String> {
    match node {
        UiNode::Input(input) => Some(match input.display_factor.filter(|_| input.input_kind == "number").zip(input.value.trim().parse::<f64>().ok()) {
            Some((_, stored)) => number_field_text(input, stored),
            None => input.value.clone(),
        }),
        UiNode::IconSelect(select) => Some(select.value.clone()),
        UiNode::NumberStepper(stepper) => Some(semio_framework_ui_wgpu_controls::stepper::stepper_value_text(stepper.value, stepper.precision, stepper.display_factor)),
        _ => None,
    }
}

/// ⏎️ Whether an editable node commits only on Enter/blur (`Trigger::Commit`, `commit == "blur"`)
/// rather than on every keystroke (`Trigger::Change`) — React's `InputView`'s own `commitOnBlur`.
pub(super) fn commits_on_blur(node: &UiNode) -> bool {
    matches!(node, UiNode::Input(input) if input.commit.as_deref() == Some("blur"))
}

/// ✍️ The action an editable node's CURRENT buffer commits: `{value: <text>}`, numeric for a
/// `Component::Input` of kind `number` (React's `InputView`: `kind === "number" ? Number(raw) : raw`),
/// and constrained by that input's own `min`/`max`/`step` before it leaves — the canvas twin of the
/// `<input type="number" min max step>` attributes React hands the browser to enforce
/// (`🗣️Interpreter/🟦️.tsx`'s `InputView`). The trigger is the node's own commit moment: `Commit` for a
/// `commit: "blur"` input, `Change` otherwise, exactly as `reconcile::input_commit_action` resolved
/// the binding it is firing.
pub(super) fn edit_commit_action(node: &Node, text: &str) -> Option<FiredAction> {
    match &node.spec.0 {
        UiNode::Input(input) => {
            let value = if input.input_kind == "number" { DslValue::float(constrain_number_field(typed_number(text, input.display_factor, input.precision, current_stored(input), input.min, input.max, input.limits.as_ref()).ok()?, input)) } else { DslValue::String(text.to_string()) };
            let trigger = if commits_on_blur(&node.spec.0) { Trigger::Commit } else { Trigger::Change };
            fired_action(&input.on_change, trigger, value)
        }
        UiNode::IconSelect(select) => fired_action(&select.on_change, Trigger::Change, DslValue::String(text.to_string())),
        UiNode::NumberStepper(stepper) => typed_number(text, stepper.display_factor, stepper.precision, std::iter::once(stepper.value).chain(stepper.snaps.iter().copied()), stepper.min, stepper.max, stepper.limits.as_ref()).ok().map(|value| constrain_stepper_value(value, stepper)).and_then(|value| fired_action(&stepper.on_absolute, Trigger::Change, DslValue::float(value))),
        _ => None,
    }
}

/// ⏎️⎋️🔁️ The three moments React's window-search line binds BESIDE `onChange` — `Trigger::Submit`
/// (Enter, carrying the TRIMMED line, React's `input.onSubmit(draft.trim())`), `Trigger::Abort`
/// (Escape, bare) and `Trigger::RepeatLast` (Space on an empty line, bare). Producing them here is
/// what makes a retained command line behave like React's: `onChange` keeps firing on every
/// keystroke — that is the guest's autocomplete feed — while Enter runs the verb, instead of the
/// single keystroke-or-blur dispatch every `Input` used to be reduced to
/// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s `Search`, `onKeyDown`).
///
/// ⚖️ `None` for a node that binds nothing for this moment, which is precisely when the caller must
/// fall through to the ordinary edit behaviour (Escape closes an overlay, Space types a space).
pub(super) fn search_line_action(node: &Node, trigger: Trigger, text: &str) -> Option<FiredAction> {
    let UiNode::Input(input) = &node.spec.0 else { return None };
    match trigger {
        Trigger::Submit => fired_action(input.on_submit.as_ref()?, Trigger::Submit, DslValue::String(text.trim().to_string())),
        Trigger::Abort => bare_action(input.on_abort.as_ref()?, Trigger::Abort),
        Trigger::RepeatLast => bare_action(input.on_repeat_last.as_ref()?, Trigger::RepeatLast),
        _ => None,
    }
}

/// 🎯️ A number field's committed value: a detent kept exactly (it may sit off the step ladder), else
/// [`constrain_number_input`] rounded to its precision.
pub fn constrain_number_field(value: f64, input: &semio_framework_ui_declaration::ui::UiInputNode) -> f64 {
    if input.snaps.contains(&value) {
        return value;
    }
    let constrained = constrain_number_input(value, input.min, input.max, input.step);
    match (input.precision, input.display_factor) {
        (None, _) => constrained,
        (Some(precision), None) => ui_contract::round_ui_number(constrained, precision),
        (Some(precision), Some(factor)) => ui_contract::round_ui_number(constrained * factor, precision) / factor,
    }
}

pub(super) fn constrain_stepper_value(value: f64, stepper: &UiNumberStepperNode) -> f64 {
    let clamped = value.max(stepper.min.unwrap_or(f64::NEG_INFINITY)).min(stepper.max.unwrap_or(f64::INFINITY));
    match (stepper.precision, stepper.display_factor) {
        (None, _) => clamped,
        (Some(precision), None) => ui_contract::round_ui_number(clamped, precision),
        (Some(precision), Some(factor)) => ui_contract::round_ui_number(clamped * factor, precision) / factor,
    }
}

/// 🦉️ A stepper's live stored value: its admitted edit buffer read back from display units (a candidate keeping its exact
/// stored value), else its declared value — a refused draft is never a value.
pub(super) fn number_stepper_live_value(node: &Node, stepper: &UiNumberStepperNode) -> f64 {
    node.state.edit.as_ref().and_then(|edit| typed_number(&edit.text, stepper.display_factor, stepper.precision, std::iter::once(stepper.value).chain(stepper.snaps.iter().copied()), stepper.min, stepper.max, stepper.limits.as_ref()).ok()).unwrap_or(stepper.value)
}

pub(super) fn slider_live_value(node: &Node, slider: &UiSliderNode) -> f64 {
    node.state.slider_draft_value.unwrap_or(slider.value)
}

/// 🧲️ A detent is kept exactly (it may sit off the step ladder); every other value is stepped and clamped.
pub(super) fn constrain_slider_value(value: f64, slider: &UiSliderNode) -> f64 {
    if slider.snaps.contains(&value) {
        return value;
    }
    if slider.precision.is_some() {
        return value.clamp(slider.min, slider.max.max(slider.min));
    }
    constrain_number_input(value, Some(slider.min), Some(slider.max), Some(slider.step))
}

/// 🪐️ The stored number a typed text of a numeric control means (display units read back by the contract's
/// `ui_number_typed_value`, a candidate keeping its exact stored value), or `Err` with the refusal a renderer shows — the
/// crossed bound's producer-localized text (empty for an unlabelled bound or an unreadable text). A refused text is never
/// dispatched; the draft is kept.
pub fn typed_number(text: &str, display_factor: Option<f64>, precision: Option<u16>, candidates: impl IntoIterator<Item = f64>, min: Option<f64>, max: Option<f64>, limits: Option<&ui_contract::UiNumberLimits>) -> Result<f64, String> {
    let typed = text.trim().parse::<f64>().ok().filter(|value| value.is_finite()).ok_or_else(String::new)?;
    let stored = ui_contract::ui_number_typed_value(typed, display_factor, precision, candidates);
    match ui_contract::ui_number_crossed_bound(stored, min, max, limits) {
        Some(bound) => Err(bound.refusal.map(|label| label.0.as_str().to_string()).unwrap_or_default()),
        None => Ok(stored),
    }
}

/// 🛰️ A number field's stored value and detents — the candidates a typed display value keeps exactly.
pub(super) fn current_stored(input: &semio_framework_ui_declaration::ui::UiInputNode) -> impl Iterator<Item = f64> + '_ {
    input.value.trim().parse::<f64>().ok().into_iter().chain(input.snaps.iter().copied())
}

/// 🧿️ The text a number field's edit buffer shows for a stored value: its display text at its precision.
pub(super) fn number_field_text(input: &semio_framework_ui_declaration::ui::UiInputNode, stored: f64) -> String {
    ui_contract::ui_number_display_text(stored, input.display_factor, input.precision)
}

/// 🚧️ The refusal a numeric control's current edit buffer earns, `None` when it is admitted (or the node is no numeric
/// control) — the one verdict commit, blur and paint share.
pub fn edit_refusal(node: &Node) -> Option<String> {
    let text = node.state.edit.as_ref()?.text.as_str();
    match &node.spec.0 {
        UiNode::Input(input) if input.input_kind == "number" => typed_number(text, input.display_factor, input.precision, current_stored(input), input.min, input.max, input.limits.as_ref()).err(),
        UiNode::NumberStepper(stepper) => typed_number(text, stepper.display_factor, stepper.precision, std::iter::once(stepper.value).chain(stepper.snaps.iter().copied()), stepper.min, stepper.max, stepper.limits.as_ref()).err(),
        UiNode::Slider(slider) => typed_number(text, slider.display_factor, slider.precision, std::iter::once(slider_live_value(node, slider)).chain(slider.snaps.iter().copied()), Some(slider.min), Some(slider.max), slider.limits.as_ref()).err(),
        _ => None,
    }
}

pub(super) fn number_stepper_sign_at(bounds: Rect, x: f32, y: f32, inline: FlowInline, border: f32) -> Option<f64> {
    let [decrement, _, increment] = number_stepper_segments(bounds, inline, border);
    if decrement.contains(x, y) {
        Some(-1.0)
    } else if increment.contains(x, y) {
        Some(1.0)
    } else {
        None
    }
}

pub(super) fn number_stepper_can_step(node: &Node, stepper: &UiNumberStepperNode, sign: f64) -> bool {
    let value = number_stepper_live_value(node, stepper);
    if sign < 0.0 {
        stepper.min.is_none_or(|min| value > min)
    } else {
        stepper.max.is_none_or(|max| value < max)
    }
}

/// 👆️ The action a press/drag at `(x, y)` over `bounds` commits for a pointer-valued control —
/// `Toggle` flips its own `presence.selected`, `Slider`/`Ring` read the gesture position off the
/// same geometry `paint` drew them at (`layout::{slider_value_at, ring_t_at}`), and a
/// `NumberStepper`'s square side buttons step its value (relative through `on_delta` when that binding
/// exists, absolute through `on_absolute` otherwise — React's `NumberStepperView` makes exactly that
/// choice). `None` for the stepper's own value segment, for an unbound trigger, and for every
/// variant whose press means something else (`Button`/`Select`/`Stack`, handled by the caller).
pub fn pointer_commit_action(node: &Node, bounds: Rect, x: f32, y: f32, inline: FlowInline, border: f32, inline_suffix_width: f32, control_gap: f32) -> Option<FiredAction> {
    match &node.spec.0 {
        UiNode::Toggle(toggle) => fired_action(&toggle.on_change, Trigger::Change, DslValue::Bool(!toggle.presence.selected)),
        UiNode::Slider(slider) => {
            let unit_width = slider.unit_label().map(|_| inline_suffix_width);
            let track = slider_control_presentation(bounds, slider.axis_position(slider.value), 0.0, 1.0, unit_width, control_gap, inline).slider.track_cell;
            track.contains(x, y).then(|| fired_action(&slider.on_change, Trigger::Change, DslValue::float(semio_framework_ui_wgpu_controls::slider::slider_node_pointer_value(slider, track, x, y)))).flatten()
        }
        UiNode::Ring(ring) => fired_action(&ring.on_change, Trigger::Change, DslValue::float(ring_t_at(bounds, x, y))),
        UiNode::NumberStepper(stepper) => {
            let sign = number_stepper_sign_at(bounds, x, y, inline, border)?;
            if !number_stepper_can_step(node, stepper, sign) {
                return None;
            }
            number_stepper_fired(node, stepper, sign)
        }
        _ => None,
    }
}

/// 🎚️ The value one arrow/`Home`/`End`/`Page` key commits on a focused `Slider` — the shared keyboard
/// law (`ui_contract::slider_key_value`, pinned by `🧫️number-controls`), so React, the retained canvas and
/// the shell dialog land a key on the same value. The inline pair arrives already mirrored for `rtl`
/// (`mirrored_inline_key`) and `Shift` walks ten rungs. `None` for every other key, so it never swallows one.
pub(super) fn slider_key_value_from(slider: &UiSliderNode, current: f64, key: &str, shift: bool) -> Option<f64> {
    let key = match key {
        "ArrowRight" | "ArrowUp" => ui_contract::SliderKey::Increment,
        "ArrowLeft" | "ArrowDown" => ui_contract::SliderKey::Decrement,
        "PageUp" => ui_contract::SliderKey::PageUp,
        "PageDown" => ui_contract::SliderKey::PageDown,
        "Home" => ui_contract::SliderKey::Home,
        "End" => ui_contract::SliderKey::End,
        _ => return None,
    };
    Some(ui_contract::slider_key_value(current, slider.min, slider.max, slider.step, slider.precision, slider.display_factor, slider.snaps.iter().copied(), key, shift))
}

pub fn slider_key_value(slider: &UiSliderNode, key: &str, shift: bool) -> Option<f64> {
    slider_key_value_from(slider, slider.value, key, shift)
}

/// 🧬️ The contract kind a retained input's kind name spells (`reconcile::input_kind`).
fn text_input_kind(kind: &str) -> ui_contract::InputKind {
    match kind {
        "longText" => ui_contract::InputKind::LongText,
        "number" => ui_contract::InputKind::Number,
        "date" => ui_contract::InputKind::Date,
        "color" => ui_contract::InputKind::Color,
        "file" => ui_contract::InputKind::File,
        _ => ui_contract::InputKind::Text,
    }
}

/// 🖊️ What the shared text keyboard law (`ui_contract::text_input_key`, pinned by `🧫️text-controls`) makes of `key` on a
/// retained text field of `kind`: `primary` is Ctrl or ⌘. `None` for a key that is not the field's.
pub fn text_field_key(kind: &str, key: &str, modifiers: EventModifiers) -> Option<ui_contract::TextInputKeyAction> {
    let key = match key {
        "Enter" | "NumpadEnter" => ui_contract::TextInputKey::Enter,
        "Escape" => ui_contract::TextInputKey::Escape,
        _ => return None,
    };
    ui_contract::text_input_key(text_input_kind(kind), key, ui_contract::TextInputModifiers { primary: modifiers.ctrl || modifiers.meta, shift: modifiers.shift, alt: modifiers.alt })
}

/// 🎹️ The law key a physical key names on a stepper or a number field (design §18; the twin of `uiNumberFieldKey`, pinned by
/// the `fieldKeys` rows of `🧫️number-controls`): `ArrowUp`/`ArrowDown` one step (`Shift`: large), `PageUp`/`PageDown` the
/// adjacent detent, `Home`/`End` the hard bound — only toward a bound the field has, so the caret keeps them otherwise — and
/// `None` for every other key, the inline arrows included.
pub fn number_field_key(key: &str, shift: bool, min: Option<f64>, max: Option<f64>) -> Option<(ui_contract::SliderKey, bool)> {
    match key {
        "ArrowUp" => Some((ui_contract::SliderKey::Increment, shift)),
        "ArrowDown" => Some((ui_contract::SliderKey::Decrement, shift)),
        "PageUp" => Some((ui_contract::SliderKey::PageUp, false)),
        "PageDown" => Some((ui_contract::SliderKey::PageDown, false)),
        "Home" => min.filter(|min| min.is_finite()).map(|_| (ui_contract::SliderKey::Home, false)),
        "End" => max.filter(|max| max.is_finite()).map(|_| (ui_contract::SliderKey::End, false)),
        _ => None,
    }
}

/// 🔘️ Whether the retained select `id` is published segmented — every option visible at once as a radio group, chosen
/// without a popup.
pub(super) fn select_is_segmented(tree: &UiTree, id: NodeId) -> bool {
    tree.document_id(id).and_then(|document_id| tree.document().and_then(|document| document.record(document_id))).is_some_and(|record| matches!(&record.component, ui_contract::Component::Select(select) if select.appearance == ui_contract::SelectAppearance::Segmented))
}

/// 🪞️ What a value an assistive technology wrote into a control's mirror means: the control's own typed buffer (the path
/// a keystroke takes), a slider's typed readout (the path its double-clicked readout takes), or a ready dispatch for a
/// control that keeps no draft.
pub(super) enum MirroredValue {
    Buffer,
    Readout,
    Fired(Option<FiredAction>),
}

/// ➕️➖️ One `NumberStepper` increment/decrement of `sign`, taking the relative `Delta` path only for
/// a node that DECLARES that binding — see the long note at this function's pointer-side caller.
fn number_stepper_fired(node: &Node, stepper: &UiNumberStepperNode, sign: f64) -> Option<FiredAction> {
    let binds_delta = node.intent.as_ref().map_or_else(|| !stepper.on_delta.action.is_empty(), |intent| intent.binds(Trigger::Delta));
    if binds_delta {
        fired_action(&stepper.on_delta, Trigger::Delta, DslValue::float(sign * stepper.step))
    } else {
        fired_action(&stepper.on_absolute, Trigger::Change, DslValue::float(constrain_stepper_value(number_stepper_live_value(node, stepper) + sign * stepper.step, stepper)))
    }
}

/// 🎚️ Whether a press on this variant keeps committing while the pointer drags — React delegates
/// `Slider`/`Ring` to controls that report every intermediate value, not only the release.
pub(super) fn commits_while_dragging(node: &UiNode) -> bool {
    matches!(node, UiNode::Slider(_) | UiNode::Ring(_))
}

/// 🎚️ The scrub protocol's argument names (`🛠️tool-machine` `SCRUB_GESTURE_ARG`/`SCRUB_COMMIT_ARG`/`SCRUB_ABORT_ARG`,
/// pinned by `🛠️tool-machine/🧫️fixtures/🧫️scrub-law`): a continuous control's dispatch carries its value, the press it
/// belongs to and whether it is the release; a host cancel carries the press and its reason and no value.
pub const SCRUB_GESTURE_ARG: &str = "gesture";

pub const SCRUB_COMMIT_ARG: &str = "commit";

pub const SCRUB_ABORT_ARG: &str = "abort";

/// 🔢️ Process-wide press serial: every press identity is unique for the life of the guest it addresses.
static SCRUB_PRESS_SERIAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 🎚️ Whether `node` is a continuous control whose value rides presses: a `Slider`, a `Ring`, a `NumberStepper`'s absolute
/// value, or a number or colour `Input` without a commit policy (React's `SliderView`, `RingView`, `NumberStepperView` and
/// `InputView` ride their continuous lane exactly then; a typed field's blur is the release).
pub(super) fn is_continuous_control(node: &UiNode) -> bool {
    match node {
        UiNode::Slider(_) | UiNode::Ring(_) | UiNode::NumberStepper(_) => true,
        UiNode::Input(input) => matches!(input.input_kind.as_str(), "number" | "color") && !commits_on_blur(node),
        _ => false,
    }
}

/// 🎚️ `fired` as one dispatch of the press `id` is in: its payload becomes `{value, gesture, commit}`. A press opens on
/// its first dispatch; a release closes it, so a release with no open press is a one-shot press of its own.
pub(super) fn pressed(tree: &mut UiTree, window_id: &str, id: NodeId, mut fired: FiredAction, release: bool) -> FiredAction {
    let value = fired.input.take().unwrap_or(DslValue::Null);
    let Some(node) = tree.node_mut(id) else { return fired };
    let gesture = node.state.scrub_gesture.get_or_insert_with(|| format!("{window_id}/{}:{}", id.identity_parts().0, SCRUB_PRESS_SERIAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1)).clone();
    if release {
        node.state.scrub_gesture = None;
        node.state.scrub_offered = None;
    } else {
        node.state.scrub_offered = Some(value.clone());
    }
    fired.input = Some(DslValue::Object(vec![(INTENT_VALUE_FIELD.to_string(), value), (SCRUB_GESTURE_ARG.to_string(), DslValue::String(gesture)), (SCRUB_COMMIT_ARG.to_string(), DslValue::Bool(release))]));
    fired
}

/// 🧯️ The host cancel of `id`'s open press (`blur`, `captureLost`): `{gesture, abort}` on the control's own binding, the
/// press closed; `None` when no press is open. The guest drops the press with zero trace.
pub(super) fn cancelled_press(tree: &mut UiTree, id: NodeId, reason: &str) -> Option<FiredAction> {
    let node = tree.node_mut(id)?;
    let gesture = node.state.scrub_gesture.take()?;
    node.state.scrub_offered = None;
    press_cancel(&node.spec.0, gesture, reason)
}

/// 🔗️ The binding every dispatch of a continuous control's presses goes through.
fn press_binding(node: &UiNode) -> Option<&ActionDescriptor> {
    match node {
        UiNode::Slider(slider) => Some(&slider.on_change),
        UiNode::Ring(ring) => Some(&ring.on_change),
        UiNode::Input(input) => Some(&input.on_change),
        UiNode::NumberStepper(stepper) => Some(&stepper.on_absolute),
        _ => None,
    }
}

/// 🪃️ The host cancel `{gesture, abort}` of the press `gesture` on `node`'s own binding.
pub(super) fn press_cancel(node: &UiNode, gesture: String, reason: &str) -> Option<FiredAction> {
    fired_action(press_binding(node)?, Trigger::Change, DslValue::Object(vec![(SCRUB_GESTURE_ARG.to_string(), DslValue::String(gesture)), (SCRUB_ABORT_ARG.to_string(), DslValue::String(reason.to_string()))]))
}

/// 🏁️ The release `{value, gesture, commit: true}` of the press `gesture` on `node`'s own binding.
pub(super) fn press_release(node: &UiNode, gesture: String, value: DslValue) -> Option<FiredAction> {
    fired_action(press_binding(node)?, Trigger::Change, DslValue::Object(vec![(INTENT_VALUE_FIELD.to_string(), value), (SCRUB_GESTURE_ARG.to_string(), DslValue::String(gesture)), (SCRUB_COMMIT_ARG.to_string(), DslValue::Bool(true))]))
}

/// 📐️ One node's absolute painted rect: its own accepted layout plus every ancestor's origin — the
/// same accumulation `scene_slots::collect_scene_slots`/`hit_test_node`/`paint::paint_node` each
/// walk independently, resolved once here at dispatch time rather than by a whole-tree pass.
pub(super) fn absolute_rect(tree: &UiTree, id: NodeId) -> Option<Rect> {
    tree.absolute_rect(id)
}
//#endregion 🔖️Commit

//#region 🔖️Bubble
/// 🫧️ Walks from `from` up through `parent` links (including `from` itself), calling `handler(id)`
/// for each ancestor until it returns `true` ("handled, stop bubbling") or the root is reached.
pub fn bubble<F: FnMut(NodeId) -> bool>(tree: &UiTree, from: NodeId, mut handler: F) {
    let mut cursor = Some(from);
    while let Some(id) = cursor {
        if handler(id) {
            return;
        }
        cursor = tree.node(id).and_then(|node| node.parent);
    }
}

/// 🌳️ Whether `id` is `ancestor` itself or a descendant of it, walking the parent chain.
pub(super) fn is_descendant(tree: &UiTree, id: NodeId, ancestor: NodeId) -> bool {
    let mut found = false;
    bubble(tree, id, |current| {
        if current == ancestor {
            found = true;
            true
        } else {
            false
        }
    });
    found
}
//#endregion 🔖️Bubble
// #endregion events
