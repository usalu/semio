// #region cursor
//! 🖱️ Theme-aware Semio cursor URLs for wgpu canvas hover parity with React.

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::ui::UiNode;
use crate::wgpu::events::CaptureKind;
use crate::wgpu::input::{DragAxis, HitKind, HitTarget};
use crate::wgpu::tree::{NodeFlags, UiTree};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioCursor {
    Default,
    Pointer,
    Selectable,
    Foldable,
    Grab,
    Grabbing,
    Text,
    EwResize,
    NsResize,
    NwseResize,
    NeswResize,
    Move,
    Crosshair,
    NotAllowed,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CursorDragState {
    pub tree_drag: bool,
    pub dock_drag: bool,
    pub pointer_drag_active: bool,
    pub pointer_drag_axis: Option<DragAxis>,
    pub pointer_drag_kind: Option<HitKind>,
}

pub fn resolve_semio_cursor<E>(hit: Option<&HitTarget<E>>, drag: CursorDragState) -> SemioCursor {
    if drag.tree_drag || drag.dock_drag {
        return SemioCursor::Grabbing;
    }
    if drag.pointer_drag_active {
        return cursor_for_active_drag(drag.pointer_drag_kind, drag.pointer_drag_axis);
    }
    let Some(hit) = hit else {
        return SemioCursor::Default;
    };
    if let Some(id) = hit.control_id.as_deref() {
        if id.contains(".chevron.") || id.starts_with("section.chevron.") {
            return SemioCursor::Foldable;
        }
    }
    if matches!(hit.kind, HitKind::PanelResize) {
        return SemioCursor::EwResize;
    }
    if hit.kind == HitKind::DockJoinCorner {
        return SemioCursor::Move;
    }
    if hit.kind == HitKind::DockSplit {
        return hit.drag_axis.map_or(SemioCursor::Default, axis_cursor);
    }
    if hit.kind == HitKind::ScrollRegion {
        if let Some(axis) = hit.drag_axis {
            return axis_cursor(axis);
        }
    }
    match hit.kind {
        HitKind::Input => SemioCursor::Text,
        HitKind::Select => SemioCursor::Foldable,
        HitKind::Slider => SemioCursor::Grab,
        HitKind::Window => SemioCursor::Grab,
        HitKind::TreeItem => {
            if hit.drag_data.is_some() {
                SemioCursor::Grab
            } else {
                SemioCursor::Selectable
            }
        }
        HitKind::TreeDropTarget => SemioCursor::Move,
        HitKind::World3d => SemioCursor::Default,
        HitKind::Button | HitKind::Toggle | HitKind::PanelTab | HitKind::NavbarItem | HitKind::ContextMenu | HitKind::DropdownItem => SemioCursor::Selectable,
        HitKind::ScrollRegion | HitKind::PanelResize | HitKind::DockSplit | HitKind::DockJoinCorner => SemioCursor::Default,
        HitKind::Generic => SemioCursor::Selectable,
    }
}

fn cursor_for_active_drag(kind: Option<HitKind>, axis: Option<DragAxis>) -> SemioCursor {
    match kind {
        Some(HitKind::Slider) => SemioCursor::Grabbing,
        Some(HitKind::PanelResize) => SemioCursor::EwResize,
        Some(HitKind::DockSplit) => axis.map_or(SemioCursor::Default, axis_cursor),
        Some(HitKind::DockJoinCorner) => SemioCursor::Move,
        Some(HitKind::ScrollRegion) => axis.map_or(SemioCursor::Default, axis_cursor),
        _ => axis.map_or(SemioCursor::Grabbing, axis_cursor),
    }
}

fn axis_cursor(axis: DragAxis) -> SemioCursor {
    match axis {
        DragAxis::Horizontal => SemioCursor::EwResize,
        DragAxis::Vertical => SemioCursor::NsResize,
        DragAxis::Both => SemioCursor::NwseResize,
        DragAxis::Ring => SemioCursor::Crosshair,
    }
}

/// 🖱️ M5's retained-tree counterpart to `resolve_semio_cursor`: derives a cursor from the
/// `events::EventRouter`'s own hovered/capture `NodeId`s (via its `hovered()`/`capture()` accessors)
/// rather than the immediate-mode `input::HitTarget`. Mostly wiring existing pieces together — an
/// active `CaptureKind` wins outright (dragging/scrolling a thumb never re-derives from whatever's
/// merely hovered underneath), otherwise it falls back to the hovered node's own `NodeFlags`/
/// `UiNode` variant.
pub fn resolve_semio_cursor_from_tree(tree: &UiTree, hovered: Option<NodeId>, capture: Option<(NodeId, CaptureKind)>) -> SemioCursor {
    if let Some((_, kind)) = capture {
        match kind {
            CaptureKind::Drag => return SemioCursor::Grabbing,
            CaptureKind::ScrollThumb(axis) => {
                return match axis {
                    crate::wgpu::events::ScrollAxis::Horizontal => SemioCursor::EwResize,
                    crate::wgpu::events::ScrollAxis::Vertical => SemioCursor::NsResize,
                };
            }
            CaptureKind::Press => {}
        }
    }
    let Some(target) = capture.map(|(id, _)| id).or(hovered) else {
        return SemioCursor::Default;
    };
    let Some(node) = tree.node(target) else {
        return SemioCursor::Default;
    };
    if node.flags.contains(NodeFlags::DRAG_SOURCE) {
        return SemioCursor::Grab;
    }
    match &node.spec.0 {
        UiNode::Input(_) => SemioCursor::Text,
        UiNode::Select(_) | UiNode::IconSelect(_) => SemioCursor::Foldable,
        UiNode::Slider(_) | UiNode::NumberStepper(_) | UiNode::Ring(_) => SemioCursor::Grab,
        UiNode::Button(_) | UiNode::Toggle(_) => SemioCursor::Selectable,
        _ if node.flags.contains(NodeFlags::DROP_TARGET) => SemioCursor::Default,
        _ => SemioCursor::Default,
    }
}

pub fn semio_cursor_css(cursor: SemioCursor, dark: bool) -> &'static str {
    match (cursor, dark) {
        (SemioCursor::Default, false) => "url(/🖼️assets/👆️cursor/🖱️default/☀️light.svg) 0 0, default",
        (SemioCursor::Default, true) => "url(/🖼️assets/👆️cursor/🖱️default/🌙️dark.svg) 0 0, default",
        (SemioCursor::Pointer, false) => "url(/🖼️assets/👆️cursor/👆️pointer/☀️light.svg) 0 0, pointer",
        (SemioCursor::Pointer, true) => "url(/🖼️assets/👆️cursor/👆️pointer/🖋️dark-source.svg) 0 0, pointer",
        (SemioCursor::Selectable, false) => "url(/🖼️assets/👆️cursor/☑️selectable/☀️light.svg) 0 0, pointer",
        (SemioCursor::Selectable, true) => "url(/🖼️assets/👆️cursor/☑️selectable/🌙️dark.svg) 0 0, pointer",
        (SemioCursor::Foldable, false) => "url(/🖼️assets/👆️cursor/🪭️foldable/☀️light.svg) 0 0, pointer",
        (SemioCursor::Foldable, true) => "url(/🖼️assets/👆️cursor/🪭️foldable/🌙️dark.svg) 0 0, pointer",
        (SemioCursor::Grab, false) => "url(/🖼️assets/👆️cursor/✋️grab/☀️light.svg) 0 0, grab",
        (SemioCursor::Grab, true) => "url(/🖼️assets/👆️cursor/✋️grab/🌙️dark.svg) 0 0, grab",
        (SemioCursor::Grabbing, _) => "url(/🖼️assets/👆️cursor/✊️grabbing/☀️light.svg) 0 0, grabbing",
        (SemioCursor::Text, _) => "text",
        (SemioCursor::EwResize, false) => "url(/🖼️assets/👆️cursor/↔️ew-resize/☀️light.svg) 16 2, ew-resize",
        (SemioCursor::EwResize, true) => "url(/🖼️assets/👆️cursor/↔️ew-resize/🌙️dark.svg) 16 2, ew-resize",
        (SemioCursor::NsResize, false) => "url(/🖼️assets/👆️cursor/↕️ns-resize/☀️light.svg) 2 16, ns-resize",
        (SemioCursor::NsResize, true) => "url(/🖼️assets/👆️cursor/↕️ns-resize/🌙️dark.svg) 2 16, ns-resize",
        (SemioCursor::NwseResize, false) => "url(/🖼️assets/👆️cursor/↘️nwse-resize/☀️light.svg) 16 16, nwse-resize",
        (SemioCursor::NwseResize, true) => "url(/🖼️assets/👆️cursor/↘️nwse-resize/🌙️dark.svg) 16 16, nwse-resize",
        (SemioCursor::NeswResize, false) => "url(/🖼️assets/👆️cursor/↗️nesw-resize/🌙️dark.svg) 16 16, nesw-resize",
        (SemioCursor::NeswResize, true) => "url(/🖼️assets/👆️cursor/↗️nesw-resize/🌙️dark.svg) 16 16, nesw-resize",
        (SemioCursor::Move, false) => "url(/🖼️assets/👆️cursor/🧭️move/✏️light-source.svg) 16 16, move",
        (SemioCursor::Move, true) => "url(/🖼️assets/👆️cursor/🧭️move/🌙️dark.svg) 16 16, move",
        (SemioCursor::Crosshair, false) => "url(/🖼️assets/👆️cursor/🎯️crosshair/☀️light.svg) 16 16, crosshair",
        (SemioCursor::Crosshair, true) => "url(/🖼️assets/👆️cursor/🎯️crosshair/🌙️dark.svg) 16 16, crosshair",
        (SemioCursor::NotAllowed, false) => "url(/🖼️assets/👆️cursor/🚫️not-allowed/☀️light.svg) 0 0, not-allowed",
        (SemioCursor::NotAllowed, true) => "url(/🖼️assets/👆️cursor/🚫️not-allowed/🌙️dark.svg) 0 0, not-allowed",
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub fn apply_canvas_cursor(canvas: &web_sys::HtmlCanvasElement, cursor: SemioCursor, dark: bool, last: &mut Option<(SemioCursor, bool)>) {
    use wasm_bindgen::JsCast;
    let key = (cursor, dark);
    if last.as_ref() == Some(&key) {
        return;
    }
    *last = Some(key);
    let css = semio_cursor_css(cursor, dark);
    if let Some(element) = canvas.dyn_ref::<web_sys::HtmlElement>() {
        let _ = element.style().set_property("cursor", css);
    }
}

#[cfg(not(target_os = "wasi"))]
pub fn apply_window_cursor(window: &winit::window::Window, cursor: SemioCursor, dark: bool, last: &mut Option<(SemioCursor, bool)>) {
    let key = (cursor, dark);
    if last.as_ref() == Some(&key) {
        return;
    }
    *last = Some(key);
    let _ = dark;
    window.set_cursor(winit_cursor_icon(cursor));
}

#[cfg(not(target_os = "wasi"))]
fn winit_cursor_icon(cursor: SemioCursor) -> winit::window::CursorIcon {
    use winit::window::CursorIcon;
    match cursor {
        SemioCursor::Default => CursorIcon::Default,
        SemioCursor::Pointer | SemioCursor::Selectable | SemioCursor::Foldable => CursorIcon::Pointer,
        SemioCursor::Grab => CursorIcon::Grab,
        SemioCursor::Grabbing => CursorIcon::Grabbing,
        SemioCursor::Text => CursorIcon::Text,
        SemioCursor::EwResize => CursorIcon::EwResize,
        SemioCursor::NsResize => CursorIcon::NsResize,
        SemioCursor::NwseResize => CursorIcon::NwseResize,
        SemioCursor::NeswResize => CursorIcon::NeswResize,
        SemioCursor::Move => CursorIcon::Move,
        SemioCursor::Crosshair => CursorIcon::Crosshair,
        SemioCursor::NotAllowed => CursorIcon::NotAllowed,
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/🔬️targets-wgpu-cursor-unit/🦀️.rs"]
mod tests;
// #endregion cursor
