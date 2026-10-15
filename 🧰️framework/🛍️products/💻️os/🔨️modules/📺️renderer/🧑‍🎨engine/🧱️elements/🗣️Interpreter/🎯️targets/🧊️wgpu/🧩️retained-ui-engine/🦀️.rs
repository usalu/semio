//! 🧩️ Retained UI engine, scene intents and clipboard io.
//!
//! The per-worker retained UI engine cell, the document close queue, the scene intent queue and pointer owners,
//! the retained hit registry, event dispatch, command application, input receipts and the clipboard io pump.

use super::chrome_ledger::accessibility_window_is_visible;
#[cfg(all(not(target_arch = "wasm32"), test))]
use super::chrome_ledger::{
    acknowledge_presented_input, begin_accessibility_visible_documents,
    note_accessibility_visible_document, publish_accessibility_visible_documents,
    published_accessibility_nodes_for_test, seal_presented_input_candidate,
};
#[cfg(not(target_arch = "wasm32"))]
use super::ink_and_vfs_focus::complete_pending_native_ink_clipboard;
#[cfg(all(not(target_arch = "wasm32"), test))]
use super::ink_and_vfs_focus::{
    abort_focused_ink_clipboard_stream, apply_focused_ink_editor_key,
    apply_focused_vfs_control_key, blur_focused_ink_editor_at, commit_focused_ink_clipboard_stream,
    commit_focused_text_editor_stream, focus_ink_editor, focus_ink_surface, focused_ink_editor,
    focused_ink_surface, push_focused_ink_clipboard_stream, push_focused_text_editor_stream,
    reserve_pending_native_ink_clipboard, rgba_png_data_url, start_focused_ink_clipboard_stream,
    start_focused_text_editor_stream, with_live_ink_surface, InkClipboardAddress,
    InkClipboardStream, FOCUSED_INK_EDITOR, FOCUSED_INK_SURFACE, INK_CLIPBOARD_STREAMS,
    PENDING_NATIVE_INK_CLIPBOARD,
};
use super::ink_and_vfs_focus::{
    close_retiring_focus_clipboard_one, close_window_focus_clipboard_one, focus_vfs_control,
    FOCUSED_VFS_CONTROL,
};
#[cfg(all(not(target_arch = "wasm32"), not(test)))]
use super::ink_and_vfs_focus::{
    complete_pending_native_text_editor_clipboard, PendingNativeInkClipboardToken,
    PendingNativeTextEditorClipboardToken,
};
use super::introspection::{
    block_list_accessibility_entries, event_feed_accessibility_entries,
    graph_timeline_accessibility_entries, scene_virtual_accessibility_node_id,
    table_button_accessibility_entries, table_editable_text_accessibility_entries,
    table_stepper_accessibility_entries, text_editor_accessibility_entries,
    vfs_accessibility_entries,
};
use super::retained_document_consumer::document_debug_log;
#[cfg(all(not(target_arch = "wasm32"), test))]
use super::retained_document_consumer::{apply_retained_document_page, UiDocumentFrameCursor};
use super::scene_interaction::drive_scene_interaction_step;
#[cfg(all(not(target_arch = "wasm32"), test))]
use super::scene_interaction::{
    close_scene_interaction_step, progress_presented_input_candidate,
    scene_interaction_terminal_is_empty, PresentedInputCandidateProgress,
};
#[cfg(all(not(target_arch = "wasm32"), test))]
use super::text_and_table_focus::{
    apply_focused_node_graph_note_key, apply_focused_table_editable_text_key,
    apply_focused_table_stepper_key, apply_focused_text_editor_key, apply_focused_text_editor_text,
    blur_focused_table_editable_text_for_pointer, blur_focused_text_editor_for_pointer,
    focused_table_editable_text_draft, refresh_node_graph_caret_after_pointer,
    sync_focused_table_editable_text_draft, FocusedTextEditor, FOCUSED_NODE_GRAPH_CARET,
    FOCUSED_TABLE_EDITABLE_TEXT,
};
use super::text_and_table_focus::{
    blur_text_editor, clear_focused_table_editable_text,
    commit_focused_table_editable_text_with_engine, focus_table_editable_text, focus_table_stepper,
    install_text_editor_focus, reset_text_editor_caret, set_focused_table_editable_text_draft,
    text_editor_focus_in, with_live_text_editor, FOCUSED_TABLE_STEPPER, FOCUSED_TEXT_EDITOR,
};
use crate::element_contracts::{ScenePointerTarget, WorkerCell, SCENE_POINTER_OWNER_CAPACITY};
use serde_json::Value;
use std::sync::MutexGuard;
use ui_contract::SurfaceId;
#[cfg(all(not(target_arch = "wasm32"), test))]
use ui_contract::UiDocumentLease;
use ui_wgpu::wgpu::{ActionDescriptor, DragPayload, NodeId, Rect, UiNode};
#[cfg(all(not(target_arch = "wasm32"), test))]
use ui_wgpu::wgpu::{Theme, UiComponentSceneNode, UiPresence};

//#region RetainedEngineCutover
/* 🧵️ The wave-3 cutover: `render_ui_node`'s live implementation is now `ui_wgpu::wgpu::engine::Ui`
 * (retained-mode `apply_tree`/`frame`/`dispatch_event`), not `ui_node_to_widget`+`render_widget`.
 * One process-wide `Ui` façade (its own internal `HashMap<window_id, UiWindow>` already partitions
 * per-window retained state — see `report-w0-engine-facade.md`) lives in a `thread_local!`, since neither call site of
 * `render_ui_node` (`shell::ShellChrome`'s `render_window_content`/`render_floating_panel`) is in a
 * region this ticket may touch, so there was no `ShellTypes` struct field available to hang it on
 * (struct field additions there are an Integrator choke point per `region-claims.json`).
 *
 * `window_id` is a genuinely new parameter — the only way to key per-window retained state — added
 * to `render_ui_node`'s public signature. Its two real call sites (`render_window_content`,
 * `render_floating_panel`, both in the off-limits `shell::ShellChrome` region) were touched with the
 * smallest possible diff: appending the one identifier string each already has in scope
 * (`window_id`/`active_tab_id`). See the ticket report for why this was judged unavoidable rather than
 * filed as a pure wiring request: without it the crate does not compile, and no identity for the
 * retained per-window bucket can be derived from anything else already flowing into this function.
 *
 * ✅️ RESOLVED (was a KNOWN, CONFIRMED GAP): `ui_wgpu::wgpu::Ui` no longer owns a private `FontAtlas`/
 * `Option<IconAtlas>` at all — `set_icons`/the old `atlas`/`icons` fields are gone. `Ui::frame` now
 * takes `atlas: &mut FontAtlas, icons: Option<&IconAtlas>` as parameters, mirroring how
 * `flex::LayoutEngine::compute`/`paint::paint_tree` already receive them internally. `render_ui_node`
 * below passes `ctx.atlas`/`ctx.icons` straight through — the SAME `FontAtlas`/`IconAtlas` instances
 * the shell already `GpuContext::upload_font_atlas`/`upload_icon_atlas`s every frame for chrome/dock/
 * panel text — so retained-mode content now shares the one real, GPU-uploaded glyph/icon texture
 * instead of reading from (or clobbering) a second, independent one. See
 * `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w3-interpreter-cutover.md`'s "CRITICAL FINDING"
 * for the original gap and the follow-up ticket work that closed it. */
pub(super) static UI_ENGINE: UiEngineCell = UiEngineCell::new();

/// ⛔️ The named refusal of a retained UI engine access before the shell resolved its locale.
pub(crate) const UI_ENGINE_LOCALE_UNRESOLVED: &str = "ui.engine.locale-unresolved: the retained UI engine is used before the shell resolved its locale";

/// 🧫️ The locale a law's engine starts in when the law builds no shell — a test fixture, never a production default.
#[cfg(test)]
pub(crate) const TEST_UI_ENGINE_LOCALE: semio_framework_ui_locale::Locale = semio_framework_ui_locale::Locale::En;

/// 🌐️ The retained UI engine cell. There is no default language, so the engine exists only once the shell resolved its
/// locale ([`install_ui_engine_locale`]); every access before that is refused by name ([`UI_ENGINE_LOCALE_UNRESOLVED`]).
pub(crate) struct UiEngineCell(pub(super) WorkerCell<Option<ui_wgpu::wgpu::Ui>>);

/// 🔐️ The held engine: dereferences to the `Ui`, refusing by name while none was installed.
pub(crate) struct UiEngineGuard<'a>(MutexGuard<'a, Option<ui_wgpu::wgpu::Ui>>);

impl std::ops::Deref for UiEngineGuard<'_> {
    type Target = ui_wgpu::wgpu::Ui;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect(UI_ENGINE_LOCALE_UNRESOLVED)
    }
}

impl std::ops::DerefMut for UiEngineGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().expect(UI_ENGINE_LOCALE_UNRESOLVED)
    }
}

/// 🌱️ The engine before any locale: none in production; a law's fixture locale under `cfg(test)`.
fn initial_ui_engine() -> Option<ui_wgpu::wgpu::Ui> {
    #[cfg(test)]
    return Some(ui_wgpu::wgpu::Ui::new(TEST_UI_ENGINE_LOCALE));
    #[cfg(not(test))]
    None
}

impl UiEngineCell {
    pub(super) const fn new() -> Self {
        Self(WorkerCell::new(initial_ui_engine))
    }

    pub(crate) fn borrow(&self) -> UiEngineGuard<'_> {
        UiEngineGuard(self.0.borrow())
    }

    pub(crate) fn borrow_mut(&self) -> UiEngineGuard<'_> {
        self.borrow()
    }

    pub(crate) fn with<R>(&self, apply: impl FnOnce(&Self) -> R) -> R {
        apply(self)
    }
}

/// 🏁️ Installs the retained UI engine in the shell's resolved locale — the first resolution builds it, every later one
/// (a `setLocale`, a preferences load) re-sets the language of its own chrome.
pub(crate) fn install_ui_engine_locale(locale: semio_framework_ui_locale::Locale) {
    let mut engine = UI_ENGINE.0.borrow();
    match engine.as_mut() {
        Some(ui) => ui.set_locale(locale),
        None => *engine = Some(ui_wgpu::wgpu::Ui::new(locale)),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct UiDocumentCloseOwner {
    pub(super) window_id: SurfaceId,
    token: ui_wgpu::wgpu::engine::UiSurfaceToken,
    pub(super) generation: u64,
}

#[derive(Default)]
struct UiDocumentCloseQueue {
    owners: std::collections::VecDeque<UiDocumentCloseOwner>,
}

impl UiDocumentCloseQueue {
    fn try_upsert(&mut self, owner: UiDocumentCloseOwner) -> bool {
        if let Some(existing) = self.owners.iter_mut().find(|existing| existing.window_id == owner.window_id) {
            *existing = owner;
            return true;
        }
        if self.owners.len() == ui_wgpu::wgpu::engine::UI_LAYOUT_SURFACE_SLOTS {
            return false;
        }
        self.owners.push_back(owner);
        true
    }

    fn owns_window(&self, window_id: &str) -> bool {
        self.owners.iter().any(|owner| owner.window_id.as_ref() == window_id)
    }
}

static UI_DOCUMENT_CLOSE_QUEUE: WorkerCell<UiDocumentCloseQueue> = WorkerCell::new(Default::default);

/// 🧹️ Retains the exact mounted document generation that a closed window must retire.
pub(crate) fn request_ui_document_close(window_id: &str) -> bool {
    let Some((token, generation)) = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        Some((engine.surface_token(window_id)?, engine.surface_generation(window_id)?))
    }) else {
        return true;
    };
    let Ok(window_id) = SurfaceId::try_from(window_id) else { return false };
    let admitted = UI_DOCUMENT_CLOSE_QUEUE.with(|cell| {
        let mut queue = cell.borrow_mut();
        queue.try_upsert(UiDocumentCloseOwner { window_id, token, generation })
    });
    admitted && UI_ENGINE.with(|cell| cell.borrow_mut().begin_surface_close(token))
}

pub(crate) fn ui_document_close_pending() -> bool {
    UI_DOCUMENT_CLOSE_QUEUE.with(|cell| !cell.borrow().owners.is_empty())
}

pub(crate) fn ui_document_close_pending_for(window_id: &str) -> bool {
    UI_DOCUMENT_CLOSE_QUEUE.with(|cell| cell.borrow().owns_window(window_id))
}

/// 🧵️ Advances one exact retained-window retirement unit for the frame work lane.
pub(crate) fn close_ui_document_one(cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    if cx.should_yield(){return Ok(false)}let mut failure=None;
    let advanced=(||{
    if close_retiring_focus_clipboard_one() {
        return true;
    }
    if crate::scenes::close_retired_scene_surface_one() {
        return true;
    }
    let Some(owner) = UI_DOCUMENT_CLOSE_QUEUE.with(|cell| cell.borrow().owners.front().cloned()) else { return false };
    let window_id = owner.window_id.as_ref();
    if UI_ENGINE.with(|cell|{let engine=cell.borrow();engine.layout_retirement_refusal(window_id).is_some()&&!engine.layout_retirement_failure_pending(window_id)}){return false;}
    if close_window_focus_clipboard_one(window_id, owner.generation) {
        return true;
    }
    if SCENE_INTENTS.with(|cell| cell.borrow_mut().close_window_step(window_id, owner.generation)) {
        return true;
    }
    if SCENE_POINTER_OWNERS.with(|cell| {
        let mut owners = cell.borrow_mut();
        let Some(index) = owners.slots.iter().position(|entry| entry.target.window_id == window_id && entry.target.window_generation == owner.generation) else { return false };
        owners.slots.remove(index);
        true
    }) {
        return true;
    }
    let mounted = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        engine.surface_token(window_id) == Some(owner.token) && engine.surface_generation(window_id) == Some(owner.generation)
    });
    if mounted && crate::scenes::retire_canvas_window_step(window_id) {
        return true;
    }
    if !crate::scenes::scene_surface_retirement_can_admit() {
        return true;
    }
    let (complete, retirement) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        if engine.surface_token(window_id) != Some(owner.token) || engine.surface_generation(window_id) != Some(owner.generation) {
            return (true, None);
        }
        match engine.close_surface_one(owner.token,cx){
            Err(error)=>{failure=Some(error);(false,None)},
            Ok(step)=>match step{
            ui_wgpu::wgpu::engine::UiSurfaceCloseStep::Refused(refusal) => {
                document_debug_log(&format!("[TRACE] ui-doc retained close refusal window={window_id} stage={} kind={}", refusal.stage, refusal.kind.as_str()));
                (false, None)
            }
            ui_wgpu::wgpu::engine::UiSurfaceCloseStep::Pending => (false, None),
            ui_wgpu::wgpu::engine::UiSurfaceCloseStep::RetiredScene(retirement) => (false, Some(retirement)),
            ui_wgpu::wgpu::engine::UiSurfaceCloseStep::Complete => (true, None),
        },
        }
    });
    if let Some(retired) = retirement {
        let owner = ScenePointerTarget::from(retired);
        crate::scenes::retire_scene_identity(&owner);
    }
    if complete {
        UI_DOCUMENT_CLOSE_QUEUE.with(|cell| {
            let mut queue = cell.borrow_mut();
            if queue.owners.front() == Some(&owner) {
                queue.owners.pop_front();
            }
        });
    }
    true
})();
    match failure{Some(error)=>Err(error),None=>Ok(advanced)}
}

const SCENE_INTENT_CAPACITY: usize = 256;
const SCENE_INTENT_ID_BYTES: usize = 256;

/// 🖱️ One queued scene pointer/wheel event. `modifiers` rides all four variants because React's own
/// surface handlers read `shiftKey`/`ctrlKey`/`metaKey`/`altKey` off the DOM pointer event itself —
/// the merge mode a press selects is a property of THAT press, not of a separately sampled keyboard
/// state (`marqueeModeFromModifiers`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:900`).
#[derive(Clone, Copy)]
pub(super) enum SceneIntentEvent {
    PointerDown { x: f32, y: f32, button: i16, modifiers: ui_wgpu::wgpu::EventModifiers },
    PointerUp { x: f32, y: f32, button: i16, modifiers: ui_wgpu::wgpu::EventModifiers },
    PointerMove { x: f32, y: f32, modifiers: ui_wgpu::wgpu::EventModifiers },
    Scroll { x: f32, y: f32, delta_y: f32, modifiers: ui_wgpu::wgpu::EventModifiers },
}

pub(super) struct SceneInteractionIntent {
    pub(super) generation: u64,
    pub(super) tree_revision: u64,
    pub(super) surface_generation: u64,
    pub(super) pointer_id: Option<ui_render::PointerId>,
    pub(super) window_id: String,
    pub(super) host_id: String,
    pub(super) node: NodeId,
    pub(super) kind: ui_wgpu::wgpu::SurfaceKind,
    pub(super) rect: Rect,
    pub(super) event: SceneIntentEvent,
    pub(super) driver_drag: ui_wgpu::wgpu::UiDriverDrag,
    pub(super) ink_job: Option<crate::scenes::InkInteractionJob>,
    pub(super) retiring: bool,
}

struct SceneIntentQueue {
    pub(super) slots: Box<[Option<SceneInteractionIntent>; SCENE_INTENT_CAPACITY]>,
    head: usize,
    pub(super) len: usize,
    next_generation: u64,
    pub(super) retiring: Option<SceneInteractionIntent>,
}

impl Default for SceneIntentQueue {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), head: 0, len: 0, next_generation: 1, retiring: None }
    }
}

impl SceneIntentQueue {
    pub(super) fn push(
        &mut self,
        tree_revision: u64,
        surface_generation: u64,
        pointer_id: Option<ui_render::PointerId>,
        window_id: &str,
        host_id: &str,
        node: NodeId,
        kind: ui_wgpu::wgpu::SurfaceKind,
        rect: Rect,
        event: SceneIntentEvent,
        driver_drag: ui_wgpu::wgpu::UiDriverDrag,
    ) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if window_id.len() > SCENE_INTENT_ID_BYTES || host_id.len() > SCENE_INTENT_ID_BYTES {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        if self.len == SCENE_INTENT_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let generation = self.next_generation;
        self.next_generation = self.next_generation.wrapping_add(1).max(1);
        let index = (self.head + self.len) % SCENE_INTENT_CAPACITY;
        self.slots[index] =
            Some(SceneInteractionIntent { generation, tree_revision, surface_generation, pointer_id, window_id: window_id.to_owned(), host_id: host_id.to_owned(), node, kind, rect, event, driver_drag, ink_job: None, retiring: false });
        self.len += 1;
        Ok(())
    }

    pub(super) fn pop_front(&mut self) -> Option<SceneInteractionIntent> {
        if self.len == 0 {
            return None;
        }
        let intent = self.slots[self.head].take();
        self.head = (self.head + 1) % SCENE_INTENT_CAPACITY;
        self.len -= 1;
        intent
    }

    pub(super) fn push_front(&mut self, intent: SceneInteractionIntent) {
        debug_assert!(self.len < SCENE_INTENT_CAPACITY);
        self.head = (self.head + SCENE_INTENT_CAPACITY - 1) % SCENE_INTENT_CAPACITY;
        self.slots[self.head] = Some(intent);
        self.len += 1;
    }

    pub(super) fn close_step(&mut self) -> bool {
        if let Some(intent) = self.retiring.as_mut() {
            if !intent.close_step() {
                return false;
            }
            self.retiring = None;
            return false;
        }
        self.retiring = self.pop_front();
        self.retiring.is_none()
    }

    fn close_window_step(&mut self, window_id: &str, generation: u64) -> bool {
        if let Some(intent) = self.retiring.as_mut().filter(|intent| intent.window_id == window_id && intent.surface_generation == generation) {
            if intent.close_step() {
                self.retiring = None;
            }
            return true;
        }
        let Some(offset) = (0..self.len).find(|offset| self.slots[(self.head + offset) % SCENE_INTENT_CAPACITY].as_ref().is_some_and(|intent| intent.window_id == window_id && intent.surface_generation == generation)) else { return false };
        let index = (self.head + offset) % SCENE_INTENT_CAPACITY;
        let intent = self.slots[index].as_mut().expect("occupied window intent");
        intent.retiring = true;
        if intent.close_step() {
            self.slots[index] = None;
            for cursor in offset..self.len - 1 {
                let next = (self.head + cursor + 1) % SCENE_INTENT_CAPACITY;
                self.slots[(self.head + cursor) % SCENE_INTENT_CAPACITY] = self.slots[next].take();
            }
            self.len -= 1;
        }
        true
    }

    pub(super) fn is_empty(&self) -> bool {
        self.len == 0 && self.retiring.is_none()
    }

    pub(super) fn blocks_presented_candidate(&self, engine: &ui_wgpu::wgpu::Ui, witness: u64) -> bool {
        let blocks = |intent: &SceneInteractionIntent| {
            engine.candidate_is_sealed_for(&intent.window_id, witness) && engine.tree_revision(&intent.window_id) == Some(intent.tree_revision) && engine.surface_generation(&intent.window_id) == Some(intent.surface_generation)
        };
        self.retiring.as_ref().is_some_and(blocks) || (0..self.len).any(|offset| self.slots[(self.head + offset) % SCENE_INTENT_CAPACITY].as_ref().is_some_and(blocks))
    }

    fn cancel_pointer(&mut self, pointer_id: ui_render::PointerId, owners: &[ScenePointerOwner]) {
        for slot in self.slots.iter_mut().filter_map(Option::as_mut) {
            if slot.pointer_id == Some(pointer_id)
                && owners.iter().any(|owner| owner.target.window_id == slot.window_id && owner.target.host_id == slot.host_id && owner.target.window_generation == slot.surface_generation)
                && slot.ink_job.as_ref().is_none_or(|job| job.matches_pointer_owner(slot.surface_generation, pointer_id))
            {
                slot.retiring = true;
            }
        }
    }
}

impl SceneInteractionIntent {
    pub(super) fn close_step(&mut self) -> bool {
        let Some(job) = self.ink_job.as_mut() else {
            return true;
        };
        if job.close_step() {
            self.ink_job = None;
            true
        } else {
            false
        }
    }
}

pub(super) static SCENE_INTENTS: WorkerCell<SceneIntentQueue> = WorkerCell::new(Default::default);

#[derive(Clone)]
struct ScenePointerOwner {
    pub(super) pointer_id: ui_render::PointerId,
    pub(super) target: ScenePointerTarget,
}


#[derive(Default)]
struct ScenePointerOwners {
    pub(super) slots: Vec<ScenePointerOwner>,
}

pub(super) static SCENE_POINTER_OWNERS: WorkerCell<ScenePointerOwners> = WorkerCell::new(Default::default);

/// 🪪️ Captures the exact retained scene identity published by one document node.
/// 🪟️ Every windowed tree container the retained surface `window_id` presents, measured against its scroll viewport
/// (`Ui::tree_window_measures`) — the Shell's tree window observer reads the shared engine through this.
pub(crate) fn tree_window_measures(window_id: &str) -> Option<(f64, Vec<ui_wgpu::wgpu::tree_window::TreeWindowContainerMeasure>)> {
    UI_ENGINE.with(|cell| cell.borrow().tree_window_measures(window_id))
}

pub fn retained_scene_target(window_id: &str, node: NodeId) -> Option<ScenePointerTarget> {
    UI_ENGINE.with(|cell| retained_scene_target_in(&cell.borrow(), window_id, node))
}

pub fn retained_scene_target_at(window_id: &str, x: f32, y: f32) -> Option<ScenePointerTarget> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let node = engine.scene_at(window_id, x, y)?.node;
        retained_scene_target_in(&engine, window_id, node)
    })
}


fn retained_scene_target_in(engine: &ui_wgpu::wgpu::Ui, window_id: &str, node: NodeId) -> Option<ScenePointerTarget> {
    let retained = engine.tree(window_id)?.node(node)?;
    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
    if window_id.len() > SCENE_INTENT_ID_BYTES || scene.host_id.is_empty() || scene.host_id.len() > SCENE_INTENT_ID_BYTES || scene.surface_id.len() > SCENE_INTENT_ID_BYTES {
        return None;
    }
    Some(ScenePointerTarget {
        document_id: engine.retained_document_id(window_id, node)?,
        host_id: scene.host_id.clone(),
        window_id: window_id.into(),
        surface_id: scene.surface_id.clone(),
        kind: scene.component_kind,
        node,
        key: retained.key.clone(),
        window_generation: engine.surface_generation(window_id)?,
        component_generation: retained.component_generation(),
    })
}

fn candidate_scene_target_in(engine: &ui_wgpu::wgpu::Ui, window_id: &str, node: NodeId) -> Option<ScenePointerTarget> {
    let retained = engine.candidate_tree(window_id)?.node(node)?;
    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
    if window_id.len() > SCENE_INTENT_ID_BYTES || scene.host_id.is_empty() || scene.host_id.len() > SCENE_INTENT_ID_BYTES || scene.surface_id.len() > SCENE_INTENT_ID_BYTES {
        return None;
    }
    Some(ScenePointerTarget {
        document_id: engine.candidate_document_id(window_id, node)?,
        host_id: scene.host_id.clone(),
        window_id: window_id.into(),
        surface_id: scene.surface_id.clone(),
        kind: scene.component_kind,
        node,
        key: retained.key.clone(),
        window_generation: engine.candidate_surface_generation(window_id)?,
        component_generation: retained.component_generation(),
    })
}

/// 🔒️ A replacement key, kind, document generation, or component mount cannot inherit a captured gesture.
pub fn scene_pointer_target_is_live(target: &ScenePointerTarget) -> bool {
    if ui_document_close_pending_for(&target.window_id) || crate::scenes::scene_host_retiring(&target.host_id) {
        return false;
    }
    let retained = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&target.window_id) != Some(target.window_generation) {
            return false;
        }
        let Some(retained) = engine.tree(&target.window_id).and_then(|tree| tree.node(target.node)) else { return false };
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return false };
        engine.retained_document_id(&target.window_id, target.node) == Some(target.document_id)
            && retained.key == target.key
            && retained.component_generation() == target.component_generation
            && scene.component_kind == target.kind
            && scene.host_id == target.host_id
            && scene.surface_id == target.surface_id
    });
    retained && UI_DOCUMENT_CLOSE_QUEUE.with(|cell| !cell.borrow().owners.iter().any(|owner| owner.window_id.as_ref() == target.window_id && owner.generation == target.window_generation))
}

pub(crate) fn component_scene_is_presented(witness: &ui_wgpu::wgpu::reconcile::UiComponentSceneWitness<'_>) -> bool {
    UI_ENGINE.with(|cell| cell.borrow().presented_component_scene_matches(witness))
}

pub fn claim_scene_pointer_owner(target: ScenePointerTarget, pointer_id: ui_render::PointerId) -> bool {
    if !scene_pointer_target_is_live(&target) {
        return false;
    }
    SCENE_POINTER_OWNERS.with(|cell| {
        let mut owners = cell.borrow_mut();
        owners.slots.retain(|owner| owner.pointer_id != pointer_id && scene_pointer_target_is_live(&owner.target));
        if owners.slots.len() == SCENE_POINTER_OWNER_CAPACITY {
            return false;
        }
        owners.slots.push(ScenePointerOwner { pointer_id, target });
        true
    })
}

pub fn claim_scene_pointer_at(window_id: &str, x: f32, y: f32, pointer_id: ui_render::PointerId) -> bool {
    retained_scene_target_at(window_id, x, y).is_some_and(|target| claim_scene_pointer_owner(target, pointer_id))
}

pub fn captured_scene_pointer(pointer_id: ui_render::PointerId) -> Option<ScenePointerTarget> {
    SCENE_POINTER_OWNERS.with(|cell| cell.borrow().slots.iter().find(|owner| owner.pointer_id == pointer_id).map(|owner| owner.target.clone())).filter(scene_pointer_target_is_live)
}

pub fn release_scene_pointer(pointer_id: ui_render::PointerId) -> Option<ScenePointerTarget> {
    SCENE_POINTER_OWNERS
        .with(|cell| {
            let mut owners = cell.borrow_mut();
            let index = owners.slots.iter().position(|owner| owner.pointer_id == pointer_id)?;
            Some(owners.slots.remove(index).target)
        })
        .filter(scene_pointer_target_is_live)
}

pub fn cancel_scene_pointer(pointer_id: ui_render::PointerId, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ScenePointerTarget> {
    let owners = SCENE_POINTER_OWNERS.with(|cell| {
        let mut registry = cell.borrow_mut();
        let mut cancelled = Vec::new();
        registry.slots.retain(|owner| {
            if owner.pointer_id == pointer_id {
                cancelled.push(owner.clone());
                false
            } else {
                true
            }
        });
        cancelled
    });
    let owners: Vec<_> = owners.into_iter().filter(|owner| scene_pointer_target_is_live(&owner.target)).collect();
    SCENE_INTENTS.with(|cell| cell.borrow_mut().cancel_pointer(pointer_id, &owners));
    for owner in &owners {
        let owner = &owner.target;
        match owner.kind {
            ui_wgpu::wgpu::SurfaceKind::TextEditor => {
                crate::engine_canvas::text_editor_pointer_cancel_into(&owner.host_id);
            }
            ui_wgpu::wgpu::SurfaceKind::Paint2d => {
                crate::engine_canvas::paint2d_pointer_cancel_into(&owner.host_id);
            }
            ui_wgpu::wgpu::SurfaceKind::InkCanvas => {
                let controller_id = UI_ENGINE.with(|cell| {
                    let engine = cell.borrow();
                    match &engine.tree(&owner.window_id).and_then(|tree| tree.node(owner.node))?.spec.0 {
                        UiNode::ComponentScene(scene) => Some(scene.controller_id.clone()),
                        _ => None,
                    }
                });
                if let Some(controller_id) = controller_id {
                    let _ = crate::scenes::ink_gesture_abort_into(&controller_id, &owner.surface_id, &owner.host_id, input);
                }
                crate::scenes::ink_pointer_cancel_into(&owner.host_id);
            }
            _ => {}
        }
    }
    owners.into_iter().map(|owner| owner.target).collect()
}
/// 👆️ Last-seen `(pointer_down, pointer_button)` per `window_id`, so `dispatch_pointer_events` can
/// detect Down/Up edges from `InputState`'s per-frame aggregate.
#[cfg(test)]
static POINTER_EDGE_STATE: WorkerCell<std::collections::HashMap<String, (bool, i16)>> = WorkerCell::new(Default::default);

/** 🖇️ Public hook for the sibling `w3-shell-input-cutover` workstream (region `shell::ShellInput`,
 * which this ticket must not touch): routes a fully-formed `ui_wgpu::wgpu::UiEvent` (built from raw
 * winit input, e.g. real key events/IME/focus-scoped routing this function's own per-frame
 * pointer-only synthesis below deliberately does not attempt) into the same process-wide retained
 * engine `render_ui_node` itself drives, and forwards any resulting `UiCommand::App` action into the
 * same reserved bounded-action/`app.dispatch_actions` pipeline other actions already use. Returns the raw
 * command list too, for `FocusChanged`/`OverlayClosed`/`DropCommitted`/clipboard commands a caller may
 * want to react to itself (e.g. an actual OS clipboard read for `ClipboardPasteRequested` is a `host`
 * concern, not this function's). */
/** 🎯️ Pushes `window_id`'s retained pointer registry — one entry per interactive node of its last
 * published paint, at the rect that paint drew it at — into the host's own `InputState`, and answers
 * the control ids it minted so the caller can tell a document hit from a chrome hit.
 *
 * 🩸️ Without this the shell's registry held the chrome and nothing else: a pointer inside a
 * published `mounted_layout` row rect resolved the WINDOW's own `HitKind::ScrollRegion`, so no row
 * action dispatched, no wheel propagated to a scene and no hover ever resolved (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6). The
 * window's own `ScrollRegion` is registered by the chrome BEFORE the body paints, so every body
 * child outranks it in `InputState::hit_at`'s reverse scan. */
pub fn register_retained_hit_targets(window_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<String> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let registrations = engine.window_hit_targets(window_id);
        let mut ids = Vec::with_capacity(registrations.len());
        for registration in registrations {
            ids.push(registration.control_id.clone());
            input.register_hit(registration.to_hit_target());
        }
        ids
    })
}

pub fn register_clipped_retained_hit_targets(window_id: &str, clip: ui_wgpu::wgpu::Rect, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<(String, Option<ScenePointerTarget>, Rect)> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let mut ids = Vec::new();
        for registration in engine.window_hit_targets(window_id) {
            let mut target = registration.to_hit_target();
            if !registration.overlay {
                let x = registration.rect.x.max(clip.x);
                let y = registration.rect.y.max(clip.y);
                let right = (registration.rect.x + registration.rect.w).min(clip.x + clip.w);
                let bottom = (registration.rect.y + registration.rect.h).min(clip.y + clip.h);
                if right <= x || bottom <= y {
                    continue;
                }
                target.rect = ui_wgpu::wgpu::Rect::new(x, y, right - x, bottom - y);
            }
            let scene = registration.scene.as_ref().and_then(|_| candidate_scene_target_in(&engine, window_id, registration.node));
            ids.push((registration.control_id.clone(), scene, target.rect));
            input.register_hit(target);
        }
        ids
    })
}

/// 🖱️ The retained window whose content holds pointer capture right now — a host routes every move
/// there until release, so a `Slider`/`Ring` drag survives leaving the control's own rect.
pub fn retained_pointer_capture_window(pointer_id: ui_render::PointerId) -> Option<String> {
    UI_ENGINE.with(|cell| cell.borrow().window_with_pointer_capture(pointer_id.0).map(str::to_owned))
}

pub fn retained_any_pointer_capture() -> Option<(String, ui_render::PointerId)> {
    UI_ENGINE.with(|cell| cell.borrow().any_pointer_capture().map(|(window_id, pointer_id)| (window_id.to_owned(), ui_render::PointerId(pointer_id))))
}

/// 📐️ The SOLVED content height of one retained surface — what a floating panel hugs up to
/// `calc(100% - 2 · spacing)`, React's `<Panel>` sizing (`🖼️Panel/🟦️.tsx`). `None` until that surface's
/// layout has been accepted once, so the caller keeps its own band for the first frame.
pub fn retained_content_height(window_id: &str) -> Option<f32> {
    UI_ENGINE.with(|cell| cell.borrow().surface_content_height(window_id))
}

#[cfg(test)]
/// 🌳️ Whether production reconcile retained `window_id` with a Tree root.
pub(crate) fn retained_root_is_tree(window_id: &str) -> bool {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let Some(tree) = engine.tree(window_id) else { return false };
        tree.root.and_then(|root| tree.node(root)).is_some_and(|node| matches!(&node.spec.0, ui_wgpu::wgpu::UiNode::Tree(_)))
    })
}

/// 🫳️ Retained panel/catalogue drags (`EventRouter::DragSession`) the legacy shell `tree_drag` path never sees.
pub fn active_retained_drag_sessions() -> Vec<(String, f32, f32, std::collections::HashMap<String, String>)> {
    UI_ENGINE.with(|cell| cell.borrow().active_drag_sessions())
}

pub fn canvas_catalogue_drag_over(window_id: &str, x: f32, y: f32, drag_data: &std::collections::HashMap<String, String>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    if !drag_data.contains_key(crate::scenes::CANVAS_CATALOGUE_DRAG_MIME) {
        return crate::scenes::cancel_canvas_catalogue_drag(input);
    }
    let mut types = drag_data.keys().cloned().collect::<Vec<_>>();
    types.sort();
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let hit = engine.scene_at(window_id, x, y)?;
        if hit.kind != ui_wgpu::wgpu::SurfaceKind::Canvas2d {
            return None;
        }
        let generation = engine.surface_generation(window_id)?;
        let tree = engine.tree(window_id)?;
        let UiNode::ComponentScene(scene) = &tree.node(hit.node)?.spec.0 else { return None };
        Some(crate::scenes::canvas_catalogue_drag_over_into(scene, hit.rect, window_id, generation, x, y, &types, input))
    });
    match outcome {
        Some(Ok(accepted)) => accepted,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            false
        }
        None => {
            crate::scenes::cancel_canvas_catalogue_drag(input);
            false
        }
    }
}

pub fn canvas_catalogue_drop(window_id: &str, x: f32, y: f32, drag_data: &std::collections::HashMap<String, String>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    let Some(raw_payload) = drag_data.get(crate::scenes::CANVAS_CATALOGUE_DRAG_MIME) else {
        return false;
    };
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let hit = engine.scene_at(window_id, x, y)?;
        if hit.kind != ui_wgpu::wgpu::SurfaceKind::Canvas2d {
            return None;
        }
        let generation = engine.surface_generation(window_id)?;
        let tree = engine.tree(window_id)?;
        let UiNode::ComponentScene(scene) = &tree.node(hit.node)?.spec.0 else { return None };
        Some(crate::scenes::canvas_catalogue_drop_into(scene, hit.rect, window_id, generation, x, y, raw_payload, input))
    });
    match outcome {
        Some(Ok(accepted)) => accepted,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            false
        }
        None => {
            crate::scenes::cancel_canvas_catalogue_drag(input);
            false
        }
    }
}

fn stamp_window_clock(engine: &mut ui_wgpu::wgpu::Ui, window_id: &str) -> Option<ui_wgpu::wgpu::UiWindowClockStep> {
    let seconds = semio_framework_job::default_now_us()? as f64 / 1_000_000.0;
    engine.advance_window_clock(window_id, seconds)
}

pub(super) fn apply_presented_scene_caret(window_id: &str, caret: ui_wgpu::wgpu::UiPresentedCaret) -> bool {
    if caret.source != ui_wgpu::wgpu::UiCaretSource::Scene {
        return false;
    }
    let scene = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_token(window_id) != Some(caret.surface) {
            return None;
        }
        let node = engine.presented_node_at(window_id, caret.surface, caret.document_id)?;
        let tree = engine.tree(window_id)?;
        let UiNode::ComponentScene(scene) = &tree.node(node)?.spec.0 else { return None };
        Some((scene.host_id.clone(), scene.surface_id.clone(), scene.component_kind))
    });
    scene.is_some_and(|(host_id, surface_id, kind)| crate::engine_canvas::component_scene_set_caret_visible(&host_id, &surface_id, kind, caret.visible))
}

fn apply_window_clock_caret(window_id: &str, step: Option<&ui_wgpu::wgpu::UiWindowClockStep>) {
    if let Some(caret) = step.and_then(|step| step.caret) {
        apply_presented_scene_caret(window_id, caret);
    }
}

/// ⏲️ Advances one accepted surface and delivers its timer commands through the ordinary boundary.
pub(crate) fn advance_retained_window_clock(window_id: &str, surface: ui_wgpu::wgpu::UiSurfaceToken, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Option<ui_wgpu::wgpu::UiWindowClockStep> {
    let (step, mut commands) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        if engine.surface_token(window_id) != Some(surface) {
            return (None, Vec::new());
        }
        let step = stamp_window_clock(&mut engine, window_id);
        (step, engine.drain_commands())
    });
    apply_ui_commands(&mut commands, None, input);
    apply_window_clock_caret(window_id, step.as_ref());
    step
}

pub fn dispatch_ui_event(window_id: &str, event: ui_wgpu::wgpu::UiEvent, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ui_wgpu::wgpu::UiCommand> {
    let (mut commands, clock) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        let clock = stamp_window_clock(&mut engine, window_id);
        let mut commands = engine.drain_commands();
        commands.extend(engine.dispatch_event(window_id, event));
        (commands, clock)
    });
    apply_window_clock_caret(window_id, clock.as_ref());
    apply_ui_commands(&mut commands, None, input);
    commands
}

pub fn dispatch_ui_pointer_event(window_id: &str, pointer_id: ui_render::PointerId, event: ui_wgpu::wgpu::UiEvent, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ui_wgpu::wgpu::UiCommand> {
    dispatch_presented_ui_pointer_event(window_id, pointer_id, event, input)
}

/// 🎞️ Routes pointer state through the exact presented retained tree and router.
pub fn dispatch_presented_ui_pointer_event(window_id: &str, pointer_id: ui_render::PointerId, event: ui_wgpu::wgpu::UiEvent, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ui_wgpu::wgpu::UiCommand> {
    if matches!(event, ui_wgpu::wgpu::UiEvent::PointerDown { .. }) {
        FOCUSED_TABLE_STEPPER.with(|cell| *cell.borrow_mut() = None);
        FOCUSED_VFS_CONTROL.with(|cell| *cell.borrow_mut() = None);
    }
    let (mut commands, clock) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        let clock = stamp_window_clock(&mut engine, window_id);
        let mut commands = engine.drain_commands();
        commands.extend(engine.dispatch_pointer_event(window_id, pointer_id.0, event));
        (commands, clock)
    });
    apply_window_clock_caret(window_id, clock.as_ref());
    apply_ui_commands(&mut commands, Some(pointer_id), input);
    commands
}

pub fn dispatch_accessibility_event(window_id: &str, window_generation: u64, node_id: u64, node_key: &str, event: ui_wgpu::wgpu::AccessibilityUiEvent, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Option<Vec<ui_wgpu::wgpu::UiCommand>> {
    if !accessibility_window_is_visible(window_id) {
        return None;
    }
    let text_editor = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        text_editor_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.key == node_key && scene_virtual_accessibility_node_id(&entry.key) == node_id)
    });
    if let Some(entry) = text_editor {
        match event {
            ui_wgpu::wgpu::AccessibilityUiEvent::Focus | ui_wgpu::wgpu::AccessibilityUiEvent::Activate => {
                let focus = UI_ENGINE.with(|cell| text_editor_focus_in(&cell.borrow(), window_id, window_generation, entry.scene_node, &entry.host_id))?;
                install_text_editor_focus(focus.clone());
                reset_text_editor_caret(&focus);
            }
            ui_wgpu::wgpu::AccessibilityUiEvent::Blur => {
                let focus = FOCUSED_TEXT_EDITOR.with(|cell| {
                    let mut focus = cell.borrow_mut();
                    if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == window_generation && focus.node == entry.scene_node && focus.host_id == entry.host_id) {
                        return focus.take();
                    }
                    None
                });
                if let Some(focus) = focus {
                    blur_text_editor(&focus);
                }
            }
            ui_wgpu::wgpu::AccessibilityUiEvent::Value(value) => {
                let focus = UI_ENGINE.with(|cell| text_editor_focus_in(&cell.borrow(), window_id, window_generation, entry.scene_node, &entry.host_id))?;
                let outcome = with_live_text_editor(&focus, |scene| crate::engine_canvas::text_editor_replace_all_into(scene, &value, input));
                match outcome {
                    Some(Ok(_)) => {
                        reset_text_editor_caret(&focus);
                    }
                    Some(Err(fault)) => input.record_action_fault(fault),
                    None => return None,
                }
            }
        }
        return Some(Vec::new());
    }
    let table_editable = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        table_editable_text_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.cell.key == node_key && scene_virtual_accessibility_node_id(&entry.cell.key) == node_id)
    });
    if let Some(entry) = table_editable {
        match event {
            ui_wgpu::wgpu::AccessibilityUiEvent::Focus | ui_wgpu::wgpu::AccessibilityUiEvent::Activate => {
                let result = UI_ENGINE.with(|cell| focus_table_editable_text(&cell.borrow(), window_id, window_generation, entry.scene_node, &entry.host_id, entry.cell.target.clone(), input));
                if let Err(fault) = result {
                    input.record_action_fault(fault);
                }
            }
            ui_wgpu::wgpu::AccessibilityUiEvent::Blur => {
                let outcome = UI_ENGINE.with(|cell| commit_focused_table_editable_text_with_engine(&cell.borrow(), input));
                match outcome {
                    Some(Ok(crate::scenes::TableEditableTextCommitOutcome::Conflict | crate::scenes::TableEditableTextCommitOutcome::Publishing | crate::scenes::TableEditableTextCommitOutcome::AwaitingEcho)) => {}
                    Some(Err(fault)) => input.record_action_fault(fault),
                    _ => clear_focused_table_editable_text(input),
                }
            }
            ui_wgpu::wgpu::AccessibilityUiEvent::Value(value) => {
                set_focused_table_editable_text_draft(window_id, entry.scene_node, &entry.host_id, &entry.cell.target.row_id, &entry.cell.target.column_id, &value);
                let outcome = UI_ENGINE.with(|cell| commit_focused_table_editable_text_with_engine(&cell.borrow(), input));
                if let Some(Err(fault)) = outcome {
                    input.record_action_fault(fault);
                }
            }
        }
        return Some(Vec::new());
    }
    let table_stepper = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        table_stepper_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.cell.key == node_key && scene_virtual_accessibility_node_id(&entry.cell.key) == node_id)
    });
    if let Some(entry) = table_stepper {
        match event {
            ui_wgpu::wgpu::AccessibilityUiEvent::Focus | ui_wgpu::wgpu::AccessibilityUiEvent::Activate => focus_table_stepper(window_id, window_generation, entry.scene_node, &entry.host_id, entry.cell.target),
            ui_wgpu::wgpu::AccessibilityUiEvent::Blur => {
                FOCUSED_TABLE_STEPPER.with(|cell| {
                    let mut focus = cell.borrow_mut();
                    if focus.as_ref().is_some_and(|focus| {
                        focus.window_id == window_id
                            && focus.window_generation == window_generation
                            && focus.node == entry.scene_node
                            && focus.host_id == entry.host_id
                            && focus.row_id == entry.cell.target.row_id
                            && focus.column_id == entry.cell.target.column_id
                    }) {
                        *focus = None;
                    }
                });
            }
            ui_wgpu::wgpu::AccessibilityUiEvent::Value(_) => {}
        }
        return Some(Vec::new());
    }
    let table_button = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        table_button_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.cell.key == node_key && scene_virtual_accessibility_node_id(&entry.cell.key) == node_id)
    });
    if let Some(entry) = table_button {
        if matches!(event, ui_wgpu::wgpu::AccessibilityUiEvent::Activate) {
            let outcome = UI_ENGINE.with(|cell| {
                let engine = cell.borrow();
                let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                (scene.host_id == entry.host_id && scene.component_kind == ui_wgpu::wgpu::SurfaceKind::Table).then_some(())?;
                crate::scenes::table_button_accessibility_activate(scene, &entry.cell.key, input)
            });
            match outcome {
                Some(Ok(())) => {}
                Some(Err(fault)) => input.record_action_fault(fault),
                None => return None,
            }
        }
        return Some(Vec::new());
    }
    let vfs_control = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        vfs_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.control.key == node_key && scene_virtual_accessibility_node_id(&entry.control.key) == node_id)
    });
    if let Some(entry) = vfs_control {
        let target = crate::scenes::VfsAccessibilityFocusTarget { row_id: entry.control.row_id, kind: entry.control.kind };
        match event {
            ui_wgpu::wgpu::AccessibilityUiEvent::Focus => focus_vfs_control(window_id, window_generation, entry.scene_node, &entry.host_id, target),
            ui_wgpu::wgpu::AccessibilityUiEvent::Activate => {
                focus_vfs_control(window_id, window_generation, entry.scene_node, &entry.host_id, target.clone());
                let outcome = UI_ENGINE.with(|cell| {
                    let engine = cell.borrow();
                    let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                    (scene.host_id == entry.host_id && scene.component_kind == ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem).then_some(())?;
                    crate::scenes::vfs_accessibility_activate(scene, &target, input)
                });
                match outcome {
                    Some(Ok(())) => {}
                    Some(Err(fault)) => input.record_action_fault(fault),
                    None => return None,
                }
            }
            ui_wgpu::wgpu::AccessibilityUiEvent::Blur => {
                FOCUSED_VFS_CONTROL.with(|cell| {
                    let mut focus = cell.borrow_mut();
                    if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == window_generation && focus.node == entry.scene_node && focus.host_id == entry.host_id && focus.target == target) {
                        *focus = None;
                    }
                });
            }
            ui_wgpu::wgpu::AccessibilityUiEvent::Value(_) => {}
        }
        return Some(Vec::new());
    }
    let block_list_control = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        block_list_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.control.key == node_key && scene_virtual_accessibility_node_id(&entry.control.key) == node_id)
    });
    if let Some(entry) = block_list_control {
        if matches!(event, ui_wgpu::wgpu::AccessibilityUiEvent::Activate) {
            let outcome = UI_ENGINE.with(|cell| {
                let engine = cell.borrow();
                let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                (scene.host_id == entry.host_id && scene.component_kind == ui_wgpu::wgpu::SurfaceKind::BlockList).then_some(())?;
                crate::scenes::block_list_accessibility_activate(scene, &entry.control.key, input)
            });
            match outcome {
                Some(Ok(())) => {}
                Some(Err(fault)) => input.record_action_fault(fault),
                None => return None,
            }
        }
        return Some(Vec::new());
    }
    let event_feed_control = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        event_feed_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.control.key == node_key && scene_virtual_accessibility_node_id(&entry.control.key) == node_id)
    });
    if let Some(entry) = event_feed_control {
        if matches!(event, ui_wgpu::wgpu::AccessibilityUiEvent::Activate) {
            let outcome = UI_ENGINE.with(|cell| {
                let engine = cell.borrow();
                let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                (scene.host_id == entry.host_id && scene.component_kind == ui_wgpu::wgpu::SurfaceKind::EventFeed).then_some(())?;
                crate::scenes::event_feed_accessibility_activate(scene, &entry.control.key, input)
            });
            match outcome {
                Some(Ok(())) => {}
                Some(Err(fault)) => input.record_action_fault(fault),
                None => return None,
            }
        }
        return Some(Vec::new());
    }
    let graph_timeline_control = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        graph_timeline_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.control.key == node_key && scene_virtual_accessibility_node_id(&entry.control.key) == node_id)
    });
    if let Some(entry) = graph_timeline_control {
        if matches!(event, ui_wgpu::wgpu::AccessibilityUiEvent::Activate) {
            let outcome = UI_ENGINE.with(|cell| {
                let engine = cell.borrow();
                let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                (scene.host_id == entry.host_id && scene.component_kind == ui_wgpu::wgpu::SurfaceKind::GraphTimeline).then_some(())?;
                crate::scenes::graph_timeline_accessibility_activate(scene, &entry.control.key, input)
            });
            match outcome {
                Some(Ok(())) => {}
                Some(Err(fault)) => input.record_action_fault(fault),
                None => return None,
            }
        }
        return Some(Vec::new());
    }
    let (mut commands, clock) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        let clock = stamp_window_clock(&mut engine, window_id);
        let dispatched = engine.dispatch_accessibility_event(window_id, window_generation, node_id, node_key, event)?;
        let mut commands = engine.drain_commands();
        commands.extend(dispatched);
        Some((commands, clock))
    })?;
    apply_window_clock_caret(window_id, clock.as_ref());
    apply_ui_commands(&mut commands, None, input);
    Some(commands)
}

pub fn retained_node_is_select(window_id: &str, node_id: NodeId) -> bool {
    UI_ENGINE.with(|cell| cell.borrow().tree(window_id).and_then(|tree| tree.node(node_id)).is_some_and(|node| matches!(node.spec.0, UiNode::Select(_))))
}

/// 🧩️ Closed component kind retained with a focused reconciliation key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RetainedNodeFocusKind {
    Stack,
    Text,
    Button,
    Separator,
    Input,
    Select,
    Toggle,
    KeyValue,
    Slider,
    NumberStepper,
    Ring,
    IconSelect,
    Progress,
    Field,
    Section,
    Group,
    Tree,
    Image,
    ComponentScene,
    ExternalSlot,
}

impl RetainedNodeFocusKind {
    fn from_node(node: &UiNode) -> Self {
        match node {
            UiNode::Stack(_) => Self::Stack,
            UiNode::Text(_) => Self::Text,
            UiNode::Button(_) => Self::Button,
            UiNode::Separator(_) => Self::Separator,
            UiNode::Input(_) => Self::Input,
            UiNode::Select(_) => Self::Select,
            UiNode::Toggle(_) => Self::Toggle,
            UiNode::KeyValue(_) => Self::KeyValue,
            UiNode::Slider(_) => Self::Slider,
            UiNode::NumberStepper(_) => Self::NumberStepper,
            UiNode::Ring(_) => Self::Ring,
            UiNode::IconSelect(_) => Self::IconSelect,
            UiNode::Progress(_) => Self::Progress,
            UiNode::Field(_) => Self::Field,
            UiNode::Section(_) => Self::Section,
            UiNode::Group(_) => Self::Group,
            UiNode::Tree(_) => Self::Tree,
            UiNode::Image(_) => Self::Image,
            UiNode::ComponentScene(_) => Self::ComponentScene,
            UiNode::ExternalSlot(_) => Self::ExternalSlot,
        }
    }
}

/// 🔑️ Captures the reconciliation key and component kind held by a focused retained node.
pub(crate) fn retained_node_focus_identity(window_id: &str, node_id: NodeId) -> Option<(ui_wgpu::wgpu::NodeKey, RetainedNodeFocusKind)> {
    UI_ENGINE.with(|cell| cell.borrow().tree(window_id).and_then(|tree| tree.node(node_id)).map(|node| (node.key.clone(), RetainedNodeFocusKind::from_node(&node.spec.0))))
}

/// 🎯️ Rejects stale node generations, replacement component kinds, and hidden surfaces before retained keyboard dispatch.
pub(crate) fn retained_node_has_visible_focus(window_id: &str, node_id: NodeId, expected_key: &ui_wgpu::wgpu::NodeKey, expected_kind: RetainedNodeFocusKind) -> bool {
    accessibility_window_is_visible(window_id)
        && UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            engine.window_has_focus(window_id) && engine.tree(window_id).and_then(|tree| tree.node(node_id)).is_some_and(|node| &node.key == expected_key && RetainedNodeFocusKind::from_node(&node.spec.0) == expected_kind)
        })
}

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
fn apply_ui_commands(commands: &mut Vec<ui_wgpu::wgpu::UiCommand>, pointer_id: Option<ui_render::PointerId>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    for command in commands.iter() {
        match command {
            ui_wgpu::wgpu::UiCommand::App { window_id, intent } => {
                if input.admit_intent(intent) != ui_wgpu::wgpu::UiIntentAdmission::Accepted {
                    continue;
                }
                let descriptor = intent.descriptor();
                if let Err(fault) = publish_retained_action(input, window_id, &descriptor, intent) {
                    input.record_action_fault(fault);
                    return;
                }
            }
            ui_wgpu::wgpu::UiCommand::ClipboardCopy { text, .. } | ui_wgpu::wgpu::UiCommand::ClipboardCut { text, .. } => {
                #[cfg(any(test,target_arch="wasm32"))]
                write_os_clipboard(text, input);
            }
            ui_wgpu::wgpu::UiCommand::ClipboardPasteRequested { window_id } => {
                #[cfg(any(test,target_arch="wasm32"))]
                apply_clipboard_paste_requested(window_id, input);
            }
            ui_wgpu::wgpu::UiCommand::DropCommitted { window_id, target, payload, .. } => {
                if let Err(fault) = apply_drop_committed(window_id, *target, payload, input) {
                    input.record_action_fault(fault);
                    return;
                }
            }
            ui_wgpu::wgpu::UiCommand::Scene { window_id, node, kind, rect, event, .. } => {
                apply_scene_ui_command(window_id, *node, *kind, *rect, event, pointer_id, input);
            }
            ui_wgpu::wgpu::UiCommand::DropCancelled { .. } | ui_wgpu::wgpu::UiCommand::OverlayClosed { .. } | ui_wgpu::wgpu::UiCommand::FocusChanged { .. } => {}
        }
    }
    #[cfg(all(not(test),not(target_arch="wasm32")))]
    {
        let mut index=0;
        while index<commands.len(){
            let clipboard=matches!(commands[index],ui_wgpu::wgpu::UiCommand::ClipboardCopy{..}|ui_wgpu::wgpu::UiCommand::ClipboardCut{..}|ui_wgpu::wgpu::UiCommand::ClipboardPasteRequested{..});
            if !clipboard{index+=1;continue}
            if !input.original_clipboard_has_capacity(){break}
            let command=commands.remove(index);
            let(recipient,text)=match command{
                ui_wgpu::wgpu::UiCommand::ClipboardCopy{window_id,text}|ui_wgpu::wgpu::UiCommand::ClipboardCut{window_id,text}=>{(ui_wgpu::wgpu::ClipboardIntentRecipient::Write(window_id),Some(text))},
                ui_wgpu::wgpu::UiCommand::ClipboardPasteRequested{window_id}=>(ui_wgpu::wgpu::ClipboardIntentRecipient::Window(window_id),None),
                _=>unreachable!(),
            };
            input.retain_original_clipboard_intent(recipient,text).unwrap_or_else(|_|panic!("original clipboard command lost its admitted source or fixed recipient"));
        }
    }
    drive_scene_interaction_step(input);
}

struct RetainedInputSubmission {
    token: std::num::NonZeroU64,
    pub(super) window: String,
    pub(super) draft: ui_wgpu::wgpu::engine::RetainedInputDraft,
}

struct RetainedInputReceipts {
    sequence: u64,
    pub(super) slots: [Option<RetainedInputSubmission>; ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY],
}

impl Default for RetainedInputReceipts {
    fn default() -> Self { Self { sequence: 0, slots: std::array::from_fn(|_| None) } }
}

static RETAINED_INPUT_RECEIPTS: WorkerCell<RetainedInputReceipts> = WorkerCell::new(Default::default);

struct RetainedInputReceiptReservation(Option<ui_wgpu::wgpu::ActionQueueReceipt>);

impl Drop for RetainedInputReceiptReservation {
    fn drop(&mut self) { if let Some(receipt) = self.0.take() { settle_retained_input_receipt(receipt, false); } }
}

fn reserve_retained_input_receipt(window: &str, intent: &ui_wgpu::wgpu::UiIntentCommand) -> Result<RetainedInputReceiptReservation, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(draft) = UI_ENGINE.with(|cell| cell.borrow().retained_input_draft(window, intent)) else { return Ok(RetainedInputReceiptReservation(None)) };
    RETAINED_INPUT_RECEIPTS.with(|cell| {
        let mut receipts = cell.borrow_mut();
        let slot = receipts.slots.iter().position(Option::is_none).ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
        let sequence = receipts.sequence.checked_add(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
        let token = sequence.checked_mul(ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY as u64).and_then(|prefix| prefix.checked_add(slot as u64)).and_then(std::num::NonZeroU64::new).ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
        receipts.sequence = sequence;
        receipts.slots[slot] = Some(RetainedInputSubmission { token, window: window.to_string(), draft });
        Ok(RetainedInputReceiptReservation(Some(ui_wgpu::wgpu::ActionQueueReceipt { source: ui_wgpu::wgpu::ActionQueueReceiptSource::RetainedInput, token, member: 0, abort_correlation_on_error: false })))
    })
}

/// 🛟️ Consumes one exact native input receipt without disturbing newer drafts or retained node replacements.
pub(crate) fn settle_retained_input_receipt(receipt: ui_wgpu::wgpu::ActionQueueReceipt, discard: bool) {
    if receipt.source != ui_wgpu::wgpu::ActionQueueReceiptSource::RetainedInput { return; }
    let submission = RETAINED_INPUT_RECEIPTS.with(|cell| {
        let mut receipts = cell.borrow_mut();
        let slot = receipts.slots.get_mut((receipt.token.get() % ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY as u64) as usize)?;
        if slot.as_ref()?.token != receipt.token { return None; }
        slot.take()
    });
    if discard { if let Some(submission) = submission { UI_ENGINE.with(|cell| { cell.borrow_mut().discard_retained_input_draft(&submission.window, &submission.draft); }); } }
}

fn publish_retained_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, window_id: &str, action: &ActionDescriptor, intent: &ui_wgpu::wgpu::UiIntentCommand) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let mut reservation = input.reserve_action(&action.controller_id, &action.action, ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
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
fn apply_drop_committed(window_id: &str, target: NodeId, payload: &DragPayload, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let Some(action) = drop_target_action(window_id, target) else {
        return Ok(());
    };
    let Some(patch) = decode_drop_payload(payload) else {
        return Ok(());
    };
    let mut reservation = input.reserve_action(&action.controller_id, &action.action, ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
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

fn write_json_action_value(builder: &mut ui_wgpu::wgpu::BoundedActionBuilder, key: Option<&str>, value: &Value) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    match value {
        Value::Null => builder.null(key),
        Value::Bool(value) => builder.boolean(key, *value),
        Value::Number(value) => builder.number(key, value.as_f64().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?),
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

fn drop_target_action(window_id: &str, target: NodeId) -> Option<ActionDescriptor> {
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
fn decode_drop_payload(payload: &DragPayload) -> Option<serde_json::Map<String, Value>> {
    let (_, encoded) = payload.iter().find(|(mime, value)| mime.starts_with("application/x-semio-") && !value.trim().is_empty())?;
    match serde_json::from_str::<Value>(encoded).ok()? {
        Value::Object(map) => Some(map),
        _ => None,
    }
}

/// 🔀️ `{...existing, ...patch}` (patch wins on key collision) — mirrors
/// `framework/renderer/react/index.tsx`'s `dispatchUiAction` merge order exactly.
#[cfg(test)]
fn merge_action_args(existing: Option<&semio_framework::DslValue>, patch: serde_json::Map<String, Value>) -> Option<semio_framework::DslValue> {
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

/// 📋️ Writes to the OS clipboard via `ui_wgpu::wgpu::host` — the one indirection this module's own tests
/// swap for a mock (`MOCK_CLIPBOARD_WRITES`), so `cargo test` never touches a real display/clipboard.
#[cfg(all(not(test),target_arch="wasm32"))]
fn write_os_clipboard(text: &str, _input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    #[cfg(target_arch = "wasm32")]
    {
        let text = text.to_string();
        semio_framework_async::browser::spawn_local(async move {
            ui_wgpu::wgpu::clipboard_write_text(&text).await;
        });
    }
}

#[cfg(test)]
fn write_os_clipboard(text: &str, _input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow_mut().push(text.to_string()));
}

#[cfg(all(not(target_arch="wasm32"),not(test)))]
struct ClipboardInvocationInput { original:semio_framework_job::OriginalCallerSourceLoan,source:ui_wgpu::wgpu::ClipboardIntentSource,recipient:ui_wgpu::wgpu::ClipboardIntentRecipient,slot:u8,generation:u64 }
#[cfg(all(not(target_arch="wasm32"),not(test)))]
struct MountedClipboardIo {
    invocation:ClipboardInvocationInput,
    initializer:ui_wgpu::wgpu::ClipboardIoJobInitializer,
    pub(super) params:Option<semio_framework_job_worker_authority::BatchJobParams>,
    parameter_cancel:Option<semio_framework_async::CancelTokenRetirement>,
    preparation:Option<semio_framework_job_worker::WorkerJobSessionPreparation<ui_wgpu::wgpu::ClipboardIoJob>>,
    session:Option<semio_framework_job_worker::WorkerJobSession<ui_wgpu::wgpu::ClipboardIoJob>>,
    pub(super) outcome:Option<semio_framework_job_worker::WorkerJobOutcome<ui_wgpu::wgpu::ClipboardIoJob>>,
    pub(super) copied:Option<String>,
    delivered:bool,
    closing:bool,
}
#[cfg(all(not(target_arch="wasm32"),not(test)))]
struct MountedClipboardSlot { generation:u64,mounted:Option<MountedClipboardIo> }
#[cfg(all(not(target_arch="wasm32"),not(test)))]
static MOUNTED_CLIPBOARD_IO:WorkerCell<[MountedClipboardSlot;32]>=WorkerCell::new(||std::array::from_fn(|_|MountedClipboardSlot{generation:0,mounted:None}));
#[cfg(all(not(target_arch="wasm32"),not(test)))]
fn clipboard_refusal(message:&'static str)->semio_framework_job::OriginalTurnFailure {semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message).into()}
#[cfg(all(not(target_arch="wasm32"),not(test)))]
fn clipboard_demand_failure(error:semio_framework_job_worker::WorkerJobDemandError)->semio_framework_job::OriginalTurnFailure{match error{semio_framework_job_worker::WorkerJobDemandError::Refused(error)=>error.into(),semio_framework_job_worker::WorkerJobDemandError::Contention(_)=>clipboard_refusal("clipboard original demand remains checked out")}}
#[cfg(all(not(target_arch="wasm32"),not(test)))]
fn clipboard_alias_failure(error:semio_framework_async::CancelTokenRetirementError)->semio_framework_job::OriginalTurnFailure{match error{semio_framework_async::CancelTokenRetirementError::Refused(error)=>error.into(),semio_framework_async::CancelTokenRetirementError::Blocked(_)=>clipboard_refusal("clipboard original cancellation alias remains borrowed")}}
#[cfg(all(not(target_arch="wasm32"),not(test)))]
pub(super) fn validate_clipboard_source(source:ui_wgpu::wgpu::ClipboardIntentSource,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{if source.operation!=cx.operation()||source.generation!=cx.generation()||!cx.original_source_epoch().is_some_and(|epoch|epoch>=source.epoch){return Err(clipboard_refusal("clipboard intent requires its actual initiating source and authenticated incoming epoch"))}Ok(())}
#[cfg(all(not(target_arch="wasm32"),not(test)))]
pub(super) fn submit_clipboard_io(source:&mut Option<ui_wgpu::wgpu::OriginalClipboardIntent>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let Some(intent)=source.as_ref()else{return Ok(true)};validate_clipboard_source(intent.source,cx)?;
    MOUNTED_CLIPBOARD_IO.with(|cell|{let mut slots=cell.borrow_mut();let Some((index,slot))=slots.iter_mut().enumerate().find(|(_,slot)|slot.mounted.is_none()&&slot.generation<u64::MAX)else{return Ok(false)};
        let grant=cx.retained_grant();let bytes=std::mem::size_of::<MountedClipboardIo>();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)};
        let parent=cx.original_source_parent()?;let Some(original)=parent.admit_loan(cx)?else{return Ok(false)};
        let intent=source.take().expect("admitted original clipboard intent");let invocation:ClipboardInvocationInput=ClipboardInvocationInput{original,source:intent.source,recipient:intent.recipient,slot:index as u8,generation:slot.generation+1};
        let initializer=match intent.text{Some(text)=>ui_wgpu::wgpu::ClipboardIoJobInitializer::write(text),None=>ui_wgpu::wgpu::ClipboardIoJobInitializer::read()};
        slot.generation=invocation.generation;slot.mounted=Some(MountedClipboardIo{invocation,initializer,params:None,parameter_cancel:None,preparation:None,session:None,outcome:None,copied:None,delivered:false,closing:false});
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_bytes:bytes,..Default::default()})?;Ok(true)
    })
}
#[cfg(all(not(target_arch="wasm32"),not(test)))]
pub(super) fn deliver_clipboard_recipient(recipient:&ui_wgpu::wgpu::ClipboardIntentRecipient,result:Result<Option<ui_wgpu::wgpu::ClipboardContent>,ui_wgpu::wgpu::BoundedActionFault>,input:&mut ui_wgpu::wgpu::InputState<ActionDescriptor>){
    match recipient {
        ui_wgpu::wgpu::ClipboardIntentRecipient::Write(_)=>if let Err(fault)=result{input.record_action_fault(fault)},
        ui_wgpu::wgpu::ClipboardIntentRecipient::Window(window)=>match result{Ok(Some(ui_wgpu::wgpu::ClipboardContent::Text(text)))=>UI_ENGINE.with(|cell|{let mut engine=cell.borrow_mut();let commands=engine.dispatch_event(window,ui_wgpu::wgpu::UiEvent::Paste{text});engine.enqueue_async_commands(commands)}),Err(fault)=>input.record_action_fault(fault),_=>{}},
        ui_wgpu::wgpu::ClipboardIntentRecipient::Text{slot,generation}=>complete_pending_native_text_editor_clipboard(PendingNativeTextEditorClipboardToken{slot:*slot,generation:*generation},result),
        ui_wgpu::wgpu::ClipboardIntentRecipient::Ink{slot,generation}=>complete_pending_native_ink_clipboard(PendingNativeInkClipboardToken{slot:*slot,generation:*generation},result),
    }
}
#[cfg(all(not(target_arch="wasm32"),not(test)))]
pub(super) fn pump_clipboard_io_one(input:&mut ui_wgpu::wgpu::InputState<ActionDescriptor>,cx:&mut semio_framework_job::StepContext<'_>,cleanup:bool)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    use semio_framework_job::{InteractiveJob};
use semio_framework_job_worker::{WorkerJobPoll as Poll};use semio_framework_job_worker_authority::{WorkerJobInitializer};
    MOUNTED_CLIPBOARD_IO.with(|cell|{let mut slots=cell.borrow_mut();let Some((index,slot))=slots.iter_mut().enumerate().find(|(_,slot)|slot.mounted.as_ref().is_some_and(|owner|owner.invocation.source.operation==cx.operation()&&owner.invocation.source.generation==cx.generation()))else{return Ok(false)};
        let owner=slot.mounted.as_mut().unwrap();validate_clipboard_source(owner.invocation.source,cx)?;if owner.invocation.slot as usize!=index||owner.invocation.generation!=slot.generation{return Err(clipboard_refusal("clipboard recipient generation changed before original acknowledgement"))}if !owner.invocation.original.terminal_is_empty(){owner.invocation.original.validate_context(cx)?;}
        if cx.should_yield(){return Ok(true)}if cleanup||cx.is_cancelled(){owner.closing=true;}
        if owner.closing {
            if let Some(outcome)=owner.outcome.as_mut(){let step=outcome.acknowledge_outcome(cx.retained_grant());cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);if outcome.outcome_descriptor().is_some(){return Ok(true)}owner.outcome.take().unwrap().begin_close();return Ok(true)}
            if let Some(session)=owner.session.as_mut(){session.begin_close();if session.has_original_cancel_alias_witness(cx.original_cancel_token()).map_err(clipboard_demand_failure)?{if let Some(step)=session.return_original_cancel_alias_step(cx.original_cancel_token(),cx.retained_grant()).map_err(clipboard_demand_failure)?{cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);return Ok(true)}}let _=session.close_step(cx)?;if session.terminal_is_empty(){owner.session=None;}return Ok(true)}
            if let Some(preparation)=owner.preparation.as_mut(){preparation.begin_close();let _=preparation.close_step(&mut owner.initializer,cx)?;if preparation.terminal_is_empty(){owner.preparation=None;}return Ok(true)}
            if !owner.initializer.partial_terminal_is_empty(){let demand=owner.initializer.partial_retirement_demands(cx.retained_grant().maximum_copy_bytes)?;let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|unsafe{owner.initializer.close_partial_step(std::ptr::null_mut(),grant)})?;return Ok(true)}
            if owner.params.is_some(){if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(true)}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);let params=owner.params.take().unwrap();owner.parameter_cancel=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));return Ok(true)}
            if let Some(cancel)=owner.parameter_cancel.as_mut(){let step=cancel.return_alias_step(cx.original_cancel_token(),cx.retained_grant()).map_err(clipboard_alias_failure)?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);if cancel.terminal_is_empty(){owner.parameter_cancel=None;}return Ok(true)}
            let release=match &owner.invocation.recipient{ui_wgpu::wgpu::ClipboardIntentRecipient::Write(window)|ui_wgpu::wgpu::ClipboardIntentRecipient::Window(window)=>window.capacity(),_=>0}+owner.copied.as_ref().map_or(0,String::capacity);
            if release>0{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth<2||grant.maximum_release_bytes<release{return Ok(true)}drop(std::mem::replace(&mut owner.invocation.recipient,ui_wgpu::wgpu::ClipboardIntentRecipient::Write(String::new())));drop(owner.copied.take());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()})?;cx.consume_fuel(1);return Ok(true)}
            if !owner.invocation.original.terminal_is_empty(){let parent=cx.original_source_parent()?;let _=owner.invocation.original.close_step(parent,cx)?;return Ok(true)}
            if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(true)}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);slot.mounted=None;return Ok(true)
        }
        if owner.session.is_none(){
            if owner.params.is_none(){let Some(cancel)=cx.admit_original_cancel_alias()?else{return Ok(true)};cx.consume_fuel(1);owner.params=Some(semio_framework_job_worker_authority::BatchJobParams{operation:cx.operation(),generation:cx.generation(),cancel,config:semio_framework_job_worker_authority::BatchDriveConfig{retained:cx.retained_grant(),site:"renderer_original_clipboard",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:cx.deadline_us().saturating_sub(cx.now_us().unwrap_or(cx.deadline_us()))},now_us:cx.clock_source()});return Ok(true)}
            if owner.preparation.is_none(){if let Some((preparation,_))=semio_framework_job_worker::WorkerJobSessionPreparation::try_begin(&mut owner.initializer,&mut owner.params,cx)?{owner.preparation=Some(preparation);cx.consume_fuel(1)}return Ok(true)}
            let(session,progress)=owner.preparation.as_mut().unwrap().advance(&mut owner.initializer,&mut owner.params,cx)?;cx.consume_fuel(progress.copied_items as u64);if let Some(session)=session{owner.session=Some(session);owner.preparation=None;}return Ok(true)
        }
        let session=owner.session.as_mut().unwrap();
        if owner.outcome.is_none(){match session.poll(){Poll::Idle=>{if let Some((ticket,kind))=session.try_step_on_original_caller(cx)?{let _=session.take_retained_step_receipt().map_err(|_|clipboard_refusal("clipboard original callback receipt remains checked out"))?;owner.outcome=Some(if kind==Poll::Terminal{session.take_terminal()}else{session.take_outcome(ticket)}.map_err(|_|clipboard_refusal("clipboard original outcome remains owned by its producer"))?);}return Ok(true)},Poll::Closing=>{owner.closing=true;return Ok(true)},Poll::TerminalEmpty=>{owner.session=None;owner.closing=true;return Ok(true)},_=>return Ok(true)}}
        let outcome=owner.outcome.as_mut().unwrap();
        if !owner.delivered&&outcome.outcome_descriptor().is_some_and(|descriptor|descriptor.is_acknowledged()){
            let terminal=outcome.outcome_descriptor().unwrap().is_terminal();let step=outcome.acknowledge_outcome(cx.retained_grant());cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);if outcome.outcome_descriptor().is_some(){return Ok(true)}let original=owner.outcome.take().unwrap();if terminal{original.begin_close();owner.closing=true;}else if let Err(original)=original.resume(){owner.outcome=Some(original);return Err(clipboard_refusal("clipboard acknowledged original yield remains checked out"))}return Ok(true)
        }
        if !owner.delivered{
            let Some(view)=outcome.outcome()?else{let original=owner.outcome.take().unwrap();if let Err(original)=original.resume(){owner.outcome=Some(original);return Err(clipboard_refusal("clipboard yielding original outcome refused resumption"))}return Ok(true)};
            let result=match &view{semio_framework_job::JobOutcomeView::Yield{..}=>{let step=outcome.acknowledge_outcome(cx.retained_grant());cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);if outcome.outcome_descriptor().is_some(){return Ok(true)}let original=owner.outcome.take().unwrap();if let Err(original)=original.resume(){owner.outcome=Some(original);return Err(clipboard_refusal("clipboard acknowledged yield refused original resumption"))}return Ok(true)},semio_framework_job::JobOutcomeView::Complete{..}=>{
                match ui_wgpu::wgpu::ClipboardIoJob::read_candidate(&view){Some(ui_wgpu::wgpu::ClipboardContentBorrow::Text(text))=>{
                    let grant=cx.retained_grant();if owner.copied.is_none(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<text.len(){return Ok(true)}let copied=String::with_capacity(text.len());let capacity=copied.capacity();cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);owner.copied=Some(copied);return Ok(true)}
                    let copied=owner.copied.as_mut().unwrap();if copied.len()<text.len(){let grant=cx.retained_grant();let mut end=(copied.len()+grant.maximum_copy_bytes).min(text.len());while !text.is_char_boundary(end){end-=1}let bytes=end-copied.len();if grant.maximum_items==0||grant.maximum_depth==0||bytes==0{return Ok(true)}copied.push_str(&text[copied.len()..end]);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(true)}Ok(Some(ui_wgpu::wgpu::ClipboardContent::Text(owner.copied.take().unwrap())))},Some(ui_wgpu::wgpu::ClipboardContentBorrow::ImageRgba8{..})=>Err(ui_wgpu::wgpu::BoundedActionFault::Structure),None=>Ok(None)}},_=>Err(ui_wgpu::wgpu::BoundedActionFault::Structure)};
            if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(true)}deliver_clipboard_recipient(&owner.invocation.recipient,result,input);owner.delivered=true;cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
        }
        let step=outcome.acknowledge_outcome(cx.retained_grant());cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);if outcome.outcome_descriptor().is_none(){owner.outcome.take().unwrap().begin_close();owner.closing=true;}Ok(true)
    })
}

pub(super) fn write_owned_os_clipboard(text:String,input:&mut ui_wgpu::wgpu::InputState<ActionDescriptor>){
    #[cfg(any(test,target_arch="wasm32"))]
    write_os_clipboard(&text,input);
    #[cfg(all(not(test),not(target_arch="wasm32")))]
    input.retain_original_clipboard_intent(ui_wgpu::wgpu::ClipboardIntentRecipient::Write(String::new()),Some(text)).unwrap_or_else(|_|panic!("clipboard write retained source admission is required"));
}

#[cfg(test)]
thread_local! {
    static MOCK_CLIPBOARD_WRITES: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
    static MOCK_CLIPBOARD_READ: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

/** 🧪️ Test-only native clipboard handling. The in-memory mock is deliberately synchronous so
 * focused retained-engine tests can assert the resulting paste commands without an OS clipboard or
 * worker race. Production native handling below submits the blocking read to the I/O lane. */
#[cfg(all(not(target_arch = "wasm32"), test))]
fn apply_clipboard_paste_requested(window_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    let Some(text) = read_os_clipboard() else { return };
    let mut commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window_id, ui_wgpu::wgpu::UiEvent::Paste { text }));
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
fn apply_clipboard_paste_requested(window_id: &str, _input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    let window_id = window_id.to_string();
    semio_framework_async::browser::spawn_local(async move {
        if let Some(text) = ui_wgpu::wgpu::clipboard_read_text().await {
            UI_ENGINE.with(|cell| {
                let mut engine = cell.borrow_mut();
                let commands = engine.dispatch_event(&window_id, ui_wgpu::wgpu::UiEvent::Paste { text });
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
#[cfg(test)]
fn pointer_button_from_code(code: i16) -> ui_wgpu::wgpu::PointerButton {
    match code {
        1 => ui_wgpu::wgpu::PointerButton::Middle,
        2 => ui_wgpu::wgpu::PointerButton::Secondary,
        _ => ui_wgpu::wgpu::PointerButton::Primary,
    }
}

/// 🖱️ Inverse of `pointer_button_from_code` — the DOM-standard `i16` code `scenes`' own handlers
/// expect, for `apply_scene_ui_command` to recover from a `UiCommand::Scene`'s `PointerButton`.
fn pointer_button_code(button: ui_wgpu::wgpu::PointerButton) -> i16 {
    match button {
        ui_wgpu::wgpu::PointerButton::Primary => 0,
        ui_wgpu::wgpu::PointerButton::Middle => 1,
        ui_wgpu::wgpu::PointerButton::Secondary => 2,
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
fn apply_scene_ui_command(window_id: &str, node: NodeId, kind: ui_wgpu::wgpu::SurfaceKind, rect: Rect, event: &ui_wgpu::wgpu::UiEvent, pointer_id: Option<ui_render::PointerId>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    if ui_document_close_pending_for(window_id) {
        return;
    }
    let event = match event {
        ui_wgpu::wgpu::UiEvent::PointerDown { x, y, button, modifiers } => SceneIntentEvent::PointerDown { x: *x, y: *y, button: pointer_button_code(*button), modifiers: *modifiers },
        ui_wgpu::wgpu::UiEvent::PointerUp { x, y, button, modifiers } => SceneIntentEvent::PointerUp { x: *x, y: *y, button: pointer_button_code(*button), modifiers: *modifiers },
        ui_wgpu::wgpu::UiEvent::PointerMove { x, y, modifiers } => SceneIntentEvent::PointerMove { x: *x, y: *y, modifiers: *modifiers },
        ui_wgpu::wgpu::UiEvent::Scroll { x, y, delta_y, modifiers, .. } => SceneIntentEvent::Scroll { x: *x, y: *y, delta_y: *delta_y, modifiers: *modifiers },
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
        input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::Structure);
        return;
    };
    if matches!(event, SceneIntentEvent::PointerDown { .. }) {
        if let Some(pointer_id) = pointer_id {
            if !retained_scene_target(window_id, node).is_some_and(|target| claim_scene_pointer_owner(target, pointer_id)) {
                input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
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

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs"]
mod ui_command_wiring_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🛍️wgpu-app-catalogue/🦀️.rs"]
mod app_catalogue_tests;
//#endregion RetainedEngineCutover
