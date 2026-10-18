use semio_framework_ui_tui_terminal::ansi::{emit_runs, AnsiPatch};
use semio_framework_ui_tui_core::cell::{diff, Cell, CellBuffer, DiffRun};
use semio_framework_ui_tui_frame::chrome::{ChromeState, WindowHit};
use semio_framework_ui_tui_widgets::dialog::DialogState;
use semio_framework_ui_tui_core::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use semio_framework_ui_tui_core::geometry::{Pos, Rect, Size};
use semio_framework_ui_tui_widgets::menu::{MenuItem, MenuState};
use semio_framework_ui_tui_widgets::palette::PaletteState;
use semio_framework_ui_tui_frame::scene::{AxisState, Node, NodeContent, NodeId, Scene, LAYOUT_DIRTY, PAINT_DIRTY};
use semio_framework_ui_tui_core::text::WidthMode;
use semio_framework_ui_tui_core::theme::{Role, Status, Theme};
use semio_framework_ui_tui_widgets::tooltip::{TooltipState, TOOLTIP_DELAY_MS};
use semio_framework_ui_tui_core::signal::{CursorSpec, WidgetSignal};
use semio_framework_ui_tui_widgets::widget::WidgetState;
use ui_styling::appearance::AppearanceName;

/// ⏱️ Presses on the same cell within this window count as one multi-click.
const CLICK_WINDOW_MS: u64 = 500;
/// 🎞️ Minimum spacing between painted frames when the host paces with `render_due`.
const FRAME_BUDGET_MS: u64 = 16;
/// 🌀️ How long one spinner frame of a running tab or widget shows.
const SPIN_FRAME_MS: u64 = 125;
/// 📱️ Terminals shorter than this many rows get the one-row window chrome.
pub const COMPACT_ROWS: u16 = 30;

type Signals = Vec<(NodeId, WidgetSignal)>;

/// 🖱️ What the pointer is holding between a button press and its release.
enum Press {
    Widget { node: NodeId, button: MouseButton, clicks: u8 },
    Splitter { axis: NodeId, index: usize, last: u16 },
    Tab { chrome: NodeId, from: usize, over: usize, start: Pos, moved: bool },
}

struct Click {
    button: MouseButton,
    pos: Pos,
    at_ms: u64,
    count: u8,
}

/// 🥞️ Where an overlay sits relative to the terminal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Placement {
    Anchored(Pos),
    Centered,
    TopCentered,
    Tip(Pos),
}

struct Overlay {
    id: NodeId,
    placement: Placement,
    modal: bool,
    dismiss_outside: bool,
    saved_focus: Option<NodeId>,
}

/// 🏭 The retained-mode pipeline: layout-if-dirty, paint, damage-diff, ANSI.
pub struct Tui {
    pub scene: Scene,
    pub theme: Theme,
    size: Size,
    front: CellBuffer,
    back: CellBuffer,
    focus: Option<NodeId>,
    hovered: Option<NodeId>,
    capture: Option<NodeId>,
    press: Option<Press>,
    last_click: Option<Click>,
    overlays: Vec<Overlay>,
    now_ms: u64,
    ticked: bool,
    last_render_ms: Option<u64>,
    hover_pos: Option<Pos>,
    hover_since_ms: u64,
    hover_tip: Option<String>,
    tooltip: Option<NodeId>,
    layout_stale: bool,
    full_redraw: bool,
    app_focused: bool,
    pending: Signals,
    animated: bool,
    width_mode: WidthMode,
}

fn focusable(scene: &Scene, id: NodeId) -> bool {
    matches!(scene.try_node(id).map(|node| &node.content), Some(NodeContent::Widget(widget)) if widget.interactive()) && scene.shown(id)
}

fn collect_focusables(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
    let node = scene.node(id);
    if !node.visible {
        return;
    }
    if matches!(&node.content, NodeContent::Widget(widget) if widget.interactive()) {
        out.push(id);
    }
    for &child in node.children() {
        collect_focusables(scene, child, out);
    }
}

fn collect_windows(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
    let node = scene.node(id);
    if matches!(&node.content, NodeContent::Chrome(ChromeState::Window(_))) {
        out.push(id);
    }
    for &child in node.children() {
        collect_windows(scene, child, out);
    }
}

fn collect_widgets(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
    let node = scene.node(id);
    if !node.visible {
        return;
    }
    if matches!(node.content, NodeContent::Widget(_)) {
        out.push(id);
    }
    for &child in node.children() {
        collect_widgets(scene, child, out);
    }
}

fn clamp_to(rect: Rect, viewport: Size) -> Rect {
    let width = rect.width.min(viewport.width);
    let height = rect.height.min(viewport.height);
    Rect::new(rect.x.min(viewport.width - width), rect.y.min(viewport.height - height), width, height)
}

#[path = "🎯️construction-and-focus/🦀️.rs"]
mod construction_and_focus;

#[path = "⏱️frame-layout-and-overlays/🦀️.rs"]
mod frame_layout_and_overlays;

#[path = "⌨️input-dispatch/🦀️.rs"]
mod input_dispatch;

#[path = "🖱️mouse-and-hover/🦀️.rs"]
mod mouse_and_hover;

#[path = "🖌️paint-and-gutters/🦀️.rs"]
mod paint_and_gutters;
use paint_and_gutters::gutter_at;
