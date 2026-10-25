//! 🖋️ Virtual file system, ink editor and ink clipboard focus.
//!
//! Focus state of the virtual file system controls and the ink editor, plus the ink and text editor native
//! clipboard streams that follow a focus change.

#[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
use crate::scene_intents_and_clipboard::{deliver_clipboard_recipient, pump_clipboard_io_one, submit_clipboard_io, validate_clipboard_source};
use semio_framework_os_renderer_wgpu_contracts::{ui_document_close_pending_for, UI_ENGINE};
use crate::scene_intents_and_clipboard::write_owned_os_clipboard;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use crate::scene_intents_and_clipboard::read_os_clipboard;
#[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
use crate::text_and_table_focus::apply_text_editor_text_at;
use crate::text_and_table_focus::{clear_node_graph_caret, focused_text_editor, focused_text_editor_matches, with_live_text_editor, FocusedTextEditor, FOCUSED_NODE_GRAPH_CARET, FOCUSED_TABLE_EDITABLE_TEXT, FOCUSED_TABLE_STEPPER, FOCUSED_TEXT_EDITOR};
#[cfg(not(target_arch = "wasm32"))]
use crate::element_contracts::WorkerCell;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_arena::NodeId;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_declaration::ui::{UiComponentSceneNode, UiNode};

//#region 🗂️VirtualFileSystemFocus
struct FocusedVfsControl {
    pub window_id: String,
    pub window_generation: u64,
    pub node: NodeId,
    pub host_id: String,
    pub target: semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityFocusTarget,
}

thread_local! {
    pub static FOCUSED_VFS_CONTROL: std::cell::RefCell<Option<FocusedVfsControl>> = const { std::cell::RefCell::new(None) };
}

pub fn focus_vfs_control(window_id: &str, window_generation: u64, node: NodeId, host_id: &str, target: semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityFocusTarget) {
    FOCUSED_VFS_CONTROL.with(|cell| {
        *cell.borrow_mut() = Some(FocusedVfsControl { window_id: window_id.to_owned(), window_generation, node, host_id: host_id.to_owned(), target });
    });
}

pub fn focused_vfs_control() -> Option<(String, u64, NodeId, String, semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityFocusTarget)> {
    FOCUSED_VFS_CONTROL.with(|cell| cell.borrow().as_ref().map(|focus| (focus.window_id.clone(), focus.window_generation, focus.node, focus.host_id.clone(), focus.target.clone())))
}

pub fn blur_focused_vfs_control_for_pointer(down: bool, button: i16) -> bool {
    down && button == 0 && FOCUSED_VFS_CONTROL.with(|cell| cell.borrow_mut().take().is_some())
}

pub fn apply_focused_vfs_control_key(key: &semio_framework_ui_wgpu_input::KeyAction, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let Some((window_id, window_generation, node, host_id, target)) = focused_vfs_control() else { return false };
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&window_id) != Some(window_generation) || ui_document_close_pending_for(&window_id) {
            return None;
        }
        let retained = engine.tree(&window_id).and_then(|tree| tree.node(node))?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
        if scene.component_kind != semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem || scene.host_id != host_id || semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&host_id) {
            return None;
        }
        semio_framework_os_renderer_wgpu_scene_text_vfs::vfs_accessibility_apply_key(scene, &target, key, input)
    });
    match outcome {
        Some(Ok(semio_framework_os_renderer_wgpu_scene_text_vfs::VfsAccessibilityKeyOutcome::Consumed)) => true,
        Some(Ok(semio_framework_os_renderer_wgpu_scene_text_vfs::VfsAccessibilityKeyOutcome::Unhandled)) => false,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => {
            FOCUSED_VFS_CONTROL.with(|cell| *cell.borrow_mut() = None);
            false
        }
    }
}
//#endregion 🗂️VirtualFileSystemFocus

//#region 🖋️InkEditorFocus
struct FocusedInkEditor {
    pub window_id: String,
    pub window_generation: u64,
    pub node: NodeId,
    pub host_id: String,
}

thread_local! {
    pub static FOCUSED_INK_EDITOR: std::cell::RefCell<Option<FocusedInkEditor>> = const { std::cell::RefCell::new(None) };
}

pub fn focus_ink_editor(window_id: &str, window_generation: u64, node: NodeId, host_id: &str) {
    FOCUSED_INK_EDITOR.with(|cell| *cell.borrow_mut() = Some(FocusedInkEditor { window_id: window_id.to_owned(), window_generation, node, host_id: host_id.to_owned() }));
}

pub fn focused_ink_editor() -> Option<(String, u64, NodeId, String)> {
    FOCUSED_INK_EDITOR.with(|cell| cell.borrow().as_ref().map(|focus| (focus.window_id.clone(), focus.window_generation, focus.node, focus.host_id.clone())))
}

fn clear_focused_ink_editor(host_id: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) {
    semio_framework_os_renderer_wgpu_scene_ink::cancel_ink_edit(host_id, input);
    FOCUSED_INK_EDITOR.with(|cell| *cell.borrow_mut() = None);
}

pub fn apply_focused_ink_editor_key(key: &semio_framework_ui_wgpu_input::KeyAction, modifiers: &semio_framework_ui_wgpu_input::PointerModifiers, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let Some((window_id, generation, node, host_id)) = focused_ink_editor() else { return false };
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&window_id) != Some(generation) {
            return None;
        }
        if ui_document_close_pending_for(&window_id) {
            return None;
        }
        let retained = engine.tree(&window_id).and_then(|tree| tree.node(node))?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
        if scene.component_kind != semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas || scene.host_id != host_id || semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&host_id) {
            return None;
        }
        Some(semio_framework_os_renderer_wgpu_scene_ink::ink_edit_key_into(scene, key, modifiers, input))
    });
    match outcome {
        Some(Ok(consumed)) => {
            if !semio_framework_os_renderer_wgpu_scene_ink::ink_edit_active(&host_id) {
                FOCUSED_INK_EDITOR.with(|cell| *cell.borrow_mut() = None);
            }
            consumed
        }
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => {
            clear_focused_ink_editor(&host_id, input);
            false
        }
    }
}

pub fn blur_focused_ink_editor_at(x: f32, y: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let Some((window_id, generation, node, host_id)) = focused_ink_editor() else { return false };
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&window_id) != Some(generation) {
            return None;
        }
        if ui_document_close_pending_for(&window_id) {
            return None;
        }
        let retained = engine.tree(&window_id).and_then(|tree| tree.node(node))?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
        if scene.component_kind != semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas || scene.host_id != host_id || semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&host_id) {
            return None;
        }
        Some(semio_framework_os_renderer_wgpu_scene_ink::ink_edit_blur_into(scene, x, y, input))
    });
    match outcome {
        Some(Ok(committed)) => {
            if committed {
                FOCUSED_INK_EDITOR.with(|cell| *cell.borrow_mut() = None);
            }
            committed
        }
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => {
            clear_focused_ink_editor(&host_id, input);
            false
        }
    }
}
//#endregion 🖋️InkEditorFocus

//#region 📋️InkClipboardFocus
#[derive(Clone)]
pub struct InkClipboardAddress {
    pub window_id: String,
    pub window_generation: u64,
    pub node: NodeId,
    pub host_id: String,
    pub rect: Rect,
}

thread_local! {
    pub static FOCUSED_INK_SURFACE: std::cell::RefCell<Option<InkClipboardAddress>> = const { std::cell::RefCell::new(None) };
}

pub fn focus_ink_surface(window_id: &str, window_generation: u64, node: NodeId, host_id: &str, rect: Rect) {
    FOCUSED_INK_SURFACE.with(|cell| *cell.borrow_mut() = Some(InkClipboardAddress { window_id: window_id.to_owned(), window_generation, node, host_id: host_id.to_owned(), rect }));
}

pub fn focused_ink_surface() -> Option<InkClipboardAddress> {
    FOCUSED_INK_SURFACE.with(|cell| cell.borrow().clone())
}

pub fn with_live_ink_surface<T>(address: &InkClipboardAddress, apply: impl FnOnce(&UiComponentSceneNode) -> T) -> Option<T> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if ui_document_close_pending_for(&address.window_id) {
            return None;
        }
        if engine.surface_generation(&address.window_id) != Some(address.window_generation) {
            return None;
        }
        let retained = engine.tree(&address.window_id).and_then(|tree| tree.node(address.node))?;
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
        (scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas && scene.host_id == address.host_id && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&address.host_id)).then(|| apply(scene))
    })
}

fn focused_ink_editor_owns(address: &InkClipboardAddress) -> bool {
    focused_ink_editor().is_some_and(|(window_id, generation, node, host_id)| window_id == address.window_id && generation == address.window_generation && node == address.node && host_id == address.host_id && semio_framework_os_renderer_wgpu_scene_ink::ink_edit_active(&host_id))
}

fn apply_ink_clipboard_content(address: &InkClipboardAddress, kind: semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind, content: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let editor = focused_ink_editor_owns(address);
    let result = with_live_ink_surface(address, |scene| {
        if editor {
            if kind == semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind::Text {
                semio_framework_os_renderer_wgpu_scene_ink::ink_edit_paste_into(scene, content)
            } else {
                Ok(false)
            }
        } else {
            let target = semio_framework_os_renderer_wgpu_scene_ink::ink_clipboard_target(scene, address.rect)?;
            semio_framework_os_renderer_wgpu_scene_ink::ink_clipboard_paste_into(scene, target, kind, content, input)
        }
    });
    match result {
        Some(Ok(consumed)) => consumed,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => {
            FOCUSED_INK_SURFACE.with(|cell| *cell.borrow_mut() = None);
            false
        }
    }
}

pub fn apply_focused_ink_paste(text: &str, image_data_url: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let Some(address) = focused_ink_surface() else { return false };
    let kind = if image_data_url { semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind::ImageDataUrl } else { semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind::Text };
    apply_ink_clipboard_content(&address, kind, text, input)
}

#[cfg(not(target_arch = "wasm32"))]
struct PendingNativeInkClipboard {
    pub source:semio_framework_ui_wgpu_input::ClipboardIntentSource,
    pub address: InkClipboardAddress,
    pub outcome: Option<Result<Option<semio_framework_ui_wgpu_hosting::host::ClipboardContent>, semio_framework_ui_action::BoundedActionFault>>,
}

#[cfg(not(target_arch = "wasm32"))]
struct PendingNativeInkClipboardSlot {
    pub generation: u64,
    pub mounted: Option<PendingNativeInkClipboard>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
pub struct PendingNativeInkClipboardToken {
    pub slot: u8,
    pub generation: u64,
}

#[cfg(not(target_arch = "wasm32"))]
pub static PENDING_NATIVE_INK_CLIPBOARD:WorkerCell<[PendingNativeInkClipboardSlot;32]>=WorkerCell::new(||std::array::from_fn(|_|PendingNativeInkClipboardSlot{generation:0,mounted:None}));

#[cfg(not(target_arch = "wasm32"))]
pub fn complete_pending_native_ink_clipboard(token: PendingNativeInkClipboardToken, result: Result<Option<semio_framework_ui_wgpu_hosting::host::ClipboardContent>, semio_framework_ui_action::BoundedActionFault>) {
    PENDING_NATIVE_INK_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.get_mut(usize::from(token.slot)).filter(|slot| slot.generation == token.generation) else { return };
        if let Some(pending) = slot.mounted.as_mut() {
            pending.outcome = Some(result);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn reserve_pending_native_ink_clipboard(address: InkClipboardAddress,source:semio_framework_ui_wgpu_input::ClipboardIntentSource) -> Option<PendingNativeInkClipboardToken> {
    PENDING_NATIVE_INK_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let (index, slot) = slots.iter_mut().enumerate().find(|(_, slot)| slot.mounted.is_none() && slot.generation != u64::MAX)?;
        slot.generation = slot.generation.checked_add(1)?;
        slot.mounted = Some(PendingNativeInkClipboard { source,address, outcome: None });
        Some(PendingNativeInkClipboardToken { slot: u8::try_from(index).ok()?, generation: slot.generation })
    })
}

#[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
fn queue_ink_clipboard_read(address: InkClipboardAddress,input:&mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let source=input.original_clipboard_source.ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
    let Some(token) = reserve_pending_native_ink_clipboard(address,source) else { return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits) };
    input.retain_original_clipboard_intent(semio_framework_ui_wgpu_input::ClipboardIntentRecipient::Ink{slot:token.slot,generation:token.generation},None).unwrap_or_else(|_|panic!("ink clipboard original source admission required"));
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
struct PendingNativeTextEditorClipboard {
    pub source:semio_framework_ui_wgpu_input::ClipboardIntentSource,
    pub focus: FocusedTextEditor,
    pub outcome: Option<Result<Option<semio_framework_ui_wgpu_hosting::host::ClipboardContent>, semio_framework_ui_action::BoundedActionFault>>,
}

#[cfg(not(target_arch = "wasm32"))]
struct PendingNativeTextEditorClipboardSlot {
    pub generation: u64,
    pub mounted: Option<PendingNativeTextEditorClipboard>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
pub struct PendingNativeTextEditorClipboardToken {
    pub slot: u8,
    pub generation: u64,
}

#[cfg(not(target_arch = "wasm32"))]
pub static PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD:WorkerCell<[PendingNativeTextEditorClipboardSlot;32]>=WorkerCell::new(||std::array::from_fn(|_|PendingNativeTextEditorClipboardSlot{generation:0,mounted:None}));

#[cfg(not(target_arch = "wasm32"))]
pub fn complete_pending_native_text_editor_clipboard(token: PendingNativeTextEditorClipboardToken, result: Result<Option<semio_framework_ui_wgpu_hosting::host::ClipboardContent>, semio_framework_ui_action::BoundedActionFault>) {
    PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.get_mut(usize::from(token.slot)).filter(|slot| slot.generation == token.generation) else { return };
        if let Some(pending) = slot.mounted.as_mut() {
            pending.outcome = Some(result);
        }
    });
}

#[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
pub fn queue_text_editor_clipboard_read(focus: FocusedTextEditor,input:&mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let source=input.original_clipboard_source.ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
    let token = PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let (index, slot) = slots.iter_mut().enumerate().find(|(_, slot)| slot.mounted.is_none() && slot.generation != u64::MAX)?;
        slot.generation = slot.generation.checked_add(1)?;
        slot.mounted = Some(PendingNativeTextEditorClipboard { source,focus, outcome: None });
        Some(PendingNativeTextEditorClipboardToken { slot: u8::try_from(index).ok()?, generation: slot.generation })
    });
    let Some(token) = token else { return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits) };
    input.retain_original_clipboard_intent(semio_framework_ui_wgpu_input::ClipboardIntentRecipient::Text{slot:token.slot,generation:token.generation},None).unwrap_or_else(|_|panic!("text clipboard original source admission required"));
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn rgba_png_data_url(width: u32, height: u32, bytes: Vec<u8>) -> Option<String> {
    use base64::Engine as _;
    let mut job = semio_framework_pixels::png_encoding::PngEncodeJob::new(semio_framework_pixels::RasterImage { width, height, pixels: bytes }).ok()?;
    while !job.advance().ok()?.done {}
    Some(format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(job.into_result().ok()?.data)))
}

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
fn clipboard_content_capacity(outcome:&Option<Result<Option<semio_framework_ui_wgpu_hosting::host::ClipboardContent>,semio_framework_ui_action::BoundedActionFault>>)->usize{match outcome{Some(Ok(Some(semio_framework_ui_wgpu_hosting::host::ClipboardContent::Text(text))))=>text.capacity(),Some(Ok(Some(semio_framework_ui_wgpu_hosting::host::ClipboardContent::ImageRgba8{bytes,..})))=>bytes.capacity(),_=>0}}
#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
fn clipboard_recipient_can_close(cx:&semio_framework_job::StepContext<'_>,release:usize)->bool{let grant=cx.retained_grant();!cx.should_yield()&&grant.maximum_items>0&&grant.maximum_release_bytes>=release&&grant.maximum_depth>=3}

#[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
fn drive_clipboard_io_control(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,cx:&mut semio_framework_job::StepContext<'_>,cleanup:bool) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
    for index in 0..input.original_clipboard_intents.len(){if let Some(intent)=input.original_clipboard_intents[index].as_ref(){
        validate_clipboard_source(intent.source,cx)?;
        if cleanup||cx.is_cancelled(){let release=intent.text.as_ref().map_or(0,String::capacity)+match &intent.recipient{semio_framework_ui_wgpu_input::ClipboardIntentRecipient::Write(window)|semio_framework_ui_wgpu_input::ClipboardIntentRecipient::Window(window)=>window.capacity(),_=>0};let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_release_bytes<release||grant.maximum_depth<2{return Ok(true)}let intent=input.original_clipboard_intents[index].take().unwrap();deliver_clipboard_recipient(&intent.recipient,Err(semio_framework_ui_action::BoundedActionFault::Structure),input);drop(intent);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()})?;cx.consume_fuel(1);return Ok(true)}
        if !submit_clipboard_io(&mut input.original_clipboard_intents[index],cx)?{return Ok(true)}return Ok(true)
    }}
    if pump_clipboard_io_one(input,cx,cleanup)?{return Ok(true)}
    let (pending_text,blocked)=PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD.with(|cell|->Result<_,semio_framework_job::OriginalTurnFailure>{let mut slots=cell.borrow_mut();let Some(slot)=slots.iter_mut().find(|slot|slot.mounted.as_ref().is_some_and(|pending|pending.source.operation==cx.operation()&&pending.source.generation==cx.generation()&&(cleanup||pending.outcome.is_some())))else{return Ok((None,false))};let pending=slot.mounted.as_ref().unwrap();validate_clipboard_source(pending.source,cx)?;let release=pending.focus.window_id.capacity()+pending.focus.host_id.capacity()+clipboard_content_capacity(&pending.outcome);if !clipboard_recipient_can_close(cx,release){return Ok((None,true))}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()})?;cx.consume_fuel(1);Ok((slot.mounted.take(),false))})?;
    if blocked{return Ok(true)}
    if let Some(pending) = pending_text {
        if cleanup||cx.is_cancelled(){drop(pending);return Ok(true)}
        return Ok(match pending.outcome.expect("ready text editor clipboard completion") {
            Ok(Some(semio_framework_ui_wgpu_hosting::host::ClipboardContent::Text(text))) => apply_text_editor_text_at(&pending.focus, &text, input),
            Ok(Some(semio_framework_ui_wgpu_hosting::host::ClipboardContent::ImageRgba8 { .. })) | Ok(None) => false,
            Err(fault) => {
                input.record_action_fault(fault);
                true
            }
        });
    }
    let(pending,blocked)=PENDING_NATIVE_INK_CLIPBOARD.with(|cell|->Result<_,semio_framework_job::OriginalTurnFailure>{let mut slots=cell.borrow_mut();let Some(slot)=slots.iter_mut().find(|slot|slot.mounted.as_ref().is_some_and(|pending|pending.source.operation==cx.operation()&&pending.source.generation==cx.generation()&&(cleanup||pending.outcome.is_some())))else{return Ok((None,false))};let pending=slot.mounted.as_ref().unwrap();validate_clipboard_source(pending.source,cx)?;let release=pending.address.window_id.capacity()+pending.address.host_id.capacity()+clipboard_content_capacity(&pending.outcome);if !clipboard_recipient_can_close(cx,release){return Ok((None,true))}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()})?;cx.consume_fuel(1);Ok((slot.mounted.take(),false))})?;
    if blocked{return Ok(true)}
    let Some(pending) = pending else { return Ok(false) };
    if cleanup||cx.is_cancelled(){drop(pending);return Ok(true)}
    Ok(match pending.outcome.expect("ready clipboard completion") {
        Ok(Some(semio_framework_ui_wgpu_hosting::host::ClipboardContent::Text(text))) => apply_ink_clipboard_content(&pending.address, semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind::Text, &text, input),
        Ok(Some(semio_framework_ui_wgpu_hosting::host::ClipboardContent::ImageRgba8 { width, height, bytes })) => match rgba_png_data_url(width, height, bytes) {
            Some(data) => apply_ink_clipboard_content(&pending.address, semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind::ImageDataUrl, &data, input),
            None => {
                input.record_action_fault(semio_framework_ui_action::BoundedActionFault::Structure);
                true
            }
        },
        Ok(None) => false,
        Err(fault) => {
            input.record_action_fault(fault);
            true
        }
    })
}

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
pub fn drive_clipboard_io_step(input:&mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{drive_clipboard_io_control(input,cx,false)}
#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
pub fn close_clipboard_io_step(input:&mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{drive_clipboard_io_control(input,cx,true).map(|pending|!pending)}
#[cfg(any(target_arch="wasm32",any(test, feature = "testkit")))]
pub fn close_clipboard_io_step(_input:&mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{Ok(true)}

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn drive_clipboard_io_step(_input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
    Ok(false)
}

pub fn apply_focused_ink_surface_key(key: &semio_framework_ui_wgpu_input::KeyAction, modifiers: &semio_framework_ui_wgpu_input::PointerModifiers, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let Some(address) = focused_ink_surface() else { return false };
    if focused_ink_editor_owns(&address) || !modifiers.ctrl_or_meta() || modifiers.alt || modifiers.shift {
        return false;
    }
    let semio_framework_ui_wgpu_input::KeyAction::Char(key) = key else { return false };
    if key.eq_ignore_ascii_case("c") {
        let result = with_live_ink_surface(&address, semio_framework_os_renderer_wgpu_scene_ink::ink_clipboard_copy_text);
        match result {
            Some(Ok(Some(text))) => write_owned_os_clipboard(text, input),
            Some(Err(fault)) => input.record_action_fault(fault),
            None => FOCUSED_INK_SURFACE.with(|cell| *cell.borrow_mut() = None),
            Some(Ok(None)) => {}
        }
        return true;
    }
    if !key.eq_ignore_ascii_case("v") {
        return false;
    }
    #[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
    if let Some(text) = read_os_clipboard() {
        apply_ink_clipboard_content(&address, semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind::Text, &text, input);
    }
    #[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
    if let Err(fault) = queue_ink_clipboard_read(address,input) {
        input.record_action_fault(fault);
    }
    true
}

pub struct InkClipboardStream {
    pub id: u64,
    pub address: InkClipboardAddress,
    pub image_data_url: bool,
    pub text: String,
}

thread_local! {
    pub static INK_CLIPBOARD_STREAMS: std::cell::RefCell<[Option<InkClipboardStream>; 32]> = std::cell::RefCell::new(std::array::from_fn(|_| None));
}

pub fn start_focused_ink_clipboard_stream(stream: u64, image_data_url: bool, declared_bytes: usize) -> Result<bool, &'static str> {
    if declared_bytes > 3 * (16 * 1024 - 1) {
        return Err("ink clipboard stream exceeded declared byte credits");
    }
    let Some(address) = focused_ink_surface() else { return Ok(false) };
    let live = with_live_ink_surface(&address, |_| ()).is_some();
    if !live || (image_data_url && focused_ink_editor_owns(&address)) {
        return Ok(false);
    }
    INK_CLIPBOARD_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        if slots.iter().flatten().any(|mounted| mounted.id == stream) {
            return Err("ink clipboard stream identifier is already mounted");
        }
        let Some(slot) = slots.iter_mut().find(|slot| slot.is_none()) else { return Err("ink clipboard stream credits exhausted") };
        *slot = Some(InkClipboardStream { id: stream, address, image_data_url, text: String::new() });
        Ok(true)
    })
}

pub fn push_focused_ink_clipboard_stream(stream: u64, text: &str) -> Result<bool, &'static str> {
    INK_CLIPBOARD_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(mounted) = slots.iter_mut().flatten().find(|mounted| mounted.id == stream) else { return Ok(false) };
        if mounted.text.len().checked_add(text.len()).is_none_or(|bytes| bytes >= 16 * 1024) {
            return Err("ink clipboard stream exceeded payload byte credits");
        }
        mounted.text.push_str(text);
        Ok(true)
    })
}

pub fn commit_focused_ink_clipboard_stream(stream: u64, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let mounted = INK_CLIPBOARD_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let slot = slots.iter().position(|slot| slot.as_ref().is_some_and(|mounted| mounted.id == stream))?;
        slots[slot].take()
    });
    mounted.is_some_and(|mounted| {
        let kind = if mounted.image_data_url { semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind::ImageDataUrl } else { semio_framework_os_renderer_wgpu_scene_ink::InkClipboardContentKind::Text };
        apply_ink_clipboard_content(&mounted.address, kind, &mounted.text, input)
    })
}

pub fn abort_focused_ink_clipboard_stream(stream: u64) -> bool {
    INK_CLIPBOARD_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter().position(|slot| slot.as_ref().is_some_and(|mounted| mounted.id == stream)) else { return false };
        slots[slot] = None;
        true
    })
}

struct TextEditorInputStream {
    pub id: u64,
    pub focus: FocusedTextEditor,
    pub text: String,
}

thread_local! {
    pub static TEXT_EDITOR_INPUT_STREAMS: std::cell::RefCell<[Option<TextEditorInputStream>; 32]> = std::cell::RefCell::new(std::array::from_fn(|_| None));
}

pub fn start_focused_text_editor_stream(stream: u64, declared_bytes: usize) -> Result<bool, &'static str> {
    if declared_bytes >= 16 * 1024 {
        return Err("text editor stream exceeded declared byte credits");
    }
    let Some(focus) = focused_text_editor() else { return Ok(false) };
    if with_live_text_editor(&focus, |_| ()).is_none() {
        return Ok(false);
    }
    TEXT_EDITOR_INPUT_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        if slots.iter().flatten().any(|mounted| mounted.id == stream) {
            return Err("text editor stream identifier is already mounted");
        }
        let Some(slot) = slots.iter_mut().find(|slot| slot.is_none()) else { return Err("text editor stream credits exhausted") };
        *slot = Some(TextEditorInputStream { id: stream, focus, text: String::new() });
        Ok(true)
    })
}

pub fn push_focused_text_editor_stream(stream: u64, text: &str) -> Result<bool, &'static str> {
    TEXT_EDITOR_INPUT_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(mounted) = slots.iter_mut().flatten().find(|mounted| mounted.id == stream) else { return Ok(false) };
        if mounted.text.len().checked_add(text.len()).is_none_or(|bytes| bytes >= 16 * 1024) {
            return Err("text editor stream exceeded payload byte credits");
        }
        mounted.text.push_str(text);
        Ok(true)
    })
}

pub fn commit_focused_text_editor_stream(stream: u64, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let mounted = TEXT_EDITOR_INPUT_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let slot = slots.iter().position(|slot| slot.as_ref().is_some_and(|mounted| mounted.id == stream))?;
        slots[slot].take()
    });
    let Some(mounted) = mounted else { return false };
    if !focused_text_editor_matches(&mounted.focus) {
        return false;
    }
    let outcome = with_live_text_editor(&mounted.focus, |scene| semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_replace_selection_into(scene, &mounted.text, input));
    match outcome {
        Some(Ok(consumed)) => consumed,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            true
        }
        None => false,
    }
}

pub fn abort_focused_text_editor_stream(stream: u64) -> bool {
    TEXT_EDITOR_INPUT_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter().position(|slot| slot.as_ref().is_some_and(|mounted| mounted.id == stream)) else { return false };
        slots[slot] = None;
        true
    })
}

pub fn close_window_focus_clipboard_one(window_id: &str, generation: u64) -> bool {
    if let Some(focus) = FOCUSED_NODE_GRAPH_CARET.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| focus.target.window_id == window_id && focus.target.window_generation == generation) {
            focus.take()
        } else {
            None
        }
    }) {
        clear_node_graph_caret(&focus);
        return true;
    }
    if FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == generation) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if FOCUSED_TABLE_STEPPER.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == generation) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if FOCUSED_TEXT_EDITOR.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == generation) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if TEXT_EDITOR_INPUT_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|stream| stream.focus.window_id == window_id && stream.focus.window_generation == generation)) else { return false };
        slot.take();
        true
    }) {
        return true;
    }
    if FOCUSED_INK_EDITOR.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == generation) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if FOCUSED_INK_SURFACE.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == generation) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if INK_CLIPBOARD_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|stream| stream.address.window_id == window_id && stream.address.window_generation == generation)) else { return false };
        slot.take();
        true
    }) {
        return true;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if PENDING_NATIVE_INK_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter_mut().find(|slot| slot.mounted.as_ref().is_some_and(|pending| pending.address.window_id == window_id && pending.address.window_generation == generation)) else { return false };
        slot.mounted.take();
        true
    }) {
        return true;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter_mut().find(|slot| slot.mounted.as_ref().is_some_and(|pending| pending.focus.window_id == window_id && pending.focus.window_generation == generation)) else { return false };
        slot.mounted.take();
        true
    }) {
        return true;
    }
    false
}

pub fn close_retiring_focus_clipboard_one() -> bool {
    if FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&focus.host_id)) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if FOCUSED_TABLE_STEPPER.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&focus.host_id)) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if FOCUSED_TEXT_EDITOR.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&focus.host_id)) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if TEXT_EDITOR_INPUT_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|stream| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&stream.focus.host_id))) else { return false };
        slot.take();
        true
    }) {
        return true;
    }
    if FOCUSED_INK_EDITOR.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&focus.host_id)) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if FOCUSED_INK_SURFACE.with(|cell| {
        let mut focus = cell.borrow_mut();
        if focus.as_ref().is_some_and(|focus| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&focus.host_id)) {
            focus.take();
            true
        } else {
            false
        }
    }) {
        return true;
    }
    if INK_CLIPBOARD_STREAMS.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|stream| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&stream.address.host_id))) else { return false };
        slot.take();
        true
    }) {
        return true;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if PENDING_NATIVE_INK_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter_mut().find(|slot| slot.mounted.as_ref().is_some_and(|pending| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&pending.address.host_id))) else { return false };
        slot.mounted.take();
        true
    }) {
        return true;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        let Some(slot) = slots.iter_mut().find(|slot| slot.mounted.as_ref().is_some_and(|pending| semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&pending.focus.host_id))) else { return false };
        slot.mounted.take();
        true
    }) {
        return true;
    }
    false
}
//#endregion 📋️InkClipboardFocus
