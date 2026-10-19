//! 🎯️ Chrome ledger and chrome accessibility.
//!
//! The chrome input ledger, the world camera ledger rows, the chrome accessibility projection and the chrome dump.

use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::{InkClipboardAddress, FOCUSED_INK_EDITOR, FOCUSED_INK_SURFACE, FOCUSED_VFS_CONTROL, INK_CLIPBOARD_STREAMS, TEXT_EDITOR_INPUT_STREAMS};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::{PENDING_NATIVE_INK_CLIPBOARD, PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD};
use crate::introspection::build_accessibility_dump;
use semio_framework_os_renderer_wgpu_ui_engine_focus::scene_intents_and_clipboard::SCENE_INTENTS;
use semio_framework_os_renderer_wgpu_contracts::{SCENE_POINTER_OWNERS, UI_ENGINE};
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::{FocusedNodeGraphCaret, FOCUSED_NODE_GRAPH_CARET, FOCUSED_TABLE_EDITABLE_TEXT, FOCUSED_TABLE_STEPPER, FOCUSED_TEXT_EDITOR};
use crate::element_contracts::{WorkerCell, SCENE_POINTER_OWNER_CAPACITY};
use serde_json::Value;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_arena::NodeId;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_declaration::ui::UiNode;

//#region 🎯️ChromeLedger
/** 🎯️ The chrome's own introspection lane — the one thing this region's header calls structurally
 * unreachable. The shell's navbar, footer, dock, window caps, pane chips and overlays are
 * immediate-mode and never enter `UI_ENGINE`, so `dumpStructure` cannot see them; what they DO
 * publish is a pointer registry (`ui_wgpu::wgpu::InputState`'s hits, one row per interactive rect of
 * the last complete chrome walk) and a single action chokepoint
 * (`🐚️Shell/🎯️targets/🧊️wgpu`'s `dispatch_action`). Those two are exactly what a behavioural-parity
 * probe needs where React reads the DOM: a control to aim at by id, and the action that control
 * actually dispatched.
 *
 * 🩺️ Armed only while `SEMIO_RUNTIME_DIAGNOSTICS` is (`runtime_diagnostics_enabled`), like every
 * other per-frame census W3a gated out, and bounded on every axis — at most `CHROME_HIT_CAPACITY`
 * registry rows per published walk, `CHROME_ACTION_CAPACITY` remembered dispatches, and
 * `CHROME_ACTION_ARGS_BYTES` of each dispatch's arguments. */
const CHROME_HIT_CAPACITY: usize = 2_048;
pub const CHROME_ACTION_CAPACITY: usize = 128;
pub const CHROME_ACTION_ARGS_BYTES: usize = 512;

/// 🎯️ One registered pointer target of the last published chrome walk, at the ABSOLUTE page rect it
/// was registered with — the rect a probe aims a pointer at, no dock-plan reconstruction needed.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpHitTarget {
    pub control_id: String,
    pub kind: String,
    pub rect: [f32; 4],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drag_axis: Option<String>,
}

/// 🎬️ One dispatched action, in dispatch order. `seq` is monotonic for the life of the isolate, so a
/// probe reads the ledger before a step and after it and takes everything above the remembered `seq`.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpDispatchedAction {
    pub seq: u64,
    pub controller_id: String,
    pub action: String,
    /// 🪪️ React's `InputOriginV1` — `user` for a control press, `gesture` for a pointer/camera stream
    /// an engine surface derived from raw input. The DOM host's input ledger records it on every row
    /// (`🏛️ShellHost/🎯️input-ledger/🟦️.ts`), and a probe that reads one shape from both renderers
    /// cannot tell a world drag from a navbar click without it: before this the parity probe stamped
    /// every wgpu row `"shell"` by hand (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `🐍️parity-interact-probe.mjs`).
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    at_ms: f64,
}

/// 🪟️ One surface the shell carries this frame, at the LEVEL React's DOM states through
/// `data-level` — the missing half of a surface-delta comparison. A probe reading React sees a
/// window, a docked panel and a dialog as three different things (`[data-window-id]`,
/// `[data-level="panel"]`, `[data-level="dialog"]`); the wgpu shell models a panel AS a window, so
/// without this census every panel a step opened read as a new window and no step involving a panel
/// could ever match (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w9c-behaviour-parity-run-2.md` steps 3–8).
///
/// `element_id` is React's OWN id for that surface — `panelTabElementId(tabId)` for a panel — so the
/// two renderers name the same surface with the same string rather than with two conventions a probe
/// would have to translate.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpSurface {
    pub id: String,
    pub level: String,
    pub element_id: String,
}

#[derive(Default)]
pub struct ChromeLedger {
    pub hits: Vec<DumpHitTarget>,
    pub surfaces: Vec<DumpSurface>,
    pub generation: u64,
    pub actions: std::collections::VecDeque<DumpDispatchedAction>,
    next_seq: u64,
    /// 🩺️ The last census a PARKED chrome walk published. `generation: 0` says only "no walk ever
    /// completed"; this says WHICH phase is holding the frame, which is the whole difference
    /// between "the map paints nothing" and "the shell presents nothing"
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w14a-*`).
    parked: Option<String>,
}

pub static CHROME_LEDGER: WorkerCell<ChromeLedger> = WorkerCell::new(Default::default);

/// 🎥️ The live orbit this surface last recorded, if diagnostics were on when it painted.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn world_camera_ledger_row(surface_id: &str) -> Option<Value> {
    WORLD_CAMERA_LEDGER.borrow().get(surface_id).cloned()
}

/// ⏱️ Wall clock for the ledger's stamps. The renderer wasm runs inside a DEDICATED WORKER, which owns
/// no `window` and therefore no `window.performance` — `js_sys::Date::now()` is the one clock both
/// realms answer.
fn chrome_ledger_now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|since| since.as_secs_f64() * 1000.0).unwrap_or_default()
    }
}

/// ✂️ Args as JSON, truncated on a CHARACTER boundary so the dump is always parseable text.
pub fn chrome_action_args(action: &ActionDescriptor) -> Option<String> {
    let args = action.args.as_ref()?;
    let mut json = semio_framework_pack_json::from_dsl_value(args).to_string();
    if json.len() > CHROME_ACTION_ARGS_BYTES {
        let cut = (0..=CHROME_ACTION_ARGS_BYTES).rev().find(|index| json.is_char_boundary(*index)).unwrap_or_default();
        json.truncate(cut);
    }
    Some(json)
}

/// 🎯️ Projects one published registry row, naming the retained body that minted its control id when
/// `owners` knows it — chrome rows own no window and answer `None`, which is how a probe tells the two apart.
pub fn chrome_hit_row(hit: &semio_framework_ui_wgpu_input::HitTarget<ActionDescriptor>, owners: &std::collections::HashMap<String, (String, Rect)>) -> DumpHitTarget {
    let control_id = hit.control_id.clone().unwrap_or_default();
    DumpHitTarget {
        window_id: owners.get(&control_id).map(|(window_id, _)| window_id.clone()),
        kind: format!("{:?}", hit.kind),
        rect: [hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h],
        controller_id: hit.event.as_ref().map(|event| event.controller_id.clone()),
        action: hit.event.as_ref().map(|event| event.action.clone()),
        drag_axis: hit.drag_axis.map(|axis| format!("{axis:?}")),
        control_id,
    }
}

/** 🎯️ Replaces the published registry snapshot — called once per COMPLETE chrome walk, from
 * `🐚️Shell/🎯️targets/🧊️wgpu`'s `publish_retained_hit_registry`, right after `InputState::publish_hits`
 * promotes the staged rows. An abandoned walk publishes nothing and therefore records nothing, so the
 * snapshot and the pointer's own authority can never disagree. */
/// ♿️ The shell CHROME's accessible names, published once per complete chrome walk by
/// `🐚️Shell/🎯️targets/🧊️wgpu`'s `publish_retained_hit_registry`.
///
/// 🩸️ Unlike [`CHROME_LEDGER`] beside it this is NOT diagnostics-gated: it is the production
/// accessibility path for everything outside a retained document. `build_accessibility_dump`
/// announces it as the `shell.chrome` window, so `🚀️browser-boot/🟦️.ts`'s mirror gives every navbar
/// chip, footer pill, panel tab and pane chip a real ARIA element — which is the gap a DOM renderer
/// never has, because React writes `aria-label` onto the element it renders
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, packet W15d; audit W14 §C4/§B14).
pub fn note_chrome_accessibility(generation: u64, nodes: Vec<ui_contract::AccessibilityProjectionNode>) {
    *CHROME_ACCESSIBILITY.borrow_mut() = ChromeAccessibilityPublication { generation, nodes };
}

/// ♿️ The window id the chrome announces under — a shell-owned surface, never one of the app's.
pub const SHELL_CHROME_ACCESSIBILITY_WINDOW_ID: &str = "shell.chrome";

/// ♿️ Every announced window's accessibility projection — the chrome's and each visible document window's, in reading
/// order — for the native shell's platform bridge (`♿️native-accessibility`): the same dump the browser build's DOM
/// mirror renders, so both shells announce one tree (ticket 26/09/23 slice WG10).
#[cfg(not(target_arch = "wasm32"))]
pub fn native_accessibility_windows() -> Vec<semio_framework_os_renderer_wgpu_native_platform::native_accessibility::NativeAccessibilityWindow> {
    UI_ENGINE
        .with(|cell| build_accessibility_dump(&cell.borrow(), None))
        .windows
        .into_iter()
        .map(|window| semio_framework_os_renderer_wgpu_native_platform::native_accessibility::NativeAccessibilityWindow { window_id: window.window_id, window_generation: window.window_generation, nodes: window.nodes })
        .collect()
}

#[derive(Clone, Default)]
struct ChromeAccessibilityPublication {
    pub generation: u64,
    pub nodes: Vec<ui_contract::AccessibilityProjectionNode>,
}

pub static CHROME_ACCESSIBILITY: WorkerCell<ChromeAccessibilityPublication> = WorkerCell::new(Default::default);

const ACCESSIBILITY_VISIBLE_WINDOW_CAPACITY: usize = 256;

#[derive(Default)]
struct AccessibilityVisibleWindows {
    pub published: Vec<String>,
    staging: Vec<String>,
}

pub static ACCESSIBILITY_VISIBLE_WINDOWS: WorkerCell<AccessibilityVisibleWindows> = WorkerCell::new(Default::default);

pub fn staged_retained_clock_surfaces() -> Vec<(String, semio_framework_ui_wgpu_engine::UiSurfaceToken)> {
    let visible = ACCESSIBILITY_VISIBLE_WINDOWS.borrow().staging.clone();
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        visible.into_iter().filter_map(|id| engine.surface_token(&id).map(|surface| (id, surface))).collect()
    })
}

/// 🎬️ The accepted original engine supplies its retained tree loans to the actual paid media continuation.
pub fn drive_presented_media_slots(source:&semio_framework_os_renderer_wgpu_media_slots::original_collect::PresentedRosterSource<'_>,cursor:&mut semio_framework_os_renderer_wgpu_media_slots::original_collect::OriginalPresentedRoster,output:&mut Vec<semio_framework_os_renderer_wgpu_media_slots::PresentedMediaSlot>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{UI_ENGINE.with(|cell|cursor.advance(source,&mut cell.borrow_mut(),output,cx))}
/// ♻️ The same original engine receives every retained accepted tree loan before media source retirement completes.
pub fn close_presented_media_slots(cursor:&mut semio_framework_os_renderer_wgpu_media_slots::original_collect::OriginalPresentedRoster,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{UI_ENGINE.with(|cell|cursor.close_step(&mut cell.borrow_mut(),cx))}

pub fn presented_retained_clock_deadline(surfaces: &[(String, semio_framework_ui_wgpu_engine::UiSurfaceToken)]) -> Option<f64> {
    UI_ENGINE.with(|cell| cell.borrow().surfaces_next_clock_deadline(surfaces))
}

pub fn begin_accessibility_visible_documents() {
    ACCESSIBILITY_VISIBLE_WINDOWS.borrow_mut().staging.clear();
}

pub fn note_accessibility_visible_document(window_id: &str) {
    let mut windows = ACCESSIBILITY_VISIBLE_WINDOWS.borrow_mut();
    if windows.staging.len() < ACCESSIBILITY_VISIBLE_WINDOW_CAPACITY && !windows.staging.iter().any(|visible| visible == window_id) {
        windows.staging.push(window_id.to_string());
    }
}

pub fn publish_accessibility_visible_documents() {
    let mut windows = ACCESSIBILITY_VISIBLE_WINDOWS.borrow_mut();
    let AccessibilityVisibleWindows { published, staging } = &mut *windows;
    std::mem::swap(published, staging);
}

pub fn seal_presented_input_candidate(witness: u64) -> bool {
    let visible = ACCESSIBILITY_VISIBLE_WINDOWS.borrow().staging.clone();
    UI_ENGINE.with(|cell| cell.borrow_mut().seal_presented_input_candidate(witness, &visible))
}

pub fn presented_input_candidate_matches(witness: u64) -> bool {
    UI_ENGINE.with(|cell| cell.borrow().presented_input_candidate_matches(witness))
}

struct PresentedEditorNodeRebase {
    pub window_id: String,
    pub window_generation: u64,
    old_node: NodeId,
    pub host_id: String,
    candidate_node: Option<NodeId>,
}

struct PresentedInkClipboardRebase {
    old: InkClipboardAddress,
    candidate: Option<(NodeId, Rect)>,
}

struct PresentedEditorRebase {
    pub text: Option<PresentedEditorNodeRebase>,
    pub node_graph: Option<PresentedEditorNodeRebase>,
    text_streams: [Option<(u64, PresentedEditorNodeRebase)>; 32],
    #[cfg(not(target_arch = "wasm32"))]
    native_text: [Option<(u64, PresentedEditorNodeRebase)>; 32],
    table_editable: Option<PresentedEditorNodeRebase>,
    pub table: Option<PresentedEditorNodeRebase>,
    vfs: Option<PresentedEditorNodeRebase>,
    ink: Option<PresentedEditorNodeRebase>,
    ink_surface: Option<PresentedInkClipboardRebase>,
    ink_streams: [Option<(u64, PresentedInkClipboardRebase)>; 32],
    #[cfg(not(target_arch = "wasm32"))]
    native_ink: [Option<(u64, PresentedInkClipboardRebase)>; 32],
}

fn candidate_editor_geometry(engine: &semio_framework_ui_wgpu_engine::Ui, witness: u64, window_id: &str, window_generation: u64, node: NodeId, host_id: &str, kind: semio_framework_ui_declaration::ui::SurfaceKind) -> Option<(NodeId, Rect)> {
    if engine.candidate_surface_generation(window_id) != Some(window_generation) {
        return None;
    }
    let (candidate, rect) = engine.candidate_scene_geometry_for_presented_node(window_id, witness, node)?;
    let retained = engine.candidate_tree(window_id)?.node(candidate)?;
    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
    (scene.host_id == host_id && scene.component_kind == kind).then_some((candidate, rect))
}

fn prepare_presented_editor_node_rebase(engine: &semio_framework_ui_wgpu_engine::Ui, witness: u64, window_id: &str, window_generation: u64, node: NodeId, host_id: &str, kind: semio_framework_ui_declaration::ui::SurfaceKind) -> Option<PresentedEditorNodeRebase> {
    engine.candidate_is_sealed_for(window_id, witness).then(|| PresentedEditorNodeRebase {
        window_id: window_id.to_owned(),
        window_generation,
        old_node: node,
        host_id: host_id.to_owned(),
        candidate_node: candidate_editor_geometry(engine, witness, window_id, window_generation, node, host_id, kind).map(|(node, _)| node),
    })
}

fn prepare_presented_node_graph_caret_rebase(engine: &semio_framework_ui_wgpu_engine::Ui, witness: u64, focus: &FocusedNodeGraphCaret) -> Option<PresentedEditorNodeRebase> {
    let mut rebase = prepare_presented_editor_node_rebase(engine, witness, &focus.target.window_id, focus.target.window_generation, focus.target.node, &focus.target.host_id, semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph)?;
    if let Some(node) = rebase.candidate_node {
        let survives = engine
            .candidate_tree(&focus.target.window_id)
            .and_then(|tree| tree.node(node))
            .is_some_and(|retained| matches!(&retained.spec.0, UiNode::ComponentScene(scene) if scene.surface_id == focus.target.surface_id) && retained.key == focus.target.key && retained.component_generation() == focus.target.component_generation);
        if !survives {
            rebase.candidate_node = None;
        }
    }
    Some(rebase)
}

fn prepare_presented_ink_clipboard_rebase(engine: &semio_framework_ui_wgpu_engine::Ui, witness: u64, address: &InkClipboardAddress) -> Option<PresentedInkClipboardRebase> {
    engine
        .candidate_is_sealed_for(&address.window_id, witness)
        .then(|| PresentedInkClipboardRebase { old: address.clone(), candidate: candidate_editor_geometry(engine, witness, &address.window_id, address.window_generation, address.node, &address.host_id, semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas) })
}

fn prepare_presented_editor_rebase(engine: &semio_framework_ui_wgpu_engine::Ui, witness: u64) -> PresentedEditorRebase {
    let text =
        FOCUSED_TEXT_EDITOR.with(|cell| cell.borrow().as_ref().and_then(|focus| prepare_presented_editor_node_rebase(engine, witness, &focus.window_id, focus.window_generation, focus.node, &focus.host_id, semio_framework_ui_declaration::ui::SurfaceKind::TextEditor)));
    let node_graph = FOCUSED_NODE_GRAPH_CARET.with(|cell| cell.borrow().as_ref().and_then(|focus| prepare_presented_node_graph_caret_rebase(engine, witness, focus)));
    let text_streams = TEXT_EDITOR_INPUT_STREAMS.with(|cell| {
        let streams = cell.borrow();
        std::array::from_fn(|index| {
            let stream = streams[index].as_ref()?;
            Some((stream.id, prepare_presented_editor_node_rebase(engine, witness, &stream.focus.window_id, stream.focus.window_generation, stream.focus.node, &stream.focus.host_id, semio_framework_ui_declaration::ui::SurfaceKind::TextEditor)?))
        })
    });
    #[cfg(not(target_arch = "wasm32"))]
    let native_text = PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD.with(|cell| {
        let slots = cell.borrow();
        std::array::from_fn(|index| {
            let slot = &slots[index];
            let pending = slot.mounted.as_ref()?;
            Some((slot.generation, prepare_presented_editor_node_rebase(engine, witness, &pending.focus.window_id, pending.focus.window_generation, pending.focus.node, &pending.focus.host_id, semio_framework_ui_declaration::ui::SurfaceKind::TextEditor)?))
        })
    });
    let table =
        FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().and_then(|focus| prepare_presented_editor_node_rebase(engine, witness, &focus.window_id, focus.window_generation, focus.node, &focus.host_id, semio_framework_ui_declaration::ui::SurfaceKind::Table)));
    let table_editable =
        FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| cell.borrow().as_ref().and_then(|focus| prepare_presented_editor_node_rebase(engine, witness, &focus.window_id, focus.window_generation, focus.node, &focus.host_id, semio_framework_ui_declaration::ui::SurfaceKind::Table)));
    let vfs = FOCUSED_VFS_CONTROL.with(|cell| {
        cell.borrow().as_ref().and_then(|focus| {
            let mut rebase = prepare_presented_editor_node_rebase(engine, witness, &focus.window_id, focus.window_generation, focus.node, &focus.host_id, semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem)?;
            if rebase.candidate_node.is_some() && !semio_framework_os_renderer_wgpu_scene_feeds::vfs_accessibility_control_survives_candidate(&focus.host_id, &focus.target, witness) {
                rebase.candidate_node = None;
            }
            Some(rebase)
        })
    });
    let ink =
        FOCUSED_INK_EDITOR.with(|cell| cell.borrow().as_ref().and_then(|focus| prepare_presented_editor_node_rebase(engine, witness, &focus.window_id, focus.window_generation, focus.node, &focus.host_id, semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas)));
    let ink_surface = FOCUSED_INK_SURFACE.with(|cell| cell.borrow().as_ref().and_then(|address| prepare_presented_ink_clipboard_rebase(engine, witness, address)));
    let ink_streams = INK_CLIPBOARD_STREAMS.with(|cell| {
        let streams = cell.borrow();
        std::array::from_fn(|index| {
            let stream = streams[index].as_ref()?;
            Some((stream.id, prepare_presented_ink_clipboard_rebase(engine, witness, &stream.address)?))
        })
    });
    #[cfg(not(target_arch = "wasm32"))]
    let native_ink = PENDING_NATIVE_INK_CLIPBOARD.with(|cell| {
        let slots = cell.borrow();
        std::array::from_fn(|index| {
            let slot = &slots[index];
            let pending = slot.mounted.as_ref()?;
            Some((slot.generation, prepare_presented_ink_clipboard_rebase(engine, witness, &pending.address)?))
        })
    });
    PresentedEditorRebase {
        text,
        node_graph,
        text_streams,
        #[cfg(not(target_arch = "wasm32"))]
        native_text,
        table_editable,
        table,
        vfs,
        ink,
        ink_surface,
        ink_streams,
        #[cfg(not(target_arch = "wasm32"))]
        native_ink,
    }
}

fn ink_clipboard_address_matches(actual: &InkClipboardAddress, expected: &InkClipboardAddress) -> bool {
    actual.window_id == expected.window_id && actual.window_generation == expected.window_generation && actual.node == expected.node && actual.host_id == expected.host_id && actual.rect == expected.rect
}

fn commit_presented_ink_clipboard_rebase(address: &mut InkClipboardAddress, rebase: &PresentedInkClipboardRebase) -> bool {
    if !ink_clipboard_address_matches(address, &rebase.old) {
        return true;
    }
    let Some((node, rect)) = rebase.candidate else { return false };
    address.node = node;
    address.rect = rect;
    true
}

fn commit_presented_editor_rebase(rebase: PresentedEditorRebase) {
    if let Some(rebase) = rebase.text {
        FOCUSED_TEXT_EDITOR.with(|cell| {
            let mut focus = cell.borrow_mut();
            let matches = focus.as_ref().is_some_and(|focus| focus.window_id == rebase.window_id && focus.window_generation == rebase.window_generation && focus.node == rebase.old_node && focus.host_id == rebase.host_id);
            if matches {
                match rebase.candidate_node {
                    Some(node) => focus.as_mut().expect("matched Text focus").node = node,
                    None => *focus = None,
                }
            }
        });
    }
    if let Some(rebase) = rebase.node_graph {
        FOCUSED_NODE_GRAPH_CARET.with(|cell| {
            let mut focus = cell.borrow_mut();
            let matches = focus.as_ref().is_some_and(|focus| focus.target.window_id == rebase.window_id && focus.target.window_generation == rebase.window_generation && focus.target.node == rebase.old_node && focus.target.host_id == rebase.host_id);
            if matches {
                match rebase.candidate_node {
                    Some(node) => focus.as_mut().expect("matched NodeGraph caret focus").target.node = node,
                    None => *focus = None,
                }
            }
        });
    }
    TEXT_EDITOR_INPUT_STREAMS.with(|cell| {
        let mut streams = cell.borrow_mut();
        for (index, prepared) in rebase.text_streams.into_iter().enumerate() {
            let Some((id, prepared)) = prepared else { continue };
            let Some(stream) = streams[index]
                .as_mut()
                .filter(|stream| stream.id == id && stream.focus.window_id == prepared.window_id && stream.focus.window_generation == prepared.window_generation && stream.focus.node == prepared.old_node && stream.focus.host_id == prepared.host_id)
            else {
                continue;
            };
            match prepared.candidate_node {
                Some(node) => stream.focus.node = node,
                None => streams[index] = None,
            }
        }
    });
    #[cfg(not(target_arch = "wasm32"))]
    PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        for (index, prepared) in rebase.native_text.into_iter().enumerate() {
            let Some((generation, prepared)) = prepared else { continue };
            let slot = &mut slots[index];
            if slot.generation != generation {
                continue;
            }
            let Some(pending) = slot
                .mounted
                .as_mut()
                .filter(|pending| pending.focus.window_id == prepared.window_id && pending.focus.window_generation == prepared.window_generation && pending.focus.node == prepared.old_node && pending.focus.host_id == prepared.host_id)
            else {
                continue;
            };
            match prepared.candidate_node {
                Some(node) => pending.focus.node = node,
                None => slot.mounted = None,
            }
        }
    });
    if let Some(rebase) = rebase.table_editable {
        FOCUSED_TABLE_EDITABLE_TEXT.with(|cell| {
            let mut focus = cell.borrow_mut();
            let matches = focus.as_ref().is_some_and(|focus| focus.window_id == rebase.window_id && focus.window_generation == rebase.window_generation && focus.node == rebase.old_node && focus.host_id == rebase.host_id);
            if matches {
                match rebase.candidate_node {
                    Some(node) => focus.as_mut().expect("matched editable Table focus").node = node,
                    None => *focus = None,
                }
            }
        });
    }
    if let Some(rebase) = rebase.table {
        FOCUSED_TABLE_STEPPER.with(|cell| {
            let mut focus = cell.borrow_mut();
            let matches = focus.as_ref().is_some_and(|focus| focus.window_id == rebase.window_id && focus.window_generation == rebase.window_generation && focus.node == rebase.old_node && focus.host_id == rebase.host_id);
            if matches {
                match rebase.candidate_node {
                    Some(node) => focus.as_mut().expect("matched Table stepper focus").node = node,
                    None => *focus = None,
                }
            }
        });
    }
    if let Some(rebase) = rebase.vfs {
        FOCUSED_VFS_CONTROL.with(|cell| {
            let mut focus = cell.borrow_mut();
            let matches = focus.as_ref().is_some_and(|focus| focus.window_id == rebase.window_id && focus.window_generation == rebase.window_generation && focus.node == rebase.old_node && focus.host_id == rebase.host_id);
            if matches {
                match rebase.candidate_node {
                    Some(node) => focus.as_mut().expect("matched VFS focus").node = node,
                    None => *focus = None,
                }
            }
        });
    }
    if let Some(rebase) = rebase.ink {
        FOCUSED_INK_EDITOR.with(|cell| {
            let mut focus = cell.borrow_mut();
            let matches = focus.as_ref().is_some_and(|focus| focus.window_id == rebase.window_id && focus.window_generation == rebase.window_generation && focus.node == rebase.old_node && focus.host_id == rebase.host_id);
            if matches {
                match rebase.candidate_node {
                    Some(node) => focus.as_mut().expect("matched Ink focus").node = node,
                    None => *focus = None,
                }
            }
        });
    }
    if let Some(surface) = rebase.ink_surface {
        FOCUSED_INK_SURFACE.with(|cell| {
            let mut focus = cell.borrow_mut();
            if focus.as_mut().is_some_and(|address| !commit_presented_ink_clipboard_rebase(address, &surface)) {
                *focus = None;
            }
        });
    }
    INK_CLIPBOARD_STREAMS.with(|cell| {
        let mut streams = cell.borrow_mut();
        for (index, prepared) in rebase.ink_streams.into_iter().enumerate() {
            let Some((id, prepared)) = prepared else { continue };
            let Some(stream) = streams[index].as_mut().filter(|stream| stream.id == id && ink_clipboard_address_matches(&stream.address, &prepared.old)) else { continue };
            if !commit_presented_ink_clipboard_rebase(&mut stream.address, &prepared) {
                streams[index] = None;
            }
        }
    });
    #[cfg(not(target_arch = "wasm32"))]
    PENDING_NATIVE_INK_CLIPBOARD.with(|cell| {
        let mut slots = cell.borrow_mut();
        for (index, prepared) in rebase.native_ink.into_iter().enumerate() {
            let Some((generation, prepared)) = prepared else { continue };
            let slot = &mut slots[index];
            if slot.generation != generation {
                continue;
            }
            let Some(pending) = slot.mounted.as_mut().filter(|pending| ink_clipboard_address_matches(&pending.address, &prepared.old)) else { continue };
            if !commit_presented_ink_clipboard_rebase(&mut pending.address, &prepared) {
                slot.mounted = None;
            }
        }
    });
}

pub fn acknowledge_presented_input(witness: u64) -> bool {
    let visible = ACCESSIBILITY_VISIBLE_WINDOWS.borrow().staging.clone();
    let (acknowledged, retired_canvas_hosts) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        if SCENE_INTENTS.with(|cell| cell.borrow().blocks_presented_candidate(&engine, witness)) {
            return (false, Vec::new());
        }
        let editor_rebase = prepare_presented_editor_rebase(&engine, witness);
        let acknowledged = SCENE_POINTER_OWNERS.with(|cell| {
            let mut owners = cell.borrow_mut();
            let rebased: [Option<Option<NodeId>>; SCENE_POINTER_OWNER_CAPACITY] = std::array::from_fn(|index| {
                let target = &owners.slots.get(index)?.target;
                if !engine.candidate_is_sealed_for(&target.window_id, witness) {
                    return None;
                }
                Some(engine.candidate_scene_node_for_presented_node(&target.window_id, witness, target.node).filter(|node| {
                    let Some(retained) = engine.candidate_tree(&target.window_id).and_then(|tree| tree.node(*node)) else { return false };
                    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return false };
                    engine.candidate_surface_generation(&target.window_id) == Some(target.window_generation)
                        && retained.component_generation() == target.component_generation
                        && retained.key == target.key
                        && scene.component_kind == target.kind
                        && scene.host_id == target.host_id
                        && scene.surface_id == target.surface_id
                }))
            });
            if !engine.acknowledge_presented_input(witness) {
                return false;
            }
            let mut index = 0;
            owners.slots.retain_mut(|owner| {
                let rebase = rebased[index];
                index += 1;
                match rebase {
                    None => true,
                    Some(Some(node)) => {
                        owner.target.node = node;
                        true
                    }
                    Some(None) => false,
                }
            });
            true
        });
        if acknowledged {
            commit_presented_editor_rebase(editor_rebase);
        }
        let retired_canvas_hosts =
            if acknowledged { visible.iter().filter_map(|window_id| engine.retired_component_scene(window_id)).filter(|retired| retired.kind == semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d).map(|retired| retired.host_id).collect() } else { Vec::new() };
        (acknowledged, retired_canvas_hosts)
    });
    for host_id in retired_canvas_hosts {
        semio_framework_os_renderer_wgpu_scene_state::request_canvas_pointer_gesture_cancel_for_host(&host_id);
    }
    acknowledged
}

pub fn discard_presented_input_candidate(witness: u64) -> bool {
    UI_ENGINE.with(|cell| cell.borrow_mut().discard_presented_input_candidate(witness))
}

pub fn accessibility_window_is_visible(window_id: &str) -> bool {
    ACCESSIBILITY_VISIBLE_WINDOWS.borrow().published.iter().any(|visible| visible == window_id)
}

/// 🎯️ The first node a visible retained window offers to `pick` from its accessibility projection (the one the ARIA mirror
/// reads), addressed exactly as a reader's accessibility event addresses it — so the shell can move keyboard focus to it
/// through [`dispatch_accessibility_event`]. Visible windows are asked in publication order; `None` when none shows it.
pub fn visible_retained_accessibility_target(pick: impl Fn(&[ui_contract::AccessibilityProjectionNode]) -> Option<usize>) -> Option<ui_render::AccessibilityTarget> {
    let visible = ACCESSIBILITY_VISIBLE_WINDOWS.borrow().published.clone();
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        visible.iter().find_map(|window_id| {
            let nodes = semio_framework_ui_wgpu_hosting::accessibility::accessibility_projection(engine.tree(window_id)?);
            let node = nodes.get(pick(&nodes)?)?;
            Some(ui_render::AccessibilityTarget { window_id: window_id.clone(), window_generation: engine.surface_generation(window_id)?, node_id: node.node_id, node_key: node.key.clone() })
        })
    })
}

#[cfg(any(test, feature = "testkit"))]
pub fn published_accessibility_nodes_for_test(window_id: &str) -> Vec<ui_contract::AccessibilityProjectionNode> {
    let dump = UI_ENGINE.with(|cell| build_accessibility_dump(&cell.borrow(), None));
    dump.windows.into_iter().find(|window| window.window_id == window_id).map(|window| window.nodes).unwrap_or_default()
}

#[cfg(any(test, feature = "testkit"))]
pub fn candidate_tree_item_selected_for_test(window_id: &str, key: &str) -> Option<bool> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let tree = engine.candidate_tree(window_id)?;
        semio_framework_ui_wgpu_hosting::accessibility::accessibility_projection(tree).into_iter().find(|node| node.key == key).and_then(|node| node.selected)
    })
}

#[cfg(any(test, feature = "testkit"))]
pub fn published_accessibility_target_for_test(window_id: &str, key: &str) -> Option<ui_render::AccessibilityTarget> {
    let dump = UI_ENGINE.with(|cell| build_accessibility_dump(&cell.borrow(), None));
    let window = dump.windows.into_iter().find(|window| window.window_id == window_id)?;
    let node = window.nodes.into_iter().find(|node| node.key == key)?;
    Some(ui_render::AccessibilityTarget { window_id: window.window_id, window_generation: window.window_generation, node_id: node.node_id, node_key: node.key })
}

#[cfg(any(test, feature = "testkit"))]
pub fn accessibility_visible_window_ids() -> Vec<String> {
    ACCESSIBILITY_VISIBLE_WINDOWS.borrow().published.clone()
}

pub fn note_chrome_hit_registry(hits: &[semio_framework_ui_wgpu_input::HitTarget<ActionDescriptor>], owners: &std::collections::HashMap<String, (String, Rect)>) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    ledger_publish_hits(&mut CHROME_LEDGER.borrow_mut(), hits, owners);
}

/// 🎯️ The pure half of `note_chrome_hit_registry`, over a ledger a law can own.
pub fn ledger_publish_hits(ledger: &mut ChromeLedger, hits: &[semio_framework_ui_wgpu_input::HitTarget<ActionDescriptor>], owners: &std::collections::HashMap<String, (String, Rect)>) {
    ledger.hits = hits.iter().take(CHROME_HIT_CAPACITY).map(|hit| chrome_hit_row(hit, owners)).collect();
    ledger.generation += 1;
}

/** 🪟️ Replaces the published surface census — `(surface id, level, React's element id)` per live
 * surface, from the SAME complete chrome walk that publishes the hit registry, so the two can never
 * describe different frames. See [`DumpSurface`] for why a level is what makes the census worth
 * publishing at all. */
pub fn note_chrome_surfaces(surfaces: &[(String, &'static str, String)]) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    ledger_publish_surfaces(&mut CHROME_LEDGER.borrow_mut(), surfaces);
}

/// 🪟️ The pure half of `note_chrome_surfaces`, bounded by the registry's own row ceiling and sorted,
/// so a probe diffs two censuses without re-sorting them and a frame that reorders its walk does not
/// read as a surface change.
pub fn ledger_publish_surfaces(ledger: &mut ChromeLedger, surfaces: &[(String, &'static str, String)]) {
    let mut rows: Vec<DumpSurface> = surfaces.iter().take(CHROME_HIT_CAPACITY).map(|(id, level, element_id)| DumpSurface { id: id.clone(), level: (*level).to_string(), element_id: element_id.clone() }).collect();
    rows.sort_by(|left, right| left.level.cmp(&right.level).then_with(|| left.id.cmp(&right.id)));
    rows.dedup();
    ledger.surfaces = rows;
}

/** 🎬️ Records ONE dispatched action. `🐚️Shell/🎯️targets/🧊️wgpu`'s `dispatch_action` is the single
 * funnel every chrome press, keybinding, command-palette entry, panel row and retained body action
 * crosses, so the ledger is the target's complete answer to "what did that click do". */
pub fn note_dispatched_action(action: &ActionDescriptor, origin: &str) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    ledger_push_action(&mut CHROME_LEDGER.borrow_mut(), action, origin);
}

/// 🎬️ The pure half of `note_dispatched_action`: mints the next `seq` and trims the oldest entry past
/// `CHROME_ACTION_CAPACITY`, so the ledger's footprint is fixed however long a session runs.
pub fn ledger_push_action(ledger: &mut ChromeLedger, action: &ActionDescriptor, origin: &str) {
    let window_id = action.args.as_ref().and_then(|args| args.get("windowId").or_else(|| args.get("surfaceId"))).and_then(semio_framework::DslValue::as_str).map(str::to_string);
    ledger.next_seq += 1;
    let entry = DumpDispatchedAction { seq: ledger.next_seq, controller_id: action.controller_id.clone(), action: action.action.clone(), origin: origin.to_string(), window_id, args: chrome_action_args(action), at_ms: chrome_ledger_now_ms() };
    ledger.actions.push_back(entry);
    while ledger.actions.len() > CHROME_ACTION_CAPACITY {
        ledger.actions.pop_front();
    }
}

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpChrome {
    pub armed: bool,
    pub generation: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parked: Option<String>,
    pub hits: Vec<DumpHitTarget>,
    pub surfaces: Vec<DumpSurface>,
    pub actions: Vec<DumpDispatchedAction>,
}

/// 🎯️ A named window keeps that window's body rows PLUS every chrome row (which owns no window and is
/// what the caller is usually aiming at); an unnamed read answers the whole registry.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn project_chrome_dump(ledger: &ChromeLedger, requested: Option<&str>, armed: bool) -> DumpChrome {
    let named = requested.filter(|id| !id.is_empty()).map(str::to_string);
    let hits = match &named {
        Some(id) => ledger.hits.iter().filter(|hit| hit.window_id.as_deref().is_none_or(|owner| owner == id)).cloned().collect(),
        None => ledger.hits.clone(),
    };
    let surfaces = match &named {
        Some(id) => ledger.surfaces.iter().filter(|surface| &surface.id == id).cloned().collect(),
        None => ledger.surfaces.clone(),
    };
    DumpChrome { armed, generation: ledger.generation, window_id: named, parked: ledger.parked.clone(), hits, surfaces, actions: ledger.actions.iter().cloned().collect() }
}

/** 🩺️ Records the census of a chrome walk that has overstayed one phase. Published by
 * `🐚️Shell/🎯️targets/🧊️wgpu`'s `render_chrome_step`, which is the ONLY caller, and read back through
 * `dumpChrome`'s `parked` field — the one machine-readable answer to "the canvas is empty and the
 * console is quiet". Unlike the rest of this ledger it is NOT diagnostics-gated: a walk that cannot
 * finish presents no frame at all, and a probe must be able to see that without re-booting the shell
 * with diagnostics armed. */
pub fn note_chrome_walk_parked(census: &str) {
    let mut ledger = CHROME_LEDGER.borrow_mut();
    if census.len() <= CHROME_ACTION_ARGS_BYTES {
        ledger.parked = Some(census.to_string());
    }
}
//#endregion 🎯️ChromeLedger

pub use semio_framework_os_renderer_wgpu_scene_state::{note_world3d_live_camera, take_world_camera_diagnostic, WORLD_CAMERA_LEDGER};
