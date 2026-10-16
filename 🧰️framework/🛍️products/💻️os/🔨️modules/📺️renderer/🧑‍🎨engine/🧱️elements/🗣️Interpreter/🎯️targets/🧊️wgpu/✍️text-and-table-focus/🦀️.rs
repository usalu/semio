//! ✍️ Text editor, node graph caret and table focus.
//!
//! Focus and caret state of the text editor, the node graph caret, the editable table text and the table steppers.

#[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
use crate::ink_and_vfs_focus::queue_text_editor_clipboard_read;
use crate::scene_intents_and_clipboard::{apply_presented_scene_caret, write_owned_os_clipboard};
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use crate::scene_intents_and_clipboard::read_os_clipboard;
use semio_framework_os_renderer_wgpu_contracts::{ui_document_close_pending_for, UI_ENGINE};
use crate::element_contracts::ScenePointerTarget;
use ui_wgpu::wgpu::{ActionDescriptor, NodeId, UiComponentSceneNode, UiNode};

//#region ✍️TextEditorFocus
/** ✍️ The text-editor surface the keyboard belongs to — the wgpu twin of the focused `<textarea>`
 * React's `TextEditorHost` keeps: a key event has no hit point, so the surface a keystroke is
 * addressed to is the one whose body was last pressed. Cleared implicitly whenever that node stops
 * resolving to the same live `TextEditor` scene.
 *
 * @see `🧱️elements/✏️TextEditor/🟦️.tsx` — `onKeyDown` */
#[derive(Clone)]
pub struct FocusedTextEditor {
    pub window_id: String,
    pub window_generation: u64,
    pub node: NodeId,
    pub host_id: String,
    pub surface: ui_wgpu::wgpu::UiSurfaceToken,
    pub document_id: ui_contract::UiNodeId,
}

thread_local! {
    pub static FOCUSED_TEXT_EDITOR: std::cell::RefCell<Option<FocusedTextEditor>> = const { std::cell::RefCell::new(None) };
}

pub fn text_editor_focus_in(engine: &ui_wgpu::wgpu::Ui, window_id: &str, window_generation: u64, node: NodeId, host_id: &str) -> Option<FocusedTextEditor> {
    if engine.surface_generation(window_id) != Some(window_generation) {
        return None;
    }
    let surface = engine.surface_token(window_id)?;
    let document_id = engine.presented_document_id(window_id, node)?;
    let scene = engine.tree(window_id)?.node(node).and_then(|node| match &node.spec.0 {
        UiNode::ComponentScene(scene) => Some(scene),
        _ => None,
    })?;
    (scene.component_kind == ui_wgpu::wgpu::SurfaceKind::TextEditor && scene.host_id == host_id).then(|| FocusedTextEditor { window_id: window_id.to_string(), window_generation, node, host_id: host_id.to_string(), surface, document_id })
}

pub fn install_text_editor_focus(focus: FocusedTextEditor) {
    FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = Some(focus));
}

pub fn reset_text_editor_caret(focus: &FocusedTextEditor) -> bool {
    let Some(seconds) = semio_framework_job::default_now_us().map(|now| now as f64 / 1_000_000.0) else { return false };
    let caret = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        if engine.surface_generation(&focus.window_id) != Some(focus.window_generation) {
            return None;
        }
        engine.arm_presented_scene_caret(&focus.window_id, focus.surface, focus.document_id, seconds)
    });
    caret.is_some_and(|caret| apply_presented_scene_caret(&focus.window_id, caret))
}

/// 🫥️ The editor lost keyboard focus: its open typing run ends as ONE edit (`blur`, design §13.2) and its caret clears.
pub fn blur_text_editor(focus: &FocusedTextEditor) -> bool {
    semio_framework_os_renderer_wgpu_engine_text::text_editor::end_text_editor_typing(&focus.host_id, semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::TextEditorTypingEnd::Blur);
    let caret = UI_ENGINE.with(|cell| cell.borrow_mut().clear_presented_scene_caret(&focus.window_id, focus.surface, focus.document_id));
    caret.is_some_and(|caret| apply_presented_scene_caret(&focus.window_id, caret))
}

pub fn focused_text_editor() -> Option<FocusedTextEditor> {
    FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().clone())
}

pub fn focused_text_editor_matches(expected: &FocusedTextEditor) -> bool {
    FOCUSED_TEXT_EDITOR.with(|cell| {
        cell.borrow().as_ref().is_some_and(|focus| {
            focus.window_id == expected.window_id
                && focus.window_generation == expected.window_generation
                && focus.node == expected.node
                && focus.host_id == expected.host_id
                && focus.surface == expected.surface
                && focus.document_id == expected.document_id
        })
    })
}

pub fn with_live_text_editor<T>(focus: &FocusedTextEditor, apply: impl FnOnce(&UiComponentSceneNode) -> T) -> Option<T> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&focus.window_id) != Some(focus.window_generation) || engine.surface_token(&focus.window_id) != Some(focus.surface) || ui_document_close_pending_for(&focus.window_id) {
            return None;
        }
        let tree = engine.tree(&focus.window_id)?;
        if engine.presented_node_at(&focus.window_id, focus.surface, focus.document_id) != Some(focus.node) {
            return None;
        }
        let retained = tree.node(focus.node)?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
        (scene.component_kind == ui_wgpu::wgpu::SurfaceKind::TextEditor && scene.host_id == focus.host_id && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&focus.host_id)).then(|| apply(scene))
    })
}

pub fn apply_text_editor_text_at(focus: &FocusedTextEditor, text: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    if !focused_text_editor_matches(focus) {
        return false;
    }
    let outcome = with_live_text_editor(focus, |scene| semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_replace_selection_into(scene, text, input));
    match outcome {
        Some(Ok(consumed)) => consumed,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => false,
    }
}

fn apply_focused_text_editor_clipboard_key(key: &ui_wgpu::wgpu::KeyAction, modifiers: &ui_wgpu::wgpu::PointerModifiers, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Option<bool> {
    if !modifiers.ctrl_or_meta() || modifiers.alt || modifiers.shift {
        return None;
    }
    let ui_wgpu::wgpu::KeyAction::Char(ch) = key else { return None };
    let cut = ch.eq_ignore_ascii_case("x");
    let paste = ch.eq_ignore_ascii_case("v");
    if !cut && !paste && !ch.eq_ignore_ascii_case("c") {
        return None;
    }
    let Some(focus) = focused_text_editor() else { return Some(false) };
    if paste {
        if with_live_text_editor(&focus, |_| ()).is_none() {
            FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
            return Some(false);
        }
        #[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
        if let Some(text) = read_os_clipboard() {
            apply_text_editor_text_at(&focus, &text, input);
        }
        #[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
        if let Err(fault) = queue_text_editor_clipboard_read(focus,input) {
            input.record_action_fault(fault);
        }
        return Some(true);
    }
    let outcome = with_live_text_editor(&focus, |scene| {
        let Some(text) = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_selection_text(scene) else { return Ok(false) };
        write_owned_os_clipboard(text, input);
        if cut {
            semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_replace_selection_into(scene, "", input)
        } else {
            Ok(true)
        }
    });
    Some(match outcome {
        Some(Ok(consumed)) => consumed,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => {
            FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
            false
        }
    })
}

/// ⌨️ Commits host-delivered paste or composition text into the exact accepted editor focus.
pub fn apply_focused_text_editor_text(text: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    let Some(focus) = focused_text_editor() else { return false };
    let consumed = apply_text_editor_text_at(&focus, text, input);
    if consumed {
        reset_text_editor_caret(&focus);
    }
    if !consumed && with_live_text_editor(&focus, |_| ()).is_none() {
        FOCUSED_TEXT_EDITOR.with(|cell| *cell.borrow_mut() = None);
        blur_text_editor(&focus);
    }
    consumed
}

/// 🖱️ A primary press outside the focused mounted editor transfers keyboard ownership.
pub fn blur_focused_text_editor_for_pointer(target: Option<&ScenePointerTarget>, down: bool, button: i16) -> bool {
    if !down || button != 0 {
        return false;
    }
    let retired = FOCUSED_TEXT_EDITOR.with(|cell| {
        let mut focused = cell.borrow_mut();
        let Some(focus) = focused.as_ref() else { return None };
        if target
            .is_some_and(|target| target.kind == ui_wgpu::wgpu::SurfaceKind::TextEditor && target.window_id == focus.window_id && target.window_generation == focus.window_generation && target.node == focus.node && target.host_id == focus.host_id)
        {
            return None;
        }
        focused.take()
    });
    let Some(retired) = retired else { return false };
    blur_text_editor(&retired);
    true
}

#[derive(Clone)]
pub struct FocusedNodeGraphCaret {
    pub target: ScenePointerTarget,
    pub surface: ui_wgpu::wgpu::UiSurfaceToken,
}

thread_local! {
    pub static FOCUSED_NODE_GRAPH_CARET: std::cell::RefCell<Option<FocusedNodeGraphCaret>> = const { std::cell::RefCell::new(None) };
}

fn node_graph_caret_focus_in(engine: &ui_wgpu::wgpu::Ui, target: &ScenePointerTarget) -> Option<FocusedNodeGraphCaret> {
    if target.kind != ui_wgpu::wgpu::SurfaceKind::NodeGraph || engine.surface_generation(&target.window_id) != Some(target.window_generation) {
        return None;
    }
    let surface = engine.surface_token(&target.window_id)?;
    if engine.presented_node_at(&target.window_id, surface, target.document_id) != Some(target.node) || !engine.presented_component_scene_matches(&target.component_witness()) {
        return None;
    }
    Some(FocusedNodeGraphCaret { target: target.clone(), surface })
}

fn with_live_node_graph_caret<T>(focus: &FocusedNodeGraphCaret, apply: impl FnOnce(&UiComponentSceneNode) -> T) -> Option<T> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&focus.target.window_id) != Some(focus.target.window_generation)
            || engine.surface_token(&focus.target.window_id) != Some(focus.surface)
            || engine.presented_node_at(&focus.target.window_id, focus.surface, focus.target.document_id) != Some(focus.target.node)
            || !engine.presented_component_scene_matches(&focus.target.component_witness())
        {
            return None;
        }
        let retained = engine.tree(&focus.target.window_id)?.node(focus.target.node)?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
        Some(apply(scene))
    })
}

fn reset_node_graph_caret(focus: &FocusedNodeGraphCaret) -> bool {
    let Some(seconds) = semio_framework_job::default_now_us().map(|now| now as f64 / 1_000_000.0) else { return false };
    let caret = UI_ENGINE.with(|cell| cell.borrow_mut().arm_presented_scene_caret(&focus.target.window_id, focus.surface, focus.target.document_id, seconds));
    caret.is_some_and(|caret| apply_presented_scene_caret(&focus.target.window_id, caret))
}

pub fn clear_node_graph_caret(focus: &FocusedNodeGraphCaret) -> bool {
    let caret = UI_ENGINE.with(|cell| cell.borrow_mut().clear_presented_scene_caret(&focus.target.window_id, focus.surface, focus.target.document_id));
    caret.is_some_and(|caret| apply_presented_scene_caret(&focus.target.window_id, caret))
}

/// 📝 Arms cadence only after the exact accepted NodeGraph enters note editing.
pub fn refresh_node_graph_caret_after_pointer(target: &ScenePointerTarget) -> bool {
    let focus = UI_ENGINE.with(|cell| node_graph_caret_focus_in(&cell.borrow(), target));
    let editable = focus.as_ref().is_some_and(|focus| with_live_node_graph_caret(focus, semio_framework_os_renderer_wgpu_engine_graph::node_graph::component_scene_has_editable_caret).unwrap_or(false));
    if editable {
        let focus = focus.expect("editable NodeGraph focus");
        FOCUSED_NODE_GRAPH_CARET.with(|cell| *cell.borrow_mut() = Some(focus.clone()));
        return reset_node_graph_caret(&focus);
    }
    let retired = FOCUSED_NODE_GRAPH_CARET.with(|cell| cell.borrow_mut().take());
    retired.as_ref().is_some_and(clear_node_graph_caret)
}

/// 🖱️ Transfers NodeGraph note focus only on a primary press outside the exact accepted scene.
pub fn blur_focused_node_graph_caret_for_pointer(target: Option<&ScenePointerTarget>, down: bool, button: i16) -> bool {
    if !down || button != 0 {
        return false;
    }
    let retired = FOCUSED_NODE_GRAPH_CARET.with(|cell| {
        let mut focused = cell.borrow_mut();
        let Some(focus) = focused.as_ref() else { return None };
        if target.is_some_and(|target| target.kind == ui_wgpu::wgpu::SurfaceKind::NodeGraph && target.same_component_host(&focus.target)) {
            return None;
        }
        focused.take()
    });
    retired.as_ref().is_some_and(clear_node_graph_caret)
}

/// ⌨️ Routes note editing to one exact accepted NodeGraph and resets its solid caret phase.
pub fn apply_focused_node_graph_note_key(action: &ui_wgpu::wgpu::KeyAction, modifiers: &ui_wgpu::wgpu::PointerModifiers) -> bool {
    let Some(focus) = FOCUSED_NODE_GRAPH_CARET.with(|cell| cell.borrow().clone()) else { return false };
    let applied = with_live_node_graph_caret(&focus, |scene| semio_framework_os_renderer_wgpu_engine_graph::node_graph::component_scene_apply_note_edit_key(scene, action.clone(), modifiers)).unwrap_or(false);
    if !applied {
        if with_live_node_graph_caret(&focus, |_| ()).is_none() {
            FOCUSED_NODE_GRAPH_CARET.with(|cell| *cell.borrow_mut() = None);
            clear_node_graph_caret(&focus);
        }
        return false;
    }
    if with_live_node_graph_caret(&focus, semio_framework_os_renderer_wgpu_engine_graph::node_graph::component_scene_has_editable_caret).unwrap_or(false) {
        reset_node_graph_caret(&focus);
    } else {
        FOCUSED_NODE_GRAPH_CARET.with(|cell| *cell.borrow_mut() = None);
        clear_node_graph_caret(&focus);
    }
    true
}

/// 🍿️ The POPUPS see the key first: `Ctrl/Cmd+Space` opens completions, `F2` starts a rename,
/// and while either is open its own arrow/commit/dismiss keys win over editing — React's own
/// `onKeyDown` prelude order (`🧱️elements/✏️TextEditor/🟦️.tsx:556-608`). A key the popups do
/// not claim falls through to the buffer unchanged.
/** ⌨️ Applies one key to the focused text editor, returning whether it consumed the key. The ONLY
 * production caller of `engine_canvas::text_editor_apply_key_into`: the renderer's own `handle_key`
 * offers every key here before the shell's chord table sees it, so typing into an editor never
 * reaches a global accelerator — the same precedence React gets for free from DOM focus. */
pub fn apply_focused_text_editor_key(key: &ui_wgpu::wgpu::KeyAction, modifiers: &ui_wgpu::wgpu::PointerModifiers, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    if let Some(consumed) = apply_focused_text_editor_clipboard_key(key, modifiers, input) {
        return consumed;
    }
    let Some(focus) = focused_text_editor() else {
        return false;
    };
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&focus.window_id) != Some(focus.window_generation) || engine.surface_token(&focus.window_id) != Some(focus.surface) {
            return None;
        }
        if ui_document_close_pending_for(&focus.window_id) {
            return None;
        }
        let tree = engine.tree(&focus.window_id)?;
        if engine.presented_node_at(&focus.window_id, focus.surface, focus.document_id) != Some(focus.node) {
            return None;
        }
        let retained = tree.node(focus.node)?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else {
            return None;
        };
        if scene.component_kind != ui_wgpu::wgpu::SurfaceKind::TextEditor || scene.host_id != focus.host_id || semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&focus.host_id) {
            return None;
        }
        match semio_framework_os_renderer_wgpu_scene_text_vfs::text_editor_popup_key(scene, key, modifiers, input) {
            Ok(true) => return Some(Ok(true)),
            Ok(false) => {}
            Err(fault) => return Some(Err(fault)),
        }
        Some(semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_apply_key_into(scene, key, modifiers, input))
    });
    match outcome {
        Some(Ok(consumed)) => {
            if consumed {
                reset_text_editor_caret(&focus);
            }
            consumed
        }
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => {
            let retired = FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow_mut().take());
            if let Some(retired) = retired {
                blur_text_editor(&retired);
            }
            false
        }
    }
}
//#endregion ✍️TextEditorFocus

//#region ✏️TableEditableTextFocus
struct FocusedTableEditableText {
    pub window_id: String,
    pub window_generation: u64,
    pub node: NodeId,
    pub host_id: String,
    pub row_id: String,
    pub column_id: String,
    pub base: String,
    pub draft: String,
    pub awaiting_echo: Option<String>,
}

thread_local! {
    pub static FOCUSED_TABLE_EDITABLE_TEXT: std::cell::RefCell<Option<FocusedTableEditableText>> = const { std::cell::RefCell::new(None) };
}

pub fn focused_table_editable_text_address() -> Option<(String, u64, NodeId, String, String, String, String, String, Option<String>)> {
    FOCUSED_TABLE_EDITABLE_TEXT
        .with(|cell| cell.borrow().as_ref().map(|focus| (focus.window_id.clone(), focus.window_generation, focus.node, focus.host_id.clone(), focus.row_id.clone(), focus.column_id.clone(), focus.base.clone(), focus.draft.clone(), focus.awaiting_echo.clone())))
}

pub fn sync_focused_table_editable_text_draft(input: &ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| {
        if let Some(focus) = cell.borrow_mut().as_mut() {
            focus.draft = input.text_view().to_owned();
        }
    });
}

pub fn focused_table_editable_text_draft(window_id: &str, node: NodeId, host_id: &str, row_id: &str, column_id: &str) -> Option<String> {
    FOCUSED_TABLE_EDITABLE_TEXT
        .with(|cell| cell.borrow().as_ref().filter(|focus| focus.window_id == window_id && focus.node == node && focus.host_id == host_id && focus.row_id == row_id && focus.column_id == column_id).map(|focus| focus.draft.clone()))
}

pub fn set_focused_table_editable_text_draft(window_id: &str, node: NodeId, host_id: &str, row_id: &str, column_id: &str, draft: &str) {
    FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| {
        if let Some(focus) = cell.borrow_mut().as_mut().filter(|focus| focus.window_id == window_id && focus.node == node && focus.host_id == host_id && focus.row_id == row_id && focus.column_id == column_id) {
            focus.draft = draft.to_owned();
        }
    });
}

pub fn commit_focused_table_editable_text_with_engine(engine: &ui_wgpu::wgpu::Ui, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Option<Result<semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome, ui_wgpu::wgpu::BoundedActionFault>> {
    let (window_id, window_generation, node, host_id, row_id, column_id, base, draft, awaiting_echo) = focused_table_editable_text_address()?;
    if engine.surface_generation(&window_id) != Some(window_generation) || ui_document_close_pending_for(&window_id) {
        return None;
    }
    let retained = engine.tree(&window_id).and_then(|tree| tree.node(node))?;
    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
    if scene.component_kind != ui_wgpu::wgpu::SurfaceKind::Table || scene.host_id != host_id || semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&host_id) {
        return None;
    }
    let outcome = semio_framework_os_renderer_wgpu_scene_tables::table_editable_text_commit_owned(scene, &row_id, &column_id, &base, draft.clone(), awaiting_echo.as_deref(), input);
    if let Some(Ok(outcome)) = &outcome {
        FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| {
            let mut focus = cell.borrow_mut();
            let Some(focus) = focus.as_mut().filter(|focus| focus.window_id == window_id && focus.window_generation == window_generation && focus.node == node && focus.host_id == host_id && focus.row_id == row_id && focus.column_id == column_id) else { return };
            match outcome {
                semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Publishing => {
                    if let Some(accepted) = focus.awaiting_echo.take() {
                        focus.base = accepted;
                    }
                    focus.awaiting_echo = Some(draft);
                }
                semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Unchanged => {
                    if let Some(accepted) = focus.awaiting_echo.take() {
                        focus.base = accepted;
                    }
                }
                _ => {}
            }
        });
    }
    outcome
}

pub fn clear_focused_table_editable_text(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| *cell.borrow_mut() = None);
    input.blur_input();
}

pub fn focus_table_editable_text(
    engine: &ui_wgpu::wgpu::Ui,
    window_id: &str,
    window_generation: u64,
    node: NodeId,
    host_id: &str,
    target: semio_framework_os_renderer_wgpu_scene_state::TableEditableTextFocusTarget,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let same = focused_table_editable_text_address().is_some_and(|(focused_window, focused_generation, focused_node, focused_host, focused_row, focused_column, _, _, _)| {
        focused_window == window_id && focused_generation == window_generation && focused_node == node && focused_host == host_id && focused_row == target.row_id && focused_column == target.column_id
    });
    if same {
        return Ok(());
    }
    if let Some(outcome) = commit_focused_table_editable_text_with_engine(engine, input).transpose()? {
        if matches!(outcome, semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Conflict | semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Publishing | semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::AwaitingEcho) {
            return Ok(());
        }
    }
    clear_focused_table_editable_text(input);
    let control_id = target.control_id(host_id);
    input.focus_input_owned(control_id, target.value.clone());
    FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| {
        *cell.borrow_mut() =
            Some(FocusedTableEditableText { window_id: window_id.to_owned(), window_generation, node, host_id: host_id.to_owned(), row_id: target.row_id, column_id: target.column_id, base: target.value.clone(), draft: target.value, awaiting_echo: None });
    });
    Ok(())
}

/** 🖱️ Commits an editable cell when a primary press leaves its Table component. */
pub fn blur_focused_table_editable_text_for_pointer(target: Option<&ScenePointerTarget>, down: bool, button: i16, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    if !down || button != 0 {
        return false;
    }
    let Some((window_id, window_generation, node, host_id, _, _, _, _, _)) = focused_table_editable_text_address() else { return false };
    if target.is_some_and(|target| target.kind == ui_wgpu::wgpu::SurfaceKind::Table && target.window_id == window_id && target.window_generation == window_generation && target.node == node && target.host_id == host_id) {
        return false;
    }
    let outcome = UI_ENGINE.with(|cell| commit_focused_table_editable_text_with_engine(&cell.borrow(), input));
    match outcome {
        Some(Ok(semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Conflict | semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Publishing | semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::AwaitingEcho)) => true,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        _ => {
            clear_focused_table_editable_text(input);
            true
        }
    }
}

/** ⌨️ Enter commits, Shift+Enter inserts a newline, and Escape discards the focused cell draft. */
pub fn apply_focused_table_editable_text_key(key: &ui_wgpu::wgpu::KeyAction, modifiers: &ui_wgpu::wgpu::PointerModifiers, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    if focused_table_editable_text_address().is_none() {
        return false;
    }
    match key {
        ui_wgpu::wgpu::KeyAction::Escape => {
            input.cancel_retained_action();
            clear_focused_table_editable_text(input);
            true
        }
        ui_wgpu::wgpu::KeyAction::Enter if modifiers.shift => {
            input.insert_char('\n');
            true
        }
        ui_wgpu::wgpu::KeyAction::Enter => {
            let outcome = UI_ENGINE.with(|cell| commit_focused_table_editable_text_with_engine(&cell.borrow(), input));
            match outcome {
                Some(Ok(semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Conflict | semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Publishing | semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::AwaitingEcho)) => true,
                Some(Err(fault)) => {
                    input.record_action_fault(fault);
                    true
                }
                _ => {
                    clear_focused_table_editable_text(input);
                    true
                }
            }
        }
        _ => false,
    }
}
//#endregion ✏️TableEditableTextFocus

//#region 📊️TableStepperFocus
struct FocusedTableStepper {
    pub window_id: String,
    pub window_generation: u64,
    pub node: NodeId,
    pub host_id: String,
    pub row_id: String,
    pub column_id: String,
}

thread_local! {
    pub static FOCUSED_TABLE_STEPPER: std::cell::RefCell<Option<FocusedTableStepper>> = const { std::cell::RefCell::new(None) };
}

pub fn focus_table_stepper(window_id: &str, window_generation: u64, node: NodeId, host_id: &str, target: semio_framework_os_renderer_wgpu_scene_state::TableStepperFocusTarget) {
    FOCUSED_TABLE_STEPPER.with(|cell| {
        *cell.borrow_mut() = Some(FocusedTableStepper { window_id: window_id.to_owned(), window_generation, node, host_id: host_id.to_owned(), row_id: target.row_id, column_id: target.column_id });
    });
}

/// 🖱️ A primary press transfers DOM-style focus away from the current Table spinbutton;
/// the accepted centre-segment intent re-arms it later in the same pointer turn when appropriate.
pub fn blur_focused_table_stepper_for_pointer(down: bool, button: i16) -> bool {
    down && button == 0 && FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow_mut().take().is_some())
}

/** ⌨️ Offers a key to the accepted Table stepper focus. The retained row/column address is
 * re-resolved against the live scene before the current cell descriptor is dispatched, so a
 * replaced row, column, component or window generation cannot receive a delayed key. */
pub fn apply_focused_table_stepper_key(key: &ui_wgpu::wgpu::KeyAction, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    let focus = FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().map(|focus| (focus.window_id.clone(), focus.window_generation, focus.node, focus.host_id.clone(), focus.row_id.clone(), focus.column_id.clone())));
    let Some((window_id, window_generation, node, host_id, row_id, column_id)) = focus else {
        return false;
    };
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&window_id) != Some(window_generation) || ui_document_close_pending_for(&window_id) {
            return None;
        }
        let retained = engine.tree(&window_id).and_then(|tree| tree.node(node))?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
        if scene.component_kind != ui_wgpu::wgpu::SurfaceKind::Table || scene.host_id != host_id || semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&host_id) {
            return None;
        }
        semio_framework_os_renderer_wgpu_scene_tables::table_stepper_apply_key(scene, &row_id, &column_id, key, input)
    });
    match outcome {
        Some(Ok(semio_framework_os_renderer_wgpu_scene_tables::TableStepperKeyOutcome::Consumed)) => true,
        Some(Ok(semio_framework_os_renderer_wgpu_scene_tables::TableStepperKeyOutcome::Unhandled)) => false,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => {
            FOCUSED_TABLE_STEPPER.with(|cell| *cell.borrow_mut() = None);
            false
        }
    }
}
//#endregion 📊️TableStepperFocus
